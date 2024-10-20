use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct NodeManagement {
    nodes: Vec<Node>,
    node_status: HashMap<String, NodeStatus>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    id: String,
    name: String,
    status: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NodeStatus {
    cpu_usage: f64,
    memory_usage: u32,
    disk_usage: u64,
}


impl Default for Node {
    fn default() -> Self {
        Self {
            id: "default-id".to_string(),
            name: "Unnamed Node".to_string(),
            status: "unknown".to_string(),
        }
    }
}

impl Default for NodeStatus {
    fn default() -> Self {
        Self {
            cpu_usage: 0.0,
            memory_usage: 0,
            disk_usage: 0,
        }
    }
}

impl Default for NodeManagement {
    fn default() -> Self {
        let mut node_status = HashMap::new();
        node_status.insert("default-id".to_string(), NodeStatus::default());

        Self {
            nodes: vec![Node::default()],
            node_status,
        }
    }
}

#[allow(dead_code)]
impl Node {
    pub fn new(id: &str, name: &str, status: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            status: status.to_string(),
        }
    }

    pub fn update_status(&mut self, status: &str) {
        self.status = status.to_string();
    }

    pub fn update_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_status(&self) -> &str {
        &self.status
    }
}

#[allow(dead_code)]
impl NodeStatus {
    pub fn new(cpu_usage: f64, memory_usage: u32, disk_usage: u64) -> Self {
        Self {
            cpu_usage,
            memory_usage,
            disk_usage,
        }
    }

    pub fn update_cpu_usage(&mut self, cpu_usage: f64) {
        self.cpu_usage = cpu_usage;
    }

    pub fn update_memory_usage(&mut self, memory_usage: u32) {
        self.memory_usage = memory_usage;
    }

    pub fn update_disk_usage(&mut self, disk_usage: u64) {
        self.disk_usage = disk_usage;
    }
    pub fn get_cpu_usage(&self) -> f64 {
        self.cpu_usage
    }
    pub fn get_memory_usage(&self) -> u32 {
        self.memory_usage
    }
    pub fn get_disk_usage(&self) -> u64 {
        self.disk_usage
    }
}

#[allow(dead_code)]
impl NodeManagement {
    pub fn new(nodes: Vec<Node>, node_status: HashMap<String, NodeStatus>) -> Self {
        Self { nodes, node_status }
    }

    pub fn add_node(&mut self, node: Node, status: NodeStatus) {
        let id = node.id.clone();
        self.nodes.push(node);
        self.node_status.insert(id, status);
    }

    pub fn remove_node(&mut self, id: &str) {
        self.nodes.retain(|node| node.id != id);
        self.node_status.remove(id);
    }

    pub fn get_node(&self, id: &str) -> Option<&Node> {
        self.nodes.iter().find(|node| node.id == id)
    }

    pub fn update_node(&mut self, id: &str, name: Option<&str>, status: Option<&str>) {
        if let Some(node) = self.nodes.iter_mut().find(|node| node.id == id) {
            if let Some(name) = name {
                node.update_name(name);
            }
            if let Some(status) = status {
                node.update_status(status);
            }
        }
    }

    pub fn get_node_status(&self, id: &str) -> Option<&NodeStatus> {
        self.node_status.get(id)
    }

    pub fn update_node_status(
        &mut self,
        id: &str,
        cpu_usage: Option<f64>,
        memory_usage: Option<u32>,
        disk_usage: Option<u64>,
    ) {
        if let Some(status) = self.node_status.get_mut(id) {
            if let Some(cpu) = cpu_usage {
                status.update_cpu_usage(cpu);
            }
            if let Some(memory) = memory_usage {
                status.update_memory_usage(memory);
            }
            if let Some(disk) = disk_usage {
                status.update_disk_usage(disk);
            }
        }
    }
}
