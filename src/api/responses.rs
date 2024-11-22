use std::fmt::Display;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct SessionResponse {
    pub token: String,
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
