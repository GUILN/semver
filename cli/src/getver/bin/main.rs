use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser)]
    version: String,
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("Hello from get ver!");

    Ok(())
}
