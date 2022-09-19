use clap::{Parser, Subcommand};
use git2::Repository;
use rand::distributions::{Alphanumeric, DistString};
use std::fs;
use rusqlite::{params, Connection, Result};
//mod litespeed;


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

fn add_Package() -> Result<()> {
    let conn = Connection::open_in_memory()?;
    conn.execute(
        "INSERT INTO pkgs (version, name, path, repo_url) VALUES (?1,?2,?3,?4)",
        (&"", &"", &"", &""),
    )?;

    Ok(())
}

fn main() {
    let cli = Cli::parse();
    let url = "https://github.com/RK33DV/unitytergen";
    let fldr = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
    let path = format!("RoxPaks/Packages/src/{}", fldr);
    match &cli.command {
        Commands::Install { package } => {
            println!("Installing{:?}", package);
            let _repo = match Repository::clone(url, path) {
               Ok(repo) => repo,
               Err(e) => panic!("installation failed : {}", e),
            };

        }
        Commands::Remove { package } => {
            println!("Removing source code...");
            fs::remove_dir_all("RoxPaks")
            .expect("Error removing source code :(");
         println!("source code removed successfully!");
    }
}
}

fn install() -> Result<()> {
    let conn = Connection::open_in_memory()?;
    conn.execute(
        "INSERT INTO pkgs (version, name, ) VALUES (?1, ?2)",
        (&me.name, &me.data),
    )?;

    Ok(())
}