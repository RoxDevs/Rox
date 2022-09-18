use rusqlite::{params, Connection, Result};
use rusqlite::NO_PARAMS;
struct package {
    tarball: String,
    win: i64,
    lin: i64,
    mac: i64,
    major: String,
    minor: String,
    rev: String
}

mod addPackage{
    fn addPackage() {
        conn.execute(
            "INSERT INTO package (tarball, win, lin, mac, major, minor,rev) VALUES (?1,?2,?3,?4,5?,6?,7?)",
            params!(),
        )?;
    }
    fn removePackage() {
        
    }
    fn updatePackage() {
        
    }
}