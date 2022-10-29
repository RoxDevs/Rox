use csv::WriterBuilder;
use git2::Repository;
use rusqlite::{Connection, Result};
use std::fs::{create_dir, read_dir, remove_dir, remove_dir_all, File};

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
    println!("{:#?}", path);

    if !path.try_exists().unwrap() {
        create_dir(path.clone()).unwrap();
    }
    path.push("repo");
    if !path.is_dir() {
        Repository::clone_recurse(url, path.clone()).unwrap();
    }
    path.pop();
    path.push("repos.db");
    if !path.try_exists().unwrap() {
        File::create(path.clone()).unwrap();
    }
    let file = File::open(path.clone()).unwrap();
    println!("{:#?}", path);
    let paths = read_dir(path.clone()).unwrap();
    if !&path.is_file() {
        File::create(path.clone()).unwrap();
    }
    /*  let wrtr = WriterBuilder::new()
    .delimiter(b',')
    .from_path(path.clone())
    .unwrap(); */
    for path in paths {
        if format!("{:#?}", path.as_ref().unwrap().file_name()) == r#"".git""# {
            continue;
        }
        let path = path.unwrap();
        println!("{:#?}", path.file_name());
    }
    remove_dir_all(path.clone()).unwrap();
    path.pop();
    remove_dir(path).unwrap()
}
