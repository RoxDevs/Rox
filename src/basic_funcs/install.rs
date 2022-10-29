use git2::Repository;
// use rusqlite::{Connection};
use colored::Colorize;
use rusqlite::{Connection, Result, NO_PARAMS};

use crate::config::Config;

#[derive(Debug, Clone)]
struct Pkg {
    version: String,
    name: String,
    path: String,
    repo_url: String,
}

pub fn install(pkg_name: String, path: String, conf: &Config) {
    let mut db_path = conf.path.clone();
    db_path.push("pakageLDB.db");
    println!("{}", format!("Installing {}\n", pkg_name).green().bold());

    let a = || -> Result<()> {
        let conn = Connection::open(format!("{}", db_path.to_str().unwrap())).unwrap();
        let statement = format!("SELECT * FROM pkgs WHERE name='{}'", pkg_name);
        let mut stmt = conn.prepare(&statement.as_str()).unwrap();
        let pkg_iter = stmt
            .query_map(
                [],
                |row| {
                    Ok(Pkg {
                        version: row.get(0)?,
                        name: row.get(1)?,
                        path: row.get(2)?,
                        repo_url: row.get(3)?,
                    })
                },
            )
            .unwrap();

        let mut result = Vec::new();

        for r in pkg_iter {
            result.push(r);
        }
        let result = result
            .iter()
            .map(|a| a.as_ref().unwrap().clone())
            .collect::<Vec<Pkg>>();

        println!("Fetched data from DB\n");
        println!("{}", "Installing...\n".yellow());
        let _repo = match Repository::clone(&result.get(0).unwrap().repo_url, &path) {
            Ok(repo) => repo,
            Err(e) => panic!("Installation Failed: {}", e),
        };
        println!(
            "{}",
            format!("Finished Installation of package {}\n", pkg_name)
                .green()
                .bold()
        );
        println!("{}", "Enjoy your Package ❤️ - Rox\n".red());
        Ok(())
    };
    a().unwrap();
}

/// Attach to database &
pub fn install_db(package: String, pkg_name: String, path: String, ver: String, conf: &Config) {
    let mut db_path = conf.path.clone();
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
            NO_PARAMS,
        )?;
        conn.execute(
            "INSERT INTO Pkgs (version, name, path, repo_url) VALUES (?1,?2,?3,?4)",
            (ver, pkg_name, path, package.to_string()),
        )?;

        Ok(())
    };

    a().unwrap();
}
