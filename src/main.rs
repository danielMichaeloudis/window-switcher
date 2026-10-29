use std::{mem::MaybeUninit, str::FromStr, sync::atomic::AtomicI32, time::Instant};

use handy_keys::{Error, Hotkey, HotkeyId, HotkeyManager, HotkeyState, Key, Modifiers};
use x_win::get_active_window;

use crate::open_windows::{get_open_windows, set_active_window};

mod open_windows;

static KEYS: [AtomicI32; 10] = [const { AtomicI32::new(0) }; 10];

fn main() {
    let mut key_windows: [(Option<HotkeyId>, _); _] = [(None, 0); 10];

    let hotkeys = get_hotkeys().expect("Failed to init hotkeys");

    while let Ok(hotkey) = hotkeys.recv() {
        if hotkey.state != HotkeyState::Pressed {
            continue;
        }
        let Hotkey { modifiers, key } = hotkeys
            .get_hotkey(hotkey.id)
            .expect("We got hotkey id from recv fn");
        // Handle setting window key.
        if modifiers.contains(Modifiers::CTRL)
            && let Some(idx) = key.and_then(|k| k.to_string().parse::<usize>().ok())
        {
            if let Ok(current) = get_active_window() {
                if let (Some(hotkey), _) = key_windows[idx] {
                    let _ = hotkeys.unregister(hotkey);
                }
                key_windows[idx] = (
                    Some(
                        hotkeys
                            .register(Hotkey::new(Modifiers::OPT, key).unwrap())
                            .expect("Just unregistered hotkey"),
                    ),
                    current.id,
                );
            } else {
                eprintln!("Failed to get current");
            }
            continue;
        }

        if let Some(idx) = key.and_then(|k| k.to_string().parse::<usize>().ok())
            && key_windows[idx].0.is_some()
        {
            println!("Setting window: {}", key_windows[idx].1);
            if !set_active_window(key_windows[idx].1) {
                eprintln!("Error setting window")
            }
        }
    }
}

fn get_hotkeys() -> Result<HotkeyManager, Error> {
    let man = HotkeyManager::new()?;
    man.register(Hotkey::new(Modifiers::CTRL | Modifiers::OPT, Key::Num0)?)?;
    man.register(Hotkey::new(Modifiers::CTRL | Modifiers::OPT, Key::Num1)?)?;
    man.register(Hotkey::new(Modifiers::CTRL | Modifiers::OPT, Key::Num2)?)?;
    man.register(Hotkey::new(Modifiers::CTRL | Modifiers::OPT, Key::Num3)?)?;
    man.register(Hotkey::new(Modifiers::CTRL | Modifiers::OPT, Key::Num4)?)?;
    man.register(Hotkey::new(Modifiers::CTRL | Modifiers::OPT, Key::Num5)?)?;
    man.register(Hotkey::new(Modifiers::CTRL | Modifiers::OPT, Key::Num6)?)?;
    man.register(Hotkey::new(Modifiers::CTRL | Modifiers::OPT, Key::Num7)?)?;
    man.register(Hotkey::new(Modifiers::CTRL | Modifiers::OPT, Key::Num8)?)?;
    man.register(Hotkey::new(Modifiers::CTRL | Modifiers::OPT, Key::Num9)?)?;
    Ok(man)
}
