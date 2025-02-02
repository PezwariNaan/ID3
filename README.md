# ID3 Decision Tree Implementation in Rust 🌳🤖✨

## Introduction ✨📚🔍

This repository provides an implementation of the ID3 (Iterative Dichotomiser 3) algorithm in Rust. The ID3 algorithm is a foundational approach in machine learning for constructing decision trees used in classification tasks. 🚀 This implementation showcases Rust’s type safety and memory management features while applying key machine learning concepts. 🛠️

## How the ID3 Algorithm Works ⚙️📈🌟

**Entropy Calculation**: Determines the disorder within the dataset.

**Information Gain Computation**: Finds the best attribute for splitting the dataset.

**Dataset Partitioning**: Splits data based on the chosen attribute.

**Recursive Tree Construction**: Builds the tree until reaching a stopping criterion.

## Features ✨🔧🚀

Pure Rust implementation.

Uses an information-theoretic approach for attribute selection.

Handles categorical and boolean attributes.

Recursively builds a decision tree from a dataset.

Efficient memory management leveraging Rust’s ownership model.

## Installation 🛠️💻📦

To use this implementation, ensure you have Rust installed. Clone the repository and run the program using:
``` Bash
# Clone the repository
git clone https://github.com/your-username/id3-rust.git

# Navigate to the project directory
cd id3-rust

# Run the program
cargo run
```
## Code Overview 📝📌📜

1. Defining Data Structures 🏗️📊🛠️

The TableType enum represents the different types of feature values:
``` Rust
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
enum TableType {
    Int(i32),
    Str(&'static str),
    Bool(bool),
}

The DecisionTree enum defines the structure of the decision tree:

#[derive(Debug)]
enum DecisionTree {
    Leaf(TableType),
    Node {
        feature: String,
        value: TableType,
        children: Vec<DecisionTree>,
    },
}
```
2. Implementing the ID3 Algorithm ⚙️📈🌟

Entropy Computation 📉⚖️🔢
``` Rust
fn calculate_entropy(&self, column: &str) -> f64 {
    let mut entropy = 0.0;
    let unique_values = self.get_unique(column);
    
    for &value in &unique_values {
        let p = self.calculate_probability(column, value);
        if p > 0.0 {
            entropy += p * -p.log2();
        }
    }
    
    entropy
}
```
Selecting the Best Splitting Feature 🔄🌍🔍
``` Rust
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

    (best_feature.to_string(), partitions)
}
```
Constructing the Decision Tree 🌳🔄🌟
``` Rust
fn build_tree(&self) -> DecisionTree {
    let unique_vegetation = self.get_unique("vegetation");
    if unique_vegetation.len() == 1 {
        return DecisionTree::Leaf(unique_vegetation[0]);
    }
    
    let (best_feature, partitions) = self.partition_table();
    if partitions.is_empty() {
        let most_common = self.vegetation.iter().max_by_key(|&v| self.vegetation.iter().filter(|&x| x == v).count()).unwrap();
        return DecisionTree::Leaf(*most_common);
    }
    
    let mut children = Vec::new();
    for partition in partitions {
        children.push(partition.build_tree());
    }
    
    DecisionTree::Node {
        feature: best_feature.clone(),
        value: self.get_unique(&best_feature)[0],
        children,
    }
}
```
3. Running the ID3 Algorithm 🎮🚀🌟
```Rust
fn main() {
    let veg_table = Table::new();
    let decision_tree: DecisionTree = veg_table.build_tree();
    println!("{:#?}", decision_tree);
}
```
Output Example 📜🔎🎯

Upon execution, the program will print the constructed decision tree in a structured format:
``` 
Node {
    feature: "slope",
    value: Str("steep"),
    children: [
        Leaf(Str("chapparal")),
        Node {
            feature: "stream",
            value: Bool(true),
            children: [
                Leaf(Str("riparian")),
                Leaf(Str("conifer"))
            ]
        }
    ]
}
```
## Conclusion 🏆🔬💡

This Rust-based implementation of the ID3 algorithm demonstrates the power of decision tree learning while leveraging Rust’s safety and performance advantages. 🎯 By calculating entropy, partitioning the dataset, and recursively constructing a decision tree, this approach provides an efficient and robust classification mechanism. 🚀✨

## License 📜⚖️🛡️

This project is licensed under the MIT License. See the LICENSE file for details.

## Contributions 🤝🌍✨

Contributions, issues, and feature requests are welcome! Feel free to fork this repository, open issues, or submit pull requests. 🎉
