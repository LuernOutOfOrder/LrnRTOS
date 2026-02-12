/*
File info: DeltaList primitive type.

Test coverage:

Tested:

Not tested:

Reasons:

Tests files:
- 'src/tests/primitives/delta_list.rs'

References:
*/

use crate::LogLevel;
use crate::log;

#[derive(Clone, Copy, Debug)]
pub struct DeltaList<const N: usize> {
    list: [Option<DeltaItem>; N],
    head: usize,
    tail: usize,
    count: usize,
}

impl<const N: usize> DeltaList<N> {
    pub const fn new() -> Self {
        DeltaList {
            list: [const { None }; N],
            // Oldest node, the top of the linked list
            head: 0,
            // Newest node, the bottom of the linked list, doesn't have a node below it.
            tail: 0,
            count: 0,
        }
    }

    pub fn push(&mut self, current_tick: usize, id: usize, value: usize) {
        // Get the size of the list
        let size = self.size();
        // Compute delta of the new node
        let mut delta = value - current_tick;
        if size == self.list.len() {
            log!(LogLevel::Warn, "The delta-list is full, abort push.");
            return;
        }
        // If the list is empty, push the new node to index 0 of the list
        if size == 0 {
            self.list[0] = Some(DeltaItem {
                id,
                delta,
                next_node: None,
            });
            self.head = 0;
            return;
        }
        let mut current_node: usize = self.head;
        let mut available_index: Option<usize> = None;
        // Iterate to find an available index.
        for i in 0..self.list.len() {
            let find_available_index = &self.list[i];
            if find_available_index.is_none() && available_index.is_none() {
                available_index = Some(i);
            }
        }
        if available_index.is_none() {
            log!(
                LogLevel::Error,
                "The delta-list is full, abort push. Consider increasing the blocked queue size."
            );
            return;
        }
        self.count += 1;
        let mut new_node = DeltaItem {
            id,
            delta,
            next_node: None,
        };
        let mut prev_node_ptr: Option<usize> = None;
        // Iterate over the linked list to find the spot for the new node inside it.
        for i in 0..self.list.len() {
            // Allow expect, we get the size by iterating all over the list until reaching None.
            // We shouldn't get a None value, unless something is wrong. So we want to fail-fast.
            // Get current node
            let mut node = self.list[current_node].expect("Failed to get node behind Option<>");
            if delta > node.delta {
                // Compute delta by subtracting the current node delta
                delta -= node.delta;
                // If the current node is tail.
                if node.next_node.is_none() {
                    // Update current node to point to the new tail(new_node)
                    node.next_node = available_index;
                    // Update current node in list
                    self.list[current_node] = Some(node);
                    // Update tail to point to the new_node
                    self.tail = available_index.expect("Failed to get the usize behind the Option<>. Maybe there's isn't available space in the delta-list.");
                    // Update new node delta to use correct one
                    new_node.delta = delta;
                    // Push new node to available index in list
                    self.list[available_index.unwrap()] = Some(new_node);
                    break;
                }
                // If node.next_node is some, continue to the next node.
                prev_node_ptr = Some(current_node);
                // Safe because we check if is_none before.
                #[allow(clippy::unwrap_used)]
                let node_next_node = node.next_node.unwrap();
                current_node = node_next_node;
                continue;
            // If delta is < to current node delta
            } else {
                if prev_node_ptr.is_none() {
                    #[allow(clippy::expect_used)]
                    let mut old_head =
                        self.list[self.head].expect("Failed to get the delta-list head node.");
                    old_head.delta -= delta;
                    self.list[self.head] = Some(old_head);
                    new_node.next_node = Some(self.head);
                    new_node.delta = delta;
                    self.head = available_index.expect("Available index should not be None.");
                    self.list[self.head] = Some(new_node);
                    break;
                }
                // Update new node delta to use correct one
                new_node.delta = delta;
                // Update new node to point to the next node(the current node)
                new_node.next_node = Some(current_node);
                // Get the previous node and update it to point to the new node
                let mut prev_node =
                    self.list[prev_node_ptr.expect("Failed to get the usize behind the Option<>")];
                if prev_node.is_none() {
                    log!(
                        LogLevel::Warn,
                        "The previous node in the delta-list is none. This shouldn't be possible."
                    );
                    return;
                }
                prev_node
                    .expect("Previous node should not be None")
                    .next_node = available_index;
                self.list[prev_node_ptr.expect("Failed to get the usize behind the Option<>")] =
                    prev_node;
                // Update current node delta
                node.delta -= new_node.delta;
                self.list[current_node] = Some(node);
                self.list[available_index.expect("Available index should not be None")] =
                    Some(new_node);
                break;
            }
        }
    }

    pub fn size(&self) -> usize {
        let mut output: usize = 0;
        for i in 0..self.list.len() {
            if self.list[i].is_none() {
                output = i;
                break;
            }
        }
        return output;
    }

    #[cfg(feature = "test")]
    pub fn get_index(&self, idx: usize) -> DeltaItem {
        // Unwrap directly because it's in test env.
        self.list[idx].unwrap()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct DeltaItem {
    pub id: usize,
    pub delta: usize,
    // The node before this one.
    // If this is None, then this node is the tail.
    pub next_node: Option<usize>,
}

impl DeltaItem {
    pub const fn new() -> Self {
        DeltaItem {
            id: 0,
            delta: 0,
            next_node: None,
        }
    }
}
