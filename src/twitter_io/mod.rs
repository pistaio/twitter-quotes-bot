use oauth2::{
    AuthorizationCode,
    AuthUrl,
    ClientId,
    ClientSecret,
    CsrfToken,
    PkceCodeChallenge,
    RedirectUrl,
    Scope,
    TokenResponse,
    TokenUrl, AccessToken, RefreshToken, AuthType
};
use oauth2::basic::BasicClient;
use oauth2::reqwest::http_client;
use reqwest::Url;
use reqwest::header::{CONTENT_TYPE, HeaderMap};
use serde::{Serialize, Deserialize};

use std::collections::HashMap;
use std::net::TcpListener;
use std::io::{BufRead, BufReader, Write};
use std::fs::{self, File};
use std::path::Path;
use std::thread;

use crate::{file_io, format_quote};

mod constant;
use self::constant::{CLIENT_ID, CLIENT_SECRET, REDIRECT_URL};

const TOKENS_PATH: &str = "data/tokens/twitter.txt";

#[derive(Serialize, Deserialize)]
struct TweetRequest {
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply: Option<TweetReply>
}

#[derive(Serialize, Deserialize, Clone)]
struct TweetReply {
    in_reply_to_tweet_id: String 
}

#[derive(Serialize, Deserialize, Debug)]
struct TweetResponse {
    data: Data
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    id: String,
    text: String
}

#[derive(Debug)]
struct Token {
    access: oauth2::AccessToken,
    refresh: oauth2::RefreshToken 
}

pub fn tweet_quote(quote: String) {
    if !Path::new(TOKENS_PATH).exists() {
        thread::spawn(move || {
                generate_tokens();
        }).join().expect("Thread panicked")
    } else {
        // tweets is a vector because it can be a thread
        let tweets: Vec<String> = format_quote::convert_to_tweet(quote.to_owned());

        println!("{:?}", tweets);

        let contents = fs::read_to_string(TOKENS_PATH)
            .expect("Something went wrong reading the file");

        let mut tokens_str: Vec<&str> = contents.split("\n").collect();
        tokens_str.retain(|x| *x != "");
        assert_eq!(tokens_str.len(), 2);

        let tokens = Token {
            access: oauth2::AccessToken::new(tokens_str[0].to_string()),
            refresh: oauth2::RefreshToken::new(tokens_str[1].to_string())
        };

        let mut index = 0;
        let mut response  = handle_post_tweet_response(tokens, tweets[index].to_owned(), None);
        index += 1;
        while index < tweets.len() {
            let reply = TweetReply {
                in_reply_to_tweet_id: response.0 
            };

            response = handle_post_tweet_response(response.1, tweets[index].to_owned(), Some(reply));
            index += 1;
        }

        // Only after all tweets have been tweeted, remove quote from file
        if index == tweets.len() {
            file_io::remove_quote_from_markdown(quote)
        }
    }
}

fn handle_post_tweet_response(
    token: Token, 
    tweet: String, 
    tweet_id: Option<TweetReply>
) -> (String, Token) {
    match post_tweet(token, tweet.clone(), tweet_id.clone()) {
        Ok(response) => response,
        Err(e) => { panic!("Error: {}", e) }
    }
}

fn post_tweet(token: Token, tweet: String, tweet_id: Option<TweetReply>) 
    -> Result<(String, Token), Box<dyn std::error::Error>> {

    let endpoint = "https://api.twitter.com/2/tweets";

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

    let body = TweetRequest {
        text: tweet.clone(),
        reply: tweet_id.clone(),
    };

    let client = reqwest::blocking::Client::new();
    let request = client
                    .post(endpoint)
                    .bearer_auth(token.access.secret())
                    .headers(headers)
                    .json(&body);

    let response = request.send()?;
    // println!("{:#?}", response);

    match response.status() {
        reqwest::StatusCode::CREATED => {
            let response_body: TweetResponse = response.json()?;
            let id = response_body.data.id;
            println!("posted tweet id: {}", id);
            // Pass new access_token and refresh_token incase of refresh token flow
            // was previously run.
            Ok((id, token))
        },
        reqwest::StatusCode::UNAUTHORIZED => {
            // Run refresh token flow
            println!("----------------Here we are (unauthorized)----------------");
            match refresh_access_token(token.refresh) {
                Ok(token) => post_tweet(token, tweet, tweet_id),
                Err(e) => return Err(e)
            }
        },
        _ => {
            // Panic
            panic!("Uh oh! Something unexpected happened");
        }
    }
}

