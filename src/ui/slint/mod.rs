use std::{rc::Rc, sync::RwLock};

use controller::{
    content::tabs::TabsController,
    sidebar::{
        sidebar_filters_controller::SidebarFilterController,
        sidebar_generators_controller::SidebarGeneratorController,
    },
    Controller,
};

use slint::{PlatformError, SharedString, ComponentHandle};

use crate::{
    model::{self},
    ui::slint::ui_modules::AppWindow
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
        let main_ctrl = Controller::from(&self.model, &self.ui);

        let ctrl = main_ctrl.clone();
        self.ui
            .on_generators_entry_selected(move |current_workspace, id, is_folder| {
                let ctrl = SidebarGeneratorController::from(ctrl.clone());
                let current_workspace = current_workspace.as_str();
                let id = id.as_str();
                if is_folder {
                    ctrl.reverse_folding(current_workspace, id);
                } else {
                    ctrl.open_dashboard(current_workspace, id);
                }
            });

        let ctrl = main_ctrl.clone();
        self.ui.on_workspace_changed(move |workspace_name| {
            SidebarGeneratorController::from(ctrl.clone())
                .change_workspace(workspace_name.as_str());
        });

        let ctrl = main_ctrl.clone();
        self.ui.on_filter_searched_tags(move |searched| {
            SidebarFilterController::from(ctrl.clone()).filter_displayed_tags(searched.as_str())
        });

        let ctrl = main_ctrl.clone();
        self.ui.on_reverse_filter_activation(move |filter_name| {
            SidebarFilterController::from(ctrl.clone())
                .invert_filter_activation(filter_name.as_str())
        });

        let ctrl = main_ctrl.clone();
        self.ui.on_reset_filters(move |current_searched| {
            SidebarFilterController::from(ctrl.clone()).reset_filters(current_searched.as_str())
        });

        let ctrl = main_ctrl.clone();
        self.ui
            .on_close_tab(move |data| TabsController::from(ctrl.clone()).tabs_close(data));
    }

    pub fn run(&self) -> Result<(), slint::PlatformError> {
        self.ui.run()
    }
}
