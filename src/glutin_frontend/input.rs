use glutin::{Event, WindowEvent, ModifiersState, ElementState, VirtualKeyCode};
use input::InputEvent;

fn to_char_event(ch: char, keymod: ModifiersState) -> Option<InputEvent> {
    if ch.is_alphabetic() {
        if keymod.shift {
            let chars = ch.to_uppercase().collect::<Vec<char>>();
            return Some(InputEvent::Char(chars[0]));
        } else {
            // ch must be lowercase
            return Some(InputEvent::Char(ch));
        }
    }

    let translated_ch = if keymod.shift {
        match ch {
            '1' => '!',
            '2' => '@',
            '3' => '#',
            '4' => '$',
            '5' => '%',
            '6' => '^',
            '7' => '&',
            '8' => '*',
            '9' => '(',
            '0' => ')',
            '.' => '>',
            ',' => '<',
            '/' => '?',
            _ => return None,
        }
    } else {
        ch
    };

    Some(InputEvent::Char(translated_ch))
}

fn convert_key(keycode: VirtualKeyCode, keymod: ModifiersState) -> Option<InputEvent> {
    match keycode {
        VirtualKeyCode::Up => Some(InputEvent::Up),
        VirtualKeyCode::Down => Some(InputEvent::Down),
        VirtualKeyCode::Left => Some(InputEvent::Left),
        VirtualKeyCode::Right => Some(InputEvent::Right),
        VirtualKeyCode::Space => Some(InputEvent::Space),
        VirtualKeyCode::Escape => Some(InputEvent::Escape),
        VirtualKeyCode::Return => Some(InputEvent::Return),
        VirtualKeyCode::A => to_char_event('a', keymod),
        VirtualKeyCode::B => to_char_event('b', keymod),
        VirtualKeyCode::C => to_char_event('c', keymod),
        VirtualKeyCode::D => to_char_event('d', keymod),
        VirtualKeyCode::E => to_char_event('e', keymod),
        VirtualKeyCode::F => to_char_event('f', keymod),
        VirtualKeyCode::G => to_char_event('g', keymod),
        VirtualKeyCode::H => to_char_event('h', keymod),
        VirtualKeyCode::I => to_char_event('i', keymod),
        VirtualKeyCode::J => to_char_event('j', keymod),
        VirtualKeyCode::K => to_char_event('k', keymod),
        VirtualKeyCode::L => to_char_event('l', keymod),
        VirtualKeyCode::M => to_char_event('m', keymod),
        VirtualKeyCode::N => to_char_event('n', keymod),
        VirtualKeyCode::O => to_char_event('o', keymod),
        VirtualKeyCode::P => to_char_event('p', keymod),
        VirtualKeyCode::Q => to_char_event('q', keymod),
        VirtualKeyCode::R => to_char_event('r', keymod),
        VirtualKeyCode::S => to_char_event('s', keymod),
        VirtualKeyCode::T => to_char_event('t', keymod),
        VirtualKeyCode::U => to_char_event('u', keymod),
        VirtualKeyCode::V => to_char_event('v', keymod),
        VirtualKeyCode::W => to_char_event('w', keymod),
        VirtualKeyCode::X => to_char_event('x', keymod),
        VirtualKeyCode::Y => to_char_event('y', keymod),
        VirtualKeyCode::Z => to_char_event('z', keymod),
        VirtualKeyCode::Numpad0 => to_char_event('0', keymod),
        VirtualKeyCode::Numpad1 => to_char_event('1', keymod),
        VirtualKeyCode::Numpad2 => to_char_event('2', keymod),
        VirtualKeyCode::Numpad3 => to_char_event('3', keymod),
        VirtualKeyCode::Numpad4 => to_char_event('4', keymod),
        VirtualKeyCode::Numpad5 => to_char_event('5', keymod),
        VirtualKeyCode::Numpad6 => to_char_event('6', keymod),
        VirtualKeyCode::Numpad7 => to_char_event('7', keymod),
        VirtualKeyCode::Numpad8 => to_char_event('8', keymod),
        VirtualKeyCode::Numpad9 => to_char_event('9', keymod),
        VirtualKeyCode::Period => to_char_event('.', keymod),
        VirtualKeyCode::Comma => to_char_event(',', keymod),
        VirtualKeyCode::Slash => to_char_event('/', keymod),
        _ => None,
    }
}

pub fn convert_event(event: Event) -> Option<InputEvent> {
    let event = if let Event::WindowEvent { event, .. } = event {
        event
    } else {
        return None;
    };

    match event {
        WindowEvent::Closed => return Some(InputEvent::Quit),
        WindowEvent::KeyboardInput { input, .. } => {
            if input.state == ElementState::Pressed {
                if let Some(keycode) = input.virtual_keycode {
                    return convert_key(keycode, input.modifiers);
                }
            }
        }
        _ => {}
    }

    None
}
