use log::info;
use serde_json::Value;

const CATALOG_NAME: &str = "VisualStudio.vsman";
const MANIFEST_ID: &str = "Microsoft.VisualStudio.Manifests.VisualStudio";
const CHANNEL_URL: &str = "https://aka.ms/vs/17/release/channel";

fn get_catalog() -> serde_json::Value {
    let url = CHANNEL_URL;

    info!("download channel info from {url}");

    let channel_info = reqwest::blocking::get(url).unwrap().text().unwrap();
    let channel_info: Value = serde_json::from_str(&channel_info).unwrap();

    let items = channel_info["channelItems"].as_array().unwrap();
    let manifest = items.first().unwrap();

    assert_eq!(manifest["id"].as_str().unwrap(), MANIFEST_ID);
    let payload = manifest["payloads"].as_array().unwrap().first().unwrap();
    assert_eq!(payload["fileName"].as_str().unwrap(), CATALOG_NAME);
    let catalog_url = payload["url"].as_str().unwrap();

    info!("download catalog from {catalog_url}");
    let catalog = reqwest::blocking::get(catalog_url).unwrap().text().unwrap();
    let catalog: Value = serde_json::from_str(&catalog).unwrap();

    catalog
}

pub fn list_packages() {
    let catalog = get_catalog();

    let packages = serde_json::to_string_pretty(&catalog["packages"]).unwrap();
    println!("{packages}");
}
