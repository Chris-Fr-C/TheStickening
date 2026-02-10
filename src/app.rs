use iced::widget::{button, column, container, text};
use iced::window::Event as WindowEvent;
use iced::{Element, Event, Subscription, Task};
use std::time::{Duration, Instant};
use tray_icon::TrayIconEvent;

use crate::config::Config;
use crate::gamepad::GamepadHandler;
use crate::menu::SetupComponents;
use crate::setupapp::setup;
use crate::tray::{hide_window, show_window, tray_event_subscription};

pub struct StickApp {
    gamepad_handler: Option<GamepadHandler>,
    _tray_components: SetupComponents,
    last_update: Instant,
    target_interval: Duration,
    status_message: String,
    is_visible: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    Quit,
    WindowEvent(Event),
    TrayEvent(TrayIconEvent),
    WindowHidden,
    WindowShown,
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
                is_visible: true,
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
                if let Some(handler) = &mut self.gamepad_handler
                    && let Err(e) = handler.process_frame()
                {
                    eprintln!("Error processing gamepad: {}", e);
                }
                self.last_update = Instant::now();
                Task::none()
            }
            Message::Quit => iced::exit(),
            Message::WindowEvent(event) => {
                if let Event::Window(window_event) = event
                    && window_event == WindowEvent::CloseRequested
                {
                    self.is_visible = false;
                    return hide_window().map(|_| Message::WindowHidden);
                }
                Task::none()
            }
            Message::TrayEvent(event) => match event {
                TrayIconEvent::Click { .. } => {
                    if !self.is_visible {
                        self.is_visible = true;
                        return show_window().map(|_| Message::WindowShown);
                    }
                    Task::none()
                }
                _ => Task::none(),
            },
            Message::WindowHidden => Task::none(),
            Message::WindowShown => Task::none(),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let status = if self.is_visible {
            "Window is visible"
        } else {
            "Running in tray - click tray icon to show"
        };

        let content = column![
            text("TheStickening").size(24),
            text(&self.status_message).size(14),
            text(status).size(12),
            button("Quit").on_press(Message::Quit),
        ]
        .spacing(10)
        .padding(20);

        container(content).into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![
            iced::time::every(self.target_interval).map(|_| Message::Tick),
            tray_event_subscription().map(Message::TrayEvent),
            iced::event::listen().map(Message::WindowEvent),
        ])
    }
}
