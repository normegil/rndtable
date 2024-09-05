use dashboard::Dashboard;
use workspaces::{DashboardFolder, HierarchyElement, Workspace};

pub mod dashboard;
pub mod workspaces;

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
}
