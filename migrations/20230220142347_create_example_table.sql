-- Add migration script here
CREATE TABLE example(
    id uuid NOT NULL PRIMARY KEY,
    random_field TEXT NOT NULL UNIQUE,
    created_at timestamptz NOT NULL
)