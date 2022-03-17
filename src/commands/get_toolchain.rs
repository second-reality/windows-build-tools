use super::catalog::{get_catalog, CatalogIndex};
use log::info;

pub fn run(toolchain_version: String, install_dir: String) {
    let catalog = get_catalog();
    let index = CatalogIndex::new(&catalog);

    info!("get toolchain version {toolchain_version} in {install_dir}");

    assert!(index.get_package("hi!").is_none());
    todo!();
}
