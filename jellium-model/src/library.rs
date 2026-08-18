//! The plugin ranks a library's own options carry.

/// `offered` in the rank `order` names.
// a plugin the order does not name stands before every plugin it does, in the
// server's own order
// reference: library-options-plugin-order
pub fn ranked(offered: &[String], order: &[String]) -> Vec<String> {
    let mut ranked = offered.to_vec();
    ranked.sort_by_key(|name| order.iter().position(|held| held == name));
    ranked
}

#[cfg(test)]
mod tests {
    use super::*;

    fn named(names: &[&str]) -> Vec<String> {
        names.iter().map(|name| (*name).to_owned()).collect()
    }

    #[test]
    fn a_plugin_the_order_does_not_name_stands_before_every_plugin_it_does() {
        assert_eq!(
            ranked(&named(&["one", "two", "three"]), &named(&["three", "one"])),
            named(&["two", "three", "one"])
        );
    }

    #[test]
    fn plugins_the_order_names_none_of_keep_the_servers_own_order() {
        assert_eq!(
            ranked(&named(&["one", "two"]), &named(&[])),
            named(&["one", "two"])
        );
    }

    #[test]
    fn an_order_naming_an_absent_plugin_ignores_it() {
        assert_eq!(
            ranked(&named(&["one", "two"]), &named(&["nine", "two"])),
            named(&["one", "two"])
        );
    }
}
