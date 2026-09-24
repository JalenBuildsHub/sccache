use crate::errors::*;
use opendal::HttpTransporter;
use opendal_http_transport_reqwest::ReqwestTransport;
use reqwest::{Client, ClientBuilder};

/// Build an HTTP transport with a custom user agent (helps with monitoring on
/// the server side).
pub fn set_user_agent() -> Result<HttpTransporter> {
    let user_agent = format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    let client = build_client(ClientBuilder::new(), &user_agent)?;
    Ok(HttpTransporter::new(ReqwestTransport::new(client)))
}

fn build_client(builder: ClientBuilder, user_agent: &str) -> Result<Client> {
    Ok(builder
        .user_agent(user_agent)
        .build()
        .context("failed to build HTTP client")?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_build_errors_are_returned() {
        assert!(build_client(ClientBuilder::new(), "invalid\nuser-agent").is_err());
    }
}
