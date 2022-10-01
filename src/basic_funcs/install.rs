use std::fmt::format;

use git2::Repository;
// use rusqlite::{Connection};
use rusqlite::{Connection, Result};
use sql_query_builder as sql;

#[derive(Debug)]
struct pkg {
    id: String,
    version: String, 
    name: String,
    path: String,
    repo_url: String,
}

pub fn install(pkg_name: String, path: String){
// package, url, path
        
        let url = "";
        println!("Installing {:?}", pkg_name);
        let mut rep_url = "".to_string();

        let a = || -> Result<()> {
            println!("Running A");
            println!("Running A");
            let conn = Connection::open("src/packageLDB.db").unwrap();
            dbg!(&conn);
            println!("Running B");
            let mut stmt = conn.prepare(format!("SELECT * FROM pkgs WHERE name=:'{}'", pkg_name).as_str())?;
            println!("Running C");
            let pkg_iter = stmt.query_map(&[(pkg_name.as_str(), rep_url.as_str())], |row|{
                Ok(pkg {
                    id: row.get(1)?,
                    version: row.get(2)?, 
                    name: row.get(3)?,
                    path: row.get(4)?,
                    repo_url: row.get(5)?,
                })
            })?;
            println!("Running D");
            println!("Hello World");
            for r in pkg_iter{
                println!("R = {:?}", r);
            }
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

    // let new_package = pkg {
    //     // version: ver, 
    //     // name: pkg_name,
    //     // path: path,
    //     repo_url: package.to_string(),
    // };

    let a = || -> Result<()> {
        let conn = Connection::open("/home/garuda/dev/Rox/src/packageLDB.db")?;
        conn.execute(
            "INSERT INTO pkgs (version, name, path, repo_url) VALUES (?1,?2,?3,?4)",
            (/*&new_package.version, &new_package.name, &new_package.path, &new_package.repo_url*/ ver, pkg_name, path, package.to_string()),
        )?;
        //conn.execute("INSERT INTO pkgs (version, name, path, repo_url) VALUES ('1', '2', '3', '4')", NO_PARAMS)?;
    
        Ok(())
    };

    a();
}
