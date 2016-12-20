extern crate hlua;

use std::env;
use std::fs::File;
use hlua::{Lua, LuaError};

pub fn runScript(text: &str) -> Result<(), LuaError> {
    let mut lua = Lua::new();
    lua.openlibs();
    let dir = env::current_dir().unwrap();
    let seed = 1;
    let num_players = 2;
    let src = format!("
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
    ", dir = dir.display(), seed = seed, players = num_players);
    try!(lua.execute(&src));
    try!(lua.execute(text));
    try!(lua.execute("Arms:print()"));
    Ok(())
}
