use crate::{kprint_fmt, log, logs::LogLevel};

#[derive(Debug)]
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

impl<T: Copy + core::fmt::Debug, const N: usize> RingBuffer<T, N> {
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
        self.tail = (self.tail + 1) % N;
        self.count += 1;
    }

    // Pop oldest element
    pub fn pop(&mut self) -> Option<T> {
        // Check if buffer is empty
        if self.head == self.tail {
            log!(LogLevel::Warn, "Ring buffer is empty, abort pop.");
            return None;
        }
        if self.buff[self.head].is_some() {
            let element = self.buff[self.head].take();
            self.head = (self.head + 1) % N;
            self.count -= 1;
            element
        } else {
            None
        }
    }

    /// Read element at self.read index, self.read work just like self.head, but it never remove
    /// element when reading it.
    pub fn read(&mut self) -> Option<&mut T> {
        let mut update_read = self.read;
        // Iter to find the next readable value in case of the first element is None
        for _ in 0..N {
            if self.buff[update_read].is_none() {
                update_read = (self.read + 1) % N;
                continue;
            } else {
                self.read = update_read;
                return Some(self.buff[update_read].as_mut().unwrap());
            };
        }
        None
    }

    /// Update the element previously read. Not the best way to use the RingBuffer, but can be
    /// helpful.
    pub fn update(&mut self, updated: T) {
        self.buff[self.read] = Some(updated);
        self.read = (self.read + 1) % N;
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
