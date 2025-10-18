-- DevERP Database Setup Script
-- Run this script as PostgreSQL superuser (postgres)

-- Create database
CREATE DATABASE deverp;

-- Connect to the deverp database (you'll need to run \c deverp in psql)

-- Create user
CREATE USER deverp_user WITH PASSWORD '2147483647';

-- Grant privileges
GRANT ALL PRIVILEGES ON DATABASE deverp TO deverp_user;

-- Connect to deverp database and grant schema privileges
\c deverp

-- Grant schema usage
GRANT USAGE ON SCHEMA public TO deverp_user;
GRANT CREATE ON SCHEMA public TO deverp_user;

-- Grant table privileges
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA public TO deverp_user;
GRANT USAGE, SELECT ON ALL SEQUENCES IN SCHEMA public TO deverp_user;

-- Set default privileges for future tables
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO deverp_user;
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT USAGE, SELECT ON SEQUENCES TO deverp_user;

-- Enable required extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";
