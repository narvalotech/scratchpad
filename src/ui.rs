use iced::Center;
use iced::widget::{Row, button, row, column};
use iced;

#[derive(Default)]
struct State {
}

#[derive(Debug, Clone, Copy)]
enum KeyEvent {
    Unknown,
    KeyDown,
    KeyUp,
    KeyLeft,
    KeyRight,
    KeySelect,
    KeyBack,
    KeyPower,
}

impl State {
    fn update(&mut self, message: KeyEvent) {
        match message {
            KeyEvent::KeyLeft => {
                println!("left");
            }
            KeyEvent::KeyRight => {
                println!("right");
            }
            KeyEvent::KeyUp => {
                println!("up");
            }
            KeyEvent::KeyDown => {
                println!("down");
            }
            KeyEvent::KeySelect => {
                println!("select");
            }
            KeyEvent::KeyBack => {
                println!("back");
            }
            _ => { println!("nuthin"); }
        }
    }

    fn view(&self) -> Row<'_, KeyEvent> {
        row![
            // D-pad
            button("<").on_press(KeyEvent::KeyLeft),
            column![
                button("/\\").on_press(KeyEvent::KeyUp),
                button("\\/").on_press(KeyEvent::KeyDown),
            ].spacing(10).align_x(Center),
            button(">").on_press(KeyEvent::KeyRight),

            // Enter/Back
            column![
                button("SELECT").on_press(KeyEvent::KeySelect),
                button(" BACK").on_press(KeyEvent::KeyBack),
            ].spacing(10).align_x(Center),
        ]
        .spacing(10)
        .padding(20)
        .align_y(Center)
    }
}

pub fn main() -> iced::Result {
    let settings = iced::window::Settings {
        size: iced::Size{width: 240.0, height: 120.0},
        resizable: false,
        ..Default::default()
    };

    // iced::run(State::update, State::view)
    iced::application(State::default, State::update, State::view).window(settings).run()
}
