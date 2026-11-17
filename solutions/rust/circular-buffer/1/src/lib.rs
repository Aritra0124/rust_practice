pub struct CircularBuffer<T> {
    buffer: Vec<T>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self{
          buffer: Vec::with_capacity(capacity),  
        }
    }

    pub fn write(&mut self, _element: T) -> Result<(), Error> {
        if self.buffer.len() == self.buffer.capacity(){
            return Err(Error::FullBuffer);
        }
        self.buffer.push(_element);
        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.buffer.is_empty(){
            return Err(Error::EmptyBuffer);
        }else{
            return Ok(self.buffer.remove(0));
        }
    }

    pub fn clear(&mut self) {
        self.buffer.clear()
    }

    pub fn overwrite(&mut self, _element: T) {
        if self.buffer.len() == self.buffer.capacity(){
            self.buffer.remove(0);
            self.buffer.push(_element);
        }else{
            self.write(_element);
        }
    }
}
