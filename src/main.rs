#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate chrono;

use std::path::PathBuf;
use structopt::StructOpt;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod line;
pub mod models;
pub mod controller;

#[derive(StructOpt)]
#[structopt(name = "Hoarder", about = "Analyze all software on machines")]
struct Cli {
    /// File location
    #[structopt(help = "Points to the SolarWinds file")]
    file: PathBuf,
}

pub fn connect() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url).expect(
        &format!("Error connecting to {}", database_url)
    )
}

fn main() -> Result<(), std::io::Error> {
    let args = Cli::from_args();

    let mut linevec = line::Line::from_file(args.file);
    let conn = connect();
    controller::upsert(&mut linevec, &conn).iter().for_each(|error| {
        println!("{:?}", error);
    });

    Ok(())
}
