use std::fs::{create_dir, read_dir, remove_dir_all};

use git2::Repository;
use rusqlite::{Connection, Result};

use crate::config::Config;

pub fn add(package: String, pkg_name: String, ver: String, conf: &Config) {
    let mut db_path = conf.path.clone();
    db_path.push("pakageLDB.db");
    let mut path = conf.path.clone();
    path.push("pkgs");
    path.push(format!("{}", pkg_name));
    let a = || -> Result<()> {
        let conn = Connection::open(format!("{}", db_path.to_str().unwrap()))?;
        conn.execute(
            "INSERT INTO pkgs (version, name, path, repo_url) VALUES (?1,?2,?3,?4)",
            (ver, pkg_name, path.to_str(), package.to_string()),
        )?;

        Ok(())
    };

    a().unwrap();
}

pub fn add_repo(url: &str, conf: &Config) {
    let mut path = conf.path.clone();
    path.push("tmp");
    create_dir(path.clone()).unwrap();
    path.push("repo");
    Repository::clone_recurse(url, path.clone()).unwrap();
    let paths = read_dir(path.to_str().unwrap()).unwrap();

    for path in paths {
        let path = path.unwrap();
        println!("{:#?}", path)
    }
    remove_dir_all(path.clone()).unwrap()
}
