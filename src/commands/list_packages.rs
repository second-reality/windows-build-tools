use super::catalog::get_catalog;
use std::collections::HashMap;

pub fn run() {
    let catalog = get_catalog();

    for p in catalog.packages.iter().filter(|p| p.is_english()) {
        let name = &p.id;
        let deps = p
            .dependencies
            .as_ref()
            .unwrap_or(&HashMap::new())
            .iter()
            .map(|(dep, _)| dep.clone())
            .collect::<Vec<_>>()
            .join(", ");
        println!("{name} (depends on: [{deps}])");
    }
}
