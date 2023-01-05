//! Configuration data for Linkdoku

use std::sync::Arc;

use config::{Config, ConfigError, Environment, File};
use itertools::Itertools;
use linked_hash_map::LinkedHashMap;
use serde::Deserialize;
use tracing::info;
use url::Url;

use crate::cli::Cli;

#[derive(Debug, Deserialize)]
pub struct OpenIDProvider {
    pub icon: String,
    pub client_id: String,
    pub client_secret: String,
    pub discovery_doc: String,
    pub scopes: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub port: u16,
    pub database_url: Url,
    pub base_url: Url,
    pub redirect_url: String,
    pub cookie_secret: String,
    pub openid: LinkedHashMap<String, OpenIDProvider>,
}

#[allow(unstable_name_collisions)]
impl OpenIDProvider {
    fn show(&self) {
        info!("  Icon: {}", self.icon);
        info!("  Client ID: {}", self.client_id);
        info!(
            "  Client Secret: {}",
            if self.client_secret.is_empty() {
                "MISSING"
            } else {
                "*****"
            }
        );
        info!("  Discovery document: {}", self.discovery_doc);
        info!(
            "  Scopes to request: {}",
            self.scopes
                .iter()
                .map(String::as_str)
                .intersperse(", ")
                .collect::<String>()
        );
    }
}

impl Configuration {
    pub fn show(&self) {
        info!("Listen on {}", self.port);
        info!("Base URL is {}", self.base_url);
        info!("Connect to database on {}", self.safe_database_url());
        info!("OpenID connect return url: {}", self.redirect_url);
        info!(
            "Cookie secret key: {}",
            if self.cookie_secret.is_empty() {
                "MISSING"
            } else {
                "*****"
            }
        );
        for (name, prov) in &self.openid {
            info!("OpenID provider: {}", name);
            prov.show();
        }
    }

    fn safe_database_url(&self) -> Url {
        let mut ret = self.database_url.clone();
        ret.set_password(Some("****")).unwrap();
        ret
    }
}

#[derive(Clone)]
pub struct ConfigState {
    inner: Arc<Configuration>,
}

impl std::ops::Deref for ConfigState {
    type Target = Configuration;

    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}

pub fn load_configuration(cli: &Cli) -> Result<ConfigState, ConfigError> {
    let config = Config::builder()
        .add_source(File::from(cli.config.as_path()))
        .add_source(
            Environment::default()
                .prefix("LINKDOKU")
                .separator("__")
                .try_parsing(true),
        );

    Ok(ConfigState {
        inner: Arc::new(config.build()?.try_deserialize()?),
    })
}
