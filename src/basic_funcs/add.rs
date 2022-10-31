use csv::WriterBuilder;
use git2::Repository;
use rusqlite::{Connection, Result};
use std::fs::{create_dir_all, read_dir, remove_dir_all, File};

use crate::{
    config::Config,
    parser::{Project, RawProject, Ver},
};

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
    if !path.is_dir() {
        Repository::clone_recurse(url, path.clone()).unwrap();
    }
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
        let manifest: Project = RawProject::create_from_file(
            format!(
                "{}/{}",
                repo.to_str().unwrap(),
                file.file_name().to_str().unwrap()
            )
            .as_str(),
        )
        .unwrap()
        .into();
        dbg!(manifest.clone());

        let mut db_path = conf.path.clone();
        db_path.push("pakageLDB.db");

        let conn = match Connection::open(format!("{}", db_path.to_str().unwrap())) {
            Ok(conn) => conn,
            Err(_) => panic!("could not connect to the database"),
        };

        match conn.execute(
            "CREATE TABLE IF NOT EXISTS repo (
                id integer primary key AUTOINCREMENT,
                name text,
                authors text,
                git text
            );",
            [],
        ) {
            Ok(conn) => conn,
            Err(_) => panic!("could not create table repo"),
        };

        match conn.execute(
            "CREATE TABLE IF NOT EXISTS version (
                id integer primary key AUTOINCREMENT,
                id_repo integer FOREIGN KEY REFERENCES repo(id) NOT NULL,
                details_json text,
            );",
            [],
        ) {
            Ok(conn) => conn,
            Err(_) => panic!("could not create table repo"),
        };

        //inserting repo and retrieving the id for version
        let repo_id = insert_repo(&conn, &manifest).expect("could not insert repo");

        //inserting versions as json string
        for version in manifest.versions.iter() {
            match conn.execute(
                "INSERT INTO version (id_repo, details_json) VALUES (?1,?2)",
                (repo_id, VersionDetails::parse(version).inner_ref()),
            ) {
                Ok(conn) => conn,
                Err(_) => panic!("could not insert version"),
            };
        }
    }
    //clear after adding to db
    path.pop();
    path.push("tmp");
    remove_dir_all(path.clone()).unwrap();
}

// tries to insert a Repo Struct
// returns row's pk id
pub fn insert_repo(db: &Connection, manifest: &Project) -> Result<i64, rusqlite::Error> {
    match db.execute(
        "INSERT INTO repo (name, authors, git) VALUES (?1,?2,?3)",
        (
            manifest.name.to_owned(),
            Authors::parse(&manifest.authors).inner_ref(),
            manifest.git.to_owned(),
        ),
    ) {
        Ok(conn) => conn,
        Err(_) => panic!("could not insert into repo"),
    };

    Ok(db.last_insert_rowid())
}

pub struct Authors(String);

impl Authors {
    // Parses the Vector of authors into a String of comma
    // separated values avoiding all the issues with debugging
    pub fn parse(authors: &Vec<String>) -> Authors {
        if authors.is_empty() {
            return Self("Unknown Author".to_owned());
        }
        Self(
            authors
                .iter()
                .map(|x| x.to_string() + ",")
                .collect::<String>(),
        )
    }

    pub fn inner_ref(&self) -> &str {
        // The caller gets a shared reference to the inner string.
        // This gives the caller **read-only** access,
        // they have no way to compromise our invariants!
        &self.0
    }
}

pub struct VersionDetails(String);
// TODO: implement TryInto to convert from Json to Ver
impl VersionDetails {
    // Parses Version into a json String handling the errors
    pub fn parse(version: &Ver) -> VersionDetails {
        match serde_json::to_string(version) {
            Ok(version) => Self(version),
            Err(_) => panic!("Error while trying to convert version into JSON"),
        }
    }

    pub fn inner_ref(&self) -> &str {
        // The caller gets a shared reference to the inner string.
        // This gives the caller **read-only** access,
        // they have no way to compromise our invariants!
        &self.0
    }
}

//TODO add tests
