use std::{
    rc::{Rc, Weak},
    sync::RwLock,
};

use slint::{ComponentHandle, Model, ModelRc, VecModel};

use crate::{
    model,
    ui::slint::ui_modules::{AppWindow, FilterEntry, TabData},
};

mod sidebar;

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

    pub fn tabs_close(self, data: TabData) {
        let ui = upgrade_ui(self.ui);
        let tabs_rc = ui.get_tabs();
        let tabs = tabs_rc
            .as_any()
            .downcast_ref::<VecModel<TabData>>()
            .expect("We know we set a VecModel earlier");
        let found_tabs: Vec<(usize, TabData)> = tabs
            .iter()
            .enumerate()
            .filter(|(_, t)| t.workspace_name == data.workspace_name && t.id == &data.id)
            .collect();
        if found_tabs.len() != 0 {
            let index = found_tabs
                .get(0)
                .expect(
                    "Found tabs should not be empty at this point - Checked ina previous condition",
                )
                .0;
            tabs.remove(index);
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
