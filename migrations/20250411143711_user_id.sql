-- Add migration script here
-- Make a new id primary key to users so usernames can be edited
ALTER TABLE users RENAME TO users_copy;

CREATE TABLE users(
    id integer primary key autoincrement not null,
    username text unique not null,
    password text not null,
    terminate datetime
);

INSERT INTO users (username, password, terminate)
    select username, password, terminate from users_copy;

-- PERMISSIONS
-- Change the username foreign key reference to the new id in users
ALTER TABLE permissions RENAME TO permissions_copy;

CREATE TABLE permissions (
    id integer primary key autoincrement not null,
    manage_users boolean not null,
    upload_files boolean not null,
    list_files boolean not null,
    delete_files boolean not null
);

INSERT INTO permissions (manage_users, upload_files, list_files, delete_files)
    select permissions_copy.manage_users, permissions_copy.upload_files, permissions_copy.list_files, permissions_copy.delete_files from permissions_copy;

DROP TABLE permissions_copy;
ALTER TABLE permissions RENAME TO permissions_copy;

CREATE TABLE permissions (
    id integer primary key not null references users(id),
    manage_users boolean not null,
    upload_files boolean not null,
    list_files boolean not null,
    delete_files boolean not null
);

INSERT INTO permissions select * from permissions_copy;

DROP TABLE permissions_copy;
DROP TABLE users_copy;
