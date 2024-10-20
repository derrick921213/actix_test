use crate::data::NodeManagement;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{Error, ErrorKind, Result},
    path::Path,
    sync::{Arc, Mutex},
};

pub type NodeManagementMap = HashMap<String, Arc<Mutex<NodeManagement>>>;

fn check_exists(f: &Path) -> Result<NodeManagement> {
    if f.exists() {
        let file = File::open(f)?;
        let data: NodeManagement = serde_json::from_reader(file)?;
        println!("Loaded data from {:?}:", f);
        Ok(data)
    } else {
        let default_data = NodeManagement::default();
        let file = File::create(f)?;
        serde_json::to_writer_pretty(file, &default_data)?;
        println!("Created new default data at {:?}", f);
        Ok(default_data)
    }
}

pub fn init_data() -> Result<NodeManagementMap> {
    let data_path = Path::new("data");

    let all_files = [
        data_path.join("NodeManagement.json"),
    ];

    if !data_path.exists() {
        fs::create_dir(data_path).map_err(|e| {
            Error::new(
                ErrorKind::Other,
                format!("Failed to create data directory: {}", e),
            )
        })?;
    }

    let mut data_map = HashMap::new();

    for f in &all_files {
        let data = check_exists(f)?;
        if let Some(stem) = f.file_stem().and_then(|s| s.to_str()) {
            data_map.insert(stem.to_string(), Arc::new(Mutex::new(data)));
        }
    }

    Ok(data_map)
}
