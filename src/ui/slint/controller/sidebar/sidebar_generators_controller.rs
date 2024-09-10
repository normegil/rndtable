use crate::ui::slint::{
    controller::{self, Controller},
    translators,
};

use slint::{Model, VecModel};

use crate::{model, ui::slint::ui_modules::TabData};

pub struct SidebarGeneratorController {
    ctrl: Controller,
}

impl From<Controller> for SidebarGeneratorController {
    fn from(value: Controller) -> Self {
        SidebarGeneratorController { ctrl: value }
    }
}

impl SidebarGeneratorController {
    pub fn reverse_folding(self, current_workspace: &str, id: &str) {
        let model = controller::upgrade_model(self.ctrl.model);
        {
            model
                .write()
                .expect("Model is not writable, but a menu need to be fold")
                .reverse_folding(&current_workspace, &id)
                .expect("Could not fold/unfold folder");
        }
        let model_read = model.read().expect("Model should be readable");
        controller::upgrade_ui(self.ctrl.ui).set_generation_entries(translators::to_entries_model(
            &model_read
                .get_current_workspace()
                .expect("Current workspace not found - should not happen")
                .hierarchy,
        ))
    }

    pub fn open_dashboard(self, current_workspace: &str, id: &str) {
        let ui = controller::upgrade_ui(self.ctrl.ui);
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
        let model = controller::upgrade_model(self.ctrl.model);
        {
            model
                .write()
                .expect("Model is not writable, but a menu need to be fold")
                .set_current_workspace(workspace_name);
        }
        let model_read = model.read().expect("Model should be readable");
        controller::upgrade_ui(self.ctrl.ui).set_generation_entries(translators::to_entries_model(
            &model_read
                .get_current_workspace()
                .expect("Current workspace not found - should not happen")
                .hierarchy,
        ))
    }
}
