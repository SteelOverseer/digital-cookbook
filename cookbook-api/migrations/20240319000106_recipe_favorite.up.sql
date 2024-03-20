-- Add favorites column to recipe
Alter table recipes add is_favorite boolean not null DEFAULT false