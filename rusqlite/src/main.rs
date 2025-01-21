use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::error::Error;
use std::fmt;

// Custom error type
#[derive(Debug)]
pub enum DbError {
    KeyNotFound,
    InvalidOperation,
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DbError::KeyNotFound => write!(f, "Key not found in database"),
            DbError::InvalidOperation => write!(f, "Invalid operation"),
        }
    }
}

impl Error for DbError {}

// Database struct
#[derive(Clone)]
pub struct Database {
    data: Arc<Mutex<HashMap<String, String>>>,
}

impl Database {
    // Constructor - similar to Python's __init__
    pub fn new() -> Self {
        Database {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    // CRUD Operations
    
    // Create/Update - Similar to Python's dict[key] = value
    pub fn set(&self, key: String, value: String) -> Result<(), Box<dyn Error>> {
        let mut data = self.data.lock().map_err(|_| DbError::InvalidOperation)?;
        data.insert(key, value);
        Ok(())
    }

    // Read - Similar to Python's dict[key]
    pub fn get(&self, key: &str) -> Result<String, Box<dyn Error>> {
        let data = self.data.lock().map_err(|_| DbError::InvalidOperation)?;
        data.get(key)
            .cloned()
            .ok_or_else(|| DbError::KeyNotFound.into())
    }

    // Delete - Similar to Python's del dict[key]
    pub fn delete(&self, key: &str) -> Result<(), Box<dyn Error>> {
        let mut data = self.data.lock().map_err(|_| DbError::InvalidOperation)?;
        if data.remove(key).is_some() {
            Ok(())
        } else {
            Err(DbError::KeyNotFound.into())
        }
    }

    // Additional utility methods
    
    // List all keys - Similar to Python's dict.keys()
    pub fn keys(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let data = self.data.lock().map_err(|_| DbError::InvalidOperation)?;
        Ok(data.keys().cloned().collect())
    }

    // Get size - Similar to Python's len(dict)
    pub fn size(&self) -> Result<usize, Box<dyn Error>> {
        let data = self.data.lock().map_err(|_| DbError::InvalidOperation)?;
        Ok(data.len())
    }

    // Clear all data - Similar to Python's dict.clear()
    pub fn clear(&self) -> Result<(), Box<dyn Error>> {
        let mut data = self.data.lock().map_err(|_| DbError::InvalidOperation)?;
        data.clear();
        Ok(())
    }
}

// Example usage
fn main() -> Result<(), Box<dyn Error>> {
    // Create a new database instance
    let db = Database::new();

    // Set some values
    db.set("name".to_string(), "John Doe".to_string())?;
    db.set("age".to_string(), "30".to_string())?;

    // Get a value
    let name = db.get("name")?;
    println!("Name: {}", name);

    // List all keys
    let keys = db.keys()?;
    println!("All keys: {:?}", keys);

    // Delete a key
    db.delete("age")?;

    // Get database size
    let size = db.size()?;
    println!("Database size: {}", size);

    Ok(())
}