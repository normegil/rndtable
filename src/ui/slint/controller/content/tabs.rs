use crate::ui::slint::controller::{self, Controller};

use slint::{Model, VecModel};

use crate::ui::slint::ui_modules::TabData;

pub struct TabsController {
    ctrl: Controller,
}

impl From<Controller> for TabsController {
    fn from(value: Controller) -> Self {
        TabsController { ctrl: value }
    }
}

impl TabsController {
    pub fn tabs_close(self, data: TabData) {
        let ui = controller::upgrade_ui(self.ctrl.ui);
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
