use super::dashboard::Dashboard;

pub struct Workspace {
    pub name: String,
    pub hierarchy: Vec<HierarchyElement>,
}

pub enum HierarchyElement {
    Dashboard(Dashboard),
    DashboardFolder(Box<DashboardFolder>),
}

pub struct DashboardFolder {
    pub name: String,
    pub hierarchy: Vec<HierarchyElement>,
    pub folded: bool,
}
