-- Create recipes table
Create table recipes(
    id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    category_id UUID NOT NULL REFERENCES categories (id),
    name Text NOT NULL UNIQUE,
    notes Text,
    ingredients Text
)
