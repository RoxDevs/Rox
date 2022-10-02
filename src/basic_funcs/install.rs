use std::fmt::format;

use git2::Repository;
// use rusqlite::{Connection};
use rusqlite::{Connection, Result, ffi::SQLITE_NULL};
// use sql_query_builder as sql;

use colored::Colorize;

#[derive(Debug)]
#[derive(Clone)]
struct pkg {
    version: String, 
    name: String,
    path: String,
    repo_url: String,
}

pub fn install(pkg_name: String, path: String){
// package, url, path
        
        let url = "";
        println!("{}", format!("Installing {}\n", pkg_name).green().bold());
        let mut rep_url = "".to_string();

        let a = || -> Result<()> {
            let conn = Connection::open("/home/garuda/dev/Rox/src/packageLDB.db").unwrap();
            let statement = format!("SELECT * FROM pkgs WHERE name='{}'", pkg_name);
            let mut stmt = conn.prepare(&statement.as_str()).unwrap();
            let pkg_iter = stmt.query_map(/*&[(pkg_name.as_str(), rep_url.as_str())]*/[], |row|{
                Ok(pkg {
                    version: row.get(0)?, 
                    name: row.get(1)?,
                    path: row.get(2)?,
                    repo_url: row.get(3)?,
                })
            }).unwrap();

            let mut result = Vec::new();

            for r in pkg_iter{
                result.push(r);
            }
            let result = result.iter().map(|a|{a.as_ref().unwrap().clone()}).collect::<Vec<pkg>>();

            println!("Fetched data from DB\n");
            println!("{}", "Installing...\n".yellow());
            let _repo = match Repository::clone(&result.get(0).unwrap().repo_url, &path) {
                Ok(repo) => repo,
                Err(e) => panic!("Installation Failed: {}", e),
            };
            println!("{}", format!("Finished Installation of package {}\n", pkg_name).green().bold());
            println!("{}", "Enjoy your Package ❤️ ~ Rox\n".red());            
            Ok(())
        };
        a();

        // let _repo = match Repository::clone(&rep_url, &path) {
        //     Ok(repo) => repo,
        //     Err(e) => panic!("installation failed : {}", e),
        // };

    }

pub fn install_db(package: String, pkg_name: String, path: String, ver: String){
    let _repo = match Repository::clone(&package, &path) {
        Ok(repo) => repo,
        Err(e) => panic!("installation failed : {}", e),
    };

    println!("PACKAGE: {}", package);
    println!("PKG_NAME: {}", pkg_name);
    println!("PATH: {}",path);
    println!("VERSION: {}", ver);

    let a = || -> Result<()> {
        let conn = Connection::open("/home/garuda/dev/Rox/src/packageLDB.db")?;
        conn.execute(
            "INSERT INTO pkgs (version, name, path, repo_url) VALUES (?1,?2,?3,?4)",
            (/*&new_package.version, &new_package.name, &new_package.path, &new_package.repo_url*/ ver, pkg_name, path, package.to_string()),
        )?;
    
        Ok(())
    };

    a();
}
