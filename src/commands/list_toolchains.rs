use super::catalog::get_catalog;

pub fn run() {
    let catalog = get_catalog();

    let mut versions: Vec<String> = catalog
        .packages
        .iter()
        .filter_map(|p| p.toolchain_version())
        .collect();

    versions.dedup();
    versions.sort();

    for v in versions {
        println!("{v}");
    }
}
