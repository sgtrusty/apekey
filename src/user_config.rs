// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use eyre::{eyre, Context, Result};
use serde::Deserialize;
use std::{env, fs, str};
use tracing::{debug, error, instrument};

// default values
const XMONAD_HS_PATH: &str = "~/.config/xmonad/xmonad.hs";
pub const FONT_SIZE: u16 = 20;
pub const TITLE_FONT_SIZE: u16 = 28;

#[derive(Deserialize, Debug, Clone)]
pub enum Theme {
    Dark,
    Light,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UserConfig {
    pub xmonad_config: String,
    pub font: Option<FontConfig>,
    pub theme: Option<Theme>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FontConfig {
    pub title_size: Option<u16>,
    pub section_size: Option<u16>,
    pub keybind_size: Option<u16>,
    pub text_size: Option<u16>,
    pub error_size: Option<u16>,
}

impl UserConfig {
    #[instrument]
    pub fn try_read() -> Result<Self> {
        let home = env::var("HOME").context("Environment variable HOME not set")?;
        let xdg_config_path =
            env::var("XDG_CONFIG_HOME").unwrap_or_else(|_| format!("{home}/.config"));
        let config_path = format!("{xdg_config_path}/apekey/apekey.toml");
        debug!("user config path {}", config_path);
        let content = fs::read(&config_path).context(config_path)?;
        toml::from_str::<UserConfig>(str::from_utf8(&content)?).map_err(|e| {
            error!("{}", e);
            eyre!("{e}")
        })
    }
}

impl Default for UserConfig {
    fn default() -> Self {
        UserConfig {
            xmonad_config: XMONAD_HS_PATH.into(),
            font: Some(FontConfig::default()),
            theme: None,
        }
    }
}

impl Default for FontConfig {
    fn default() -> Self {
        FontConfig {
            title_size: Some(TITLE_FONT_SIZE),
            section_size: Some(FONT_SIZE),
            keybind_size: Some(FONT_SIZE),
            text_size: Some(FONT_SIZE),
            error_size: Some(FONT_SIZE),
        }
    }
}
