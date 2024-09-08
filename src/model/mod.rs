use dashboard::Dashboard;
use filters::Filter;
use thiserror::Error;
use workspaces::{DashboardFolder, HierarchyElement, Workspace};

pub mod dashboard;
pub mod filters;
pub mod workspaces;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Workspace {name} could not be found")]
    WorkspaceNotFound { name: String },
    #[error("Hierarchy element could not be found: Missing {missing}")]
    HierarchyElementNotFound { missing: String },
}

pub struct Model {
    pub current_workspace_name: String,
    pub workspaces: Vec<Workspace>,
    pub filter_list: Vec<Filter>,
}

impl Model {
    pub fn new() -> Model {
        Model {
            filter_list: vec![
                Filter {
                    name: "nsfw".to_string(),
                    enabled: false,
                },
                Filter {
                    name: "fantasy".to_string(),
                    enabled: false,
                },
                Filter {
                    name: "scifi".to_string(),
                    enabled: false,
                },
            ],
            current_workspace_name: "Cyberpunk RED".to_string(),
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

    pub fn get_current_workspace(&self) -> Result<&Workspace, Error> {
        return self
            .get_workspace(&self.current_workspace_name)
            .ok_or(Error::WorkspaceNotFound {
                name: self.current_workspace_name.to_string(),
            });
    }

    pub fn reverse_folding(&mut self, workspace_name: &str, id: &str) -> Result<(), Error> {
        let element = self.get_hierarchy_element_from_workspace(workspace_name, id)?;
        if let HierarchyElement::DashboardFolder(folder) = element {
            folder.folded = !folder.folded;
        }
        Ok(())
    }

    pub fn set_current_workspace(&mut self, workspace_name: &str) {
        self.current_workspace_name = workspace_name.to_string();
    }

    fn get_workspace<'a>(&'a self, workspace_name: &str) -> Option<&'a Workspace> {
        for workspace in &self.workspaces {
            if workspace.name == workspace_name.to_string() {
                return Some(workspace);
            }
        }
        None
    }

    fn get_workspace_mut<'a>(&'a mut self, workspace_name: &str) -> Option<&'a mut Workspace> {
        for workspace in self.workspaces.iter_mut() {
            if workspace.name == workspace_name.to_string() {
                return Some(workspace);
            }
        }
        None
    }

    fn get_hierarchy_element_from_workspace<'a>(
        &'a mut self,
        workspace_name: &str,
        id: &str,
    ) -> Result<&'a mut HierarchyElement, Error> {
        let workspace = self
            .get_workspace_mut(workspace_name)
            .ok_or(Error::WorkspaceNotFound {
                name: workspace_name.to_string(),
            })?;

        let path = id_to_path(id);
        let element = get_hierarchy_element(&mut workspace.hierarchy, path)?;

        return Ok(element);
    }
}

fn get_hierarchy_element<'a>(
    elements: &'a mut Vec<HierarchyElement>,
    path: Vec<String>,
) -> Result<&'a mut HierarchyElement, Error> {
    if path.len() == 1 {
        return get_hierarchy_element_by_name(elements, &path[0]).ok_or(
            Error::HierarchyElementNotFound {
                missing: path[0].clone(),
            },
        );
    } else {
        match get_hierarchy_element_by_name(elements, &path[0]).ok_or(
            Error::HierarchyElementNotFound {
                missing: path[0].clone(),
            },
        )? {
            HierarchyElement::Dashboard(_) => {
                return Err(Error::HierarchyElementNotFound {
                    missing: path[0].clone(),
                })
            }
            HierarchyElement::DashboardFolder(folder) => {
                return get_hierarchy_element(&mut folder.hierarchy, path[1..].to_vec());
            }
        };
    }
}

fn get_hierarchy_element_by_name<'a>(
    elements: &'a mut Vec<HierarchyElement>,
    name: &str,
) -> Option<&'a mut HierarchyElement> {
    for element in elements.iter_mut() {
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

pub fn id_to_path(id: &str) -> Vec<String> {
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
