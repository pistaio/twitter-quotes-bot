# Twitter Quotes Bot

This repo contains a Rust script that extracts quotes from a markdown file, collates, and randomly posts them to Twitter.

## Functionality

- Collate quotes from multiple markdown files into a single markdown file
- Convert quotes longer than 280 characters into a thread
- Automatically select quote from markdown file and post as tweet or thread
    - Remove quote from generated md file after tweets are posted
- Automatically refresh access token (confidential client flow)
- Test suite

## Setup

- Add your markdown file from which quotes need to be extracted to `data/original/`. (multiple files are also supported)
- Update the filename(s) in `process_chapter_markdowns()` in `main.rs`
- Add Twitter credentials (client ID, client secret, and callback URL) in `twitter_io/constants.rs`
- Run the code using `cargo run`
