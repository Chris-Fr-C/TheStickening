use iced::widget::{button, column, container, text};
use iced::{Element, Subscription, Task};
use std::time::{Duration, Instant};

use crate::config::Config;
use crate::gamepad::GamepadHandler;
use crate::menu::SetupComponents;
use crate::setupapp::setup;

pub struct StickApp {
    gamepad_handler: Option<GamepadHandler>,
    _tray_components: SetupComponents,
    last_update: Instant,
    target_interval: Duration,
    status_message: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    Quit,
}

impl StickApp {
    pub fn new(config: Config) -> Result<(Self, Task<Message>), Box<dyn std::error::Error>> {
        let tray_components = setup()?;
        let gamepad_handler = GamepadHandler::new(config)?;

        let frequency = gamepad_handler.config.frequency;
        let target_interval = Duration::from_secs_f32(1.0 / frequency);

        Ok((
            Self {
                gamepad_handler: Some(gamepad_handler),
                _tray_components: tray_components,
                last_update: Instant::now(),
                target_interval,
                status_message: String::from("TheStickening is running"),
            },
            Task::none(),
        ))
    }

    pub fn title(&self) -> String {
        String::from("TheStickening")
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Tick => {
                if let Some(handler) = &mut self.gamepad_handler {
                    if let Err(e) = handler.process_frame() {
                        eprintln!("Error processing gamepad: {}", e);
                    }
                }
                self.last_update = Instant::now();
                Task::none()
            }
            Message::Quit => iced::exit(),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let content = column![
            text("TheStickening").size(24),
            text(&self.status_message).size(14),
            button("Quit").on_press(Message::Quit),
        ]
        .spacing(10)
        .padding(20);

        container(content).into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        iced::time::every(self.target_interval).map(|_| Message::Tick)
    }
}
