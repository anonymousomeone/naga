use std::sync::{LazyLock, Mutex, mpsc::{self, Sender, Receiver}};
use eframe::egui;


pub struct Keyboard {
    #[cfg(target_os = "linux")]
    pub hook: linux_hook::LinuxKeyboardHook,
    #[cfg(target_os = "windows")]
    pub hook: windows_hook::WindowsKeyboardHook,
}

impl Keyboard {
    pub fn new() -> Self {
        #[cfg(target_os = "linux")]
        let hook = linux_hook::LinuxKeyboardHook::new();
        #[cfg(target_os = "windows")]
        let hook = windows_hook::WindowsKeyboardHook::new();

        Self {
            hook,
        }
    }
}
pub trait KeyboardHook {
    fn new() -> Self;
    fn poll(&mut self) -> Vec<egui::Event>;
    fn hook();
    fn unhook();
    fn uninstall_hook(&self);
}

// #[derive(Clone, Copy, PartialEq)]
// pub enum KeyState {
//     Up,
//     Down
// }

// #[derive(Clone, Copy)]
// pub struct Key {
//     pub key_code: u8,
//     pub pressed: bool,
// }

// impl Key {
//     pub fn new(key_code: u8, pressed: bool) -> Key {
//         Key { 
//             key_code,
//             pressed,
//         }
//     }
// }
static mut CHANNEL: LazyLock<(Sender<(u32, bool)>, Receiver<(u32, bool)>)> = LazyLock::new(|| {mpsc::channel() });
static mut ENABLED: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::from(false));

#[cfg(target_os = "linux")]
pub mod linux_hook {
    use evdev::Device;

    pub struct LinuxKeyboardHook {
        pub key_states: HashMap<u8, Key>,
        pub state_changes: Vec<Key>,
    }
    
    impl KeyboardHook for LinuxKeyboardHook {
        fn new() -> Self {
    
        }
    }

    /// Convert Linux input event keycodes to egui Key
    ///
    /// Based on the Linux input event codes from input-event-codes.h
    /// and egui's Key enum.
    ///
    /// Note: This converter handles the most common keys that have direct
    /// egui equivalents. Some Linux keycodes don't have egui equivalents.

