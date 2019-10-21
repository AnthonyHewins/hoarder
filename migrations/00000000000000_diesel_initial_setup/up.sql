-- This file was automatically created by Diesel to setup helper functions
-- and other internal bookkeeping. This file is safe to edit, any future
-- changes will be added to existing projects as new migrations.

-- Sets up a trigger for the given table to automatically set a column called
-- `updated_at` whenever the row is modified (unless `updated_at` was included
-- in the modified columns)
--
-- # Example
--
-- ```sql
-- CREATE TABLE users (id BIGSERIAL PRIMARY KEY, updated_at TIMESTAMP NOT NULL DEFAULT NOW());
--
-- SELECT diesel_manage_updated_at('users');
-- ```
CREATE OR REPLACE FUNCTION diesel_manage_updated_at(_tbl regclass) RETURNS VOID AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_set_updated_at()', _tbl);
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION diesel_set_updated_at() RETURNS trigger AS $$
BEGIN
    IF (
        NEW IS DISTINCT FROM OLD AND
        NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at
    ) THEN
        NEW.updated_at := current_timestamp;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

------------------------------------------------------------------------------------
-- Schema definitions
------------------------------------------------------------------------------------
create table sw_reports(
       id bigserial primary key,
       generation_date date not null
);

create table domains(
       id bigserial primary key,
       name varchar(200) not null unique
);

create table publishers(
       id bigserial primary key,
       name varchar(200) not null unique
);

create table employees(
       id bigserial primary key,
       name varchar(200) not null unique
);

create table machines(
       id bigserial primary key,
       domain_id bigint references domains(id) on delete set null,
       employee_id bigint references employees(id) on delete set null,
       host varchar(200) not null unique
);

create table programs(
       id bigserial primary key,
       publisher_id bigint references publishers(id) on delete set null,
       
       name varchar(260) not null unique,
       version varchar(100)
);

create table machines_programs(
       id bigserial primary key,
       machine_id bigint not null references machines(id) on delete cascade,
       program_id bigint not null references programs(id) on delete cascade,
       sw_report_id bigint not null references sw_reports(id) on delete cascade,
       path varchar(260),
       constraint one_installation_per_machine unique (machine_id, program_id)
);

