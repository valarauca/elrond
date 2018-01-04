
use std::sync::Arc;

/// Allocation of the file under going reloation
pub struct ArcAlloc<'a> {
    data: Arc<&'a [u8]>,
    len: usize,
    offset: usize,
}
impl<'a> ArcAlloc<'a> {

    pub fn new(buffer: &'a [u8], offset: usize, len: usize) -> ArcAlloc<'a> {
        ArcAlloc {
            data: Arc::new(buffer),
            len: len,
            offset: offset
        }
    }

    /// Get the internal buffer
    pub fn get_data(&self) -> &'a [u8] {
        let start = self.offset.clone();
        let end = start + self.len.clone(); 
        &self.data.as_ref()[start..end]
    }

    /// Get offset of the file
    pub fn get_offset(&self) -> usize {
        self.offset.clone()
    }

    /// Get the length of the buffer
    pub fn len(&self) -> usize {
        self.len.clone()
    }

    /// Split this 
    ///
    /// Returns `Option::None` if the index is out of bounds
    pub fn split(&mut self, index: usize) -> Option<ArcAlloc<'a>> {
        if index < self.len() {
            self.len = index;
            Some(ArcAlloc {
                data: self.data.clone(),
                len: self.len() - index,
                offset: self.offset + index,
            })
        } else {
            None
        }
    }
}
