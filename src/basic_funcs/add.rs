use rusqlite::{Connection, Result};
pub fn add(package: String, pkg_name: String, path: String, ver: String){
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