use std::{collections::HashMap, fmt::Display};

use reqwest::blocking::{self as rq, RequestBuilder, Response};
use responses::SessionResponse;
use secrecy::{ExposeSecret, SecretString};
use serde::{de::DeserializeOwned, Deserialize};

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
    pub fn put<'b>(&self, route: &'b str) -> RequestBuilder {
        self.client
            .put(self.with_route(route))
            .header("authorization", format!("Bearer {}", self.token()))
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
    pub fn connected_device(&self, mac_address: &str) -> Option<ConnectedDevice> {
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
    pub fn restart(&self) -> Result<(), ()> {
        self.put("/action/1")
            .send()
            .map_or_else(|_| Err(()), |_| Ok(()))
    }
    pub fn update_admin_login(&self, username: &str, password: &str) -> Result<(), ()> {
        todo!()
    }
    pub fn set_remote_access(&self, on: bool) -> Result<(), ()> {
        todo!()
    }
    pub fn set_remote_ping(&self, on: bool) -> Result<(), ()> {
        todo!()
    }
    pub fn wan_status() -> Option<WanStatus> {
        todo!()
    }
    pub fn lan_status() -> Option<LanStatus> {
        todo!()
    }
    pub fn device() -> Option<Device> {
        todo!()
    }
}

pub struct DnsStatus {
    pub auto: bool,
    pub dns1: String,
    pub dns2: String,
    pub dns3: String,
}

pub struct WanStatus {
    pub id: String,
    pub mode: WanMode,
    pub gateway_v4: String, // IpAddr?
    pub gateway_v6: String,
    pub dns_v4: DnsStatus,
    pub dns_v6: DnsStatus,
}

pub enum WanMode {
    PPPoE,
    DHCP,
    Static,
}

pub struct InvalidWanMode;

impl TryFrom<usize> for WanMode {
    type Error = InvalidWanMode;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::DHCP),
            1 => Ok(Self::Static),
            2 => Ok(Self::PPPoE),
            _ => Err(InvalidWanMode),
        }
    }
}

pub struct LanStatus {
    pub id: String,
    pub ip4: String,
    pub ip6_count: usize,
    pub ip6_list: Vec<String>,
    pub mac: String,
    pub mtu: usize,
    pub netmask: String,
}

pub struct Device {
    pub id: String,
    pub model: String,
    pub uptime: usize,
    pub fw_version: String,
}

#[derive(Deserialize)]
pub struct ConnectedDevice {
    pub hostname: String,
    pub id: String,
    pub ip: String,
    pub mac: String,
}

impl Display for ConnectedDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = format!(
            "<===
            <hostname> {}
            [id] {}
            [ip] {}
            [mac] {}
            ===>",
            (if self.hostname.is_empty() {
                "UNKNOWN"
            } else {
                &self.hostname
            }),
            self.id,
            self.ip,
            self.mac
        );

        write!(f, "{text}")
    }
}

/// Tries to parse a `response`'s body to JSON.
fn parse_json_response<T: DeserializeOwned>(response: Response) -> Option<T> {
    response
        .json::<T>()
        .inspect_err(|_| eprintln!("failed to parse response."))
        .ok()
}
