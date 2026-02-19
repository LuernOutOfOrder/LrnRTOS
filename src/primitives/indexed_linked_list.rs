/*
File info: IndexedLinkedList primitive type.

Test coverage: 80

Tested:
- push
- pop
- get_head

Not tested:

Reasons:
- Lot of other methods are used in push, pop, and get_head, that why the test coverage is at 80 for me.

Tests files:
- 'src/tests/primitives/indexed_linked_list.rs'

References:
*/

use crate::LogLevel;
use crate::log;

#[derive(Clone, Copy)]
pub struct IndexedLinkedList<const N: usize> {
    list: [Option<IndexedLinkedListNode>; N],
    head: usize,
    tail: usize,
    count: usize,
}

impl<const N: usize> IndexedLinkedList<N> {
    #[allow(clippy::new_without_default)]
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

    /// Push the new node in the linked list. Can update the current node in it.
    /// Avoid duplication on id. The id is unique in the list.
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
            self.count += 1;
            return;
        }
        let mut current_node: usize = self.head;
        let mut available_index: Option<usize> = None;
        // Check if there's no id duplication possible by iterating over the linked list
        {
            let mut current_node: usize = self.head;
            for _ in 0..self.list.len() {
                // Allow expect use, if we can't get the current node, the linked-list is wrong,
                // want to fail-fast
                #[allow(clippy::expect_used)]
                let get_current_node = self
                    .get_node(current_node)
                    .expect("Failed to get the asked node, linked-list may be empty or corrupted.");
                let node = get_current_node;
                if node.id == id {
                    log!(
                        LogLevel::Warn,
                        "The indexed-linked-list has already the id: {}, abort push.",
                        id
                    );
                    return;
                } else {
                    if let Some(next_node) = node.next_node {
                        current_node = next_node;
                        continue;
                    } else {
                        break;
                    }
                }
            }
        }
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
        let mut new_node = IndexedLinkedListNode {
            id,
            value,
            next_node: None,
        };
        let mut prev_node_ptr: Option<usize> = None;
        for _ in 0..self.list.len() {
            // Allow expect, if we can't get the wanted node, the list may be corrupted.
            #[allow(clippy::expect_used)]
            let node: &mut IndexedLinkedListNode = self
                .get_node(current_node)
                .expect("Failed to get the asked node, linked list may be empty or corrupted");
            // If the current value is superior than the current node value, continue, or check the
            // next_node.
            if value > node.value {
                if node.next_node.is_none() {
                    node.next_node = available_index;
                    // Update current node in list
                    // Allow expect, if available index is None, maybe there's no available space
                    // in the linked-list, and we shouldn't reach this point.
                    #[allow(clippy::expect_used)]
                    let check_available_index = available_index.expect("Failed to get the usize behind the Option<>. Maybe there's isn't available space in the linked-list.");
                    self.tail = check_available_index;
                    // Push new node to available index in list
                    // Allow unwrap, we check available index before
                    #[allow(clippy::unwrap_used)]
                    self.list[available_index.unwrap()] = Some(new_node);
                    break;
                }
                prev_node_ptr = Some(current_node);
                // Allow expect, we check the next node before, if it's None, something wrong, we
                // want to fail-fast
                #[allow(clippy::expect_used)]
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
                    // Allow expect, the available index should not be None
                    #[allow(clippy::expect_used)]
                    let check_available_index = available_index
                        .expect("Failed to get the available_index behind the Option<>");
                    self.head = check_available_index;
                    // Update the new_node to point to the old head
                    new_node.next_node = Some(prev_head);
                    // Update list to push new_node to head
                    self.list[self.head] = Some(new_node);
                    break;
                }
                // If there's a previous node.
                new_node.next_node = Some(current_node);
                // Get the previous node
                // Allow expect, if the previous_node index is not reachable or else, we want to
                // fail-fast, the linked-list could be corrupted.
                #[allow(clippy::expect_used)]
                let prev_node: &mut IndexedLinkedListNode = self
                    .get_node(prev_node_ptr.expect("Failed to get the previous_node index behind Option<>, linked-list may be corrupted"))
                    .expect("Failed to get the asked node, linked list may be empty or corrupted");
                // Update previous node to point to the new node
                prev_node.next_node = available_index;
                // Push the new node to the list
                // Allow expect, if the Available index is wrong, we want to fail fast
                #[allow(clippy::expect_used)]
                self.list[available_index.expect("Available index should not be None")] =
                    Some(new_node);
                break;
            }
        }
        self.count += 1;
    }

    /// Remove the node at the head and return the node.
    /// Update the linked list head to point to the next node.
    pub fn pop(&mut self) -> Option<IndexedLinkedListNode> {
        let head = self.head;
        let head_next_node = {
            // If we can't get the head node, return None
            let head_node = self.get_node(head);
            // if head_node.is_none() {
            //     return None;
            // }
            head_node.as_ref()?;
            // Allow unwrap, we check the value before
            #[allow(clippy::unwrap_used)]
            head_node.unwrap().next_node
        };
        if let Some(next_node) = head_next_node {
            self.head = next_node;
        } else {
            self.head = 0;
        }
        self.count -= 1;
        // Get the head node
        self.take_node(head)
    }

    pub fn get_head_node(&self) -> Option<&IndexedLinkedListNode> {
        self.list[self.head].as_ref()
    }

    pub fn get_node(&mut self, idx: usize) -> Option<&mut IndexedLinkedListNode> {
        let node = self.list[idx].as_mut();
        if let Some(is_node) = node {
            Some(is_node)
        } else {
            None
        }
    }

    /// Take the node from the given index, replace it with None in the list.
    fn take_node(&mut self, idx: usize) -> Option<IndexedLinkedListNode> {
        let node = self.list[idx];
        if node.is_some() {
            self.list[idx].take()
        } else {
            None
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
        output
    }

    pub fn get_count(&self) -> usize {
        self.count
    }

    #[cfg(feature = "test")]
    pub fn get_index(&self, idx: usize) -> IndexedLinkedListNode {
        // Unwrap directly because it's in test env.
        self.list[idx].unwrap()
    }

    #[cfg(feature = "test")]
    pub fn get_head(&self) -> usize {
        self.head
    }

    #[cfg(feature = "test")]
    pub fn get_tail(&self) -> usize {
        self.tail
    }
}

#[derive(Clone, Copy)]
pub struct IndexedLinkedListNode {
    pub id: usize,
    pub value: usize,
    // The node before this one.
    // If this is None, then this node is the tail.
    pub next_node: Option<usize>,
}

impl IndexedLinkedListNode {
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        IndexedLinkedListNode {
            id: 0,
            value: 0,
            next_node: None,
        }
    }
}
