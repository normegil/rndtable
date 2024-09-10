use std::{
    rc::{Rc, Weak},
    sync::RwLock,
};

use slint::{ComponentHandle, Model, ModelRc, VecModel};

use crate::{
    model,
    ui::slint::ui_modules::{AppWindow, FilterEntry, TabData},
};

use super::translators;

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

    pub fn reverse_folding(self, current_workspace: &str, id: &str) {
        let model = upgrade_model(self.model);
        {
            model
                .write()
                .expect("Model is not writable, but a menu need to be fold")
                .reverse_folding(&current_workspace, &id)
                .expect("Could not fold/unfold folder");
        }
        let model_read = model.read().expect("Model should be readable");
        upgrade_ui(self.ui).set_generation_entries(translators::to_entries_model(
            &model_read
                .get_current_workspace()
                .expect("Current workspace not found - should not happen")
                .hierarchy,
        ))
    }

    pub fn open_dashboard(self, current_workspace: &str, id: &str) {
        let ui = upgrade_ui(self.ui);
        let tabs_rc = ui.get_tabs();
        let tabs = tabs_rc
            .as_any()
            .downcast_ref::<VecModel<TabData>>()
            .expect("We know we set a VecModel earlier");
        let found_tabs: Vec<(usize, TabData)> = tabs
            .iter()
            .enumerate()
            .filter(|(_, t)| t.workspace_name == current_workspace && t.id == &id)
            .collect();
        let mut active_tab = tabs.row_count();
        if found_tabs.len() == 0 {
            let path = model::id_to_path(&id);
            tabs.push(TabData {
                workspace_name: current_workspace.to_string().into(),
                id: id.to_string().into(),
                name: path[path.len() - 1].to_string().into(),
                content: id.into(),
            });
        } else {
            active_tab = found_tabs
                .get(0)
                .expect(
                    "Found tabs should not be empty at this point - Checked ina previous condition",
                )
                .0;
        }
        ui.set_active_tab(
            active_tab
                .try_into()
                .expect("Usize to i32 conversion should work"),
        );
    }

    pub fn change_workspace(self, workspace_name: &str) {
        let model = upgrade_model(self.model);
        {
            model
                .write()
                .expect("Model is not writable, but a menu need to be fold")
                .set_current_workspace(workspace_name);
        }
        let model_read = model.read().expect("Model should be readable");
        upgrade_ui(self.ui).set_generation_entries(translators::to_entries_model(
            &model_read
                .get_current_workspace()
                .expect("Current workspace not found - should not happen")
                .hierarchy,
        ))
    }

    pub fn filter_displayed_tags(self, searched: &str) {
        let model = upgrade_model(self.model);
        let model_read = model.read().expect("Model should be readable");
        let filtered_filters: Vec<FilterEntry> = model_read
            .filter_list
            .iter()
            .filter(|f| f.name.contains(searched))
            .map(|f| FilterEntry {
                name: f.name.clone().into(),
                enable: f.enabled,
            })
            .collect();
        upgrade_ui(self.ui).set_filters(ModelRc::new(VecModel::from(filtered_filters)));
    }

    pub fn invert_filter_activation(self, filter_name: &str) {
        let model = upgrade_model(self.model);
        {
            let mut model_write = model
                .write()
                .expect("Model is not writable, but a menu need to be fold");
            for filter in model_write.filter_list.iter_mut() {
                if filter.name == filter_name.to_string() {
                    filter.enabled = !filter.enabled;
                }
            }
        }
    }

    pub fn reset_filters(self, current_searched: &str) {
        let model = upgrade_model(self.model);
        {
            let mut model_write = model
                .write()
                .expect("Model is not writable, but a menu need to be fold");
            for filter in model_write.filter_list.iter_mut() {
                filter.enabled = false;
            }
        }
        println!("{}", current_searched);
        let model_read = model.read().expect("Model should be readable");
        let filtered_filters: Vec<FilterEntry> = model_read
            .filter_list
            .iter()
            .filter(|f| f.name.contains(current_searched))
            .map(|f| FilterEntry {
                name: f.name.clone().into(),
                enable: f.enabled,
            })
            .collect();
        upgrade_ui(self.ui).set_filters(ModelRc::new(VecModel::from(filtered_filters)));
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

fn upgrade_model(model: Weak<RwLock<model::Model>>) -> Rc<RwLock<model::Model>> {
    model
        .upgrade()
        .expect("Model should not be dropped before the end of the program")
}

fn upgrade_ui(ui: slint::Weak<AppWindow>) -> AppWindow {
    ui.upgrade()
        .expect("UI should not be dropped before the end of the program")
}
