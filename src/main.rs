mod format_quote;
mod file_io;
mod twitter_io;

// https://docs.rs/regex/latest/regex/#example-avoid-compiling-the-same-regex-in-a-loop
#[macro_use]
extern crate lazy_static;
use std::path::Path;

#[tokio::main]
async fn main() {
    let quotes_path = "data/processed/quotes.md";
    if !Path::new(quotes_path).exists() {
        process_markdowns();
    }

    let quote = file_io::select_random_quote(quotes_path);

    let tweet = format_quote::convert_to_tweet(quote.to_owned());

    println!("{}", quote.len());
    println!("{:?}", tweet);

    // twitter_io::revoke_access_token("eFNIMWU1c01vV2R3UVpHaFhUcEZYbFlwVWI0N09uMldJUi1iS2NTamZvcE54OjE2NDc0MDk0ODIyMTQ6MToxOmF0OjE").await.expect("Some error");

    twitter_io::setup_twitter(tweet).await;
}

fn process_markdowns() {
    let path = "data/original";
    let file_names: Vec<&str> = vec!["chapter_2.md", "chapter_3.md", "chapter_4.md", "chapter_5.md"];

    let mut file_paths: Vec<String> = Vec::new();

    for file_name in &file_names {
        let file_path = format!("{}/{}", path, file_name);
        file_paths.push(file_path);
    }

    file_io::generate_quotes_markdown(file_paths);
}
