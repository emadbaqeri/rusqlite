// src/types.rs

use serde::{Serialize, Deserialize};
use std::convert::{TryFrom, TryInto};
use std::fmt;

/// Unique identifier for a page in the database file
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PageId(pub u64);

impl PageId {
    /// First page in the database (used for header)
    pub const FIRST: PageId = PageId(0);
}

impl fmt::Display for PageId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PageId({})", self.0)
    }
}

/// Supported data types for column values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataType {
    Integer,
    Text,
    Boolean,
    Float,
    // Add more types as needed
}

/// Database metadata stored in the first page
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseHeader {
    pub root_page_id: PageId,
    pub schema_version: u32,
    pub page_count: u64,
    pub freelist_head: Option<PageId>,
}

/// Page type identifier (stored in page header)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PageType {
    Metadata,
    Internal,
    Leaf,
    Freelist,
}

/// Common page header format (16 bytes)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageHeader {
    pub page_type: PageType,
    pub checksum: u32,
    pub next_page: Option<PageId>,
    pub payload_size: u16,
}

/// Full page structure with header and data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    pub header: PageHeader,
    pub data: Vec<u8>,
}

/// Row identifier type (64-bit)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RowId(pub u64);

/// Error type for data conversion failures
#[derive(Debug, thiserror::Error)]
pub enum TypeError {
    #[error("Invalid type conversion")]
    InvalidConversion,
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] bincode::Error),
    
    #[error("Page format invalid")]
    InvalidPageFormat,
}

// Conversion traits for safe type handling
impl TryFrom<&[u8]> for PageId {
    type Error = TypeError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let arr: [u8; 8] = bytes.try_into().map_err(|_| TypeError::InvalidConversion)?;
        Ok(PageId(u64::from_be_bytes(arr)))
    }
}

impl From<PageId> for [u8; 8] {
    fn from(page_id: PageId) -> Self {
        page_id.0.to_be_bytes()
    }
}
