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
    Quote,
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
            egui::Key::Quote => Ok(MO5VirtualKeyCode::Quote),
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
