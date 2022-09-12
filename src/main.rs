<<<<<<< HEAD
use clap::Parser;
mod parser;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Values {
   #[clap(short = 'o', long)]
   desiredpackage: String,

>>>>>>> d2e4ac9 (stash)
}

fn main() {
   let value = Values::parse();

   let value = value.desiredpackage;

   let installer = value;

   println!("Downloading {}", installer);
   println!("Installing {}", installer);
   println!("success!")
}
