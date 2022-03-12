# Twitter Quotes Bot

This repo contains a Rust script that posts quotes from a markdown file to Twitter.

## Functionality

- Collate quotes from multiple markdown files into a single markdown file
- Convert quotes longer than 280 characters into a thread
- Automatically select quote from markdown file and post as tweet or thread
- Test suite

- Post tweet thread support
- Delete quote after selecting it from quotes.md
- Add refresh token flow
- Handle 4xx status code (investigate why sometimes retry works with same payload)
- Generate access token and post tweet in same flow (low priority)
