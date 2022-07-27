use crate::domain::errors::Error;

pub struct Page<T> {
    offset: u64,
    limit: u64,
    total: u64,
    data: Vec<T>,
}

impl<T> Page<T> {
    pub fn new(offset: u64, limit: u64, total: u64, data: Vec<T>) -> Result<Page<T>, Error> {
        if data.len() > limit as usize {
            return Err(Error::PageOutOfRange);
        }

        Ok(Page {
            offset,
            limit,
            total,
            data,
        })
    }

    pub fn offset(&self) -> u64 {
        self.offset
    }

    pub fn limit(&self) -> u64 {
        self.limit
    }

    pub fn total(&self) -> u64 {
        self.total
    }

    pub fn into_data(self) -> Vec<T> {
        self.data
    }
}
