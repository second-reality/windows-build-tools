use crate::commands::catalog::get_catalog;
use std::collections::HashMap;

pub fn run() {
    let catalog = get_catalog();

    for p in catalog.packages.iter().filter(|p| p.is_true_package()) {
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
