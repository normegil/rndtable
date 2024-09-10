pub struct Dashboard {
    pub name: String,
    pub content: Vec<Section>,
}

pub enum Level {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

pub struct Section {
    pub title: String,
    pub level: Level,
    pub subsections: Box<Section>,
    pub content: Vec<ContentType>,
}

pub enum ContentType {
    Static,
    Simple,
    List,
    Table,
    Image,
}
