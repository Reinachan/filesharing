-- Add migration script here
create table files (
  file_name text not null primary key,
  file_type text not null,
  password text,
  destroy datetime
)