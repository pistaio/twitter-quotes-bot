mod format_quote;
mod file_io;
mod twitter_io;

// https://docs.rs/regex/latest/regex/#example-avoid-compiling-the-same-regex-in-a-loop
#[macro_use]
extern crate lazy_static;
use std::path::Path;

const QUOTES_PATH: &str = "data/processed/quotes.md";

#[tokio::main]
async fn main() {
    if !Path::new(QUOTES_PATH).exists() {
        process_chapter_markdowns();
    }

    let quote = file_io::select_random_quote(crate::QUOTES_PATH);

    // twitter_io::revoke_access_token("eFNIMWU1c01vV2R3UVpHaFhUcEZYbFlwVWI0N09uMldJUi1iS2NTamZvcE54OjE2NDc0MDk0ODIyMTQ6MToxOmF0OjE").await.expect("Some error");

    twitter_io::tweet_quote(quote).await;
}

fn process_chapter_markdowns() {
    let path = "data/original";
    let chapter_file_names: Vec<&str> = vec!["chapter_2.md", "chapter_3.md", "chapter_4.md", "chapter_5.md"];

    let mut chapter_paths: Vec<String> = Vec::new();

    for chapter_name in &chapter_file_names{
        let chapter_path = format!("{}/{}", path, chapter_name);
        chapter_paths.push(chapter_path);
    }

    file_io::generate_combined_quotes_markdown(chapter_paths);
}
