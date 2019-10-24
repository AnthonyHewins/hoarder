use chrono::NaiveDate;

use diesel::prelude::*;
use diesel::result::Error as DbError;

use crate::models::{TopLevelUpsert, sw_report::SwReport};

use line::Line;
use upsert_memo::UpsertMemo;
use self::error::SwError;

pub mod line;
pub mod error;
mod upsert_memo;

static COLUMNS: [&str; 6] = ["domain", "publisher", "owner", "hostname", "program/version", "program/computer combination OR install date OR install path"];

pub fn upsert_bytes<'a>(
    conn: &PgConnection,
    buf: &'a mut [u8],
    report_date: NaiveDate
) -> Vec<SwError> {
    upsert(conn, Line::from_bytes(buf), report_date)
}

pub fn upsert(
    conn: &PgConnection,
    lines: Vec::<Result<Line, SwError>>,
    report_date: NaiveDate
) -> Vec<SwError> {
    let mut memo = match SwReport::upsert(conn, report_date) {
        Ok(id) => UpsertMemo::new(lines.len(), conn, id),
        Err(e) => return vec![ SwError::MetadataError { err: e.to_string() } ]
    };

    let (mut col_ptr, mut lineno) = (0,0);

    lines.into_iter().filter_map(|wrapped_line| {
        lineno += 1;

        let line = match wrapped_line {
            Ok(l) => l,
            Err(e) => return Some(e)
        };

        let transaction_result = conn.transaction::<(), DbError, _>(|| {
            let domain_id = memo.upsert_domain(&line.domain)?;
            col_ptr += 1;

            let publisher_id = memo.upsert_publisher(&line.publisher)?;
            col_ptr += 1;

            let employee_id = memo.upsert_employee(line.owner.as_ref())?;
            col_ptr += 1;

            let machine_id = memo.upsert_machine(domain_id, &line.hostname, employee_id)?;
            col_ptr += 1;

            let program_id = memo.upsert_program(publisher_id, &line.program, &line.version)?;
            col_ptr += 1;

            memo.upsert_machines_programs(machine_id, program_id, memo.sw_report_id, line.path.as_ref())?;
            Ok(())
        });

        match transaction_result {
            Ok(()) => {
                col_ptr = 0;
                None
            },
            Err(e) => {
                invalidate_cache(&mut memo, &line);
                let e = SwError::LineError {
                    lineno: lineno,
                    col: COLUMNS[col_ptr].to_string(),
                    line: line,
                    err: e.to_string()
                };
                col_ptr = 0;
                Some(e)
            }
        }
    }).collect::<Vec::<SwError>>()
}

fn invalidate_cache(memo: &mut UpsertMemo, line: &Line) {
    memo.domain_memo.remove(&line.domain);
    memo.machine_memo.remove(&line.hostname);
    memo.program_memo.remove(&line.program);

    line.publisher.as_ref().map(|s| memo.publisher_memo.remove(s));
    line.owner.as_ref().map(|s| memo.employee_memo.remove(s));
}
