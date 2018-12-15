use std::collections::HashMap;
use std::collections::BTreeMap;

/// Creates the alias to be used
pub fn alias(aliases: &mut HashMap<String, String>, key: &str, value: String) -> Result<(), String> {
    println!("value: {}", value);
    aliases.insert(key.to_string(), value);
    return Ok(())
}

/// Returns an alias if it exists
pub fn get_aliased<'a>(aliases: &'a HashMap<String, String>, alias: &str) -> Option<&'a String>{
    return aliases.get(alias)
}

/// Displays all current aliases
pub fn show_aliases(aliases: &HashMap<String, String>) {
    let mut bt_map = BTreeMap::new();

    for (key, val) in aliases.iter() {
        bt_map.insert(key, val);
    }
    for (key, val) in bt_map.iter() {
        println!("{}='{}'", key, val);
    }
}