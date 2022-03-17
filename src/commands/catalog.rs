use log::{info, warn};
use serde::Deserialize;
use std::collections::HashMap;

const CATALOG_NAME: &str = "VisualStudio.vsman";
const MANIFEST_ID: &str = "Microsoft.VisualStudio.Manifests.VisualStudio";
const CHANNEL_URL: &str = "https://aka.ms/vs/17/release/channel";
const X64: &str = "x64";
const ENGLISH: &str = "en-US";

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

#[derive(Deserialize, Debug, Clone)]
pub struct Package {
    pub id: String,
    pub language: Option<String>,
    pub chip: Option<String>,
    #[serde(rename = "productArch")]
    pub product_arch: Option<String>,
    #[serde(rename = "machineArch")]
    pub machine_arch: Option<String>,
    pub dependencies: Option<HashMap<String, serde_json::Value>>, // name, version (string or map)
}

impl Package {
    pub fn is_english(&self) -> bool {
        self.language.is_none() || self.language.as_ref().unwrap() == ENGLISH
    }

    pub fn is_x64(&self) -> bool {
        (self.chip.is_none() || self.chip.as_ref().unwrap() == X64)
            && (self.product_arch.is_none() || self.product_arch.as_ref().unwrap() == X64)
            && (self.machine_arch.is_none() || self.machine_arch.as_ref().unwrap() == X64)
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

pub struct CatalogIndex {
    id_to_package: HashMap<String, Package>,
}

impl CatalogIndex {
    pub fn new(catalog: &Catalog) -> Self {
        let mut res = CatalogIndex {
            id_to_package: HashMap::default(),
        };
        for p in &catalog.packages {
            res.add_package(p);
        }
        res
    }

    fn add_package(&mut self, package: &Package) {
        if !package.is_english() || !package.is_x64() {
            return;
        }

        if self.id_to_package.contains_key(&package.id) {
            warn!("package {} is present twice", package.id);
            return;
        }
        self.id_to_package
            .insert(package.id.clone(), package.clone());
    }

    pub fn get_package(&self, id: &str) -> Option<&Package> {
        self.id_to_package.get(id)
    }
}
