use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

const PAGE_SIZE: usize = 4096; // 4KB pages

pub struct Pager {
    file: File,
    pages: HashMap<u64, Vec<u8>>, // In-memory page cache
    pub num_pages: u64,
}

impl Pager {
    pub fn new(path: &Path) -> Result<Self, DbError> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;

        let metadata = file.metadata()?;
        let num_pages = metadata.len() / PAGE_SIZE as u64;

        Ok(Pager {
            file,
            pages: HashMap::new(),
            num_pages,
        })
    }

    // Fetch a page (cached or from disk)
    pub fn get_page(&mut self, page_id: u64) -> Result<&mut Vec<u8>, DbError> {
        if !self.pages.contains_key(&page_id) {
            let mut buffer = vec![0; PAGE_SIZE];
            let offset = page_id * PAGE_SIZE as u64;

            self.file.seek(SeekFrom::Start(offset))?;
            self.file.read_exact(&mut buffer)?;

            self.pages.insert(page_id, buffer);
        }

        Ok(self.pages.get_mut(&page_id).unwrap())
    }

    // Flush a page to disk
    pub fn flush_page(&mut self, page_id: u64) -> Result<(), DbError> {
        if let Some(page) = self.pages.get(&page_id) {
            let offset = page_id * PAGE_SIZE as u64;
            self.file.seek(SeekFrom::Start(offset))?;
            self.file.write_all(page)?;
        }
        Ok(())
    }
}
