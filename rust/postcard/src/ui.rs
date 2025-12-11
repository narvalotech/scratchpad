use iced::event::{self, Event};
use iced::keyboard;
use iced::keyboard::key;
use iced::keyboard::key::Key::Named;

use iced::{Center, Subscription};
use iced::widget::{Row, button, row, column};
use iced;

#[derive(Default)]
struct State {
}

#[derive(Debug, Clone)]
enum KeyEvent {
    Unknown,
    KeyDown,
    KeyUp,
    KeyLeft,
    KeyRight,
    KeySelect,
    KeyBack,
    KeyPower,
    KeyRaw(Event)
}

impl State {
    fn subscription(&self) -> Subscription<KeyEvent> {
        event::listen().map(KeyEvent::KeyRaw)
    }

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
            KeyEvent::KeyPower => {
                println!("power");
            }
            KeyEvent::KeyRaw(event) => match event {
                Event::Keyboard(keyboard::Event::KeyPressed{key: Named(code), ..}) => {
                    let converted = match code {
                        key::Named::ArrowDown => KeyEvent::KeyDown,
                        key::Named::ArrowUp => KeyEvent::KeyUp,
                        key::Named::ArrowLeft => KeyEvent::KeyLeft,
                        key::Named::ArrowRight => KeyEvent::KeyRight,
                        key::Named::Enter => KeyEvent::KeySelect,
                        key::Named::Backspace => KeyEvent::KeyBack,
                        key::Named::Escape => KeyEvent::KeyPower,
                        _ => {
                            KeyEvent::Unknown
                        }
                    };
                    // println!("key: {:?}", event);
                    self.update(converted)
                }
                _ => {},
            },
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
    iced::application(State::default, State::update, State::view)
        .window(settings)
        .subscription(State::subscription)
        .run()
}