    pub fn linux_keycode_to_egui(keycode: u16) -> Option<egui::Key> {
        use egui::Key;
        
        match keycode {
            // Navigation/Command keys
            103 => Some(Key::ArrowUp),      // KEY_UP
            108 => Some(Key::ArrowDown),    // KEY_DOWN
            105 => Some(Key::ArrowLeft),    // KEY_LEFT
            106 => Some(Key::ArrowRight),   // KEY_RIGHT
            1   => Some(Key::Escape),       // KEY_ESC
            15  => Some(Key::Tab),          // KEY_TAB
            14  => Some(Key::Backspace),    // KEY_BACKSPACE
            28  => Some(Key::Enter),        // KEY_ENTER
            96  => Some(Key::Enter),        // KEY_KPENTER (numpad enter)
            110 => Some(Key::Insert),       // KEY_INSERT
            111 => Some(Key::Delete),       // KEY_DELETE
            102 => Some(Key::Home),         // KEY_HOME
            107 => Some(Key::End),          // KEY_END
            104 => Some(Key::PageUp),       // KEY_PAGEUP
            109 => Some(Key::PageDown),     // KEY_PAGEDOWN
            
            // Copy/Cut/Paste
            133 => Some(Key::Copy),         // KEY_COPY
            137 => Some(Key::Cut),          // KEY_CUT
            135 => Some(Key::Paste),        // KEY_PASTE
            
            // Space
            57  => Some(Key::Space),        // KEY_SPACE
            
            // Punctuation
            39  => Some(Key::Semicolon),    // KEY_SEMICOLON
            51  => Some(Key::Comma),        // KEY_COMMA
            12  => Some(Key::Minus),        // KEY_MINUS
            52  => Some(Key::Period),       // KEY_DOT
            78  => Some(Key::Plus),         // KEY_KPPLUS
            13  => Some(Key::Equals),       // KEY_EQUAL
            26  => Some(Key::OpenBracket),  // KEY_LEFTBRACE
            27  => Some(Key::CloseBracket), // KEY_RIGHTBRACE
            41  => Some(Key::Backtick),     // KEY_GRAVE
            43  => Some(Key::Backslash),    // KEY_BACKSLASH
            53  => Some(Key::Slash),        // KEY_SLASH
            40  => Some(Key::Quote),        // KEY_APOSTROPHE
            
            // Digits (main keyboard)
            11  => Some(Key::Num0),         // KEY_0
            2   => Some(Key::Num1),         // KEY_1
            3   => Some(Key::Num2),         // KEY_2
            4   => Some(Key::Num3),         // KEY_3
            5   => Some(Key::Num4),         // KEY_4
            6   => Some(Key::Num5),         // KEY_5
            7   => Some(Key::Num6),         // KEY_6
            8   => Some(Key::Num7),         // KEY_7
            9   => Some(Key::Num8),         // KEY_8
            10  => Some(Key::Num9),         // KEY_9
            
            // Digits (numpad)
            82  => Some(Key::Num0),         // KEY_KP0
            79  => Some(Key::Num1),         // KEY_KP1
            80  => Some(Key::Num2),         // KEY_KP2
            81  => Some(Key::Num3),         // KEY_KP3
            75  => Some(Key::Num4),         // KEY_KP4
            76  => Some(Key::Num5),         // KEY_KP5
            77  => Some(Key::Num6),         // KEY_KP6
            71  => Some(Key::Num7),         // KEY_KP7
            72  => Some(Key::Num8),         // KEY_KP8
            73  => Some(Key::Num9),         // KEY_KP9
            
            // Letters
            30  => Some(Key::A),            // KEY_A
            48  => Some(Key::B),            // KEY_B
            46  => Some(Key::C),            // KEY_C
            32  => Some(Key::D),            // KEY_D
            18  => Some(Key::E),            // KEY_E
            33  => Some(Key::F),            // KEY_F
            34  => Some(Key::G),            // KEY_G
            35  => Some(Key::H),            // KEY_H
            23  => Some(Key::I),            // KEY_I
            36  => Some(Key::J),            // KEY_J
            37  => Some(Key::K),            // KEY_K
            38  => Some(Key::L),            // KEY_L
            50  => Some(Key::M),            // KEY_M
            49  => Some(Key::N),            // KEY_N
            24  => Some(Key::O),            // KEY_O
            25  => Some(Key::P),            // KEY_P
            16  => Some(Key::Q),            // KEY_Q
            19  => Some(Key::R),            // KEY_R
            31  => Some(Key::S),            // KEY_S
            20  => Some(Key::T),            // KEY_T
            22  => Some(Key::U),            // KEY_U
            47  => Some(Key::V),            // KEY_V
            17  => Some(Key::W),            // KEY_W
            45  => Some(Key::X),            // KEY_X
            21  => Some(Key::Y),            // KEY_Y
            44  => Some(Key::Z),            // KEY_Z
            
            // Function keys
            59  => Some(Key::F1),           // KEY_F1
            60  => Some(Key::F2),           // KEY_F2
            61  => Some(Key::F3),           // KEY_F3
            62  => Some(Key::F4),           // KEY_F4
            63  => Some(Key::F5),           // KEY_F5
            64  => Some(Key::F6),           // KEY_F6
            65  => Some(Key::F7),           // KEY_F7
            66  => Some(Key::F8),           // KEY_F8
            67  => Some(Key::F9),           // KEY_F9
            68  => Some(Key::F10),          // KEY_F10
            87  => Some(Key::F11),          // KEY_F11
            88  => Some(Key::F12),          // KEY_F12
            183 => Some(Key::F13),          // KEY_F13
            184 => Some(Key::F14),          // KEY_F14
            185 => Some(Key::F15),          // KEY_F15
            186 => Some(Key::F16),          // KEY_F16
            187 => Some(Key::F17),          // KEY_F17
            188 => Some(Key::F18),          // KEY_F18
            189 => Some(Key::F19),          // KEY_F19
            190 => Some(Key::F20),          // KEY_F20
            
            // Navigation keys
            158 => Some(Key::BrowserBack),  // KEY_BACK
            
            // All other keycodes don't have direct egui equivalents
            _ => None,
        }
    }

