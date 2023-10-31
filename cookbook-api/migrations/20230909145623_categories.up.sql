CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
-- Create categories table
Create table categories(
    id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    name Text NOT NULL UNIQUE
)