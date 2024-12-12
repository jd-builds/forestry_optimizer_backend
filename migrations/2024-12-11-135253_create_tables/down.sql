-- Drop all tables in reverse order of creation to handle foreign key constraints

-- Drop refresh_tokens table and its indices
DROP TABLE IF EXISTS "refresh_tokens" CASCADE;

-- Drop password_reset_tokens table and its indices
DROP TABLE IF EXISTS "password_reset_tokens" CASCADE;

-- Drop email_verification_tokens table and its indices
DROP TABLE IF EXISTS "email_verification_tokens" CASCADE;

-- Drop users table and its indices
DROP TABLE IF EXISTS "users" CASCADE;

-- Drop organizations table and its indices
DROP TABLE IF EXISTS "organizations" CASCADE;

-- Drop user_role enum type
DROP TYPE IF EXISTS user_role CASCADE;

-- Drop diesel helper functions
DROP FUNCTION IF EXISTS diesel_manage_updated_at() CASCADE;
DROP FUNCTION IF EXISTS diesel_set_updated_at() CASCADE; 