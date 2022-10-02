#[allow(unused_variables)]

use clap::{Parser, Subcommand};
use git2::Repository;
use rand::distributions::{Alphanumeric, DistString};
use std::fs;
use rusqlite::{Connection, Result};

mod config;
mod parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// install a package
    Install { package: Option<String> },
    /// remove the source code of the packages
    Remove { package: Option<String> },
}

fn main() {
    let cli = Cli::parse();
    let url = "https://github.com/RK33DV/unitytergen";
    let fldr = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
    let path = format!("RoxPaks/Packages/src/{}", fldr);
    let ver = "2".to_string();
    match &cli.command {
        Commands::Install { package } => {
            println!("Installing{:?}", package);
            let a = || -> Result<()> {
                let conn = Connection::open("src/packageLDB.db")?;
                conn.execute(
                    "INSERT INTO pkgs (version, name, path, repo_url) VALUES (?1,?2,?3,?4)",
                    (&ver, &package, &fldr, &url.to_string()),
                )?;

                Ok(())
            };

            a().expect("Impossible state, skipping...");

            let _repo = match Repository::clone(url, path) {
                Ok(repo) => repo,
                Err(e) => panic!("Installation failed : {}", e),
            };

        }
        Commands::Remove { package } => {
            println!("Removing package...");
            let path = format!("RoxPaks/Packages/src/{}", package.as_ref()
                               .unwrap());

            let mut search = || -> Result<()> {
                let conn = Connection::open("src/packageLDB.db")?;
                conn.execute(
                    "DELETE FROM pkgs
                    WHERE name = ?1;",
                    (&package,)
                )?;
            
                Ok(())
            };

            search().unwrap();

            match fs::remove_dir_all(path) {
                Ok(_) => println!("Package removed successfully!"),
                Err(_) => println!("Package not found in local repository")
            }
    }
}
}
