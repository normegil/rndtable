use core::pat;

use dashboard::Dashboard;
use thiserror::Error;
use workspaces::{DashboardFolder, HierarchyElement, Workspace};

pub mod dashboard;
pub mod workspaces;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Workspace {name} could not be found")]
    WorkspaceNotFound { name: String },
    #[error("Hierarchy element could not be found: Missing {missing}")]
    HierarchyElementNotFound { missing: String },
}

pub struct Model {
    pub workspaces: Vec<Workspace>,
}

impl Model {
    pub fn new() -> Model {
        Model {
            workspaces: vec![
                Workspace {
                    name: "Cyberpunk RED".to_string(),
                    hierarchy: vec![
                        HierarchyElement::DashboardFolder(Box::new(DashboardFolder {
                            name: "Dossier".to_string(),
                            folded: true,
                            hierarchy: vec![
                                HierarchyElement::Dashboard(Dashboard {
                                    name: "Item 1".to_string(),
                                }),
                                HierarchyElement::Dashboard(Dashboard {
                                    name: "Item 2".to_string(),
                                }),
                            ],
                        })),
                        HierarchyElement::DashboardFolder(Box::new(DashboardFolder {
                            name: "Emtpy Dossier".to_string(),
                            folded: true,
                            hierarchy: vec![],
                        })),
                        HierarchyElement::Dashboard(Dashboard {
                            name: "PNJs".to_string(),
                        }),
                        HierarchyElement::Dashboard(Dashboard {
                            name: "Environments".to_string(),
                        }),
                    ],
                },
                Workspace {
                    name: "Donjons et Dragons".to_string(),
                    hierarchy: vec![
                        HierarchyElement::DashboardFolder(Box::new(DashboardFolder {
                            name: "Donjons".to_string(),
                            folded: true,
                            hierarchy: vec![
                                HierarchyElement::Dashboard(Dashboard {
                                    name: "Donjon 1".to_string(),
                                }),
                                HierarchyElement::Dashboard(Dashboard {
                                    name: "Donjon 2".to_string(),
                                }),
                            ],
                        })),
                        HierarchyElement::DashboardFolder(Box::new(DashboardFolder {
                            name: "Emtpy Dossier".to_string(),
                            folded: true,
                            hierarchy: vec![],
                        })),
                        HierarchyElement::Dashboard(Dashboard {
                            name: "Armes".to_string(),
                        }),
                        HierarchyElement::Dashboard(Dashboard {
                            name: "Objets".to_string(),
                        }),
                    ],
                },
            ],
        }
    }

    pub fn is_generator_folder(&self, workspace: &str, id: &str) -> Result<bool, Error> {
        let workspace = self
            .get_workspace(workspace)
            .ok_or(Error::WorkspaceNotFound {
                name: workspace.to_string(),
            })?;

        Ok(true)
    }

    fn get_workspace(&self, workspace_name: &str) -> Option<&Workspace> {
        for workspace in &self.workspaces {
            if workspace.name == workspace_name {
                Some(workspace);
            }
        }
        None
    }
}

fn get_hierarchy_element<'a>(
    elements: &'a Vec<HierarchyElement>,
    path: Vec<String>,
    index: usize,
) -> Result<Option<&'a HierarchyElement>, Error> {
    let element = get_hierarchy_element_by_name(elements, &path[index]).ok_or(
        Error::HierarchyElementNotFound {
            missing: path[index].to_string(),
        },
    )?;

    Ok(None)
    // for workspace in &self.workspaces {
    //     if workspace.name == workspace_name {
    //         Some(workspace);
    //     }
    // }
    // None
}

fn get_hierarchy_element_by_name<'a>(
    elements: &'a Vec<HierarchyElement>,
    name: &str,
) -> Option<&'a HierarchyElement> {
    for element in elements {
        match element {
            HierarchyElement::Dashboard(dashboard) => {
                if dashboard.name == name {
                    return Some(element);
                }
            }
            HierarchyElement::DashboardFolder(folder) => {
                if folder.name == name {
                    return Some(element);
                }
            }
        }
    }
    None
}

fn id_to_path(id: &str) -> Vec<String> {
    id.split("/")
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
}

#[cfg(test)]
mod tests {
    use super::id_to_path;

    #[test]
    fn test_id_to_path() {
        let path = id_to_path("//Root/Test/Fortiu");
        assert_eq!("Root", path[0]);
        assert_eq!("Test", path[1]);
        assert_eq!("Fortiu", path[2]);
    }
}
