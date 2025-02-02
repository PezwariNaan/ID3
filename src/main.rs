#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
enum TableType {
    Int(i32),
    Str(&'static str),
    Bool(bool),
}

#[derive(Debug)]
enum DecisionTree {
    Leaf(TableType),
    Node {
        feature: String,
        value: TableType,
        children: Vec<DecisionTree>,
    },
}


#[derive(Debug)]
struct Table {
    id: Vec<TableType>,
    stream: Vec<TableType>,
    slope: Vec<TableType>,
    elevation: Vec<TableType>,
    vegetation: Vec<TableType>,
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
            id: vec![1, 2, 3, 4, 5, 6, 7].into_iter().map(Into::into).collect(),
            stream: vec![false, true, true, false, false, true, true].into_iter().map(Into::into).collect(),
            slope: vec!["steep", "moderate", "steep", "steep", "flat", "steep", "steep"].into_iter().map(Into::into).collect(),
            elevation: vec!["high", "low", "medium", "medium", "high", "highest", "high"].into_iter().map(Into::into).collect(),
            vegetation: vec!["chapparal", "riparian", "riparian", "chapparal", "conifer", "conifer", "chapparal"].into_iter().map(Into::into).collect(),
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

    fn get_indices(&self, feature: &str, value: TableType) -> Vec<usize> {
        let indices: Vec<usize> = match feature {
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
            _ => panic!("Invalid Feature: {}", feature),
        };
        return indices
    }
}

trait ID3 {
    fn calculate_probability(&self, column: &str, value: TableType) -> f64;
    fn calculate_entropy(&self, column: &str) -> f64;
    fn calculate_information_gain(&self, column: &str, value: TableType) -> f64;
    fn partition_table(&self) -> (String, Vec<Table>);
    fn build_tree(&self) -> DecisionTree;
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
        let partition_indices: Vec<usize> = self.get_indices(column, value);
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
        return entropy - weighted_partition_entropy
    }

    fn partition_table(&self) -> (String, Vec<Table>) {
        let features = ["stream", "slope", "elevation"];
        let mut best_gain = 0.0;
        let mut best_feature = "";

        for &feature in &features {
            let unique_values = self.get_unique(feature);
            let mut feature_gain = 0.0;

            for &value in &unique_values {
                let gain = self.calculate_information_gain(feature, value);
                feature_gain += gain;
            }

            let avg_gain = feature_gain / unique_values.len() as f64;

            if avg_gain > best_gain {
                best_gain = avg_gain;
                best_feature = feature;
            }
        }

        let unique_values = self.get_unique(best_feature);
        let mut partitions = Vec::new();

        for &value in &unique_values {
            let indices: Vec<usize> = self.get_indices(best_feature, value);

            let partition = Table {
                id: indices.iter().map(|&i| self.id[i]).collect(),
                stream: indices.iter().map(|&i| self.stream[i]).collect(),
                slope: indices.iter().map(|&i| self.slope[i]).collect(),
                elevation: indices.iter().map(|&i| self.elevation[i]).collect(),
                vegetation: indices.iter().map(|&i| self.vegetation[i]).collect(),
            };
            
            partitions.push(partition);
        }

        return (best_feature.to_string(), partitions)
    }

    fn build_tree(&self) -> DecisionTree {
       let unique_vegetation = self.get_unique("vegetation");
       if unique_vegetation.len() == 1 { // Base case 1: Only one target feature
            return DecisionTree::Leaf(unique_vegetation[0])
       }

       let features = ["stream", "slope", "elevation"];
       if features.is_empty() { // Base case 2: No features
            let most_common = self
                .vegetation
                .iter()
                .max_by_key(|&v| self.vegetation.iter().filter(|&x| x == v).count())
                .unwrap();
           return DecisionTree::Leaf(*most_common)
       }

       let (best_feature, partitions) = self.partition_table();

       if partitions.is_empty() { // Base case 3: Empty dataset
            let most_common = self
                .vegetation
                .iter()
                .max_by_key(|&v| self.vegetation.iter().filter(|&x|x == v).count())
                .unwrap();
           return DecisionTree::Leaf(*most_common)
       }
        
       let mut children = Vec::new();
       for partition in partitions {
            let child_tree = partition.build_tree();
            children.push(child_tree);
       }

       DecisionTree::Node {
            feature: best_feature.clone(),
            value: self.get_unique(&best_feature)[0],
            children,
       }
    }
}

fn main() {
    let veg_table = Table::new();

    let decision_tree: DecisionTree = veg_table.build_tree();
    println!("{:#?}", decision_tree);

    return
}

