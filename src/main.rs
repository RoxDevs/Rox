#[allow(unused_variables)]

use clap::{Parser, Subcommand};
mod config;
// use git2::Repository;
// use std::fs;
// use rusqlite::{Connection, Result};

mod basic_funcs;
use basic_funcs::install::install;
use basic_funcs::install::install_db;

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
    // install a package
    Install { package: String },
    // remove the source code of the packages
    Remove { package: String },
}

// Convert package to VEC
fn parse_name(url: &str) -> Vec<&str>{
    let url = url.split("/");
    let mut parsed_arr = Vec::new(); 
    for s in url {
        parsed_arr.push(s);
    }
    return parsed_arr
}

fn main() {
    let url = "";
    
    let cli = Cli::parse();    
    let fldr = "";
    let pkg_name = "";
    
    let ver = "2".to_string();

    match &cli.command{
        Commands::Install { package } => {

            let pkg_name = parse_name(package);
            //let fldr = pkg_name[4].to_string();
            let fldr = "".to_string();
            let path = "".to_string();

            println!("Folder: {}", fldr);
            println!("Path: {}", path);
            println!("{}", pkg_name.len());

            // Validation and Appropriation
            // Package
            if package == ""{
                   todo!()
            }            
            else if pkg_name.len() > 1{
                let fldr = pkg_name[4].to_string();
                println!("fldr: {}", fldr);
                let path = format!("RoxPaks/Packages/src/{}", fldr);
                install_db(package.to_string(), pkg_name[4].to_string(), path, ver)
            }
            else if pkg_name.len() == 1{
                let fldr = package.to_string();
                let path = format!("RoxPaks/Packages/src/{}", fldr);
                install(package.to_string(), path) // url in this case is link to rox official repo                
            }

            
        }
        Commands::Remove { package:_ } => {
            todo!()
        }
    }

}

    // match &cli.command {
    //     Commands::Install { package } => {
    //         println!("Installing {:?}", package);
    //         if package == ""{
                
    //         }
    //         else if package.len() > 1{
    //             url = package
    //         }
    //         else if package.len() < 1{
    //             url = "" // url in this case is link to rox official repo                
    //         }


    //         let _repo = match Repository::clone(url, path) {
    //             Ok(repo) => repo,
    //             Err(e) => panic!("installation failed : {}", e),
    //         };

    //         let a = || -> Result<()> {
    //             let conn = Connection::open("src/packageLDB.db")?;
    //             conn.execute(
    //                 "INSERT INTO pkgs (version, name, path, repo_url) VALUES (?1,?2,?3,?4)",
    //                 (&ver, &parsed_name[3], &fldr, &url.to_string()),
    //             )?;
            
    //             Ok(())
    //         };

    //         a();


    // }
        // Commands::Remove { package: _ } => {
        //     println!("Removing source code...");

        //     fs::remove_dir_all("src/RoxPaks")
        //     .expect("Error removing source code :(");
        //  println!("source code removed successfully!");
        // }
//    }


