use std::fs;
use std::fs::File;
use std::io::Write;
use rand::seq::SliceRandom;


// Randomly select quote for markdown file of quotes
pub fn select_random_quote(file_path: &str) -> String {
    let contents = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");

    let mut quotes: Vec<String> = contents.split("\n\n").map(|s| s.to_string()).collect();

    // Remove empty quotes
    quotes.retain(|x| *x != "");

    let mut range = rand::thread_rng();
    let random_quote = quotes
                        .choose(&mut range)
                        .unwrap() // TODO: If quotes is empty, generate new quotes file
                        .to_string();

    // Remove quote from quotes.md
    quotes.retain(|x| *x != random_quote);
    write_quotes_to_markdown(quotes, Some(&random_quote))
        .unwrap_or_else(|err| println!("{:?}", err));

    return random_quote.replace("> ", "");
}


// Combine chapter markdown files into single markdown file
pub fn generate_quotes_markdown(file_paths: Vec<String>) {
    let mut quotes: Vec<String> = Vec::new();

    for file_path in file_paths {
        quotes.append(&mut read_markdown(&file_path));
    }

    write_quotes_to_markdown(quotes.to_owned(), None)
        .unwrap_or_else(|err| println!("{:?}", err));
}


fn read_markdown(file_path: &str) -> Vec<String> {
    let contents = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");

    return extract_quotes(contents);
}


fn write_quotes_to_markdown(quotes: Vec<String>, quote: Option<&str>) -> std::io::Result<()> {
    let mut file = File::create("data/processed/quotes.md").expect("Unable to create file");

    let random_quote = quote.unwrap_or_else(|| "");

    for quote in quotes {
        if quote != "" && quote != random_quote {
            writeln!(&mut file, "{}\n", quote).unwrap();
        }
    }
    Ok(())
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

    // for quote in &quotes {
    //     println!("quote:-----------------------------{}", quote);
    // }

    return quotes;
}


fn is_quote(token: String) -> bool {
    return token.chars().nth(0) == Some('>'); 
}


#[cfg(test)]
mod test;
