use crate::{
    common::{error::{ApiError, ErrorCode, Result}, pagination::PaginationParams},
    domain::{
        models::user::{Role, User},
        repositories::{base::Repository, UserRepository},
    },
    infrastructure::database::schema::public::users,
};
use async_trait::async_trait;
use chrono::Utc;
use diesel::prelude::*;
use diesel::PgConnection;
use std::sync::Arc;
use tracing::error;
use uuid::Uuid;

#[derive(Queryable, Insertable, AsChangeset)]
#[diesel(table_name = users)]
struct UserModel {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_number: String,
    pub password: String,
    pub is_supervisor: bool,
    pub org_id: Uuid,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
    pub deleted_at: Option<chrono::DateTime<Utc>>,
    pub role: Role,
    pub email_verified: bool,
}

impl From<&User> for UserModel {
    fn from(user: &User) -> Self {
        Self {
            id: user.id,
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
            email: user.email.clone(),
            phone_number: user.phone_number.clone(),
            password: user.password.clone(),
            is_supervisor: user.is_supervisor,
            org_id: user.org_id,
            created_at: user.created_at,
            updated_at: user.updated_at,
            deleted_at: user.deleted_at,
            role: user.role,
            email_verified: user.email_verified,
        }
    }
}

impl From<UserModel> for User {
    fn from(model: UserModel) -> Self {
        Self {
            id: model.id,
            first_name: model.first_name,
            last_name: model.last_name,
            email: model.email,
            phone_number: model.phone_number,
            password: model.password,
            is_supervisor: model.is_supervisor,
            org_id: model.org_id,
            created_at: model.created_at,
            updated_at: model.updated_at,
            deleted_at: model.deleted_at,
            role: model.role,
            email_verified: model.email_verified,
        }
    }
}

pub struct AuthRepositoryImpl {
    pool: Arc<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>>,
}

impl AuthRepositoryImpl {
    pub fn new(pool: Arc<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>>) -> Self {
        Self { pool }
    }

    fn get_conn(&self) -> Result<diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>>> {
        self.pool.get().map_err(|e| {
            error!(error = %e, "Failed to get database connection");
            ApiError::new(
                ErrorCode::DatabaseError,
                "Failed to get database connection",
                Default::default(),
            )
        })
    }
}

#[async_trait]
impl Repository<User> for AuthRepositoryImpl {
    async fn create(&self, entity: &User) -> Result<User> {
        let mut conn = self.get_conn()?;
        let user_model = UserModel::from(entity);
        
        diesel::insert_into(users::table)
            .values(&user_model)
            .get_result::<UserModel>(&mut conn)
            .map(Into::into)
            .map_err(|e| {
                error!(error = %e, "Failed to create user");
                ApiError::new(
                    ErrorCode::DatabaseError,
                    "Failed to create user",
                    Default::default(),
                )
            })
    }

    async fn find_by_id(&self, id: Uuid) -> Result<User> {
        let mut conn = self.get_conn()?;
        users::table
            .find(id)
            .filter(users::deleted_at.is_null())
            .first::<UserModel>(&mut conn)
            .map(Into::into)
            .map_err(|e| {
                error!(error = %e, "Failed to find user");
                ApiError::new(
                    ErrorCode::NotFound,
                    "User not found",
                    Default::default(),
                )
            })
    }

    async fn update(&self, id: Uuid, entity: &User) -> Result<User> {
        let mut conn = self.get_conn()?;
        let user_model = UserModel::from(entity);

        diesel::update(users::table.find(id))
            .set(&user_model)
            .get_result::<UserModel>(&mut conn)
            .map(Into::into)
            .map_err(|e| {
                error!(error = %e, "Failed to update user");
                ApiError::new(
                    ErrorCode::DatabaseError,
                    "Failed to update user",
                    Default::default(),
                )
            })
    }

    async fn soft_delete(&self, id: Uuid) -> Result<()> {
        let mut conn = self.get_conn()?;
        diesel::update(users::table.find(id))
            .set(users::deleted_at.eq(Some(Utc::now())))
            .execute(&mut conn)
            .map_err(|e| {
                error!(error = %e, "Failed to soft delete user");
                ApiError::new(ErrorCode::DatabaseError, "Failed to soft delete user", Default::default())
            })?;
        Ok(())
    }

