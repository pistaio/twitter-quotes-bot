# Twitter Quotes Bot

This repo contains a Rust script that posts quotes from a markdown file to Twitter.

## Functionality

- Collate quotes from multiple markdown files into a single markdown file
- Convert quotes longer than 280 characters into a thread
- Automatically select quote from markdown file and post as tweet or thread
    - Remove quote from generated md file after tweets are posted
- Automatically refresh access token (confidential client flow)
- Test suite
