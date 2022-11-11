use clap::{Parser, Subcommand};

use config::Config;
use std::env::current_exe;

mod config;
use std::fs::read_to_string;
use std::path::PathBuf;

mod basic_funcs;
use basic_funcs::install::install;
use config::RawConfig;

use crate::basic_funcs::add::add_repo;

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
    Install {
        package: String,
        ver: Option<String>,
    },
    /// remove the source code of the packages
    Remove { package: String },
    Repo {
        #[clap[subcommand]]
        cmd: RepoCommand,
    },
}

#[derive(Subcommand)]
enum RepoCommand {
    Add { url: String },
}

fn main() {
    let mut conf_path = current_exe().unwrap();
    conf_path.pop();
    conf_path.push("config.toml");
    let configuration: Config = RawConfig::from_str(&read_to_string(conf_path).unwrap())
        .unwrap()
        .into();

    println!("{:?}", configuration.path);

    let windows_path_buf = PathBuf::from(configuration.path);

    let conf = Config {
        path: windows_path_buf,
        target: configuration.target,
    };
    let cli = Cli::parse();

    match &cli.command {
        Commands::Install { package, ver } => {
                install(package.to_string(), ver.to_owned(), &conf) // url in this case is link to rox official repo
        }

        // Remove command
        Commands::Remove { package: _ } => {
            todo!()
        }
        Commands::Repo {
            cmd: RepoCommand::Add { url },
        } => add_repo(url.as_str(), &conf)}
    }