    async fn list(&self, pagination: &PaginationParams) -> Result<(Vec<User>, i64)> {
        let mut conn = self.get_conn()?;
        let users = users::table
            .filter(users::deleted_at.is_null())
            .offset(pagination.offset as i64)
            .limit(pagination.limit as i64)
            .load::<UserModel>(&mut conn)
            .map(|users| users.into_iter().map(Into::into).collect())
            .map_err(|e| {
                error!(error = %e, "Failed to list users");
                ApiError::new(ErrorCode::DatabaseError, "Failed to list users", Default::default())
            })?;

        let total = users::table
            .count()
            .get_result(&mut conn)
            .map_err(|e| {
                error!(error = %e, "Failed to count users");
                ApiError::new(ErrorCode::DatabaseError, "Failed to count users", Default::default())
            })?;

        Ok((users, total))
    }
}

#[async_trait]
impl UserRepository for AuthRepositoryImpl {
    async fn find_by_email(&self, conn: &mut PgConnection, email: &str) -> Result<Option<User>> {
        users::table
            .filter(users::email.eq(email))
            .filter(users::deleted_at.is_null())
            .first::<UserModel>(conn)
            .optional()
            .map(|opt| opt.map(Into::into))
            .map_err(|e| {
                error!(error = %e, "Failed to find user by email");
                ApiError::new(
                    ErrorCode::DatabaseError,
                    "Failed to find user by email",
                    Default::default(),
                )
            })
    }

    async fn find_by_phone_number(&self, conn: &mut PgConnection, phone: &str) -> Result<Option<User>> {
        users::table
            .filter(users::phone_number.eq(phone))
            .filter(users::deleted_at.is_null())
            .first::<UserModel>(conn)
            .optional()
            .map(|opt| opt.map(Into::into))
            .map_err(|e| {
                error!(error = %e, "Failed to find user by phone");
                ApiError::new(
                    ErrorCode::DatabaseError,
                    "Failed to find user by phone",
                    Default::default(),
                )
            })
    }

    async fn create_with_password(&self, conn: &mut PgConnection, params: crate::domain::repositories::user::CreateUserParams<'_>) -> Result<User> {
        let user = User {
            id: Uuid::new_v4(),
            first_name: params.first_name.to_string(),
            last_name: params.last_name.to_string(),
            email: params.email.to_string(),
            phone_number: params.phone_number.to_string(),
            password: params.password.to_string(),
            is_supervisor: false,
            org_id: params.org_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
            role: Role::Operator,
            email_verified: false,
        };

        let user_model = UserModel::from(&user);
        
        diesel::insert_into(users::table)
            .values(&user_model)
            .get_result::<UserModel>(conn)
            .map(Into::into)
            .map_err(|e| {
                error!(error = %e, "Failed to create user with password");
                ApiError::new(
                    ErrorCode::DatabaseError,
                    "Failed to create user",
                    Default::default(),
                )
            })
    }

    async fn update_password(&self, id: Uuid, new_password: String) -> Result<User> {
        let mut conn = self.get_conn()?;
        diesel::update(users::table.find(id))
            .set((
                users::password.eq(new_password),
                users::updated_at.eq(Utc::now()),
            ))
            .get_result::<UserModel>(&mut conn)
            .map(Into::into)
            .map_err(|e| {
                error!(error = %e, "Failed to update password");
                ApiError::new(ErrorCode::DatabaseError, "Failed to update password", Default::default())
            })
    }

    async fn update_role(&self, id: Uuid, new_role: Role) -> Result<User> {
        let mut conn = self.get_conn()?;
        diesel::update(users::table.find(id))
            .set((
                users::role.eq(new_role),
                users::updated_at.eq(Utc::now()),
            ))
            .get_result::<UserModel>(&mut conn)
            .map(Into::into)
            .map_err(|e| {
                error!(error = %e, "Failed to update role");
                ApiError::new(ErrorCode::DatabaseError, "Failed to update role", Default::default())
            })
    }
} 