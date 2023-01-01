//! Configuration data for Linkdoku

use std::{collections::HashMap, sync::Arc};

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use url::Url;

use crate::cli::Cli;

#[derive(Debug, Deserialize)]
pub struct OpenIDProvider {
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
    pub openid: HashMap<String, OpenIDProvider>,
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
