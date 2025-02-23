use iced::widget::{column, text, Column};
use iced::window::{self, Position};
use iced::{Settings, Size, Task};

#[derive(Debug)]
enum OriginalTask {}

#[derive(Debug, Clone)]
struct HelloWorld(String);

impl Default for HelloWorld {
    fn default() -> Self {
        Self("Hello, World!".to_string())
    }
}

fn update(_hw: &mut HelloWorld, _task: OriginalTask) -> impl Into<Task<OriginalTask>> {
    Task::none()
}

fn view(hw: &HelloWorld) -> Column<OriginalTask> {
    column![text(hw.0.clone())]
}

fn main() -> iced::Result {
    let settings = Settings {
        ..Default::default()
    };
    let window_settings = window::Settings {
        size: Size::new(300.0, 300.0),
        max_size: Some(Size::new(500.0, 500.0)),
        min_size: Some(Size::new(100.0, 100.0)),
        position: Position::Centered,
        ..Default::default()
    };
    iced::application("title here", update, view)
        .settings(settings)
        .window(window_settings)
        .run()
}
