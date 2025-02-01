#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
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

    fn get_unique(&self, column: &str) -> Vec<TableType> {
        let mut values: Vec<TableType> = match column {
            "stream" => self.stream.iter().cloned().collect(),
            "slope" => self.slope.iter().cloned().collect(),
            "elevation" => self.elevation.iter().cloned().collect(),
            "vegetation" => self.vegetation.iter().cloned().collect(),
            _ => panic!("Invalid Column: {}", column),
        };
        
        values.sort();
        values.dedup();
        return values
    }
}

trait ID3 {
    fn calculate_probability(&self, column: &str, value: TableType) -> f64;
    fn calculate_entropy(&self, column: &str) -> f64;
    fn calculate_information_gain(&self, column: &str, value: TableType) -> f64;
    //fn partition_table(&self) -> Table;
    //fn build_tree(&self);
}

impl ID3 for Table {
    fn calculate_probability(&self, column: &str, value: TableType) -> f64 {
        let count = match column {
            "stream" => self.stream.iter().filter(|&v| v == &value).count(),
            "slope" => self.slope.iter().filter(|&v| v == &value).count(),
            "elevation" => self.elevation.iter().filter(|&v| v == &value).count(),
            _ => panic!("Invalid Column: {}", column),
        };

        return count as f64 / self.id.len() as f64
    }

    fn calculate_entropy(&self, column: &str) -> f64 {
        let mut entropy = 0.0;
        let unique_values = match column {
            "stream" => self.get_unique("stream"),
            "slope" => self.get_unique("slope"),
            "elevation" => self.get_unique("elevation"),
            _ => panic!("Invalid Column :("),
        };
    
        for &value in &unique_values {
            let p = self.calculate_probability(column, value);
            if p > 0.0 {
                entropy += p * -p.log2();
            }
        }

        return entropy
    }
    
    fn calculate_information_gain(&self, column: &str, value: TableType) ->f64 {
        let entropy = self.calculate_entropy(column);

        let partition_indices: Vec<usize> = match column {
            "stream" => self
                .stream
                .iter()
                .enumerate()
                .filter(|&(_, &v)| v == value)
                .map(|(i, _)| i)
                .collect(),
            "slope" => self
                .slope
                .iter()
                .enumerate()
                .filter(|&(_, &v)| v == value)
                .map(|(i, _)| i)
                .collect(),
            "elevation" => self
                .elevation
                .iter()
                .enumerate()
                .filter(|&(_, &v)| v == value)
                .map(|(i, _)| i)
                .collect(),
            _ => panic!("Invalid Column :("),
        };

        let partition_size = partition_indices.len() as f64;
        let total_size = self.id.len() as f64;

        if partition_size == 0.0 {
            return 0.0
        }

        let mut partition_entropy = 0.0;
        let unique_vegetation = self.get_unique("vegetation");

        for veg in unique_vegetation {
            let count = partition_indices
                .iter()
                .filter(|&&i| self.vegetation[i] == veg)
                .count() as f64;

            if count > 0.0 {
                let p = count / partition_size;
                partition_entropy += p * -p.log2();
            }
        }

        let weighted_partition_entropy = (partition_size / total_size) * partition_entropy;
        entropy - weighted_partition_entropy
    }
    //fn partition_table(&self) -> Table {}
    //fn build_tree(&self) {}
}

fn main() {
    let veg_table = Table::new();

    let information_gain = veg_table.calculate_information_gain("elevation", TableType::Str("high"));
    println!("{}", information_gain);

    return
}

