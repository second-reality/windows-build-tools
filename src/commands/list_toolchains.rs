use super::catalog::get_catalog;

const TOOLCHAIN_PREFIX: &str = "Microsoft.VC.";

pub fn run() {
    let catalog = get_catalog();

    let toolchain_packages = catalog
        .packages
        .iter()
        .filter(|p| p.id.starts_with(TOOLCHAIN_PREFIX));
    let mut versions: Vec<String> = toolchain_packages
        .map(|p| {
            p.id.replace(TOOLCHAIN_PREFIX, "")
                .split('.')
                .take(2)
                .map(|ver| ver.to_string())
                .collect::<Vec<_>>()
                .join(".")
        })
        .collect();

    versions.dedup();
    versions.sort();

    for v in versions {
        println!("{v}");
    }
}
