use std::collections::HashMap;

use reqwest::blocking::{self as rq, RequestBuilder, Response};
use responses::{ConnectedDevice, SessionResponse};
use secrecy::{ExposeSecret, SecretString};
use serde::de::DeserializeOwned;

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
    pub client: rq::Client,
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
    pub fn get<'b>(&self, route: &'b str) -> RequestBuilder {
        self.client
            .get(self.with_route(route))
            .header("authorization", format!("Bearer {}", self.token()))
    }
    pub fn post<'b>(&self, route: &'b str) -> RequestBuilder {
        self.client.post(self.with_route(route))
    }
    pub fn connected_devices(&self) -> Option<Vec<ConnectedDevice>> {
        self.get("/connected_device").send().map_or_else(
            |e| {
                eprintln!("failed to GET /connected_device: {e:?}");
                None
            },
            parse_json_response,
        )
    }
    pub fn device(&self, mac_address: &str) -> Option<ConnectedDevice> {
        self.get(&format!("/connected_device/{mac_address}"))
            .send()
            .map_or_else(
                |e| {
                    eprintln!("failed to GET /connected_device: {e:?}");
                    None
                },
                parse_json_response,
            )
    }
}

/// Tries to parse a `response`'s body to JSON.
fn parse_json_response<T: DeserializeOwned>(response: Response) -> Option<T> {
    response
        .json::<T>()
        .inspect_err(|_| eprintln!("failed to parse response."))
        .ok()
}
