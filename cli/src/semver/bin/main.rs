use core::parse_comment;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser)]
    comment: String,
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let semantic_comment = parse_comment(args.comment.as_str())?;

    println!("{:?}", semantic_comment);
    Ok(())
}