    /// Additional helper to check if a keycode represents a modifier key
    /// (which egui handles separately via Modifiers)
    pub fn is_modifier_key(keycode: u16) -> bool {
        matches!(
            keycode,
            29 |  // KEY_LEFTCTRL
            97 |  // KEY_RIGHTCTRL
            42 |  // KEY_LEFTSHIFT
            54 |  // KEY_RIGHTSHIFT
            56 |  // KEY_LEFTALT
            100 | // KEY_RIGHTALT
            125 | // KEY_LEFTMETA
            126   // KEY_RIGHTMETA
        )
    }
}


#[cfg(target_os = "windows")]
pub mod windows_hook {
    use super::KeyboardHook;
    use super::CHANNEL;
    use super::ENABLED;

    use eframe::egui::Key;
    use eframe::egui::Event;
    use eframe::egui::Modifiers;
    use std::collections::HashMap;
    use std::thread;
    use std::time::Duration;
    use std::{mem, ptr};
    use winapi::ctypes::c_int;
    use winapi::shared::minwindef::{HINSTANCE, LPARAM, LRESULT, WPARAM};
    use winapi::shared::windef::HHOOK;
    use winapi::um::winuser::{
        CallNextHookEx, DispatchMessageW, PeekMessageW, SetWindowsHookExW, TranslateMessage, UnhookWindowsHookEx, HC_ACTION, KBDLLHOOKSTRUCT, MSG, PM_REMOVE, WH_KEYBOARD_LL, WM_KEYDOWN, WM_KEYUP, WM_SYSKEYDOWN, WM_SYSKEYUP
    };

    pub struct WindowsKeyboardHook {
        pub key_states: HashMap<u32, bool>,
        pub modifier_states: HashMap<u32, bool>,
        pub state_changes: Vec<u32>,
        hook: HHOOK,
    }
    
    impl KeyboardHook for WindowsKeyboardHook {
        fn new() -> WindowsKeyboardHook {
            // Create and install the keyboard hook
            let hook = unsafe { SetWindowsHookExW(
                WH_KEYBOARD_LL,
                Some(hook_callback),
                0 as HINSTANCE, // NULL for system-wide hooks
                0, // 0 for global hook
            )};
    
            if hook.is_null() { panic!("couldnt install hook"); }

            std::thread::spawn(|| {
                loop {
                    message_loop_keepalive();
                    thread::sleep(Duration::from_millis(10));
                }
            });

            WindowsKeyboardHook {
                key_states: HashMap::new(),
                modifier_states: HashMap::new(),
                state_changes: Vec::new(),
                hook,
            }
        }
    
