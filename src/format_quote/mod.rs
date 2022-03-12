extern crate vtext;
use vtext::tokenize::{Tokenizer, VTextTokenizerParams};
use vtext::tokenize_sentence::*;
use regex::Regex;

const SPACE_LEN: usize = 1;


pub fn convert_to_tweet(quote: String) -> Vec<String> {
    if quote.len() <= 280 {
        return vec![quote];
    } else {
        return split_to_sentences(quote);
    }
}


fn split_to_sentences(quote: String) -> Vec<String> {
    // Split a paragraph into sentences
    // Each sentence ends with a full stop (and a space if it exists)
    let tokenizer = UnicodeSentenceTokenizer::default();
    let sentences: Vec<&str> = tokenizer.tokenize(&quote).collect();

    let mut tweets: Vec<String> = vec!["".to_string(); 1];
    let mut sentence_index = 0;
    let mut tweet_index = 0;

    // Iterate through the sentences and append to tweet if the length is 
    // less than 280 characters
    while sentence_index < sentences.len() {
        let curr_sentence_length = sentences[sentence_index].len();
        let curr_tweet_length = tweets[tweet_index].len();

        // println!("index: {} {}", sentence_index, tweet_index);
        // println!("length: {} {}", curr_sentence_length, curr_tweet_length);
        // println!("{}", sentences[sentence_index]);

        if curr_sentence_length <= 280 {
            if curr_tweet_length + curr_sentence_length <= 280 {
                let tweet = &tweets[tweet_index];
                let sentence = &sentences[sentence_index];
                tweets[tweet_index] = tweet.to_owned() + sentence.to_owned();
                sentence_index += 1;
            } else {
                tweets.push("".to_string());
                tweet_index += 1;
            }
        } else {
            // Handle edge case when sentence is longer than 280 chars
            let word_sentences = split_by_words(sentences[sentence_index]);

            for sentence in word_sentences {
                tweets.push(sentence.to_string());
                tweet_index += 1;
            }
            sentence_index += 1;
        }
    }

    return tweets;
}


fn split_by_words(sentence: &str) -> Vec<String> {
    // https://docs.rs/regex/latest/regex/#example-avoid-compiling-the-same-regex-in-a-loop
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[[:punct:]]").unwrap();
    }
    let mut tweet_sized_sentences: Vec<String> = vec!["".to_string(); 1];

    let tok = VTextTokenizerParams::default().lang("en").build().unwrap();
    let tokens: Vec<&str> = tok.tokenize(sentence).collect();

    let mut token_index = 0;
    let mut tweet_index = 0;

    while token_index < tokens.len() {
        let curr_token_length = tokens[token_index].len();
        let curr_sentence_length = tweet_sized_sentences[tweet_index].len();

        if curr_token_length <= 280 {
            if curr_sentence_length + curr_token_length + SPACE_LEN <= 280 {
                let tweet = &tweet_sized_sentences[tweet_index];
                let token = tokens[token_index];
                if token_index == tokens.len()-1 {
                    // Add space at end of last token
                    tweet_sized_sentences[tweet_index] = tweet.to_owned() + token + " ";
                } else if curr_sentence_length == 0 || RE.is_match(token) {
                    // Skip space in between tokens for first token and punctuations
                    tweet_sized_sentences[tweet_index] = tweet.to_owned() + token;
                } else {
                    // Add space between tokens
                    tweet_sized_sentences[tweet_index] = tweet.to_owned() + " " + token;
                }
                token_index += 1;
            } else {
                // Add space at the end of broken sentence to stay consistent
                // with the sentence split implementation
                tweet_sized_sentences[tweet_index] += " ";
                tweet_sized_sentences.push("".to_string());
                tweet_index += 1;
            }
        } else {
            panic!("Word longer than 280 chars");
        }
    }

    return tweet_sized_sentences;
}


#[cfg(test)]
mod test;
