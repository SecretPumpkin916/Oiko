use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Project {
    pub name: String,
    pub version: String,
    pub description: String,
    pub cid: String,
    pub author: String,
}

pub struct Catalog {
    file_path: PathBuf,
    pub projects: Vec<Project>,
}

impl Catalog {
    // Load catalog from local JSON file
    pub fn load() -> Self {
        let file_path = PathBuf::from("oiko_catalog.json");
        let projects = if file_path.exists() {
            let data = fs::read_to_string(&file_path).unwrap_or_default();
            serde_json::from_str(&data).unwrap_or_default()
        } else {
            Vec::new()
        };

        Catalog { file_path, projects }
    }

    // Save catalog to local JSON file
    pub fn save(&self) -> Result<(), String> {
        let data = serde_json::to_string_pretty(&self.projects)
            .map_err(|e| format!("Failed to serialize catalog: {}", e))?;
        
        fs::write(&self.file_path, data)
            .map_err(|e| format!("Failed to write catalog file: {}", e))?;
        
        Ok(())
    }

    // Add a new project to the catalog
    pub fn add_project(&mut self, project: Project) -> Result<(), String> {
        // Prevent duplicate project names
        if self.projects.iter().any(|p| p.name == project.name) {
            return Err(format!("Project '{}' already exists. Use 'update' to change it.", project.name));
        }
        self.projects.push(project);
        self.save()
    }

    // Remove a project from the catalog
    pub fn remove_project(&mut self, name: &str) -> Result<(), String> {
        let initial_len = self.projects.len();
        self.projects.retain(|p| p.name != name);
        
        if self.projects.len() == initial_len {
            return Err(format!("Project '{}' not found in catalog.", name));
        }
        self.save()
    }
}