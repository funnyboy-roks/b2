use std::{
    collections::HashMap,
    fs,
    io::{BufRead, Write},
    path::PathBuf,
};

use anyhow::bail;
use colored::Colorize;
use reqwest::blocking as reqwest;
use serde::{Deserialize, Serialize};

use crate::api::{self, Bucket};

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub key_id: String,
    pub key: String,
    pub api_url: String,
    pub download_url: String,
    pub auth_token: String,
    pub account_id: String,
    // Bucket Name : Bucket Id
    pub buckets: HashMap<String, String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            key_id: Default::default(),
            key: Default::default(),
            api_url: Default::default(),
            download_url: Default::default(),
            auth_token: Default::default(),
            account_id: Default::default(),
            buckets: Default::default(),
        }
    }
}

impl Config {
    pub fn load(file: Option<PathBuf>) -> anyhow::Result<Self> {
        let file = if let Some(file) = file {
            file
        } else {
            let Some(dir) = directories::ProjectDirs::from("com", "funnyboyroks", "b2") else {
                bail!("No config dir available");
            };
            let mut cfg = dir.config_dir().to_path_buf();
            fs::create_dir_all(&cfg)?;
            cfg.push("config.toml");
            cfg
        };
        if file.exists() {
            let content = fs::read_to_string(file)?;
            Ok(toml::from_str(&content)?)
        } else {
            Ok(Default::default())
        }
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let Some(dir) = directories::ProjectDirs::from("com", "funnyboyroks", "b2") else {
            bail!("No config dir available");
        };
        let mut cfg = dir.config_dir().to_path_buf();
        fs::create_dir_all(&cfg)?;
        cfg.push("config.toml");

        fs::write(cfg, toml::to_string_pretty(self)?)?;

        Ok(())
    }

    pub fn auth_from_stdin(&mut self) -> anyhow::Result<()> {
        print!("{}", "Backblaze application key ID: ".blue());
        std::io::stdout().flush()?;

        let mut key_id = String::with_capacity(25);
        std::io::stdin().lock().read_line(&mut key_id)?;
        let key_id = key_id.trim();
        println!("{}", key_id.red());

        print!("{}", "Backblaze application key: ".blue());
        std::io::stdout().flush()?;

        let mut key = String::with_capacity(32);
        std::io::stdin().lock().read_line(&mut key)?;
        let key = key.trim();
        println!("{}", key.red());

        self.authorise(key_id, key)?;

        println!("{}", "Authorised!".green());

        Ok(())
    }

    pub fn authorise(&mut self, key_id: &str, key: &str) -> anyhow::Result<()> {
        let authorise_url = "https://api.backblazeb2.com/b2api/v3/b2_authorize_account";

        let client = reqwest::Client::new()
            .get(authorise_url)
            .header("Authorization", get_auth(&self.key_id, &self.key))
            .send()?;

        let json: api::AuthResponse = client.json()?;

        self.key_id = key_id.to_string();
        self.key = key.to_string();
        self.api_url = json.api_info.storage_api.api_url.clone();
        self.download_url = json.api_info.storage_api.download_url.clone();
        self.auth_token = json.authorization_token.clone();
        self.account_id = json.account_id.clone();

        Ok(())
    }

    pub fn confirm_auth(&mut self) -> anyhow::Result<()> {
        if self.key.len() * self.key_id.len() == 0 {
            self.auth_from_stdin()?;
        }
        Ok(())
    }

    pub fn api_url(&mut self, api_name: &str) -> anyhow::Result<String> {
        self.confirm_auth()?;
        Ok(format!("{}/b2api/v3/{}", self.api_url, api_name))
    }

    /// Get a [`RequestBuilder`] for GET with the "Authorization" header set
    pub fn get(&mut self, api_name: &str) -> anyhow::Result<reqwest::RequestBuilder> {
        Ok(reqwest::Client::new()
            .get(self.api_url(api_name)?)
            .header("Authorization", &self.auth_token))
    }

    /// Get the list of buckets from the api
    pub fn get_buckets(&mut self) -> anyhow::Result<()> {
        let res = self
            .get("b2_list_buckets")?
            .query(&[("accountId", &self.account_id)])
            .send()?;

        let value = res.json::<serde_json::Value>()?;
        let buckets: Vec<Bucket> = Deserialize::deserialize(value["buckets"].clone())?;

        for bucket in buckets {
            self.buckets.insert(bucket.bucket_name, bucket.bucket_id);
        }

        Ok(())
    }

    /// Return the bucket id for a name, and fetch the latest buckets from the api if we don't have
    /// the name
    /// Returns None if the bucket does not exist
    pub fn get_bucket_id<'a>(&'a mut self, name: &str) -> anyhow::Result<Option<&'a str>> {
        if self.buckets.contains_key(name) {
            return Ok(Some(&self.buckets[name]));
        }

        self.get_buckets()?; // update our buckets to make sure the user has not created a new one

        Ok(self.buckets.get(name).map(|x| x.as_str()))
    }
}

fn get_auth(key_id: &str, key: &str) -> String {
    use base64::prelude::*;
    format!(
        "Basic{}",
        BASE64_STANDARD.encode(format!("{}:{}", key_id, key))
    )
}
