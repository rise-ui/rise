use glutin::{ElementState, KeyboardInput, VirtualKeyCode};
use std::cell::RefCell;
use std::char;
use std::fmt;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct KeyCode {
  pub virtual_keycode: Option<VirtualKeyCode>,
  pub scancode: u32,
}

type RefCellOfShortcutListener = Rc<RefCell<FnMut(Shortcut)>>;

#[derive(Clone)]
pub struct Shortcut {
  listener: RefCellOfShortcutListener,
  pub keys: Vec<KeyCode>,
}

impl fmt::Debug for Shortcut {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let keys = self.keys.clone();
    let scan_chars = keys
      .into_iter()
      .map(|key| {
        if let Some(keycode) = key.virtual_keycode {
          return format!("{:?}", keycode);
        } else {
          if let Some(symbol_char) = char::from_u32(key.scancode) {
            return format!("'{}'", symbol_char);
          } else {
            return String::from("UnknownKey");
          }
        }
      })
      .collect::<Vec<String>>()
      .join(" + ");

    write!(f, "Shortcut: {}", scan_chars)
  }
}

impl Shortcut {
  pub fn new<F: FnMut(Shortcut) + 'static>(keys: Vec<KeyCode>, listener: F) -> Shortcut {
    let listener = Rc::new(RefCell::new(listener));
    Shortcut {
      listener,
      keys,
    }
  }
}

#[derive(Clone, Debug)]
pub struct Shortcuts {
  active_keys: Vec<KeyCode>,
  listeners: Vec<Shortcut>,
}

impl Shortcuts {
  pub fn new() -> Shortcuts {
    let mut listeners: Vec<Shortcut> = vec![];

    // let test_shortcut = Shortcut::new(
    //   vec![
    //     KeyCode {
    //       virtual_keycode: Some(VirtualKeyCode::LControl),
    //       scancode: 29,
    //     },
    //     KeyCode {
    //       virtual_keycode: Some(VirtualKeyCode::C),
    //       scancode: 46,
    //     },
    //   ],
    //   |event| {
    //     println!("{:?}", event);
    //   },
    // );

    // listeners.push(test_shortcut);

    Shortcuts {
      active_keys: Vec::new(),
      listeners,
    }
  }

  pub fn add_shortcut_listener(&mut self, shortcut: Shortcut) {
    self.listeners.push(shortcut);
  }

  fn match_and_call(&mut self) {
    'lsmatch: for shortcut in self.listeners.iter() {
      if shortcut.keys.len() == self.active_keys.len() {
        for (index, key) in shortcut.keys.iter().enumerate() {
          if key.scancode != self.active_keys[index].scancode {
            continue 'lsmatch;
          }
        }

        let mut closure = shortcut.listener.borrow_mut();
        (&mut *closure)(shortcut.clone());
      }
    }
  }

  pub fn send_keyboard_input(&mut self, input: KeyboardInput) {
    match input.state {
      ElementState::Pressed => {
        self.active_keys.push(KeyCode {
          virtual_keycode: input.virtual_keycode,
          scancode: input.scancode,
        });

        self.match_and_call();
      }

      ElementState::Released => {
        if let Some(index) = self.active_keys.iter().position(|ref s| s.scancode == input.scancode) {
          self.active_keys.remove(index);
        }
      }
    }
  }
}
