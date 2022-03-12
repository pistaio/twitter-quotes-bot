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
    TokenUrl, AccessToken, RefreshToken
};
use oauth2::basic::BasicClient;
use oauth2::reqwest::http_client;
use reqwest::header::CONTENT_TYPE;
use serde::Serialize;
use url::Url;

use reqwest::{self, header::HeaderMap};

use std::net::TcpListener;
use std::io::{BufRead, BufReader, Write};
use std::fs::{self, File};
use std::path::Path;

mod constant;
use self::constant::{CLIENT_ID, CLIENT_SECRET};

use std::thread;

#[derive(Serialize)]
struct Tweet<'a> {
    text: &'a str,
}

// tweet is a vector because it can be a thread
pub async fn setup_twitter(tweet: Vec<String>) {
    let file_path = "data/tokens/twitter.txt";

    if !Path::new(file_path).exists() {
        thread::spawn(move || {
                generate_tokens();
        }).join().expect("Thread panicked")
    } else {
        let contents = fs::read_to_string(file_path)
            .expect("Something went wrong reading the file");

        let tokens: Vec<&str> = contents.split("\n").collect();
        post_tweet(tokens[0], tweet.first().unwrap())
            .await.expect("Some error");
    }
}

async fn post_tweet(access_token: &str, tweet: &str) -> Result<(), Box<dyn std::error::Error>> {
    let endpoint = "https://api.twitter.com/2/tweets";

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

    let body = Tweet {
        text: tweet
    };

    let client = reqwest::Client::new();
    let request = client
                    .post(endpoint)
                    .bearer_auth(access_token)
                    .headers(headers)
                    .json(&body);

    println!("{:#?}", request);

    let response = request.send().await?;
    println!("{:#?}", response);

    Ok(())
}

fn generate_tokens() {
    let client_id = ClientId::new(CLIENT_ID.to_string());
    let client_secret = Some(ClientSecret::new(CLIENT_SECRET.to_string()));
    let auth_url = AuthUrl::new("https://twitter.com/i/oauth2/authorize".to_string())
            .expect("Error parsing auth url");
    let token_url = Some(TokenUrl::new("https://api.twitter.com/2/oauth2/token".to_string())
            .expect("Error parsing token url"));
    let redirect_url = RedirectUrl::new("http://127.0.0.1:8080".to_string())
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

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
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

            println!("Twitter returned the following code:\n{}\n", code.secret());
            println!(
                "Twitter returned the following state:\n{} (expected `{}`)\n",
                state.secret(),
                csrf_token.secret()
            );

            // Now you can trade it for an access token.
            let token_result =
                client
                .exchange_code(code)
                // Set the PKCE code verifier.
                .set_pkce_verifier(pkce_verifier)
                .request(http_client).expect("Error fetching Access token and Refresh token");

            println!("{:#?}", token_result);

            // println!("{}", token_result.access_token().secret());
            // println!("{}", token_result.refresh_token().unwrap().secret());

            save_tokens_to_file(token_result.access_token(), 
                                token_result.refresh_token().unwrap());

            break;
        }
    }
}

fn save_tokens_to_file(access_token: &AccessToken, refresh_token: &RefreshToken) {
    println!("{}", refresh_token.secret());
    println!("{}", access_token.secret());

    let mut file = File::create("data/tokens/twitter.txt").expect("Unable to create file");
    writeln!(&mut file, "{}", access_token.secret()).unwrap();
    writeln!(&mut file, "{}", refresh_token.secret()).unwrap();
}


#[cfg(test)]
mod test;