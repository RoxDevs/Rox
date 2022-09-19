use clap::{Parser, Subcommand};
use git2::Repository;
use rand::distributions::{Alphanumeric, DistString};
use std::fs;
use rusqlite::{params, Connection, Result};
//mod litespeed;

struct PackageLT {
    version: String,
    name: String,
    path: String,
    repo_url: String,
}

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
    let mut newpkg = PackageLT {
        version: "1.0".to_string(),
        name: "foo".to_string(),
        path: "foo".to_string(),
        repo_url: "foo".to_string()
    };
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