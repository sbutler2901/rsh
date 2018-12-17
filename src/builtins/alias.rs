use std::collections::HashMap;
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct Alias {
    pub cmd: String,
    pub args: Vec<String>
}

/// Creates the alias to be used
pub fn alias(aliases: &mut HashMap<String, Alias>, key: &str, value: String) -> Result<(), &'static str> {
    let mut cmd_str_iter= value.trim().split_whitespace();
    if let Some(cmd) = cmd_str_iter.next() {
        let mut alias = Alias {
            cmd: cmd.to_string(),
            args: cmd_str_iter.map(|arg| arg.to_string()).collect()
        };
        aliases.insert(key.to_string(), alias);
        return Ok(())
    }
    return Err("Command for alias not found");
}

/// Returns an alias if it exists
pub fn get_aliased<'a>(aliases: &'a HashMap<String, Alias>, alias: &str) -> Option<&'a Alias>{
    return aliases.get(alias)
}

/// Displays all current aliases
pub fn show_aliases(aliases: &HashMap<String, Alias>) {
    let mut bt_map = BTreeMap::new();

    for (key, val) in aliases.iter() {
        bt_map.insert(key, val);
    }
    for (key, val) in bt_map.iter() {
        println!("{}='{:#?}'", key, val);
    }
}