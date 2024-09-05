use model::workspaces::{HierarchyElement, Workspace};
use slint::{ModelRc, PlatformError, SharedString, VecModel};

slint::include_modules!();

mod model;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let active_model = model::Model::new();
    let current_workspace = active_model
        .workspaces
        .get(0)
        .ok_or(PlatformError::Other("Test".to_string()))?;
    let current_workspace_name = current_workspace.name.as_str();
    ui.set_workspaces(to_workspace_model(&active_model.workspaces));
    ui.set_current_workspace(SharedString::from(current_workspace_name));

    ui.set_generation_entries(to_entries_model(&current_workspace.hierarchy));

    ui.on_clicked_generator_entry(|id| {
        println!("Entry clicked: {id}");
    });

    ui.run()
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
