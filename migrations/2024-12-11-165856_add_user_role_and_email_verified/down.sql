-- This file should undo anything in `up.sql`

-- Remove role and email_verified columns from users table
ALTER TABLE users
DROP COLUMN role,
DROP COLUMN email_verified;

-- Drop user role enum type
DROP TYPE user_role;
