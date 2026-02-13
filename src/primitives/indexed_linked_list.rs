/*
File info: IndexedLinkedList primitive type.

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
pub struct IndexedLinkedList<const N: usize> {
    list: [Option<IndexedLinkedListNode>; N],
    head: usize,
    tail: usize,
    count: usize,
}

impl<const N: usize> IndexedLinkedList<N> {
    pub const fn new() -> Self {
        IndexedLinkedList {
            list: [const { None }; N],
            // Oldest node, the top of the linked list
            head: 0,
            // Newest node, the bottom of the linked list, doesn't have a node below it.
            tail: 0,
            count: 0,
        }
    }

    pub fn push(&mut self, id: usize, value: usize) {
        // Get the size of the list
        let size = self.size();
        if size == self.list.len() {
            log!(LogLevel::Warn, "The delta-list is full, abort push.");
            return;
        }
        // If the list is empty, push the new node to index 0 of the list
        if size == 0 {
            self.list[0] = Some(IndexedLinkedListNode {
                id,
                value,
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
        let mut new_node = IndexedLinkedListNode {
            id,
            value,
            next_node: None,
        };
        let mut prev_node_ptr: Option<usize> = None;
        for _ in 0..self.list.len() {
            let mut node = self.list[current_node].expect("Failed to get the current node. This shouldn't be possible unless the linked list is corrupted.");
            // If the current value is superior than the current node value, continue, or check the
            // next_node.
            if value > node.value {
                if node.next_node.is_none() {
                    node.next_node = available_index;
                    // Update current node in list
                    self.list[current_node] = Some(node);
                    self.tail = available_index.expect("Failed to get the usize behind the Option<>. Maybe there's isn't available space in the delta-list.");
                    // Push new node to available index in list
                    self.list[available_index.unwrap()] = Some(new_node);
                    break;
                }
                prev_node_ptr = Some(current_node);
                let node_next_node = node
                    .next_node
                    .expect("Failed to get the next_node behind the Option<>");
                current_node = node_next_node;
                continue;
            // Else if the current value is not superior, update the list to push the new_node
            // before the current one.
            } else {
                // If there's no previous node, than we are at the head, so update the head to
                // point to the new node.
                if prev_node_ptr.is_none() {
                    // Get the previous head
                    let prev_head = self.head;
                    // Update the head to point to the new node
                    self.head = available_index
                        .expect("Failed to get the available_index behind the Option<>");
                    // Update the new_node to point to the old head
                    new_node.next_node = Some(prev_head);
                    // Update list to push new_node to head
                    self.list[self.head] = Some(new_node);
                    break;
                }
                // If there's a previous node.
                new_node.next_node = Some(current_node);
                // Get the previous node
                let mut prev_node =
                    self.list[prev_node_ptr.expect("Failed to get the usize behind the Option<>")];
                // Check to see if there's an error getting the previous node
                if prev_node.is_none() {
                    log!(
                        LogLevel::Warn,
                        "The previous node in the delta-list is none. This shouldn't be possible."
                    );
                    return;
                }
                // Update previous node to point to the new node
                prev_node
                    .expect("Previous node should not be None")
                    .next_node = available_index;
                // Update the previous node in the list
                self.list[prev_node_ptr.expect("Failed to get the usize behind the Option<>")] =
                    prev_node;
                // Push the new node to the list
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
    pub fn get_index(&self, idx: usize) -> IndexedLinkedListNode {
        // Unwrap directly because it's in test env.
        self.list[idx].unwrap()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct IndexedLinkedListNode {
    pub id: usize,
    pub value: usize,
    // The node before this one.
    // If this is None, then this node is the tail.
    pub next_node: Option<usize>,
}

impl IndexedLinkedListNode {
    pub const fn new() -> Self {
        IndexedLinkedListNode {
            id: 0,
            value: 0,
            next_node: None,
        }
    }
}
