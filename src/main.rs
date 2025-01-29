#[derive(Debug)]
enum TableType {
    Int(i32),
    Str(&'static str),
    Bool(bool),
}

struct Table {
    id: [TableType; 7],
    stream: [TableType; 7],
    slope: [TableType; 7],
    elevation: [TableType; 7],
    vegetation: [TableType; 7],
}


impl From<i32> for TableType {
    fn from(value: i32) -> Self {
        TableType::Int(value)
    }
}

impl From<bool> for TableType {
    fn from(value: bool) -> Self {
        TableType::Bool(value)
    }
}

impl From <&'static str> for TableType {
    fn from(value:&'static str) -> Self {
        TableType::Str(value)
    }
}

impl Table {
    fn new() -> Self {
        Self {
            id: [1, 2, 3, 4, 5, 6, 7].map(Into::into),
            stream: [false, true, true, false, false, true, true].map(Into::into),
            slope: ["steep", "moderate", "steep", "steep", "flat", "steep", "steep"].map(Into::into),
            elevation: ["high", "low", "medium", "medium", "high", "highest", "high"].map(Into::into),
            vegetation: ["chapparal", "riparian", "riparian", "chapparal", "conifer", "conifer", "chapparal"].map(Into::into),
        }
    }
}

trait ID3 {
    fn calculate_probability(&self) -> f32;
    fn calculate_entropy(&self, column: TableType) -> f32;
    fn calculate_information_gain(&self, column: TableType, value: TableType) -> f32;
    fn build_tree(&self);
}

fn main() {
    let veg_table = Table::new();

    let streams: Vec<&TableType> = veg_table.stream.iter().collect();
    println!("{:?}", streams);

    return
}

