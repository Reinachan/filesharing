-- Add migration script here
create table files (
    saved_name text primary key not null,
    file_name text not null,
    file_type text not null,
    password text,
    destroy datetime
);