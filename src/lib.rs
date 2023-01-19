include!(concat!(env!("OUT_DIR"), "/mappings.rs"));

/// Finds the human-readable name of a server from its address.
pub fn get_pretty_name(address: &str) -> Option<&str> {
    SERVER_MAPPINGS.get(address).copied()
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
    fn mineplex() {
        let result = get_pretty_name("mineplex.com");
        assert_eq!(result, Some("Mineplex"));
    }
}
