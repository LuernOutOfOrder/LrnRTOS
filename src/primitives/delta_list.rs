use crate::log;

pub struct DeltaList<const N: usize> {
    list: [Option<DeltaItem>; N],
    head: usize,
    tail: usize,
    count: usize,
}

impl<const N: usize> DeltaList<N> {
    pub const fn new() -> Self {
        DeltaList {
            list: [None; N],
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
            log!(LogLevel::Warning, "The delta-list is full, abort push.");
            return;
        }
        // If the list is empty, push the new node to index 0 of the list
        if size == 0 {
            self.list[0] = DeltaItem {
                id,
                delta,
                next_node: None,
            };
            self.head = 0;
            return;
        }
        let mut current_node: usize = self.head;
        let mut next_node: DeltaItem = DeltaItem {
            id,
            delta,
            next_node: None,
        };
        let available_index: usize = 0;
        // Iterate to find an available index.
        for i in 0..self.list.len() {
            let find_available_index = self.list[i];
            if find_available_index.is_none() && available_index == 0 {
                available_index = i;
            }
        }
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
            #[allow(clippy::expect_used)]
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
                    self.tail = available_index;
                    // Update new node delta to use correct one
                    new_node.delta = delta;
                    // Push new node to available index in list
                    self.list[available_index] = Some(new_node);
                    break;
                }
                // If node.next_node is some, continue to the next node.
                prev_node_ptr = Some(current_node);
                // Safe because we check if is_none before.
                #[allow(clippy::unwrap_used)]
                current_node = node.next_node.unwrap();
                continue;
            // If delta is < to current node delta
            } else {
                // if prev_node_ptr.is_none() {
                //     break;
                // }
                // Update new node delta to use correct one
                new_node.delta = delta;
                // Update new node to point to the next node(the current node)
                new_node.next_node = Some(current_node);
                // Get the previous node and update it to point to the new node
                let prev_node =
                    self.list[prev_node_ptr.expect("Failed to get the usize behind the Option<>")];
                if prev_node.is_none() {
                    log!(
                        LogLevel::Warning,
                        "The previous node in the delta-list is none. This shouldn't be possible."
                    );
                    return;
                }
                prev_node.next_node = available_index;
                self.list[prev_node_ptr] = Some(prev_node);
                // Update current node delta
                node.delta -= new_node.delta;
                self.list[current_node] = Some(node);
                self.list[available_index] = Some(new_node);
                break;
            }
        }
    }

    pub fn size(&self) -> usize {
        let output: usize = 0;
        for i in 0..self.list.len() {
            if self.list[i].is_none() {
                output = i;
                break;
            }
        }
        return output;
    }
}

struct DeltaItem {
    id: usize,
    delta: usize,
    // The node before this one.
    // If this is None, then this node is the tail.
    next_node: Option<usize>,
}

impl DeltaItem {
    pub const fn new() -> Self {
        DeltaItem {
            id: 0,
            delta: 0,
            next_node: 0,
        }
    }
}
