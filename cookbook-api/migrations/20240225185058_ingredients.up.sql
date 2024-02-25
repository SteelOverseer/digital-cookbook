-- Create ingredients table
Create table ingredients (
    id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    recipe_id UUID NOT NULL REFERENCES recipes (id),
    ingredient_text Text NOT NULL
);

alter table recipes drop column ingredients