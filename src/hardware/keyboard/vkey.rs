#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MO5VirtualKeyCode {
    Backspace,
    Delete,
    Return,
    Insert,
    Up,
    Left,
    Right,
    Down,
    LControl,
    Escape,
    LShift,
    F11,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    Key0,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    Comma,
    Period,
    At,
    Asterisk,
    Space,
    Caret,
    Minus,
}

#[cfg(feature = "egui-display")]
impl TryFrom<egui::Key> for MO5VirtualKeyCode {
    type Error = ();

    fn try_from(vk: egui::Key) -> Result<Self, Self::Error> {
        match vk {
            egui::Key::Backspace => Ok(MO5VirtualKeyCode::Backspace),
            egui::Key::Delete => Ok(MO5VirtualKeyCode::Delete),
            egui::Key::Enter => Ok(MO5VirtualKeyCode::Return),
            egui::Key::Insert => Ok(MO5VirtualKeyCode::Insert),
            egui::Key::ArrowUp => Ok(MO5VirtualKeyCode::Up),
            egui::Key::ArrowLeft => Ok(MO5VirtualKeyCode::Left),
            egui::Key::ArrowRight => Ok(MO5VirtualKeyCode::Right),
            egui::Key::ArrowDown => Ok(MO5VirtualKeyCode::Down),
            // egui::Key::Ctrl => Ok(MO5VirtualKeyCode::LControl),
            egui::Key::Escape => Ok(MO5VirtualKeyCode::Escape),
            // egui::Key::LShift => Ok(MO5VirtualKeyCode::LShift),
            egui::Key::F11 => Ok(MO5VirtualKeyCode::F11),
            egui::Key::Num1 => Ok(MO5VirtualKeyCode::Key1),
            egui::Key::Num2 => Ok(MO5VirtualKeyCode::Key2),
            egui::Key::Num3 => Ok(MO5VirtualKeyCode::Key3),
            egui::Key::Num4 => Ok(MO5VirtualKeyCode::Key4),
            egui::Key::Num5 => Ok(MO5VirtualKeyCode::Key5),
            egui::Key::Num6 => Ok(MO5VirtualKeyCode::Key6),
            egui::Key::Num7 => Ok(MO5VirtualKeyCode::Key7),
            egui::Key::Num8 => Ok(MO5VirtualKeyCode::Key8),
            egui::Key::Num9 => Ok(MO5VirtualKeyCode::Key9),
            egui::Key::Num0 => Ok(MO5VirtualKeyCode::Key0),
            egui::Key::A => Ok(MO5VirtualKeyCode::A),
            egui::Key::B => Ok(MO5VirtualKeyCode::B),
            egui::Key::C => Ok(MO5VirtualKeyCode::C),
            egui::Key::D => Ok(MO5VirtualKeyCode::D),
            egui::Key::E => Ok(MO5VirtualKeyCode::E),
            egui::Key::F => Ok(MO5VirtualKeyCode::F),
            egui::Key::G => Ok(MO5VirtualKeyCode::G),
            egui::Key::H => Ok(MO5VirtualKeyCode::H),
            egui::Key::I => Ok(MO5VirtualKeyCode::I),
            egui::Key::J => Ok(MO5VirtualKeyCode::J),
            egui::Key::K => Ok(MO5VirtualKeyCode::K),
            egui::Key::L => Ok(MO5VirtualKeyCode::L),
            egui::Key::M => Ok(MO5VirtualKeyCode::M),
            egui::Key::N => Ok(MO5VirtualKeyCode::N),
            egui::Key::O => Ok(MO5VirtualKeyCode::O),
            egui::Key::P => Ok(MO5VirtualKeyCode::P),
            egui::Key::Q => Ok(MO5VirtualKeyCode::Q),
            egui::Key::R => Ok(MO5VirtualKeyCode::R),
            egui::Key::S => Ok(MO5VirtualKeyCode::S),
            egui::Key::T => Ok(MO5VirtualKeyCode::T),
            egui::Key::U => Ok(MO5VirtualKeyCode::U),
            egui::Key::V => Ok(MO5VirtualKeyCode::V),
            egui::Key::W => Ok(MO5VirtualKeyCode::W),
            egui::Key::X => Ok(MO5VirtualKeyCode::X),
            egui::Key::Y => Ok(MO5VirtualKeyCode::Y),
            egui::Key::Z => Ok(MO5VirtualKeyCode::Z),
            egui::Key::Comma => Ok(MO5VirtualKeyCode::Comma),
            egui::Key::Period => Ok(MO5VirtualKeyCode::Period),
            // egui::Key::At => Ok(MO5VirtualKeyCode::At),
            // egui::Key::Asterisk => Ok(MO5VirtualKeyCode::Asterisk),
            egui::Key::Space => Ok(MO5VirtualKeyCode::Space),
            // egui::Key::Caret => Ok(MO5VirtualKeyCode::Caret),
            egui::Key::Minus => Ok(MO5VirtualKeyCode::Minus),
            _ => Err(()),
        }
    }
}

#[cfg(feature = "speedy2d-display")]
impl TryFrom<speedy2d::window::VirtualKeyCode> for MO5VirtualKeyCode {
    type Error = ();

