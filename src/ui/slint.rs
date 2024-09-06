use std::rc::Rc;

use slint::{ModelRc, PlatformError, SharedString, VecModel};

use crate::model::{
    workspaces::{HierarchyElement, Workspace},
    Model,
};

slint::include_modules!();

pub struct SlintUI {
    model: Model,
    ui: AppWindow,
}

impl SlintUI {
    pub fn new(model: Model) -> Result<Rc<SlintUI>, slint::PlatformError> {
        let ui = AppWindow::new()?;
        let slint_ui = Rc::new(SlintUI { model, ui });
        slint_ui.init_data()?;
        SlintUI::register_callbacks(slint_ui.clone());
        Ok(slint_ui)
    }

    fn init_data(&self) -> Result<(), slint::PlatformError> {
        let current_workspace = self
            .model
            .workspaces
            .get(0)
            .ok_or(PlatformError::Other("Test".to_string()))?;
        let current_workspace_name = current_workspace.name.as_str();

        self.ui
            .set_workspaces(to_workspace_model(&self.model.workspaces));
        self.ui
            .set_current_workspace(SharedString::from(current_workspace_name));
        self.ui
            .set_generation_entries(to_entries_model(&current_workspace.hierarchy));
        Ok(())
    }

    fn register_callbacks(slint_ui: Rc<SlintUI>) {
        let clone_slint_ui = slint_ui.clone();
        slint_ui
            .ui
            .on_generator_entry_clicked(move |current_workspace, id| {
                clone_slint_ui
                    .generator_entry_clicked(current_workspace.to_string(), id.to_string())
            });
    }

    fn generator_entry_clicked(&self, current_workspace: String, id: String) {
        match self.model.is_generator_folder(&current_workspace, &id) {
            Ok(is_folder) => ,
            Err(e) => panic!("Generator list clicked error: {e}"),
        }
        println!("Generator Entry clicked: {current_workspace} - {id}")
    }

    pub fn run(&self) -> Result<(), slint::PlatformError> {
        self.ui.run()
    }
}

fn to_workspace_model(workspaces: &Vec<Workspace>) -> ModelRc<SharedString> {
    let tmp = workspaces
        .iter()
        .map(|s| SharedString::from(&s.name))
        .collect::<Vec<SharedString>>();
    let tmp = VecModel::from(tmp);
    ModelRc::new(tmp)
}

fn to_entries_model(entries: &Vec<HierarchyElement>) -> ModelRc<HierarchyEntry> {
    let mut hierarchy_entry = vec![];
    for element in entries {
        hierarchy_entry.extend(flatten_entry(element, 0, "/"))
    }

    let tmp = VecModel::from(hierarchy_entry);
    ModelRc::new(tmp)
}

fn flatten_entry(
    entry: &HierarchyElement,
    identation: i32,
    parent_id: &str,
) -> Vec<HierarchyEntry> {
    match entry {
        HierarchyElement::DashboardFolder(folder) => {
            let current_id = parent_id.to_string() + "/" + &folder.name;
            let mut elements = vec![HierarchyEntry {
                id: SharedString::from(parent_id.to_string() + "/" + &folder.name),
                title: SharedString::from(&folder.name),
                folded: false,
                identation,
                is_folder: true,
            }];

            if !folder.folded {
                for hierarchy_element in &folder.hierarchy {
                    elements.extend(flatten_entry(
                        hierarchy_element,
                        identation + 1,
                        &current_id,
                    ))
                }
            }
            return elements;
        }
        HierarchyElement::Dashboard(dashboard) => {
            return vec![HierarchyEntry {
                id: SharedString::from(parent_id.to_string() + "/" + &dashboard.name),
                title: SharedString::from(&dashboard.name),
                folded: false,
                identation,
                is_folder: false,
            }];
        }
    }
}
