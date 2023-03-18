-- Add migration script here
create table users (
    username text primary key not null,
    password text not null,
    terminate datetime
);

create table permissions (
    username text primary key not null references users(username),
    create_users boolean not null,
    upload_files boolean not null,
    list_files boolean not null,
    delete_files boolean not null
);