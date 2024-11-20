use crate::api::{Api, Authenticated};

pub struct AppState<'a> {
    pub api: Api<'a, Authenticated>
}
