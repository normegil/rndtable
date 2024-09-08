use std::{rc::Rc, sync::RwLock};

use slint::{ModelRc, PlatformError, SharedString, VecModel};

use crate::model::{
    filters::Filter,
    workspaces::{HierarchyElement, Workspace},
    Model,
};

slint::include_modules!();

pub struct SlintUI {
    model: Rc<RwLock<Model>>,
    ui: AppWindow,
}

impl SlintUI {
    pub fn new(model: Model) -> Result<SlintUI, slint::PlatformError> {
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
            .set_workspaces(to_workspace_model(&model_read.workspaces));
        self.ui
            .set_current_workspace(SharedString::from(current_workspace_name));
        self.ui
            .set_generation_entries(to_entries_model(&current_workspace.hierarchy));

        self.ui.set_filters(to_ui_filters(&model_read.filter_list));

        Ok(())
    }

    fn register_callbacks(&self) {
        let model_clone = Rc::downgrade(&self.model);
        let ui_clone = self.ui.as_weak();
        self.ui
            .on_generator_entry_clicked(move |current_workspace, id, is_folder| {
                if is_folder {
                    let model = model_clone
                        .upgrade()
                        .expect("Model should not be dropped before the end of the program");
                    {
                        model
                            .write()
                            .expect("Model is not writable, but a menu need to be fold")
                            .reverse_folding(&current_workspace, &id)
                            .expect("Could not fold/unfold folder");
                    }
                    let model_read = model.read().expect("Model should be readable");
                    ui_clone
                        .upgrade()
                        .expect("UI should not be dropped before the end of the program")
                        .set_generation_entries(to_entries_model(
                            &model_read
                                .get_current_workspace()
                                .expect("Current workspace not found - should not happen")
                                .hierarchy,
                        ))
                } else {
                    todo!("Display content of dashboard in a new tab");
                }
                println!("Generator Entry clicked: {current_workspace} - {id}")
            });

        let model_clone = Rc::downgrade(&self.model);
        let ui_clone = self.ui.as_weak();
        self.ui.on_workspace_changed(move |workspace_name| {
            let model = model_clone
                .upgrade()
                .expect("Model should not be dropped before the end of the program");
            {
                model
                    .write()
                    .expect("Model is not writable, but a menu need to be fold")
                    .set_current_workspace(workspace_name.as_str());
            }
            let model_read = model.read().expect("Model should be readable");
            ui_clone
                .upgrade()
                .expect("UI should not be dropped before the end of the program")
                .set_generation_entries(to_entries_model(
                    &model_read
                        .get_current_workspace()
                        .expect("Current workspace not found - should not happen")
                        .hierarchy,
                ))
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

fn to_ui_filters(filters: &Vec<Filter>) -> ModelRc<FilterEntry> {
    let filters_entry: Vec<FilterEntry> = filters
        .iter()
        .map(|f| FilterEntry {
            name: SharedString::from(f.name.to_string()),
            enable: f.enabled,
        })
        .collect();
    let filter_entries = VecModel::from(filters_entry);
    ModelRc::new(filter_entries)
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
