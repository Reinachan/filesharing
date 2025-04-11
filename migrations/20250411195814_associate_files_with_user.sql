-- Add migration script here
ALTER TABLE files RENAME TO files_copy;

create table files (
    saved_name text primary key not null,
    file_name text not null,
    file_type text not null,
    password text,
    destroy datetime,
    user_id integer not null references users(id) default 1,
    created_at datetime
);

INSERT INTO files (saved_name, file_name, file_type, password, destroy) select * from files_copy;
UPDATE files 
SET created_at = current_timestamp;


DROP TABLE files_copy;