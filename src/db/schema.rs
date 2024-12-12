// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "user_role"))]
    pub struct UserRole;
}

diesel::table! {
    use diesel::sql_types::*;

    email_verification_tokens (id) {
        id -> Uuid,
        #[max_length = 255]
        token -> Varchar,
        user_id -> Uuid,
        expires_at -> Timestamptz,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    organizations (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    password_reset_tokens (id) {
        id -> Uuid,
        #[max_length = 255]
        token -> Varchar,
        user_id -> Uuid,
        expires_at -> Timestamptz,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    refresh_tokens (id) {
        id -> Uuid,
        #[max_length = 255]
        token -> Varchar,
        user_id -> Uuid,
        expires_at -> Timestamptz,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::UserRole;

    users (id) {
        id -> Uuid,
        #[max_length = 255]
        first_name -> Varchar,
        #[max_length = 255]
        last_name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        phone_number -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        role -> UserRole,
        email_verified -> Bool,
        org_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(email_verification_tokens -> users (user_id));
diesel::joinable!(password_reset_tokens -> users (user_id));
diesel::joinable!(refresh_tokens -> users (user_id));
diesel::joinable!(users -> organizations (org_id));

diesel::allow_tables_to_appear_in_same_query!(
    email_verification_tokens,
    organizations,
    password_reset_tokens,
    refresh_tokens,
    users,
);
