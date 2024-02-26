use clap:: {
    Args,
    Parser,
    Subcommand
};

#[derive(Parser, Debug)]
#[clap(author = "Reikimann", version, about)]
/// A Rust powered Ordnet.dk webscraper
pub struct CliArgs {
    /// Word to look up
    #[arg(value_parser = validate_search_query)]
    pub query: String,
}

fn validate_search_query(arg: &str) -> Result<String, String> {
    let query = arg.trim();

    if query.len() == 0 {
        Err(String::from(
                "Empty queries are not allowed.",
                ))
    } else {
        Ok(query.to_string())
    }
}
