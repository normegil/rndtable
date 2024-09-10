use std::{
    rc::{Rc, Weak},
    sync::RwLock,
};

use slint::ComponentHandle;

use crate::{model, ui::slint::ui_modules::AppWindow};

pub mod content;
pub mod sidebar;

#[derive(Clone)]
pub struct Controller {
    model: Weak<RwLock<model::Model>>,
    ui: slint::Weak<AppWindow>,
}

impl Controller {
    pub fn from(model: &Rc<RwLock<model::Model>>, ui: &AppWindow) -> Controller {
        Controller {
            model: Rc::downgrade(model),
            ui: ui.as_weak(),
        }
    }
}

pub fn upgrade_model(model: Weak<RwLock<model::Model>>) -> Rc<RwLock<model::Model>> {
    model
        .upgrade()
        .expect("Model should not be dropped before the end of the program")
}

pub fn upgrade_ui(ui: slint::Weak<AppWindow>) -> AppWindow {
    ui.upgrade()
        .expect("UI should not be dropped before the end of the program")
}
