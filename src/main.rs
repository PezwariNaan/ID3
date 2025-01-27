struct Table {
    id: [i16; 7],
    stream: [bool; 7],
    slope: [&'static str; 7],
    elevation: [&'static str; 7],
    vegetation: [&'static str; 7],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ValueTypes {
    Str(&'static str),
    Bool(bool),
}

impl Table {
    fn new() -> Self {
    Self { 
        id: [1, 2, 3, 4, 5, 6, 7],
        stream: [false, true, true, false, false, true, true],
        slope: ["steep", "moderate", "steep", "steep", "flat", "steep", "steep"],
        elevation: ["high", "low", "medium", "medium", "high", "highest", "high"],
        vegetation: ["chapparal", "riparian", "riparian", "chapparal", "conifer", "conifer", "chapparal"],
        }
    }

    fn get_unique(&self, column: &str) -> Vec<ValueTypes> {
        let values: Vec<ValueTypes> = match column {
            "slope" => self.slope.iter().map(|&value| ValueTypes::Str(value)).collect(),
            "elevation" => self.elevation.iter().map(|&value| ValueTypes::Str(value)).collect(),
            "stream" => self.stream.iter().map(|&value| ValueTypes::Bool(value)).collect(),
            _ => panic!("Invalid Column Name :("),
        };

        let mut unique_values = Vec::new();
        for value in values {
            if !unique_values.contains(&value) {
                unique_values.push(value);
            }
        }
        
        return unique_values
    }
}

trait ID3 {
    fn calculate_probability(&self, column: &str, value: ValueTypes) -> f64;
    fn calculate_entropy(&self, column: Option<&str>) -> f64;
    fn calculate_information_gain(&self, column: &str) -> f64;
    fn generate_tree(&self);
}

impl ID3 for Table {
    fn calculate_probability(&self, column: &str, value: ValueTypes) -> f64 {
        let total_instances = self.id.len() as f64;
        let count = match column {
            "slope" => self.slope.iter().filter(|&&v| ValueTypes::Str(v) == value).count(),
            "elevation" => self.elevation.iter().filter(|&&v| ValueTypes::Str(v) == value).count(),
            "stream" => self.stream.iter().filter(|&&v| ValueTypes::Bool(v) == value).count(),
            _ => panic!("Invalid Column Name :("),
        } as f64;

        return  count / total_instances
    }

    fn calculate_entropy(&self, column: Option<&str>) -> f64 {
        match column {
            Some("slope") => {
                let unique_values = self.get_unique("slope");
                unique_values
                    .iter()
                    .map(|&value| {
                        let p = self.calculate_probability("slope", value);
                        if p > 0.0 {
                            -p * p.log2()
                        } else {
                            0.0
                        }
                    }).sum()
            }
            Some("elevation") => {
                let unique_values = self.get_unique("elevation");
                unique_values
                .iter()
                .map(|&value| {
                    let p = self.calculate_probability("elevation", value);
                    if p > 0.0 {
                        -p * p.log2()
                    } else {
                        0.0
                    }
                }).sum()
            }
            Some("stream") => {
                let unique_values = self.get_unique("stream");
                unique_values
                    .iter()
                    .map(|&value| {
                        let p = self.calculate_probability("stream", value);
                        if p > 0.0 {
                            -p * p.log2()
                        } else {
                            0.0
                        }
                    }).sum()
            }
            None => {
                let slope_entropy = self.calculate_entropy(Some("slope"));
                let elevation_entropy = self.calculate_entropy(Some("elevation"));
                let stream_entropy = self.calculate_entropy(Some("stream"));

                return slope_entropy + elevation_entropy + stream_entropy
            }

            _ => {
                panic!("Invalid Column Name :(");
            }
        }
    }

    fn calculate_information_gain(&self, column: &str) ->f64 {
        let overall_entropy = self.calculate_entropy(None);
        let unique_values = self.get_unique(column);
        let total_instances = self.id.len() as f64;

        let weighted_entropy: f64 = unique_values
            .iter()
            .map(|&value| {
                let subset_size = match column {
                    "slope" => self.slope.iter().filter(|&&v| ValueTypes::Str(v) == value).count(),
                    "elevation" => self.elevation.iter().filter(|&&v| ValueTypes::Str(v) == value).count(),
                    "stream" => self.stream.iter().filter(|&&v| ValueTypes::Bool(v) == value).count(),
                    _ => panic!("Invalid Column Name :("),
                } as f64;

                let subset_entropy = self.calculate_entropy(Some(column));

                println!("{} / {} * {}", subset_size, total_instances, subset_entropy);

                (subset_size / total_instances) * subset_entropy
            }).sum();

        println!("{} - {}", overall_entropy, weighted_entropy);

        return overall_entropy - weighted_entropy
    }

    fn generate_tree(&self) {
        println!("Generating Decision Tree :)");
        return
    }
}

fn main() {
    let veg_table = Table::new();
    
    let entropy = veg_table.calculate_information_gain("slope");
    return
}

