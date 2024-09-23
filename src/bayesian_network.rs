// src/bayesian_network.rs ~=#######D]====A===r===c====M===o===o===n====<Lord[BAYESIAN-NETWORK]Xyn>=====S===t===u====d===i===o===s====[R|$>
#[allow(dead_code)] 
use std::collections::HashMap;
pub struct BayesianNetwork {
    nodes: HashMap<String, Node>,
}

#[derive(Clone)]
struct Node {
    name: String,
    parents: Vec<String>,
    cpt: HashMap<Vec<bool>, f64>, // Conditional Probability Table
}

impl BayesianNetwork {
    /// Initializes a new, empty Bayesian Network.
    pub fn new() -> Self {
        Self { nodes: HashMap::new() }
    }

    /// Adds a node to the Bayesian Network.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the node.
    /// * `parents` - A list of parent node names.
    /// * `cpt` - The Conditional Probability Table for the node.
    pub fn add_node(&mut self, name: String, parents: Vec<String>, cpt: HashMap<Vec<bool>, f64>) {
        self.nodes.insert(name.clone(), Node { name, parents, cpt });
    }

    /// Performs inference on the Bayesian Network given some evidence.
    ///
    /// # Arguments
    ///
    /// * `evidence` - A map of node names to their observed boolean values.
    ///
    /// # Returns
    ///
    /// * `HashMap<String, f64>` - The inferred probabilities for each node.
    pub fn infer(&self, evidence: &HashMap<String, bool>) -> HashMap<String, f64> {
        let mut beliefs = HashMap::new();
        for (name, node) in &self.nodes {
            if let Some(&value) = evidence.get(name) {
                beliefs.insert(name.clone(), if value { 1.0 } else { 0.0 });
            } else {
                let mut probability = 0.0;
                for (parent_values, prob) in &node.cpt {
                    if parent_values.iter().enumerate().all(|(i, &v)| {
                        evidence.get(&node.parents[i]).map_or(true, |&e| e == v)
                    }) {
                        probability += prob;
                    }
                }
                beliefs.insert(name.clone(), probability);
            }
        }
        beliefs
    }

    /// Reasons about a specific query node given the evidence.
    ///
    /// # Arguments
    ///
    /// * `query` - The name of the node to query.
    /// * `evidence` - A map of node names to their observed boolean values.
    ///
    /// # Returns
    ///
    /// * `Option<f64>` - The probability of the query node being true.
    pub fn reason(&self, query: &str, evidence: &HashMap<String, bool>) -> Option<f64> {
        self.infer(evidence).get(query).cloned()
    }

    /// Validates a prediction based on the Bayesian Network's inference.
    ///
    /// # Arguments
    ///
    /// * `prediction` - The name of the prediction node.
    /// * `evidence` - A map of node names to their observed boolean values.
    ///
    /// # Returns
    ///
    /// * `bool` - `true` if the prediction is valid, otherwise `false`.
    pub fn validate_prediction(&self, prediction: &str, evidence: &HashMap<String, bool>) -> bool {
        if let Some(prob) = self.reason(prediction, evidence) {
            prob > 0.5
        } else {
            false
        }
    }

    /// Updates the Conditional Probability Table (CPT) of a specific node.
    ///
    /// # Arguments
    ///
    /// * `node` - The name of the node to update.
    /// * `new_cpt` - The new Conditional Probability Table.
    pub fn update_belief(&mut self, node: &str, new_cpt: HashMap<Vec<bool>, f64>) {
        if let Some(n) = self.nodes.get_mut(node) {
            n.cpt = new_cpt;
        }
    }
}