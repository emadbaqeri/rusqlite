use super::pager::Pager;
use serde::{Deserialize, Serialize};

pub trait SerializePage {
    fn serialize_page(&mut self, page_id: u64, data: &impl Serialize) -> Result<(), DbError>;
    fn deserialize_page<T: for<'a> Deserialize<'a>>(&mut self, page_id: u64) -> Result<T, DbError>;
}

impl SerializePage for Pager {
    fn serialize_page(&mut self, page_id: u64, data: &impl Serialize) -> Result<(), DbError> {
        let buffer = bincode::serialize(data)?;
        let page = self.get_page(page_id)?;
        page.copy_from_slice(&buffer);
        Ok(())
    }

    fn deserialize_page<T: for<'a> Deserialize<'a>>(&mut self, page_id: u64) -> Result<T, DbError> {
        let page = self.get_page(page_id)?;
        Ok(bincode::deserialize(page)?)
    }
}
