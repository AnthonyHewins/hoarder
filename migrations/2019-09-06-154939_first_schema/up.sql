-- Your SQL goes here
create table domains (
       id integer primary key,
       "name" varchar(256) not null
);

create table machines (
       id integer primary key,
       "host" varchar(260) not null
);

create table publishers (
       id integer primary key,
       "name" varchar(1000) not null
);

create table programs (
       id integer primary key,
       publisher_id integer references publishers(id),

       "name" varchar(1000) not null,
       version varchar(100) not null
);

create table users_programs (
       id integer primary key,
       machine_id integer references machines(id) not null,
       program_id integer references programs(id) not null,

       date_installed timestamp,
       "path" varchar(260) -- Path cannot exceed 260 chars in windows
);
