-- Create tags table
Create table tags (
    id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    name Text NOT NULL UNIQUE
)