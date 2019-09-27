-- Your SQL goes here
alter table domains add unique("name");

alter table machines add unique("host");

alter table publishers add unique("name");

alter table programs add unique("name", version);

alter table users_programs add unique(machine_id, program_id);
