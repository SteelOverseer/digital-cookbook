-- Create instructions table
Create table instructions (
    id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    recipe_id UUID NOT NULL REFERENCES recipes (id),
    instruction_text Text NOT NULL
)
