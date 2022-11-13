use clap::Parser;
/// ! [`semver`] This cli parses the semantic version commit comment.
///
/// It fails if the commit comment is not in valid format else
/// succeds and returns a json representing the commit semantics.
/// # Expected comment structure:
/// - <semantic_type>: this is a <semantic_type>.
/// - <semantic_type>! this is a <semantic_type>.
///
/// Where <semantic_type> is [`fix`, `feat`, `refact`] and [`:`, `!`] means [`non_breaking`, `breaking`] respectively.
///
/// # Example:
/// `semver --comment "feat! this is a breaking feature."`
/// `semver --comment "fix: this is a non breaking fix."`
/// `semver --comment "refact! this is a breaking refactor."`
use core::parse_comment;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// `comment` is the comment from your vcs.
    #[clap(short, long, value_parser)]
    comment: String,
    /// output-json controls if the output will be json.
    #[arg(short, long, default_value_t = false)]
    output_json: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let semantic_comment = parse_comment(args.comment.as_str())?;

    if args.output_json {
        println!("{}", semantic_comment.as_json_string()?);
    } else {
        println!("{:?}", semantic_comment)
    }

    Ok(())
}
