use druid::{keyboard_types::KeyState, Code, KeyEvent, Modifiers};

enum BasicMods {
    Cmd,
    Alt,
    Shift,
}

pub enum Mods {
    None,
    Cmd,
    Alt,
    Shift,
    AltShift,
    CmdShift,
    CmdAlt,
    CmdAltShift,
}

impl Mods {
    fn to_basic(&self) -> Vec<BasicMods> {
        match self {
            Mods::Cmd => vec![BasicMods::Cmd],
            Mods::Shift => vec![BasicMods::Shift],
            Mods::Alt => vec![BasicMods::Alt],
            Mods::AltShift => vec![BasicMods::Alt, BasicMods::Shift],
            Mods::CmdShift => vec![BasicMods::Cmd, BasicMods::Shift],
            Mods::CmdAlt => vec![BasicMods::Cmd, BasicMods::Shift],
            Mods::CmdAltShift => vec![BasicMods::Cmd, BasicMods::Alt, BasicMods::Shift],
            Mods::None => Vec::new(),
        }
    }
}

impl PartialEq for Mods {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

pub struct HotKey {
    mods: Mods,
    key_code: Code,
    state: KeyState,
}

impl HotKey {
    pub fn new(mods: Mods, key_code: Code, state: KeyState) -> Self {
        HotKey {
            mods,
            key_code,
            state,
        }
    }

    pub fn matches(&self, key: &KeyEvent) -> bool {
        match key {
            key if (key.state == self.state
                && self.validate_modifiers(&key.mods)
                && key.code == self.key_code) =>
            {
                true
            }
            _ => false
        }
    }

    fn validate_modifiers(&self, mods: &Modifiers) -> bool {
        for key_val in self.mods.to_basic() {
            let is_pressed = match key_val {
                BasicMods::Cmd => mods.ctrl(),
                BasicMods::Alt => mods.alt(),
                BasicMods::Shift => mods.shift(),
            };

            if !is_pressed {
                return false;
            };
        }

        true
    }
}
