-- Create tag map table
Create table recipe_tag_map (
    recipe_id UUID NOT NULL REFERENCES recipes (id),
    tag_id UUID NOT NULL REFERENCES tags (id)
)