use crate::ui::slint::controller::{self, Controller};

use slint::{ModelRc, VecModel};

use crate::ui::slint::ui_modules::FilterEntry;

pub struct SidebarFilterController {
    ctrl: Controller,
}

impl From<Controller> for SidebarFilterController {
    fn from(value: Controller) -> Self {
        SidebarFilterController { ctrl: value }
    }
}

impl SidebarFilterController {
    pub fn filter_displayed_tags(self, searched: &str) {
        let model = controller::upgrade_model(self.ctrl.model);
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
        controller::upgrade_ui(self.ctrl.ui)
            .set_filters(ModelRc::new(VecModel::from(filtered_filters)));
    }

    pub fn invert_filter_activation(self, filter_name: &str) {
        let model = controller::upgrade_model(self.ctrl.model);
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
        let model = controller::upgrade_model(self.ctrl.model);
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
        controller::upgrade_ui(self.ctrl.ui)
            .set_filters(ModelRc::new(VecModel::from(filtered_filters)));
    }
}
