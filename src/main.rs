use slint::{ModelRc, PlatformError, SharedString, VecModel};

slint::include_modules!();

mod model;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let workspaces = get_workspaces();
    let current_workspace = workspaces
        .get(0)
        .ok_or(PlatformError::Other("Test".to_string()))?
        .as_str();
    ui.set_workspaces(to_workspace_model(&workspaces));
    ui.set_current_workspace(SharedString::from(current_workspace));

    let entries = get_entries();
    ui.set_generation_entries(to_entries_model(&entries));

    ui.on_clicked_generator_entry(|id| {
        println!("Entry clicked: {id}");
    });

    ui.run()
}

fn to_workspace_model(workspaces: &Vec<String>) -> ModelRc<SharedString> {
    let tmp = workspaces
        .iter()
        .map(|s| SharedString::from(s))
        .collect::<Vec<SharedString>>();
    let tmp = VecModel::from(tmp);
    ModelRc::new(tmp)
}

fn to_entries_model(entries: &RecursiveHierarchyEntry) -> ModelRc<HierarchyEntry> {
    let tmp = flatten_entry(entries, 0, "/");
    let tmp = VecModel::from(tmp);
    ModelRc::new(tmp)
}

fn flatten_entry(entry: &RecursiveHierarchyEntry, identation: i32, parent_id: &str) -> Vec<HierarchyEntry> {
    let current_id = parent_id.to_string() + "/" + &entry.title;

    let mut entries = vec![HierarchyEntry {
        id: SharedString::from(&current_id),
        folded: entry.folded.unwrap_or(true),
        identation: identation,
        is_folder: entry.folded.is_some(),
        title: SharedString::from(entry.title.as_str()),
    }];

    if entry.folded.is_some_and(|folded| !folded) {
        for subentry in &entry.subentries {
            entries.extend(flatten_entry(subentry, identation + 1, &current_id))
        }
    }

    entries
}

fn get_workspaces() -> Vec<String> {
    vec![
        "Cyberpunk RED".to_string(),
        "Dungeons & Dragons".to_string(),
        "Pathfinder".to_string(),
        "Warhammer Fantasy".to_string(),
        "Autres".to_string(),
    ]
}

struct RecursiveHierarchyEntry {
    title: String,
    subentries: Vec<RecursiveHierarchyEntry>,
    folded: Option<bool>,
}

fn get_entries() -> RecursiveHierarchyEntry {
    RecursiveHierarchyEntry {
        title: "Root".to_string(),
        folded: Some(false),
        subentries: vec![
            RecursiveHierarchyEntry {
                title: "Dossier".to_string(),
                folded: Some(true),
                subentries: vec![],
            },
            RecursiveHierarchyEntry {
                title: "Autre Dossier".to_string(),
                folded: Some(true),
                subentries: vec![],
            },
            RecursiveHierarchyEntry {
                title: "PNJs".to_string(),
                folded: None,
                subentries: vec![],
            },
            RecursiveHierarchyEntry {
                title: "Environments".to_string(),
                folded: None,
                subentries: vec![],
            },
        ],
    }
}
