use iced::event::{self, Event};
use iced::keyboard;
use iced::keyboard::key;
use iced::keyboard::key::Key::Named;

use iced::{Center, Subscription};
use iced::widget::{Row, button, row, column};
use iced;

// --------------------------
use serde::{Serialize, Deserialize};
use postcard::{from_bytes_cobs, to_allocvec_cobs};

use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;

#[derive(Default)]
struct State {
}

#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
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

#[derive(Clone, Debug)]
enum UIEvent {
    Key(KeyEvent),
    Raw(Event)
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct InputEvent {
    key: KeyEvent,
}

impl State {
    fn subscription(&self) -> Subscription<UIEvent> {
        event::listen().map(UIEvent::Raw)
    }

    fn update(&mut self, message: UIEvent) {
        match message {
            UIEvent::Key(KeyEvent::KeyLeft) => {
                println!("left");
            }
            UIEvent::Key(KeyEvent::KeyRight) => {
                println!("right");
            }
            UIEvent::Key(KeyEvent::KeyUp) => {
                println!("up");
            }
            UIEvent::Key(KeyEvent::KeyDown) => {
                println!("down");
            }
            UIEvent::Key(KeyEvent::KeySelect) => {
                println!("select");
            }
            UIEvent::Key(KeyEvent::KeyBack) => {
                println!("back");
            }
            UIEvent::Key(KeyEvent::KeyPower) => {
                println!("power");
            }
            UIEvent::Raw(event) => match event {
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
                    self.update(UIEvent::Key(converted))
                }
                _ => {},
            },
            _ => { println!("nuthin"); }
        }
    }

    fn view(&self) -> Row<'_, UIEvent> {
        row![
            // D-pad
            button("<").on_press(UIEvent::Key(KeyEvent::KeyLeft)),
            column![
                button("/\\").on_press(UIEvent::Key(KeyEvent::KeyUp)),
                button("\\/").on_press(UIEvent::Key(KeyEvent::KeyDown)),
            ].spacing(10).align_x(Center),
            button(">").on_press(UIEvent::Key(KeyEvent::KeyRight)),

            // Enter/Back
            column![
                button("SELECT").on_press(UIEvent::Key(KeyEvent::KeySelect)),
                button(" BACK").on_press(UIEvent::Key(KeyEvent::KeyBack)),
            ].spacing(10).align_x(Center),
        ]
        .spacing(10)
        .padding(20)
        .align_y(Center)
    }
}

async fn write_event_to_stream(stream: &mut TcpStream, event: InputEvent) {
    let output: Vec<u8> = to_allocvec_cobs(&event).unwrap();

    stream.write_all(&output).await.unwrap();

    let decoded: InputEvent = from_bytes_cobs(&mut output.clone()).unwrap();
    println!("Written: {:?}", decoded);
}

async fn tcp_client() {
    // Connect to a peer
    let mut stream = TcpStream::connect("127.0.0.1:9999").await.unwrap();

    let e1 = InputEvent {
        key: KeyEvent::KeyUp,
    };

    let e2 = InputEvent {
        key: KeyEvent::KeyDown,
    };

    write_event_to_stream(&mut stream, e1).await;
    write_event_to_stream(&mut stream, e2).await;
}

use tokio::runtime::Builder;

pub fn main() -> iced::Result {
    let runtime = Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build()
        .unwrap();

    let _handle = runtime.spawn(tcp_client());

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
