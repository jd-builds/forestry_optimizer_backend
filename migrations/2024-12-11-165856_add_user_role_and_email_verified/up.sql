-- Your SQL goes here

-- Create user role enum type if it doesn't exist
DO $$ BEGIN
    CREATE TYPE user_role AS ENUM ('Admin', 'Manager', 'Operator');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- Add role and email_verified columns to users table if they don't exist
DO $$ BEGIN
    ALTER TABLE users ADD COLUMN role user_role NOT NULL DEFAULT 'Operator';
EXCEPTION
    WHEN duplicate_column THEN null;
END $$;

DO $$ BEGIN
    ALTER TABLE users ADD COLUMN email_verified BOOLEAN NOT NULL DEFAULT false;
EXCEPTION
    WHEN duplicate_column THEN null;
END $$;