fn refresh_access_token(refresh_token: RefreshToken) -> Result<Token, Box<dyn std::error::Error>>{
    let client_id = ClientId::new(CLIENT_ID.to_string());
    let client_secret = ClientSecret::new(CLIENT_SECRET.to_string());
    let auth_url = AuthUrl::new("https://twitter.com/i/oauth2/authorize".to_string())
        .unwrap();
    let token_url = TokenUrl::new("https://api.twitter.com/2/oauth2/token".to_string())
        .unwrap();

    let client = BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
        .set_auth_type(AuthType::BasicAuth);

    let token_result = client
        .exchange_refresh_token(&refresh_token)
        .request(http_client)?;

    println!("{:#?}", token_result);

    let access_token = token_result.access_token();
    let refresh_token = token_result.refresh_token().unwrap(); // twitter always sends refresh token

    // println!("{}", access_token.secret());
    // println!("{}", refresh_token.secret());

    save_tokens_to_file(access_token, refresh_token);

    let token = Token {
        access: access_token.clone(),
        refresh: refresh_token.clone()
    };

    Ok(token)
    // Ok((access_token.secret().clone(), refresh_token.secret().clone()))
}

#[allow(dead_code)]
pub fn revoke_access_token(access_token: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut params = HashMap::new();
    params.insert("token", access_token);
    params.insert("token_type_hint", "access_token");

    let client = reqwest::blocking::Client::new();
    let request = client.post("https://api.twitter.com/2/oauth2/revoke")
        .basic_auth(CLIENT_ID, Some(CLIENT_SECRET))
        .form(&params);

    let _response = request
        .send()?;

    Ok(())
}

fn generate_tokens() {
    let client_id = ClientId::new(CLIENT_ID.to_string());
    let client_secret = Some(ClientSecret::new(CLIENT_SECRET.to_string()));
    let auth_url = AuthUrl::new("https://twitter.com/i/oauth2/authorize".to_string())
            .expect("Error parsing auth url");
    let token_url = Some(TokenUrl::new("https://api.twitter.com/2/oauth2/token".to_string())
            .expect("Error parsing token url"));
    let redirect_url = RedirectUrl::new(REDIRECT_URL.to_string())
            .expect("Unable to parse redirect url");

    // Create an OAuth2 client by specifying the client ID, client secret, authorization URL and
    // Set the URL the user will be redirected to after the authorization process.
    let client = BasicClient::new(client_id, client_secret, auth_url, token_url)
            .set_redirect_uri(redirect_url);

    // Generate a PKCE challenge.
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_plain();

    // Generate the full authorization URL.
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        // Set the desired scopes.
        .add_scope(Scope::new("tweet.read".to_string()))
        .add_scope(Scope::new("tweet.write".to_string()))
        .add_scope(Scope::new("users.read".to_string()))
        .add_scope(Scope::new("offline.access".to_string()))
        // Set the PKCE code challenge.
        .set_pkce_challenge(pkce_challenge)
        .url();
    
    // This is the URL you should redirect the user to, in order to trigger the authorization
    // process.
    println!("Browse to: {}", auth_url);

    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            let code;
            let state;
            {
                let mut reader = BufReader::new(&stream);

                let mut request_line = String::new();
                reader.read_line(&mut request_line).unwrap();

                let redirect_url = request_line.split_whitespace().nth(1).unwrap();
                let url = Url::parse(&("http://localhost".to_string() + redirect_url)).unwrap();

                let code_pair = url
                    .query_pairs()
                    .find(|pair| {
                        let &(ref key, _) = pair;
                        key == "code"
                    })
                    .unwrap();

                let (_, value) = code_pair;
                code = AuthorizationCode::new(value.into_owned());

                let state_pair = url
                    .query_pairs()
                    .find(|pair| {
                        let &(ref key, _) = pair;
                        key == "state"
                    })
                    .unwrap();

                let (_, value) = state_pair;
                state = CsrfToken::new(value.into_owned());
            }

            let message = "Go back to your terminal :)";
            let response = format!(
                "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
                message.len(),
                message
            );
            stream.write_all(response.as_bytes()).unwrap();

            assert_eq!(state.secret(), csrf_token.secret());

            // Now you can trade it for an access token.
            let token_result =
                client
                .exchange_code(code)
                // Set the PKCE code verifier.
                .set_pkce_verifier(pkce_verifier)
                .request(http_client)
                .expect("Error generating tokens");

            // println!("{:#?}", token_result);

            save_tokens_to_file(token_result.access_token(), 
                                token_result.refresh_token().unwrap());

            break;
        }
    }
}

fn save_tokens_to_file(access_token: &AccessToken, refresh_token: &RefreshToken) {
    // println!("{}", access_token.secret());
    // println!("{}", refresh_token.secret());

    let mut file = File::create("data/tokens/twitter.txt").expect("Unable to create file");
    writeln!(&mut file, "{}", access_token.secret()).unwrap();
    writeln!(&mut file, "{}", refresh_token.secret()).unwrap();
}


#[cfg(test)]
mod test;
