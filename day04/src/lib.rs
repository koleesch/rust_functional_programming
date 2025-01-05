pub struct RingBuffer<T> {
    buffer: Vec<Option<T>>,
    size: usize,
    start: usize,
    end: usize,
}

impl<T: Clone> RingBuffer<T> {
    /// Erzeugt einen neuen Ringbuffer mit einer festen Größe
    pub fn new(capacity: usize) -> Self {
        RingBuffer {
            buffer: vec![None; capacity],
            size: 0,
            start: 0,
            end: 0,
        }
    }

    /// fügt ein Element in den Buffer ein
    pub fn push(&mut self, item: T) {
        if self.size == self.buffer.len() {
            // überschreiben des ältesten Elementes
            self.start = (self.start + 1) % self.buffer.len();
        } else {
            self.size += 1;
        }
        self.buffer[self.end] = Some(item);
        self.end = (self.end + 1) % self.buffer.len();
    }

    /// entfernt ein Element aus dem Buffer
    pub fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }

        let item = self.buffer[self.start].take();
        self.start = (self.start + 1) % self.buffer.len();
        self.size -= 1;
        item
    }

    /// Gibt die Grö0e des aktuellen Buffers zurück
    pub fn len(&self) -> usize {
        self.size
    }

    /// Prüft, ob der Buffer leer ist
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ring_buffer_push_pop() {
        let mut buffer = RingBuffer::new(3);
        buffer.push(1);
        buffer.push(2);
        buffer.push(3);

        assert_eq!(buffer.pop(), Some(1));
        buffer.push(4);
        assert_eq!(buffer.pop(), Some(2));
        assert_eq!(buffer.pop(), Some(3));
        assert_eq!(buffer.pop(), Some(4));
        assert_eq!(buffer.pop(), None);
    }

    #[test]
    fn test_ringbuffer_overwrite() {
        let mut buffer = RingBuffer::new(2);
        buffer.push(1);
        buffer.push(2);
        buffer.push(3);
        assert_eq!(buffer.pop(), Some(2));
        assert_eq!(buffer.pop(), Some(3));
        assert_eq!(buffer.pop(), None);
    }
}
