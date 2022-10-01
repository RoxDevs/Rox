use std::env::current_exe;
use std::fs::read;

#[allow(unused_variables)]

use clap::{Parser, Subcommand};
use config::Config;
mod config;
use std::fs::read_to_string;
// use git2::Repository;
// use std::fs;
// use rusqlite::{Connection, Result};

mod basic_funcs;
use basic_funcs::install::install;
use basic_funcs::install::install_db;
use config::RawConfig;

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
    let mut conf_path = current_exe().unwrap();
    conf_path.pop();
    conf_path.push("config.toml");
    let conf: Config = RawConfig::from_str(&read_to_string(conf_path).unwrap()).unwrap().into();
    let cli = Cli::parse();    
    let fldr = "";
    let pkg_name = "";
    
    let ver = "2".to_string();

    match &cli.command{
        Commands::Install { package } => {

            let pkg_name = parse_name(package);
            let fldr = "".to_string();
            let path = "".to_string();

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


