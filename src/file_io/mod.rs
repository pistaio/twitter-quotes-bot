use std::fs;
use std::fs::File;
use std::io::Write;
use rand::seq::SliceRandom;


// Randomly select quote for markdown file of quotes
pub fn select_random_quote(file_path: &str) -> String {
    let quotes: Vec<String> = read_quotes(file_path);

    let mut range = rand::thread_rng();
    let random_quote = quotes
                        .choose(&mut range)
                        .unwrap()
                        .to_string();

    println!("Quote length: {}", random_quote.len());
    return random_quote
}


// Combine chapter markdown files into single markdown file
pub fn generate_combined_quotes_markdown(chapter_paths: Vec<String>) {
    let mut quotes: Vec<String> = Vec::new();

    for chapter_path in chapter_paths {
        quotes.append(&mut read_chapter_quotes(&chapter_path));
    }

    write_quotes_to_markdown(quotes.to_owned())
        .unwrap_or_else(|err| println!("{:?}", err));
}


// Remove quote from markdown once it is tweeted
pub fn remove_quote_from_markdown(quote: String) {
    let mut quotes: Vec<String> = read_quotes(crate::QUOTES_PATH);

    // Remove quote from file
    quotes.retain(|x| *x != quote);
    write_quotes_to_markdown(quotes)
        .unwrap_or_else(|err| println!("{:?}", err));
}


fn write_quotes_to_markdown(quotes: Vec<String>) -> std::io::Result<()> {
    let mut file = File::create("data/processed/quotes.md").expect("Unable to create file");

    for quote in quotes {
        if quote != "" {
            writeln!(&mut file, "{}\n", quote).unwrap();
        }
    }
    Ok(())
}


fn read_chapter_quotes(file_path: &str) -> Vec<String> {
    let contents = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");

    return extract_quotes(contents);
}


// TODO: Implement nested blockquote support
fn extract_quotes(contents: String) -> Vec<String> {
    let tokens: Vec<&str> = contents.split("\n").collect();
    let mut quotes: Vec<String> = vec!["".to_string(); 1];

    let mut token_index = 0;
    let mut quotes_index = 0;

    for token in &tokens {
        if token.len() == 0 {
            let prev_token_index = token_index - 1;
            // Only if previous token is quote, then increment quotes index
            if prev_token_index > 0 && is_quote(tokens[prev_token_index].to_owned()) {
                quotes_index += 1;
                quotes.push("".to_string());
                assert!(quotes_index < quotes.len());
            }
        } else if is_quote(token.to_string()) {
            // let token = &token.replace("> ", "");
            if quotes[quotes_index].len() == 0 {
                quotes[quotes_index] = quotes[quotes_index].to_owned() + token;
            } else {
                quotes[quotes_index] = quotes[quotes_index].to_owned() + "\n" + token;
            }
        }
        token_index += 1
    }

    return quotes;
}


fn is_quote(token: String) -> bool {
    return token.chars().nth(0) == Some('>'); 
}


fn read_quotes(file_path: &str) -> Vec<String> {
    let contents = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");

    let mut quotes: Vec<String> = contents.split("\n\n").map(|s| s.to_string()).collect();

    // Remove empty quotes
    quotes.retain(|x| *x != "");

    // If quotes.md is empty, generate new quotes file
    if quotes.len() == 0 {
        crate::process_chapter_markdowns();
        return read_quotes(crate::QUOTES_PATH);
    }

    return quotes
}


#[cfg(test)]
mod test;
