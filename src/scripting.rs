extern crate hlua;

use std::env;
use std::fs::File;
use hlua::{Lua, LuaError};

// Whelp this function looks like a mess! :D
pub fn runScript(text: &str) -> Result<String, LuaError> {
    let mut lua = Lua::new();
    lua.openlibs();

    let dir = env::current_dir().unwrap();
    let seed = 1;
    let num_players = 2;
    try!(lua.execute(&format!("
        package.path = package.path .. ';{dir}/lua/?.lua'
        Arms = require('arms/main')
        Arms:_set_random_map_seed({seed})
        Arms:_set_number_of_players({players})

        -- Globals
        map = Arms.map
        trigger = Arms.trigger
        messages = Arms.messages
        terrain = Arms.terrain
        unit = Arms.unit
        messages = Arms.messages
    ",
        dir = dir.display(),
        seed = seed,
        players = num_players
    )));
    try!(lua.execute(text));

    // Should be able to move this value somehow with some lifetime fun, instead
    // of cloning?
    let result: String = try!(lua.execute("return require('arms/main'):to_string()"));
    Ok(result.clone())
}
