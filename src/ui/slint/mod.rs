use std::{rc::Rc, sync::RwLock};

use controller::Controller;
use slint::{ComponentHandle, Model, ModelRc, PlatformError, SharedString, VecModel};

use crate::{
    model,
    ui::slint::ui_modules::{AppWindow, FilterEntry, TabData},
};

pub mod controller;
pub mod translators;
pub mod ui_modules;

pub struct SlintUI {
    model: Rc<RwLock<model::Model>>,
    ui: AppWindow,
}

impl SlintUI {
    pub fn new(model: model::Model) -> Result<SlintUI, slint::PlatformError> {
        let ui = AppWindow::new()?;
        let slint_ui = SlintUI {
            model: Rc::new(RwLock::new(model)),
            ui,
        };
        slint_ui.init_data()?;
        slint_ui.register_callbacks();
        Ok(slint_ui)
    }

    fn init_data(&self) -> Result<(), slint::PlatformError> {
        let model_read = self
            .model
            .read()
            .expect("Model should be readable during initialization");

        let current_workspace = model_read
            .workspaces
            .get(0)
            .ok_or(PlatformError::Other("Test".to_string()))?;
        let current_workspace_name = current_workspace.name.as_str();

        self.ui
            .set_workspaces(translators::to_workspace_model(&model_read.workspaces));
        self.ui
            .set_current_workspace(SharedString::from(current_workspace_name));
        self.ui
            .set_generation_entries(translators::to_entries_model(&current_workspace.hierarchy));

        self.ui
            .set_filters(translators::to_ui_filters(&model_read.filter_list));

        Ok(())
    }

    fn register_callbacks(&self) {
        let model_clone = Rc::downgrade(&self.model);
        let ui_clone = self.ui.as_weak();
        self.ui
            .on_generators_entry_selected(move |current_workspace, id, is_folder| {
                let ctrl = Controller::new(model_clone.clone(), ui_clone.clone());
                let current_workspace = current_workspace.as_str();
                let id = id.as_str();
                if is_folder {
                    ctrl.reverse_folding(current_workspace, id);
                } else {
                    ctrl.open_dashboard(current_workspace, id);
                }
            });

        let model_clone = Rc::downgrade(&self.model);
        let ui_clone = self.ui.as_weak();
        self.ui.on_workspace_changed(move |workspace_name| {
            Controller::new(model_clone.clone(), ui_clone.clone()).change_workspace(workspace_name.as_str());
        });

        let model_clone = Rc::downgrade(&self.model);
        let ui_clone = self.ui.as_weak();
        self.ui.on_filter_searched_tags(move |searched| {
            let model = model_clone
                .upgrade()
                .expect("Model should not be dropped before the end of the program");
            let model_read = model.read().expect("Model should be readable");
            let filtered_filters: Vec<FilterEntry> = model_read
                .filter_list
                .iter()
                .filter(|f| f.name.contains(searched.as_str()))
                .map(|f| FilterEntry {
                    name: SharedString::from(f.name.to_string()),
                    enable: f.enabled,
                })
                .collect();
            ui_clone
                .upgrade()
                .expect("UI should not be dropped before the end of the program")
                .set_filters(ModelRc::new(VecModel::from(filtered_filters)));
        });

        let model_clone = Rc::downgrade(&self.model);
        self.ui.on_reverse_filter_activation(move |filter_name| {
            let model = model_clone
                .upgrade()
                .expect("Model should not be dropped before the end of the program");
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
        });

        let model_clone = Rc::downgrade(&self.model);
        let ui_clone = self.ui.as_weak();
        self.ui.on_reset_filters(move |current_searched| {
            let model = model_clone
                .upgrade()
                .expect("Model should not be dropped before the end of the program");
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
                .filter(|f| f.name.contains(current_searched.as_str()))
                .map(|f| FilterEntry {
                    name: SharedString::from(f.name.to_string()),
                    enable: f.enabled,
                })
                .collect();
            ui_clone
                .upgrade()
                .expect("UI should not be dropped before the end of the program")
                .set_filters(ModelRc::new(VecModel::from(filtered_filters)));
        });
        let ui_clone = self.ui.as_weak();
        self.ui.on_close_tab(move |data| {
            let ui = ui_clone
                    .upgrade()
                    .expect("UI should not be dropped before the end of the program");
            let tabs_rc = ui
                .get_tabs();
            let tabs = tabs_rc
                .as_any()
                .downcast_ref::<VecModel<TabData>>()
                .expect("We know we set a VecModel earlier");
            let found_tabs: Vec<(usize, TabData)> = tabs.iter().enumerate().filter(|(_, t)| t.workspace_name == data.workspace_name && t.id == &data.id).collect();
            if found_tabs.len() != 0 {
                let index = found_tabs.get(0)
                .expect("Found tabs should not be empty at this point - Checked ina previous condition")
                .0;
                tabs.remove(index);
            }
        });
    }

    pub fn run(&self) -> Result<(), slint::PlatformError> {
        self.ui.run()
    }
}
