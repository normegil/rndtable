pub struct Table {
    title: String,
    tags: Vec<Tag>,
    dice: String,
    values: Vec<TableEntry>,
}

pub struct TableEntry {
    lower_bound: i32,
    upper_bound: i32,
    tags: Vec<Tag>,
    value: String,
}

pub struct Tag {
    name: String,
}

// Support for multi-parameter entry ?