        fn poll(&mut self) -> Vec<Event> {
            #[allow(static_mut_refs)]
            let iter = unsafe { CHANNEL.1.try_iter() };
            self.state_changes.clear();
            
            for (key_code, pressed) in iter {
                if is_modifier_key(key_code) {
                    let map_state = match self.modifier_states.get(&key_code) {
                        Some(k) => k,
                        None => {
                            self.modifier_states.insert(key_code, pressed);
                            self.state_changes.push(key_code);
                            continue;
                        }
                    };
        
                    if map_state.to_owned() != pressed {
                        self.state_changes.push(key_code);
                    }
        
                    self.modifier_states.insert(key_code, pressed);
                } else {
                    let map_state = match self.key_states.get(&key_code) {
                        Some(k) => k,
                        None => {
                            self.key_states.insert(key_code, pressed);
                            self.state_changes.push(key_code);
                            continue;
                        }
                    };
        
                    if map_state.to_owned() != pressed {
                        self.state_changes.push(key_code);
                    }
        
                    self.key_states.insert(key_code, pressed);
                }
            }

            let mut modifiers = Modifiers::default();
            for (key_code, pressed) in &self.modifier_states {
                match key_code {
                    0x10 => modifiers.shift = *pressed, // VK_SHIFT (generic)
                    0xA0 => modifiers.shift = *pressed, // VK_LSHIFT
                    0xA1 => modifiers.shift = *pressed, // VK_RSHIFT
                    0x11 => modifiers.command = *pressed, // VK_CONTROL (generic)
                    0xA2 => modifiers.command = *pressed, // VK_LCONTROL
                    0xA3 => modifiers.command = *pressed, // VK_RCONTROL
                    0x12 => modifiers.alt = *pressed, // VK_MENU (Alt - generic)
                    0xA4 => modifiers.alt = *pressed, // VK_LMENU (Left Alt)
                    0xA5 => modifiers.alt = *pressed, // VK_RMENU (Right Alt)
                    // 0x5B => modifiers.shift = pressed // VK_LWIN (Left Windows key)
                    // 0x5C => modifiers.shift = pressed  // VK_RWIN (Right Windows key)
                    _ => {},
                }
            }

            let mut events = Vec::new();
            for key_code in &self.state_changes {

                match windows_vk_to_egui(*key_code) {
                    Some(key) => {
                        let event = Event::Key {
                            key,
                            pressed: self.key_states.get(key_code).unwrap().clone(),
                            modifiers: modifiers.clone(),
                            physical_key: None,
                            repeat: false,
                        };

                        events.push(event);
                    },
                    _ => {},
                };

            }

            return events;
        } 
    
        fn uninstall_hook(&self) {
            unsafe {
                UnhookWindowsHookEx(self.hook);
            }
        }
    
        fn hook() {
            #[allow(static_mut_refs)]
            unsafe{ 
                *ENABLED.lock().unwrap() = true;
            };
        }
    
        fn unhook() {
            #[allow(static_mut_refs)]
            unsafe{ 
                *ENABLED.lock().unwrap() = false;
            };
        }
    }

    /// Convert Windows Virtual-Key codes to egui Key
    ///
    /// These are the vkCode values you get from WH_KEYBOARD_LL hook's KBDLLHOOKSTRUCT.
    /// Windows virtual key codes are documented at:
    /// https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
    ///
    /// Note: This converter handles the most common keys that have direct
    /// egui equivalents. Some Windows VK codes don't have egui equivalents.

