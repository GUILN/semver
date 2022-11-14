use core::calculate_version;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Current Version
    /// #Example:
    /// v2.3.5
    #[clap(short, long, value_parser)]
    current_version: String,
    /// Semantic Version Comment
    /// 
    /// # Example:
    /// feat: this is a feature.
    #[clap(short, long, value_parser)]
    comment: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let new_version = calculate_version(args.current_version.as_str(), args.comment.as_str().try_into()?)?;

    println!("{}", new_version);

    Ok(())
}
