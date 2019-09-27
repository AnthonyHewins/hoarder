use std::collections::HashMap;
use std::path::PathBuf;

use chrono::NaiveDate;

use diesel::prelude::*;
use diesel::insert_into;
use diesel::result::Error as DbError;

use super::line::Line;
use super::schema;

struct UpsertMemo {
    pub domain_memo: HashMap<String, i32>,
    pub machine_memo: HashMap<String, i32>,
    pub publisher_memo: HashMap<String, i32>,
    pub program_memo: HashMap<String, i32>,
    pub users_programs_memo: HashMap::<(i32, i32), i32>,
}

impl UpsertMemo {
    fn new(capacity: usize) -> UpsertMemo {
        UpsertMemo {
            domain_memo: HashMap::<String, i32>::with_capacity(capacity),
            machine_memo: HashMap::<String, i32>::with_capacity(capacity),
            publisher_memo: HashMap::<String, i32>::with_capacity(capacity),
            program_memo: HashMap::<String, i32>::with_capacity(capacity),
            users_programs_memo: HashMap::<(i32, i32), i32>::with_capacity(capacity),
        }
    }

    fn upsert_domain(&mut self, arg: &String, conn: &PgConnection) -> Result<i32, DbError> {
        use schema::domains::dsl::*;

        match self.domain_memo.get(arg) {
            Some(domains_id) => Ok(*domains_id),
            None => {
                let sql_call = insert_into(domains)
                    .values( name.eq(arg) )
                    .on_conflict_do_nothing()
                    .returning(id)
                    .get_result(conn);

                let new_id = match sql_call {
                    Ok(new_id) => new_id,
                    Err(e) => match e {
                        DbError::NotFound => domains.select(id).filter(name.eq(arg)).load::<i32>(conn)?[0],
                        _ => return Err(e)
                    }
                };

                match self.domain_memo.insert(arg.clone(), new_id) {
                    None => Ok(new_id),
                    Some(k) => panic!("Error memoizing argument {}, {} was in the lookup table already", arg, k)
                }
            }
        }
    }

    fn upsert_machine(&mut self, new_domain: i32, machine: &String, conn: &PgConnection) -> Result<i32, DbError> {
        use schema::machines::dsl::*;

        match self.machine_memo.get(machine) {
            Some(machines_id) => Ok(*machines_id),
            None => {
                let new_id = insert_into(machines)
                    .values(( host.eq(machine), domain_id.eq(new_domain) ))
                    .on_conflict(host)
                    .do_update()
                    .set( domain_id.eq(new_domain) )
                    .returning(id)
                    .get_result(conn)?;

                self.machine_memo.insert(machine.to_string(), new_id);
                Ok(new_id)
            }
        }
    }

    fn upsert_publisher(&mut self, arg: &String, conn: &PgConnection) -> Result<i32, DbError> {
        use schema::publishers::dsl::*;

        match self.publisher_memo.get(arg) {
            Some(publishers_id) => Ok(*publishers_id),
            None => {
                let sql_call = insert_into(publishers)
                    .values( name.eq(arg) )
                    .on_conflict_do_nothing()
                    .returning(id)
                    .get_result(conn);

                let new_id = match sql_call {
                    Ok(new_id) => new_id,
                    Err(e) => match e {
                        DbError::NotFound => {
                            let existing_id = publishers.select(id).filter(name.eq(arg)).execute(conn)?;
                            existing_id as i32
                        }
                        _ => return Err(e)
                    }
                };

                match self.publisher_memo.insert(arg.clone(), new_id) {
                    None => Ok(new_id),
                    Some(k) => panic!("Error memoizing argument {}, {} was in the lookup table already", arg, k)
                }
            }
        }
    }

    fn upsert_program(&mut self, publisher: i32, program: &String, version_str: &String, conn: &PgConnection) -> Result<i32, DbError> {
        use schema::programs::dsl::*;

        match self.program_memo.get(program) {
            Some(programs_id) => Ok(*programs_id),
            None => {
                let new_id = insert_into(programs)
                    .values(( publisher_id.eq(publisher), name.eq(program), version.eq(version_str) ))
                    .on_conflict((name, version))
                    .do_update()
                    .set( publisher_id.eq(publisher) )
                    .returning(id)
                    .get_result(conn)?;

                self.program_memo.insert(program.to_string(), new_id);
                Ok(new_id)
            }
        }
    }

    fn upsert_users_programs(&mut self, machine: i32, program: i32, install_date: &Option<NaiveDate>, path_str: &Option<PathBuf>, conn: &PgConnection) -> Result<i32, DbError> {
        use schema::users_programs::dsl::*;

        let lossy_path = match path_str {
            None => None,
            Some(p) => Some(p.to_string_lossy())
        };
        
        let join_table_tuple = (machine, program);
        match self.users_programs_memo.get(&join_table_tuple) {
            Some(programs_id) => Ok(*programs_id),
            None => {
                let new_id = insert_into(users_programs)
                    .values(( machine_id.eq(machine), program_id.eq(program), date_installed.eq(install_date), path.eq(&lossy_path) ))
                    .on_conflict(( machine_id, program_id ))
                    .do_update()
                    .set(( date_installed.eq(install_date), path.eq(&lossy_path) ))
                    .returning(id)
                    .get_result(conn)?;

                self.users_programs_memo.insert(join_table_tuple, new_id);
                Ok(new_id)
            }
        }
    }
}

pub fn upsert(lines: &mut Vec::<Line>, conn: &PgConnection) -> Vec<DbError> {
    let mut memo = UpsertMemo::new(lines.len());

    let mut errors = vec![];
    for line in lines {
        let transaction_result = conn.transaction::<(), DbError, _>(|| {
            let domain_id = memo.upsert_domain(&line.domain, conn)?;
            let machine_id = memo.upsert_machine(domain_id, &line.hostname, conn)?;

            let publisher_id = memo.upsert_publisher(&line.publisher, conn)?;
            let program_id = memo.upsert_program(publisher_id, &line.program, &line.version, conn)?;

            memo.upsert_users_programs(machine_id, program_id, &line.date_installed, &line.path, conn)?;
            Ok(())
        });

        if transaction_result.is_err() {
            errors.push(transaction_result.unwrap_err());
        }
    }
    errors
}
