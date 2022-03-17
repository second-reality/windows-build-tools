use super::catalog::{get_catalog, CatalogIndex, Package, Payload};
use log::info;
use std::io::Write;
use std::path::Path;

enum PayloadKind {
    Vsix,
    Unknown(String),
}

fn payload_kind(payload: &Payload) -> PayloadKind {
    let extension = Path::new(&payload.url).extension();
    match extension {
        None => PayloadKind::Unknown("".to_string()),
        Some(os_str) => match os_str.to_str() {
            Some("vsix") => PayloadKind::Vsix,
            Some(other) => PayloadKind::Unknown(other.to_string()),
            _ => PayloadKind::Unknown("".to_string()),
        },
    }
}

fn download_payload(payload: &Payload, install_dir: &str) {
    let dl_dir = format!("{install_dir}/downloads");
    std::fs::create_dir_all(&dl_dir).expect("error creating output directory");

    let kind = payload_kind(payload);

    let file_ext = match kind {
        PayloadKind::Vsix => "vsix",
        _ => unimplemented!(),
    };

    let path_to_file = format!("{dl_dir}/{}.{}", payload.sha256, file_ext);
    let file = Path::new(&path_to_file);

    let need_download = if !file.exists() {
        true
    } else {
        let sha_existing = sha256::digest_file(file).unwrap();
        sha_existing != payload.sha256
    };

    if !need_download {
        return;
    }

    if file.exists() {
        std::fs::remove_file(file).unwrap();
    }
    let data = reqwest::blocking::get(&payload.url)
        .unwrap()
        .bytes()
        .unwrap();
    let mut out = std::fs::File::create(file).unwrap();
    out.write_all(&data).unwrap();
    assert_eq!(
        sha256::digest_file(file).unwrap(),
        payload.sha256,
        "sha256 of downloaded file is different from expected"
    );
}

pub fn find_packages_to_download(toolchain_version: &str) -> Vec<Package> {
    let catalog = get_catalog();
    let catalog_index = CatalogIndex::new(&catalog);

    let mut names: Vec<String> = catalog
        .packages
        .iter()
        .filter(|p| p.is_matching_toolchain_version(toolchain_version))
        .map(|p| p.id.clone())
        .collect();
    names.dedup();

    catalog_index.get_packages_with_dependencies(names)
}

pub fn run(toolchain_version: String, install_dir: String) {
    info!("get toolchain version {toolchain_version} in {install_dir}");

    let packages = find_packages_to_download(&toolchain_version);
    if packages.is_empty() {
        panic!(
            "no package found matching toolchain version {}",
            toolchain_version
        );
    }

    let payloads: Vec<Payload> = packages
        .iter()
        .flat_map(|p| p.payloads.as_ref().unwrap())
        .cloned()
        .collect();
    info!(
        "need to download {} payloads for {} packages",
        payloads.len(),
        packages.len()
    );

    for (index, payload) in payloads.iter().enumerate() {
        info!("[{}/{}] {}", index + 1, payloads.len(), payload.name);
        download_payload(payload, &install_dir);
    }
}
