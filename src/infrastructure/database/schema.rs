// @generated automatically by Diesel CLI.

pub mod public {
    pub mod sql_types {
        #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
        #[diesel(postgres_type(name = "user_role"))]
        pub struct UserRole;
    }

    diesel::table! {
        block_tree_species (id) {
            id -> Uuid,
            volume -> Int4,
            tree_species_id -> Uuid,
            block_id -> Uuid,
            job_id -> Uuid,
            created_at -> Timestamptz,
            updated_at -> Timestamptz,
            deleted_at -> Nullable<Timestamptz>,
        }
    }

    diesel::table! {
        blocks (id) {
            id -> Uuid,
            external_block_id -> Int4,
            net_merch -> Int4,
            volume_per_tree -> Float8,
            slope -> Int4,
            area -> Float8,
            job_id -> Uuid,
            org_id -> Uuid,
            created_at -> Timestamptz,
            updated_at -> Timestamptz,
            deleted_at -> Nullable<Timestamptz>,
        }
    }

    diesel::table! {
        customer_machine_rates (id) {
            id -> Uuid,
            hourly_rate -> Int4,
            machine_id -> Uuid,
            customer_id -> Uuid,
            org_id -> Uuid,
            created_at -> Timestamptz,
            updated_at -> Timestamptz,
            deleted_at -> Nullable<Timestamptz>,
        }
    }

    diesel::table! {
        customers (id) {
            id -> Uuid,
            #[max_length = 255]
            name -> Varchar,
            org_id -> Uuid,
            created_at -> Timestamptz,
            updated_at -> Timestamptz,
            deleted_at -> Nullable<Timestamptz>,
        }
    }

    diesel::table! {
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
        hours (id) {
            id -> Uuid,
            #[sql_name = "type"]
            #[max_length = 255]
            type_ -> Varchar,
            start_time -> Timestamptz,
            end_time -> Timestamptz,
            #[max_length = 255]
            notes -> Varchar,
            user_id -> Uuid,
            job_id -> Uuid,
            block_id -> Uuid,
            phase_allocation_id -> Uuid,
            machine_id -> Nullable<Uuid>,
            org_id -> Uuid,
            created_at -> Timestamptz,
            updated_at -> Timestamptz,
            deleted_at -> Nullable<Timestamptz>,
        }
    }

    diesel::table! {
        job_users (id) {
            id -> Uuid,
            #[max_length = 255]
            role -> Varchar,
            job_id -> Uuid,
            org_id -> Uuid,
            user_id -> Uuid,
            created_at -> Timestamptz,
            updated_at -> Timestamptz,
            deleted_at -> Nullable<Timestamptz>,
        }
    }

    diesel::table! {
        jobs (id) {
            id -> Uuid,
            #[max_length = 255]
            status -> Varchar,
            #[max_length = 255]
            timbermark -> Varchar,
            #[max_length = 255]
            area -> Varchar,
            customer_id -> Uuid,
            org_id -> Uuid,
            created_at -> Timestamptz,
            updated_at -> Timestamptz,
            deleted_at -> Nullable<Timestamptz>,
        }
    }

    diesel::table! {
        machines (id) {
            id -> Uuid,
            #[max_length = 255]
            name -> Varchar,
            org_id -> Uuid,
            created_at -> Timestamptz,
            updated_at -> Timestamptz,
            deleted_at -> Nullable<Timestamptz>,
        }
    }

    diesel::table! {
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
        phase_allocations (id) {
            id -> Uuid,
            #[max_length = 255]
            status -> Varchar,
            phase_id -> Uuid,
            block_id -> Uuid,
            job_id -> Uuid,
            machine_id -> Uuid,
            customer_id -> Uuid,
            org_id -> Uuid,
            created_at -> Timestamptz,
            updated_at -> Timestamptz,
            deleted_at -> Nullable<Timestamptz>,
        }
    }

    diesel::table! {
        phases (id) {
            id -> Uuid,
            #[max_length = 255]
            name -> Varchar,
            administration_deduction -> Int4,
            production_basis -> Int4,
            job_id -> Uuid,
            block_id -> Uuid,
            org_id -> Uuid,
            created_at -> Timestamptz,
            updated_at -> Timestamptz,
            deleted_at -> Nullable<Timestamptz>,
        }
    }

    diesel::table! {
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
        tree_species (id) {
            id -> Uuid,
            #[max_length = 255]
            name -> Varchar,
            org_id -> Uuid,
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
            is_supervisor -> Bool,
            org_id -> Uuid,
            created_at -> Timestamptz,
            updated_at -> Timestamptz,
            deleted_at -> Nullable<Timestamptz>,
            role -> UserRole,
            email_verified -> Bool,
        }
    }

    diesel::joinable!(block_tree_species -> blocks (block_id));
    diesel::joinable!(block_tree_species -> jobs (job_id));
    diesel::joinable!(block_tree_species -> tree_species (tree_species_id));
    diesel::joinable!(blocks -> jobs (job_id));
    diesel::joinable!(blocks -> organizations (org_id));
    diesel::joinable!(customer_machine_rates -> customers (customer_id));
    diesel::joinable!(customer_machine_rates -> machines (machine_id));
    diesel::joinable!(customer_machine_rates -> organizations (org_id));
    diesel::joinable!(customers -> organizations (org_id));
    diesel::joinable!(email_verification_tokens -> users (user_id));
    diesel::joinable!(hours -> blocks (block_id));
    diesel::joinable!(hours -> jobs (job_id));
    diesel::joinable!(hours -> machines (machine_id));
    diesel::joinable!(hours -> organizations (org_id));
    diesel::joinable!(hours -> phase_allocations (phase_allocation_id));
    diesel::joinable!(hours -> users (user_id));
    diesel::joinable!(job_users -> organizations (org_id));
    diesel::joinable!(job_users -> users (user_id));
    diesel::joinable!(jobs -> customers (customer_id));
    diesel::joinable!(jobs -> organizations (org_id));
    diesel::joinable!(machines -> organizations (org_id));
    diesel::joinable!(password_reset_tokens -> users (user_id));
    diesel::joinable!(phase_allocations -> blocks (block_id));
    diesel::joinable!(phase_allocations -> customers (customer_id));
    diesel::joinable!(phase_allocations -> jobs (job_id));
    diesel::joinable!(phase_allocations -> machines (machine_id));
    diesel::joinable!(phase_allocations -> organizations (org_id));
    diesel::joinable!(phase_allocations -> phases (phase_id));
    diesel::joinable!(phases -> blocks (block_id));
    diesel::joinable!(phases -> jobs (job_id));
    diesel::joinable!(phases -> organizations (org_id));
    diesel::joinable!(refresh_tokens -> users (user_id));
    diesel::joinable!(tree_species -> organizations (org_id));
    diesel::joinable!(users -> organizations (org_id));

    diesel::allow_tables_to_appear_in_same_query!(
        block_tree_species,
        blocks,
        customer_machine_rates,
        customers,
        email_verification_tokens,
        hours,
        job_users,
        jobs,
        machines,
        organizations,
        password_reset_tokens,
        phase_allocations,
        phases,
        refresh_tokens,
        tree_species,
        users,
    );
}
