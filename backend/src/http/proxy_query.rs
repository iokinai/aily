use serde::Deserialize;
use trl::Trl;

#[derive(Deserialize, Trl)]
#[getters]
pub struct ProxyQuery {
    url: String,
}
