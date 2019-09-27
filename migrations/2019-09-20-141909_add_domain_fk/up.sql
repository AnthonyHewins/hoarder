-- Your SQL goes here
alter table machines add column domain_id integer;
ALTER TABLE machines ADD CONSTRAINT machines_domain FOREIGN KEY (domain_id) REFERENCES domains (id) MATCH FULL;