    pub fn windows_vk_to_egui(vk_code: u32) -> Option<Key> {
        
        match vk_code {
            // Navigation/Command keys
            0x26 => Some(Key::ArrowUp),      // VK_UP
            0x28 => Some(Key::ArrowDown),    // VK_DOWN
            0x25 => Some(Key::ArrowLeft),    // VK_LEFT
            0x27 => Some(Key::ArrowRight),   // VK_RIGHT
            0x1B => Some(Key::Escape),       // VK_ESCAPE
            0x09 => Some(Key::Tab),          // VK_TAB
            0x08 => Some(Key::Backspace),    // VK_BACK
            0x0D => Some(Key::Enter),        // VK_RETURN
            0x2D => Some(Key::Insert),       // VK_INSERT
            0x2E => Some(Key::Delete),       // VK_DELETE
            0x24 => Some(Key::Home),         // VK_HOME
            0x23 => Some(Key::End),          // VK_END
            0x21 => Some(Key::PageUp),       // VK_PRIOR
            0x22 => Some(Key::PageDown),     // VK_NEXT
            
            // Space
            0x20 => Some(Key::Space),        // VK_SPACE
            
            // Punctuation and symbols
            0xBA => Some(Key::Semicolon),    // VK_OEM_1 (;:)
            0xBC => Some(Key::Comma),        // VK_OEM_COMMA (,<)
            0xBD => Some(Key::Minus),        // VK_OEM_MINUS (-_)
            0xBE => Some(Key::Period),       // VK_OEM_PERIOD (.>)
            0xBB => Some(Key::Plus),         // VK_OEM_PLUS (=+) - maps to Plus for the + character
            0xDB => Some(Key::OpenBracket),  // VK_OEM_4 ([{)
            0xDD => Some(Key::CloseBracket), // VK_OEM_6 (]})
            0xC0 => Some(Key::Backtick),     // VK_OEM_3 (`~)
            0xDC => Some(Key::Backslash),    // VK_OEM_5 (\|)
            0xBF => Some(Key::Slash),        // VK_OEM_2 (/?)
            0xDE => Some(Key::Quote),        // VK_OEM_7 ('")
            
            // Digits (main keyboard)
            0x30 => Some(Key::Num0),         // VK_0
            0x31 => Some(Key::Num1),         // VK_1
            0x32 => Some(Key::Num2),         // VK_2
            0x33 => Some(Key::Num3),         // VK_3
            0x34 => Some(Key::Num4),         // VK_4
            0x35 => Some(Key::Num5),         // VK_5
            0x36 => Some(Key::Num6),         // VK_6
            0x37 => Some(Key::Num7),         // VK_7
            0x38 => Some(Key::Num8),         // VK_8
            0x39 => Some(Key::Num9),         // VK_9
            
            // Digits (numpad)
            0x60 => Some(Key::Num0),         // VK_NUMPAD0
            0x61 => Some(Key::Num1),         // VK_NUMPAD1
            0x62 => Some(Key::Num2),         // VK_NUMPAD2
            0x63 => Some(Key::Num3),         // VK_NUMPAD3
            0x64 => Some(Key::Num4),         // VK_NUMPAD4
            0x65 => Some(Key::Num5),         // VK_NUMPAD5
            0x66 => Some(Key::Num6),         // VK_NUMPAD6
            0x67 => Some(Key::Num7),         // VK_NUMPAD7
            0x68 => Some(Key::Num8),         // VK_NUMPAD8
            0x69 => Some(Key::Num9),         // VK_NUMPAD9
            
            // Letters
            0x41 => Some(Key::A),            // VK_A
            0x42 => Some(Key::B),            // VK_B
            0x43 => Some(Key::C),            // VK_C
            0x44 => Some(Key::D),            // VK_D
            0x45 => Some(Key::E),            // VK_E
            0x46 => Some(Key::F),            // VK_F
            0x47 => Some(Key::G),            // VK_G
            0x48 => Some(Key::H),            // VK_H
            0x49 => Some(Key::I),            // VK_I
            0x4A => Some(Key::J),            // VK_J
            0x4B => Some(Key::K),            // VK_K
            0x4C => Some(Key::L),            // VK_L
            0x4D => Some(Key::M),            // VK_M
            0x4E => Some(Key::N),            // VK_N
            0x4F => Some(Key::O),            // VK_O
            0x50 => Some(Key::P),            // VK_P
            0x51 => Some(Key::Q),            // VK_Q
            0x52 => Some(Key::R),            // VK_R
            0x53 => Some(Key::S),            // VK_S
            0x54 => Some(Key::T),            // VK_T
            0x55 => Some(Key::U),            // VK_U
            0x56 => Some(Key::V),            // VK_V
            0x57 => Some(Key::W),            // VK_W
            0x58 => Some(Key::X),            // VK_X
            0x59 => Some(Key::Y),            // VK_Y
            0x5A => Some(Key::Z),            // VK_Z
            
            // Function keys
            0x70 => Some(Key::F1),           // VK_F1
            0x71 => Some(Key::F2),           // VK_F2
            0x72 => Some(Key::F3),           // VK_F3
            0x73 => Some(Key::F4),           // VK_F4
            0x74 => Some(Key::F5),           // VK_F5
            0x75 => Some(Key::F6),           // VK_F6
            0x76 => Some(Key::F7),           // VK_F7
            0x77 => Some(Key::F8),           // VK_F8
            0x78 => Some(Key::F9),           // VK_F9
            0x79 => Some(Key::F10),          // VK_F10
            0x7A => Some(Key::F11),          // VK_F11
            0x7B => Some(Key::F12),          // VK_F12
            0x7C => Some(Key::F13),          // VK_F13
            0x7D => Some(Key::F14),          // VK_F14
            0x7E => Some(Key::F15),          // VK_F15
            0x7F => Some(Key::F16),          // VK_F16
            0x80 => Some(Key::F17),          // VK_F17
            0x81 => Some(Key::F18),          // VK_F18
            0x82 => Some(Key::F19),          // VK_F19
            0x83 => Some(Key::F20),          // VK_F20
            0x84 => Some(Key::F21),          // VK_F21
            0x85 => Some(Key::F22),          // VK_F22
            0x86 => Some(Key::F23),          // VK_F23
            0x87 => Some(Key::F24),          // VK_F24
            
            // Navigation keys
            0xA6 => Some(Key::BrowserBack),  // VK_BROWSER_BACK
            
            // All other VK codes don't have direct egui equivalents
            _ => None,
        }
    }

