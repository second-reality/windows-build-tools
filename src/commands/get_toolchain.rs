use super::catalog::{get_catalog, CatalogIndex};

pub fn run() {
    let catalog = get_catalog();
    let index = CatalogIndex::new(&catalog);
    assert!(index.get_package("hi!").is_none());
    todo!();
}
