-- Add migration script here
create table files (
    saved_name text not null primary key,
    file_name text not null,
    file_type text not null,
    password text,
    destroy datetime
)