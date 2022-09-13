use clap::Parser;
use git2::Repository;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Values {
   #[clap(short = 'o', long)]
   install: String,
}

fn main() {
   let answer = Values::parse();

   let value = answer.install.to_string();
   let installer = value;
   let url = ("https://github.com/RK33DV/unitytergen");
   let path =  format!("/rox/packages/{}/", installer);
   let repo = match Repository::clone(url, path) {
    Ok(repo) => repo,
    Err(e) => panic!("installation failed : {}", e),
   };

   println!("Downloading {}", installer);
   println!("Installing {}", installer);
   println!("success!")
}