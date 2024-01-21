include!(concat!(env!("OUT_DIR"), "/mappings.rs"));

/// Finds the human-readable name of a server from its address.
pub fn get_pretty_name(address: &str) -> Option<&str> {
    let address_parts = address.split('.').collect::<Vec<_>>();

    for part_count in (1..=address_parts.len()).rev() {
        let address = address_parts[address_parts.len() - part_count..].join(".");
        if let Some(name) = SERVER_MAPPINGS.get(&address) {
            return Some(name);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::get_pretty_name;

    #[test]
    fn hypixel() {
        let result = get_pretty_name("mc.hypixel.net");
        assert_eq!(result, Some("Hypixel"));
    }

    #[test]
    fn hypixel_bare() {
        let result = get_pretty_name("hypixel.net");
        assert_eq!(result, Some("Hypixel"));
    }

    #[test]
    fn cubecraft() {
        let result = get_pretty_name("play.cubecraft.net");
        assert_eq!(result, Some("CubeCraft Games"));
    }

    #[test]
    fn cubecraft_bare() {
        let result = get_pretty_name("cubecraft.net");
        assert_eq!(result, Some("CubeCraft Games"));
    }

    #[test]
    #[ignore = "mineplex has been shut down"]
    fn mineplex() {
        let result = get_pretty_name("us.mineplex.com");
        assert_eq!(result, Some("Mineplex"));
    }

    #[test]
    #[ignore = "mineplex has been shut down"]
    fn mineplex_bare() {
        let result = get_pretty_name("mineplex.com");
        assert_eq!(result, Some("Mineplex"));
    }
}
