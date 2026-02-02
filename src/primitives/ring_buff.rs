/*
File info: RingBuffer primitive type.

Test coverage: All main functions

Tested:
- init
- push
- pop

Not tested:
- read
- count

Reasons: I don't think I need a test for one of these functions because they are already used in some test, so they are tested indirectly.

Tests files:
- 'src/tests/primitives/ring_buff.rs'

References:
- `https://en.wikipedia.org/wiki/Circular_buffer`
*/

use crate::{log, logs::LogLevel};

#[derive(Debug)]
pub struct RingBuffer<T, const N: usize> {
    buff: [Option<T>; N],
    // Oldest element in the buffer
    head: usize,
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
    pub fn read(&mut self) -> Option<T> {
        let element = self.buff[self.head];
        if element.is_none() {
            None
        } else {
            Some(self.buff[self.head].unwrap())
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

    pub fn size(&self) -> usize {
        self.count
    }

    pub fn head(&self) -> usize {
        self.head
    }

    pub fn tail(&self) -> usize {
        self.tail
    }
}
