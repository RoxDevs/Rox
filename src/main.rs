use clap::Parser;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Values {
   #[clap(short = 'o', long)]
   desiredpackage: String,
}

fn main() {
   let value = Values::parse();

   let value = value.desiredpackage;

   let installer = value;

   println!("Downloading {}", installer);
   println!("Installing {}", installer);
   println!("success!")
}