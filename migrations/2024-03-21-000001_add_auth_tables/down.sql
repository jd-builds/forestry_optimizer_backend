-- Drop email verification tokens
DROP TABLE IF EXISTS "email_verification_tokens";

-- Drop password reset tokens
DROP TABLE IF EXISTS "password_reset_tokens";

-- Drop refresh tokens
DROP TABLE IF EXISTS "refresh_tokens";

-- Remove email_verified column from users
ALTER TABLE "users" DROP COLUMN IF EXISTS "email_verified";

-- Remove role column and enum from users
ALTER TABLE "users" DROP COLUMN IF EXISTS "role";
DROP TYPE IF EXISTS user_role; 