#[allow(unused_variables)]

use clap::{Parser, Subcommand};
use git2::Repository;
use rand::distributions::{Alphanumeric, DistString};
use std::fs;
use rusqlite::{params, Connection, Result};

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
#[tokio::main]
async fn findRepo() -> Result<(), fantoccini::error::CmdError> {
    let c = ClientBuilder::native().connect("http://localhost:4444").await.expect("failed to connect to WebDriver");
    c.goto("https://github.com").await?;
    let f = c.form(Locator::Css("#form-control js-site-search-focus header-search-input jump-to-field js-jump-to-field")).await?;
    Ok(())
}

fn main() {
    findRepo();
    let cli = Cli::parse();
    let url = "https://github.com/RK33DV/unitytergen";
    let fldr = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
    let path = format!("RoxPaks/Packages/src/{}", fldr);
    let ver = "2".to_string();
    match &cli.command {
        Commands::Install { package } => {
            println!("Installing{:?}", package);
            let mut a = || -> Result<()> {
                let conn = Connection::open("src/packageLDB.db")?;
                conn.execute(
                    "INSERT INTO pkgs (version, name, path, repo_url) VALUES (?1,?2,?3,?4)",
                    (&ver, &package, &fldr, &url.to_string()),
                )?;
            
                Ok(())
            };
            a();


            let _repo = match Repository::clone(url, path) {
                Ok(repo) => repo,
                Err(e) => panic!("installation failed : {}", e),
            };

        }
        Commands::Remove { package } => {
            let mut search = || -> Result<()> {
                let conn = Connection::open("src/packageLDB.db")?;
                conn.execute(
                    "DELETE FROM pkgs
                    WHERE name = ?1;",
                    (&package,)
                )?;
            
                Ok(())
            };
            search();

            println!("Removing source code...");

            fs::remove_dir_all("RoxPaks")
            .expect("Error removing source code :(");
         println!("source code removed successfully!");
    }
}
} 