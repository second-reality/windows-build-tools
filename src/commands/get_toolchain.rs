use super::catalog::{get_catalog, CatalogIndex, Package, Payload};
use log::info;
use rayon::prelude::*;

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

pub fn download_all(payloads: Vec<Payload>, dl_dir: &str) {
    let num_payloads = payloads.len();
    let payloads: Vec<_> = payloads
        .par_iter()
        .filter(|p| !p.is_downloaded(dl_dir))
        .collect();
    let num_not_found = payloads.len();

    if num_not_found == 0 {
        info!("all packages are already downloaded");
        return;
    } else {
        info!("need to download {num_not_found}/{num_payloads} payloads");
    }

    for (index, p) in payloads.iter().enumerate() {
        info!("[{}/{}] {}", index + 1, num_not_found, p.name);
        p.download(dl_dir); // NOT in parallel
    }

    info!("check packages downloaded");
    payloads
        .par_iter()
        .for_each(|p| assert!(p.is_downloaded(dl_dir), "package not downloaded"));
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

    let mut unique_payloads_name: Vec<String> = payloads.iter().map(|p| p.name.clone()).collect();
    unique_payloads_name.dedup();
    assert_eq!(
        payloads.len(),
        unique_payloads_name.len(),
        "some payloads have same name"
    );

    let dl_dir = format!("{install_dir}/downloads");
    std::fs::create_dir_all(&dl_dir).expect("error creating output directory");

    download_all(payloads, &dl_dir);
}
