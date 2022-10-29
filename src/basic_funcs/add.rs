use csv::WriterBuilder;
use git2::Repository;
use rusqlite::{Connection, Result};
use std::fs::{read_dir, remove_dir_all, File, create_dir_all};

use crate::{config::Config, parser::{RawProject, Project}};

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
    if !path.is_dir() {
        create_dir_all(path.clone()).unwrap();
    }
    path.push("repo");
    Repository::clone_recurse(url, path.clone()).unwrap();
    path.pop();
    path.push("repo");
    let paths = read_dir(path.to_str().unwrap()).unwrap();
    path.pop();
    path.pop();
    path.push("repos.csv");
    if !path.is_file() {
        File::create(path.clone()).unwrap();
    }
    let mut wrtr = WriterBuilder::new()
        .delimiter(b',')
        .from_path(path.clone())
        .unwrap();
    wrtr.write_field(url).unwrap();
    drop(wrtr);
    for file in paths {
        if format!("{:#?}", file.as_ref().unwrap().file_name()) == r#"".git""# {
            continue;
        }
        let file = file.unwrap();
        let mut repo = path.clone();
        repo.pop();
        repo.push("tmp");
        repo.push("repo");
        let manifest: Project = RawProject::create_from_file(format!("{}/{}", repo.to_str().unwrap(),file.file_name().to_str().unwrap()).as_str()).unwrap().into();
        dbg!(manifest.clone());
        // Write SQL
    }
    //clear after adding to db
    path.pop();
    path.push("tmp");
    remove_dir_all(path.clone()).unwrap();
}
