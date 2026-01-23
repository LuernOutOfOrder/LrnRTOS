use crate::{log, logs::LogLevel};

pub struct RingBuffer<T, const N: usize> {
    buff: [Option<T>; N],
    // Oldest element in the buffer
    head: usize,
    // Just like head but doesn't pop element of the buffer
    read: usize,
    // Newest element in the buffer
    tail: usize,
    // Number of element in the buffer
    count: usize,
}

impl<T: Copy, const N: usize> RingBuffer<T, N> {
    pub const fn init() -> Self {
        RingBuffer {
            buff: [None; N],
            head: 0,
            read: 0,
            tail: 0,
            count: 0,
        }
    }

    /// Add new element to tail, increment tail.
    pub fn push(&mut self, new: T) {
        // Check if buffer is full
        if (self.tail + 1) % N == self.head {
            log!(LogLevel::Warn, "Ring buffer full, abort push.");
            return;
        }
        self.buff[self.tail] = Some(new);
        self.tail += 1 % N;
        self.count += 1;
    }

    // Pop oldest element, return an Option<&mut T> and increment head.
    pub fn pop(&mut self) -> Option<&mut T> {
        // Check if buffer is empty
        if self.head == self.tail {
            log!(LogLevel::Warn, "Ring buffer is empty, abort pop.");
            return None;
        }
        self.head += 1 % N;
        self.count -= 1;
        Some(self.buff[self.head].as_mut().unwrap())
    }

    pub fn read(&mut self) -> Option<&mut T> {
        if let Some(element) = self.buff[self.read].as_mut() {
            Some(element)
        } else {
            None
        }
    }

    pub fn state(&self) {
        if self.head == self.tail {
            // Buffer empty
        }
        if self.head != self.tail {
            // Buffer partially full
        }
    }
}
