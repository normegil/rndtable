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
    #[error("Hierarchy element search cannot be executed on empty path")]
    HierarchyElementEmptyPathSearch,
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

    pub fn is_generator_folder(&self, workspace_name: &str, id: &str) -> Result<bool, Error> {
        let element = get_hierarchy_element_from_workspace(&self.workspaces, workspace_name, id)?;

        match element {
            HierarchyElement::Dashboard(_) => return Ok(false),
            HierarchyElement::DashboardFolder(_) => return Ok(true),
        }
    }

    pub fn reverse_folding(&self, workspace_name: &str, id: &str) -> Result<(), Error> {
        let element = get_hierarchy_element_from_workspace(&self.workspaces, workspace_name, id)?;
        if let HierarchyElement::DashboardFolder(folder) = element {
            folder.folded = !folder.folded;
        }
        Ok(())
    }
}

fn get_hierarchy_element_from_workspace<'a>(
    workspaces: &'a Vec<Workspace>,
    workspace_name: &str,
    id: &str,
) -> Result<&'a HierarchyElement, Error> {
    let workspace = get_workspace(workspaces, workspace_name).ok_or(Error::WorkspaceNotFound {
        name: workspace_name.to_string(),
    })?;

    let path = id_to_path(id);
    let element = get_hierarchy_element(&workspace.hierarchy, path)?
        .ok_or(Error::HierarchyElementEmptyPathSearch)?;

    return Ok(element);
}

fn get_workspace<'a>(
    workspaces: &'a Vec<Workspace>,
    workspace_name: &str,
) -> Option<&'a Workspace> {
    for workspace in workspaces {
        if workspace.name == workspace_name.to_string() {
            return Some(workspace);
        }
    }
    None
}

fn get_hierarchy_element<'a>(
    elements: &'a mut Vec<HierarchyElement>,
    path: Vec<String>,
) -> Result<Option<&'a mut HierarchyElement>, Error> {
    let mut element: Option<&mut HierarchyElement> = None;
    for p in path {
        let mut child_list;
        if let Some(el) = element {
            match el {
                HierarchyElement::Dashboard(_) => {
                    return Err(Error::HierarchyElementNotFound {
                        missing: p.to_string(),
                    })
                }
                HierarchyElement::DashboardFolder(fold) => child_list = &mut fold.hierarchy,
            }
        } else {
            child_list = elements;
        }
        element = Some(get_hierarchy_element_by_name(&mut child_list, &p).ok_or(
            Error::HierarchyElementNotFound {
                missing: p.to_string(),
            },
        )?);
    }

    Ok(element)
}

fn get_hierarchy_element2<'a>(
    elements: &'a mut Vec<HierarchyElement>,
    path: Vec<String>,
) -> Result<Option<&'a mut HierarchyElement>, Error> {
    if path.len() == 1 {
        
        return ;
    } else {

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
