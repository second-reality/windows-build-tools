use log::info;
use serde::Deserialize;
use std::collections::HashMap;

const CATALOG_NAME: &str = "VisualStudio.vsman";
const MANIFEST_ID: &str = "Microsoft.VisualStudio.Manifests.VisualStudio";
const CHANNEL_URL: &str = "https://aka.ms/vs/17/release/channel";

#[derive(Deserialize, Debug)]
struct Payload {
    #[serde(rename = "fileName")]
    name: String,
    url: String,
}

#[derive(Deserialize, Debug)]
struct ChannelItem {
    id: String,
    payloads: Option<Vec<Payload>>,
}

#[derive(Deserialize, Debug)]
struct ChannelInfo {
    #[serde(rename = "channelItems")]
    items: Vec<ChannelItem>,
}

#[derive(Deserialize, Debug)]
pub struct Package {
    pub id: String,
    pub language: Option<String>,
    pub dependencies: Option<HashMap<String, serde_json::Value>>, // name, version (string or map)
}

impl Package {
    pub fn is_true_package(&self) -> bool {
        self.language.is_none()
    }
}

#[derive(Deserialize, Debug)]
pub struct Catalog {
    pub packages: Vec<Package>,
}

pub fn get_catalog() -> Catalog {
    let url = CHANNEL_URL;

    info!("download channel info from {url}");
    let channel_info = reqwest::blocking::get(url).unwrap().text().unwrap();
    let channel_info: ChannelInfo = serde_json::from_str(&channel_info).unwrap();

    let manifest = channel_info
        .items
        .iter()
        .find(|i| i.id == MANIFEST_ID)
        .unwrap();
    let payload = manifest
        .payloads
        .iter()
        .flatten()
        .find(|p| p.name == CATALOG_NAME)
        .unwrap();
    let catalog_url = &payload.url;

    info!("download catalog from {catalog_url}");
    let catalog = reqwest::blocking::get(catalog_url).unwrap().text().unwrap();
    serde_json::from_str(&catalog).unwrap()
}
