pub use druid::PlatformError;
use druid::{widget::Flex, AppLauncher, Widget, WidgetExt, WindowDesc};

pub mod terminal;

pub fn run_app() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder());
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(String::new())
}

fn ui_builder() -> impl Widget<String> {
    let text_editor = terminal::Terminal::<String>::new();

    Flex::column().with_child(text_editor)
}
