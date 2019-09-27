drop table users_programs;
drop table machines;
drop table programs;
drop table domains;
drop table publishers;

create table domains (
       id serial primary key,
       "name" varchar(256) not null
);

create table machines (
       id serial primary key,
       "host" varchar(260) not null
);

create table publishers (
       id serial primary key,
       "name" varchar(1000) not null
);

create table programs (
       id serial primary key,
       publisher_id integer references publishers(id),

       "name" varchar(1000) not null,
       version varchar(100) not null
);

create table users_programs (
       id serial primary key,
       machine_id integer references machines(id) not null,
       program_id integer references programs(id) not null,

       date_installed timestamp,
       "path" varchar(260) -- Path cannot exceed 260 chars in windows
);
