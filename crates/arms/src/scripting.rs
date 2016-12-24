use std::io::{Error, ErrorKind};
use std::result::Result;
use lua;

fn to_result(status: lua::ThreadStatus) -> Result<(), Error> {
    match status {
        lua::ThreadStatus::Ok => Ok(()),
        lua::ThreadStatus::RuntimeError => Err(Error::new(ErrorKind::Other, "Runtime Error")),
        lua::ThreadStatus::SyntaxError => Err(Error::new(ErrorKind::Other, "Syntax Error")),
        lua::ThreadStatus::MemoryError => Err(Error::new(ErrorKind::Other, "Memory Error")),
        lua::ThreadStatus::GcError => Err(Error::new(ErrorKind::Other, "Gc Error")),
        lua::ThreadStatus::MessageHandlerError => Err(Error::new(ErrorKind::Other, "MessageHandler Error")),
        lua::ThreadStatus::FileError => Err(Error::new(ErrorKind::Other, "File Error")),
        _ => Err(Error::new(ErrorKind::Other, "Unknown Error")),
    }
}

fn load_arms_library(mut state: &mut lua::State) -> Result<(), Error> {
    let library = include_bytes!("../../../rocks/arms/arms.out");

    try!(to_result(state.load_bufferx(library, "arms", "b")));
    try!(to_result(state.pcall(0, 0, 0)));
    Ok(())
}

fn load_prelude(mut state: &mut lua::State) -> Result<(), Error> {
    let prelude = "
        Arms = require 'arms'
        map = Arms.map
        trigger = Arms.trigger
        messages = Arms.messages
        terrain = Arms.terrain
        unit = Arms.unit
        messages = Arms.messages
    ";
    to_result(state.do_string(prelude))
}

fn load_config(mut state: &mut lua::State, seed: u32, num_players: u8) -> Result<(), Error> {
    let config = &format!("
        Arms:_set_random_map_seed({seed})
        Arms:_set_number_of_players({players})
    ", seed = seed, players = num_players);
    to_result(state.do_string(config))
}

// Validate source code by parsing it.
fn validate_source(mut state: &mut lua::State, source: &str) -> Result<(), Error> {
    // Attempt to load the source.
    try!(to_result(state.load_string(source)));
    // Unload source if it parsed successfully.
    state.pop(1);
    Ok(())
}

fn run_noreturn(mut state: &mut lua::State, text: &str) -> Result<(), Error> {
    try!(validate_source(state, text));
    // Wrap the source in a function so top-level returns don't break us.
    to_result(state.do_string(&format!("
        (function()
            {}
        end)()
    ", text)))
}

// Whelp this function looks like a mess! :D
pub fn run_lua(text: &str) -> Result<String, Error> {
    let mut lua = lua::State::new();
    lua.open_libs(); // TODO don't open os/io/package perhaps

    let result = load_arms_library(&mut lua)
        .and_then(|_| load_prelude(&mut lua))
        .and_then(|_| load_config(&mut lua, 1, 2))
        .and_then(|_| run_noreturn(&mut lua, text))
        .and_then(|_| to_result(lua.do_string("return require('arms'):to_string()")))
        .or_else(|err| {
            println!("Error: {}", lua.to_str(1).unwrap());
            Err(err)
        });

    try!(result);

    Ok(String::from(lua.check_string(1)))
}
