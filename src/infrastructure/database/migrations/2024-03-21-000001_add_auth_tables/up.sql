-- Add refresh_tokens table for JWT management
CREATE TABLE "refresh_tokens" (
    "id" UUID NOT NULL,
    "token" VARCHAR(255) NOT NULL,
    "user_id" UUID NOT NULL,
    "expires_at" TIMESTAMP WITH TIME ZONE NOT NULL,
    "created_at" TIMESTAMP WITH TIME ZONE NOT NULL,
    "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL,
    "deleted_at" TIMESTAMP WITH TIME ZONE NULL
);

ALTER TABLE "refresh_tokens" ADD PRIMARY KEY("id");
ALTER TABLE "refresh_tokens" ADD CONSTRAINT "refresh_tokens_token_unique" UNIQUE("token");
CREATE INDEX "refresh_tokens_user_id_index" ON "refresh_tokens"("user_id");
ALTER TABLE "refresh_tokens" ADD CONSTRAINT "refresh_tokens_user_id_foreign" FOREIGN KEY("user_id") REFERENCES "users"("id");

-- Add role enum and role column to users
CREATE TYPE user_role AS ENUM ('admin', 'manager', 'operator');
ALTER TABLE "users" ADD COLUMN "role" user_role NOT NULL DEFAULT 'operator';

-- Add password reset tokens
CREATE TABLE "password_reset_tokens" (
    "id" UUID NOT NULL,
    "token" VARCHAR(255) NOT NULL,
    "user_id" UUID NOT NULL,
    "expires_at" TIMESTAMP WITH TIME ZONE NOT NULL,
    "created_at" TIMESTAMP WITH TIME ZONE NOT NULL,
    "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL,
    "deleted_at" TIMESTAMP WITH TIME ZONE NULL
);

ALTER TABLE "password_reset_tokens" ADD PRIMARY KEY("id");
ALTER TABLE "password_reset_tokens" ADD CONSTRAINT "password_reset_tokens_token_unique" UNIQUE("token");
CREATE INDEX "password_reset_tokens_user_id_index" ON "password_reset_tokens"("user_id");
ALTER TABLE "password_reset_tokens" ADD CONSTRAINT "password_reset_tokens_user_id_foreign" FOREIGN KEY("user_id") REFERENCES "users"("id");

-- Add email verification tokens
CREATE TABLE "email_verification_tokens" (
    "id" UUID NOT NULL,
    "token" VARCHAR(255) NOT NULL,
    "user_id" UUID NOT NULL,
    "expires_at" TIMESTAMP WITH TIME ZONE NOT NULL,
    "created_at" TIMESTAMP WITH TIME ZONE NOT NULL,
    "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL,
    "deleted_at" TIMESTAMP WITH TIME ZONE NULL
);

ALTER TABLE "email_verification_tokens" ADD PRIMARY KEY("id");
ALTER TABLE "email_verification_tokens" ADD CONSTRAINT "email_verification_tokens_token_unique" UNIQUE("token");
CREATE INDEX "email_verification_tokens_user_id_index" ON "email_verification_tokens"("user_id");
ALTER TABLE "email_verification_tokens" ADD CONSTRAINT "email_verification_tokens_user_id_foreign" FOREIGN KEY("user_id") REFERENCES "users"("id");

-- Add email_verified column to users
ALTER TABLE "users" ADD COLUMN "email_verified" BOOLEAN NOT NULL DEFAULT FALSE; 