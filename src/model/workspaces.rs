use super::dashboard::Dashboard;

pub struct Workspace {
    name: String,
    hierarchy: Vec<HierarchyElement>
}

pub enum HierarchyElement {
    Dashboard(Dashboard),
    DashboardFolder(Box<DashboardFolder>),
}

pub struct DashboardFolder {
    name: String,
    hierarchy: Vec<HierarchyElement>,
    folded: bool,
}