    fn try_from(vk: speedy2d::window::VirtualKeyCode) -> Result<Self, Self::Error> {
        match vk {
            speedy2d::window::VirtualKeyCode::Backspace => Ok(MO5VirtualKeyCode::Backspace),
            speedy2d::window::VirtualKeyCode::Delete => Ok(MO5VirtualKeyCode::Delete),
            speedy2d::window::VirtualKeyCode::Return
            | speedy2d::window::VirtualKeyCode::NumpadEnter => Ok(MO5VirtualKeyCode::Return),
            speedy2d::window::VirtualKeyCode::Insert => Ok(MO5VirtualKeyCode::Insert),
            speedy2d::window::VirtualKeyCode::Up => Ok(MO5VirtualKeyCode::Up),
            speedy2d::window::VirtualKeyCode::Left => Ok(MO5VirtualKeyCode::Left),
            speedy2d::window::VirtualKeyCode::Right => Ok(MO5VirtualKeyCode::Right),
            speedy2d::window::VirtualKeyCode::Down => Ok(MO5VirtualKeyCode::Down),
            speedy2d::window::VirtualKeyCode::LControl => Ok(MO5VirtualKeyCode::LControl),
            speedy2d::window::VirtualKeyCode::Escape => Ok(MO5VirtualKeyCode::Escape),
            speedy2d::window::VirtualKeyCode::LShift => Ok(MO5VirtualKeyCode::LShift),
            speedy2d::window::VirtualKeyCode::F11 => Ok(MO5VirtualKeyCode::F11),
            speedy2d::window::VirtualKeyCode::Key1 => Ok(MO5VirtualKeyCode::Key1),
            speedy2d::window::VirtualKeyCode::Key2 => Ok(MO5VirtualKeyCode::Key2),
            speedy2d::window::VirtualKeyCode::Key3 => Ok(MO5VirtualKeyCode::Key3),
            speedy2d::window::VirtualKeyCode::Key4 => Ok(MO5VirtualKeyCode::Key4),
            speedy2d::window::VirtualKeyCode::Key5 => Ok(MO5VirtualKeyCode::Key5),
            speedy2d::window::VirtualKeyCode::Key6 => Ok(MO5VirtualKeyCode::Key6),
            speedy2d::window::VirtualKeyCode::Key7 => Ok(MO5VirtualKeyCode::Key7),
            speedy2d::window::VirtualKeyCode::Key8 => Ok(MO5VirtualKeyCode::Key8),
            speedy2d::window::VirtualKeyCode::Key9 => Ok(MO5VirtualKeyCode::Key9),
            speedy2d::window::VirtualKeyCode::Key0 => Ok(MO5VirtualKeyCode::Key0),
            speedy2d::window::VirtualKeyCode::A => Ok(MO5VirtualKeyCode::A),
            speedy2d::window::VirtualKeyCode::B => Ok(MO5VirtualKeyCode::B),
            speedy2d::window::VirtualKeyCode::C => Ok(MO5VirtualKeyCode::C),
            speedy2d::window::VirtualKeyCode::D => Ok(MO5VirtualKeyCode::D),
            speedy2d::window::VirtualKeyCode::E => Ok(MO5VirtualKeyCode::E),
            speedy2d::window::VirtualKeyCode::F => Ok(MO5VirtualKeyCode::F),
            speedy2d::window::VirtualKeyCode::G => Ok(MO5VirtualKeyCode::G),
            speedy2d::window::VirtualKeyCode::H => Ok(MO5VirtualKeyCode::H),
            speedy2d::window::VirtualKeyCode::I => Ok(MO5VirtualKeyCode::I),
            speedy2d::window::VirtualKeyCode::J => Ok(MO5VirtualKeyCode::J),
            speedy2d::window::VirtualKeyCode::K => Ok(MO5VirtualKeyCode::K),
            speedy2d::window::VirtualKeyCode::L => Ok(MO5VirtualKeyCode::L),
            speedy2d::window::VirtualKeyCode::M => Ok(MO5VirtualKeyCode::M),
            speedy2d::window::VirtualKeyCode::N => Ok(MO5VirtualKeyCode::N),
            speedy2d::window::VirtualKeyCode::O => Ok(MO5VirtualKeyCode::O),
            speedy2d::window::VirtualKeyCode::P => Ok(MO5VirtualKeyCode::P),
            speedy2d::window::VirtualKeyCode::Q => Ok(MO5VirtualKeyCode::Q),
            speedy2d::window::VirtualKeyCode::R => Ok(MO5VirtualKeyCode::R),
            speedy2d::window::VirtualKeyCode::S => Ok(MO5VirtualKeyCode::S),
            speedy2d::window::VirtualKeyCode::T => Ok(MO5VirtualKeyCode::T),
            speedy2d::window::VirtualKeyCode::U => Ok(MO5VirtualKeyCode::U),
            speedy2d::window::VirtualKeyCode::V => Ok(MO5VirtualKeyCode::V),
            speedy2d::window::VirtualKeyCode::W => Ok(MO5VirtualKeyCode::W),
            speedy2d::window::VirtualKeyCode::X => Ok(MO5VirtualKeyCode::X),
            speedy2d::window::VirtualKeyCode::Y => Ok(MO5VirtualKeyCode::Y),
            speedy2d::window::VirtualKeyCode::Z => Ok(MO5VirtualKeyCode::Z),
            speedy2d::window::VirtualKeyCode::Comma => Ok(MO5VirtualKeyCode::Comma),
            speedy2d::window::VirtualKeyCode::Period => Ok(MO5VirtualKeyCode::Period),
            speedy2d::window::VirtualKeyCode::At => Ok(MO5VirtualKeyCode::At),
            speedy2d::window::VirtualKeyCode::Asterisk => Ok(MO5VirtualKeyCode::Asterisk),
            speedy2d::window::VirtualKeyCode::Space => Ok(MO5VirtualKeyCode::Space),
            speedy2d::window::VirtualKeyCode::Caret => Ok(MO5VirtualKeyCode::Caret),
            speedy2d::window::VirtualKeyCode::Minus => Ok(MO5VirtualKeyCode::Minus),
            _ => Err(()),
        }
    }
}
