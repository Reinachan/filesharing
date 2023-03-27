-- Add migration script here
ALTER TABLE permissions
RENAME COLUMN create_users to manage_users;