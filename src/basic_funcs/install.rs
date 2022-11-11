use std::{fs, path::PathBuf, str::FromStr};

use git2::Repository;
// use rusqlite::{Connection};
use colored::Colorize;
use rusqlite::{Connection, Result};

use crate::config::Config;

#[derive(Debug, Clone)]
struct Pkg {
    version: String,
    name: String,
    path: String,
    repo_url: String,
}

pub fn install(pkg_name: String, ver: Option<String>, conf: &Config) {
    let mut db_path = conf.path.clone();
    db_path.push("pakageLDB.db");
    let conn = rusqlite::Connection::open(db_path).unwrap();
    println!("{}", "Enjoy your Package ❤️ - Rox\n".red());
}

/// Attach to database &
pub fn install_db(package: String, pkg_name: String, path: String, ver: String, conf: &Config) {
    if !PathBuf::from_str(&path).unwrap().is_dir() {
        fs::create_dir_all(&path).unwrap();
    }
    let mut db_path = conf.path.clone();
    dbg!(db_path.clone());
    db_path.push("pakageLDB.db");
    let _repo = match Repository::clone(&package, &path) {
        Ok(repo) => repo,
        Err(e) => panic!("installation failed : {}", e),
    };

    println!("PACKAGE: {}", package);
    println!("PKG_NAME: {}", pkg_name);
    println!("PATH: {}", path);
    println!("VERSION: {}", ver);

    let a = || -> Result<()> {
        let conn = Connection::open(format!("{}", db_path.to_str().unwrap()))?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS Pkgs (
                id integer primary key,
                version text,
                name text,
                path text,
                repo_url text
            );",
            [],
        )?;
        conn.execute(
            "INSERT INTO Pkgs (version, name, path, repo_url) VALUES (?1,?2,?3,?4)",
            (ver, pkg_name, path, package.to_string()),
        )?;

        Ok(())
    };

    a().unwrap();
}
