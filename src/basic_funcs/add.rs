// pub fn install(package: String, pkg_name: String, path: String, ver: String){
// // package, url, path


//         let mut url = "";
//         println!("Installing {:?}", package);
//         if package == ""{
//                todo!()
//         }

//         if package.len() > 1{
//             install(package.to_string(), pkg_name[4].to_string(), path, ver)
//         }
//         else if package.len() < 1{
//             install(package.to_string(), pkg_name[4].to_string(), path, ver) // url in this case is link to rox official repo                
//         }

//         let _repo = match Repository::clone(&package, &path) {
//             Ok(repo) => repo,
//             Err(e) => panic!("installation failed : {}", e),
//         };

//         let a = || -> Result<()> {
//             let conn = Connection::open("src/packageLDB.db")?;
//             conn.execute(
//                 "INSERT INTO pkgs (version, name, path, repo_url) VALUES (?1,?2,?3,?4)",
//                 (&ver, &pkg_name, &path, &url),
//             )?;
        
//             Ok(())
//         };

//         a();

//     }