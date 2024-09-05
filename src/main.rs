use ui::slint::SlintUI;

slint::include_modules!();

mod model;
mod ui;
mod utils;

fn main() -> Result<(), slint::PlatformError> {
    let model = model::Model::new();
    let ui = SlintUI::new(model)?;
    ui.run()
}
