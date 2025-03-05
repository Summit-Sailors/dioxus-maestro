-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS vector;
CREATE TYPE user_role AS ENUM('Admin', 'Moderator', 'User');
CREATE TABLE IF NOT EXISTS
  users (
    id UUID DEFAULT uuid_generate_v4 () PRIMARY KEY,
    username TEXT NOT NULL,
    email TEXT NOT NULL,
    bio TEXT NULL,
    age INTEGER NULL,
    ROLE user_role NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
  );