use iced::window::{self, Mode};
use iced::{futures::stream, Subscription, Task};
use tray_icon::TrayIconEvent;
use std::time::Duration;

pub fn tray_event_subscription() -> Subscription<TrayIconEvent> {
    Subscription::run_with_id(
        "tray-events",
        stream::unfold((), |_| async {
            tokio::time::sleep(Duration::from_millis(50)).await;
            if let Ok(event) = TrayIconEvent::receiver().try_recv() {
                Some((event, ()))
            } else {
                None
            }
        }),
    )
}

pub fn hide_window() -> Task<()> {
    window::get_latest().and_then(|id| {
        window::change_mode(id, Mode::Hidden)
    })
}

pub fn show_window() -> Task<()> {
    window::get_latest().and_then(|id| {
        window::change_mode(id, Mode::Windowed)
    })
}
