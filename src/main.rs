struct Table {
    id: [i16; 7],
    stream: [bool; 7],
    slope: [&'static str; 7],
    elevation: [&'static str; 7],
    vegetation: [&'static str; 7],
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

    fn get_unique_stream(&self) -> Vec<bool> {
        let mut unique_stream = Vec::new();
        for &i in &self.stream {
            if !unique_stream.contains(&i) {
                unique_stream.push(i)
            }
        }
        return unique_stream
    }

    fn get_unique_slope(&self) -> Vec<&str> {
        let mut unique_slope = Vec::new();
        for &i in &self.slope {
            if !unique_slope.contains(&i) {
                unique_slope.push(i)
            }
        }
        return unique_slope
    }

    fn get_unique_elevation(&self) -> Vec<&str> {
        let mut unique_elevation = Vec::new();
        for &i in &self.elevation {
            if !unique_elevation.contains(&i) {
                unique_elevation.push(i);
            }
        }
        return unique_elevation
    }

    fn get_unique_vegetation(&self) -> Vec<&str> {
        let mut unique_vegetation = Vec::new();
        for &i in &self.vegetation {
            if !unique_vegetation.contains(&i) {
                unique_vegetation.push(i);
            }
        }
        return unique_vegetation
    }
}

trait ID3 {
    fn calculate_entropy(&self);
    fn calculate_information_gain(&self);
    fn generate_tree(&self);
}

//impl ID3 for Table {}

fn main() {
    let veg_table = Table::new();
    let unique_streams = veg_table.get_unique_vegetation();
    for i in unique_streams {
        println!("{}", i);
    }

    return
}

