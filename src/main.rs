#[allow(unused_variables)]

use clap::{Parser, Subcommand};
//use std::fs;
use rusqlite::{params, Connection, Result};

mod parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}
struct pkgtoin {
    name: String,
    tarball: String,
}

#[derive(Subcommand)]
enum Commands {
    /// install a package
    Install { package: Option<String> },
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Install { package } => {
            //let mut a = || -> Result<()> {
                //let pak = package.unwrap();
                //let conn = Connection::open("src/packageLDB.db")?;
                //conn.execute(
                    //"SELECT * FROM package WHERE name LIKE '%?1%'",
                    //params![package]
                //);
            
                //Ok(())
            //};
            //a();
            search();
        }
}
}

fn search() -> Result<()> {
    let cron = Connection::open("src/packageLDB.db");
    let mut stmt = cron.prepare("SELECT name,tarball FROM package WHERE name LIKE '%(?1)%'")?;
    params!(String::from("cpp"));
    let ballz = stmt.query_map([], |row| {
        Ok(pkgtoin {
            name: row.get(0)?,
            tarball: row.get(1)?,
            })
        })?;
    for pkgtoin in ballz {
        println!("based");
    }
    Ok(())
}
