use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use mlua::{Lua, LuaSerdeExt, Result as LuaResult, Value as LuaValue};
use serde::{Deserialize, Serialize};
use smithay::input::keyboard::keysyms;
use smithay::reexports::wayland_server::protocol::wl_seat::KeyState;
use tracing::{debug, error, info};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct KeyBinding {
    pub modifiers: Vec<String>,
    pub key: String,
    pub action: String,
    pub args: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub keybindings: Vec<KeyBinding>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            keybindings: vec![
                KeyBinding {
                    modifiers: vec!["Logo".to_string()],
                    key: "Return".to_string(),
                    action: "spawn".to_string(),
                    args: Some({
                        let mut map = HashMap::new();
                        map.insert("command".to_string(), "alacritty".to_string());
                        map
                    }),
                },
                KeyBinding {
                    modifiers: vec!["Logo".to_string()],
                    key: "q".to_string(),
                    action: "quit".to_string(),
                    args: None,
                },
            ],
        }
    }
}

pub fn load_config(config_path: &Path) -> Config {
    if !config_path.exists() {
        info!("Config file not found, using default config");
        return Config::default();
    }

    let mut file = match File::open(config_path) {
        Ok(file) => file,
        Err(err) => {
            error!("Failed to open config file: {}", err);
            return Config::default();
        }
    };

    let mut contents = String::new();
    if let Err(err) = file.read_to_string(&mut contents) {
        error!("Failed to read config file: {}", err);
        return Config::default();
    }

    match parse_lua_config(&contents) {
        Ok(config) => {
            debug!("Loaded config: {:?}", config);
            config
        }
        Err(err) => {
            error!("Failed to parse config file: {}", err);
            Config::default()
        }
    }
}

fn parse_lua_config(lua_code: &str) -> LuaResult<Config> {
    let lua = Lua::new();
    lua.load(lua_code).exec()?;
    
    // Extract keybindings from Lua
    let globals = lua.globals();
    let config: Config = lua.from_value(globals.get("config")?)?;
    
    Ok(config)
}

pub fn parse_keysym(key: &str) -> Option<keysyms::Keysym> {
    match key.to_lowercase().as_str() {
        "return" => Some(keysyms::Return),
        "escape" => Some(keysyms::Escape),
        "backspace" => Some(keysyms::BackSpace),
        "tab" => Some(keysyms::Tab),
        "space" => Some(keysyms::space),
        "a" => Some(keysyms::a),
        "b" => Some(keysyms::b),
        "c" => Some(keysyms::c),
        "d" => Some(keysyms::d),
        "e" => Some(keysyms::e),
        "f" => Some(keysyms::f),
        "g" => Some(keysyms::g),
        "h" => Some(keysyms::h),
        "i" => Some(keysyms::i),
        "j" => Some(keysyms::j),
        "k" => Some(keysyms::k),
        "l" => Some(keysyms::l),
        "m" => Some(keysyms::m),
        "n" => Some(keysyms::n),
        "o" => Some(keysyms::o),
        "p" => Some(keysyms::p),
        "q" => Some(keysyms::q),
        "r" => Some(keysyms::r),
        "s" => Some(keysyms::s),
        "t" => Some(keysyms::t),
        "u" => Some(keysyms::u),
        "v" => Some(keysyms::v),
        "w" => Some(keysyms::w),
        "x" => Some(keysyms::x),
        "y" => Some(keysyms::y),
        "z" => Some(keysyms::z),
        "1" => Some(keysyms::_1),
        "2" => Some(keysyms::_2),
        "3" => Some(keysyms::_3),
        "4" => Some(keysyms::_4),
        "5" => Some(keysyms::_5),
        "6" => Some(keysyms::_6),
        "7" => Some(keysyms::_7),
        "8" => Some(keysyms::_8),
        "9" => Some(keysyms::_9),
        "0" => Some(keysyms::_0),
        _ => None,
    }
}

pub fn parse_modifier(modifier: &str) -> Option<smithay::input::keyboard::ModifiersState> {
    let mut state = smithay::input::keyboard::ModifiersState::default();
    match modifier.to_lowercase().as_str() {
        "shift" => {
            state.shift = true;
            Some(state)
        }
        "ctrl" | "control" => {
            state.ctrl = true;
            Some(state)
        }
        "alt" => {
            state.alt = true;
            Some(state)
        }
        "logo" | "super" | "win" => {
            state.logo = true;
            Some(state)
        }
        _ => None,
    }
}