    /// Additional helper to check if a VK code represents a modifier key
    /// (which egui handles separately via Modifiers)
    pub fn is_modifier_key(vk_code: u32) -> bool {
        matches!(
            vk_code,
            0x10 | // VK_SHIFT (generic)
            0xA0 | // VK_LSHIFT
            0xA1 | // VK_RSHIFT
            0x11 | // VK_CONTROL (generic)
            0xA2 | // VK_LCONTROL
            0xA3 | // VK_RCONTROL
            0x12 | // VK_MENU (Alt - generic)
            0xA4 | // VK_LMENU (Left Alt)
            0xA5 | // VK_RMENU (Right Alt)
            0x5B | // VK_LWIN (Left Windows key)
            0x5C   // VK_RWIN (Right Windows key)
        )
    }
    
    // Callback function for the keyboard hook
    unsafe extern "system" fn hook_callback(code: c_int, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        #[allow(static_mut_refs)]
        let enabled = unsafe{ ENABLED.lock().unwrap().clone() };
    
        if code >= HC_ACTION {
            let kb_struct = unsafe { *(lparam as *const KBDLLHOOKSTRUCT) };
    
            let key_code = kb_struct.vkCode;
            let event_type = wparam as u32;
            
            match event_type {
                #[allow(non_snake_case)]
                WM_KEYDOWN | WM_SYSKEYDOWN => {
                    if enabled && (windows_vk_to_egui(key_code).is_some() || is_modifier_key(key_code)) {
                        #[allow(static_mut_refs)]
                        unsafe { CHANNEL.0.send((key_code, true)).unwrap() };
                        return 1;
                    }
    
                    // is this key fwd slash (toggle interception)
                    if key_code == 0xBF {
                        #[allow(static_mut_refs)]
                        unsafe { CHANNEL.0.send((key_code, true)).unwrap() };
                        return 1;
                    }
                },
                #[allow(non_snake_case)]
                WM_KEYUP | WM_SYSKEYUP => {
                    if enabled && windows_vk_to_egui(key_code).is_some() || is_modifier_key(key_code) {
                        #[allow(static_mut_refs)]
                        unsafe { CHANNEL.0.send((key_code, false)).unwrap() };
                        return 1;
                    }
    
                    // is this key fwd slash (toggle interception)
                    if key_code == 0xBF {
                        #[allow(static_mut_refs)]
                        unsafe { CHANNEL.0.send((key_code, false)).unwrap() };
                        return 1;
                    }
                }
                _ => {}
            }
        }
    
        unsafe { return CallNextHookEx(ptr::null_mut(), code, wparam, lparam) }
    }
    
    
    pub fn message_loop_keepalive() {
        let mut msg: MSG = unsafe { mem::zeroed() };
            
        // Standard Windows message loop
        unsafe {
            if PeekMessageW(&mut msg, ptr::null_mut(), 0, 0, PM_REMOVE) > 0 {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
    }
}
