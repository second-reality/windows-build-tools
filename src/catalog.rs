use log::{info, warn};
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::path::{Path, PathBuf};

const TOOLCHAIN_PREFIX: &str = "Microsoft.VC.";
const CATALOG_NAME: &str = "VisualStudio.vsman";
const MANIFEST_ID: &str = "Microsoft.VisualStudio.Manifests.VisualStudio";
const CHANNEL_URL: &str = "https://aka.ms/vs/17/release/channel";
const X64: &str = "x64";
const ENGLISH: &str = "en-US";

#[derive(Deserialize, Debug, Clone)]
pub struct Payload {
    #[serde(rename = "fileName")]
    pub name: String,
    pub url: String,
    pub sha256: String,
}

impl Payload {
    pub fn path(&self, dl_dir: &str) -> PathBuf {
        let path = format!("{dl_dir}/{}", self.name);
        Path::new(&path).to_path_buf()
    }

    pub fn is_downloaded(&self, dl_dir: &str) -> bool {
        let file = self.path(dl_dir);
        if !file.exists() {
            return false;
        }

        use sha2::{Digest, Sha256};

        let mut hasher = Sha256::new();
        let mut file = std::fs::File::open(&file).unwrap();
        std::io::copy(&mut file, &mut hasher).unwrap();
        let sha_existing = format!("{:x}", hasher.finalize());
        sha_existing == self.sha256
    }

    pub fn download(&self, dl_dir: &str) {
        if self.is_downloaded(dl_dir) {
            return;
        }

        let file = self.path(dl_dir);
        if file.exists() {
            std::fs::remove_file(&file).unwrap();
        }
        let data = reqwest::blocking::get(&self.url).unwrap().bytes().unwrap();
        let mut out = std::fs::File::create(&file).unwrap();
        out.write_all(&data).unwrap();

        assert!(
            self.is_downloaded(dl_dir),
            "package downloaded hash is incorrect"
        )
    }
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
    pub payloads: Option<Vec<Payload>>,
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

    pub fn is_toolchain(&self) -> bool {
        self.id.starts_with(TOOLCHAIN_PREFIX)
    }

    pub fn is_matching_toolchain_version(&self, version: &str) -> bool {
        let prefix = format!("{TOOLCHAIN_PREFIX}{version}");
        self.id.starts_with(&prefix)
    }

    pub fn toolchain_version(&self) -> Option<String> {
        if !self.is_toolchain() {
            None
        } else {
            Some(
                self.id
                    .replace(TOOLCHAIN_PREFIX, "")
                    .split('.')
                    .take(2)
                    .map(|ver| ver.to_string())
                    .collect::<Vec<_>>()
                    .join("."),
            )
        }
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

    fn find_recursively_dependencies(&self, ids: HashSet<String>) -> HashSet<String> {
        let mut res: HashSet<String> = HashSet::new();
        for id in &ids {
            let package = self.get_package(id);
            res.insert(id.to_string());

            if let Some(package) = package {
                if let Some(deps) = package.dependencies {
                    for (dep_id, _) in deps {
                        res.insert(dep_id);
                    }
                }
            }
        }

        if res == ids {
            res
        } else {
            self.find_recursively_dependencies(res)
        }
    }

    pub fn get_packages_with_dependencies(&self, ids: Vec<String>) -> Vec<Package> {
        let mut all_ids: Vec<String> = self
            .find_recursively_dependencies(HashSet::from_iter(ids))
            .iter()
            .cloned()
            .collect();
        all_ids.sort();

        let mut res = vec![];
        for id in all_ids {
            let package = self.get_package(&id);

            if let Some(package) = package {
                res.push(package.to_owned());
            } else {
                warn!("package {id} does not exist (skipped)");
            }
        }
        res
    }

    pub fn get_package(&self, id: &str) -> Option<Package> {
        self.id_to_package.get(id).cloned()
    }
}
