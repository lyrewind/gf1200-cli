use std::collections::HashMap;

use reqwest::blocking as rq;
use responses::SessionResponse;
use secrecy::{ExposeSecret, SecretString};

pub mod responses;

pub trait ApiState {}
pub struct Unauthenticated;
pub struct Authenticated {
    token: SecretString,
}
impl ApiState for Unauthenticated {}
impl ApiState for Authenticated {}
pub struct Api<'a, State: ApiState> {
    url: &'a str,
    client: rq::Client,
    state: State,
}
impl<State: ApiState> Api<'_, State> {
    pub fn with_route(&self, route: &str) -> String {
        format!("{}{}", self.url, route)
    }
}
impl<'a> Api<'a, Unauthenticated> {
    pub fn new() -> Self {
        Self {
            url: "http://10.0.0.1/api/v1",
            client: rq::Client::new(),
            state: Unauthenticated,
        }
    }
    pub fn authenticate(self, username: &str, password: &str) -> Api<'a, Authenticated> {
        let token = self.get_token(username, password);
        Api {
            url: self.url,
            client: self.client,
            state: Authenticated { token },
        }
    }
    fn get_token(&self, username: &str, password: &str) -> SecretString {
        let mut body = HashMap::new();
        body.insert("username", username);
        body.insert("password", password);

        self.client
            .post(self.with_route("/session"))
            .json(&body)
            .send()
            .inspect_err(|e| eprintln!("can't send /session: {e}"))
            .unwrap()
            .error_for_status()
            .map_or_else(
                |_| {
                    eprintln!("login failed. (are the credentials correct?)");
                    std::process::exit(0);
                },
                |response| {
                    response
                        .json::<SessionResponse>()
                        .inspect_err(|e| eprintln!("can't parse /session response: {e}"))
                        .unwrap()
                        .token
                        .into()
                },
            )
    }
}
impl<'a> Api<'a, Authenticated> {
    pub fn token(&self) -> String {
        self.state.token.expose_secret().to_string()
    }
}
