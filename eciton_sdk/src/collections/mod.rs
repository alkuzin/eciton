// Eciton - experimental exokernel.
// Copyright (C) 2025 Alexander (@alkuzin).
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! Collections main module.

use core::mem;

mod bitmap;
pub use bitmap::Bitmap;

/// List node structure.
#[derive(Debug, Default, Copy, Clone)]
pub struct Node<T> {
    /// Next node index.
    pub next: Option<usize>,
    /// Previous node index.
    pub prev: Option<usize>,
    /// Next value.
    pub value: T,
}

/// Linked list structure with no heap allocation.
#[derive(Debug, Copy, Clone)]
pub struct StaticList<T, const CAPACITY: usize> {
    /// Array of nodes.
    nodes: [Node<T>;CAPACITY],
    /// List head node.
    head: Option<usize>,
    /// List tail node.
    tail: Option<usize>,
    /// Number of nodes.
    size: usize,
}

impl<T, const CAPACITY: usize> StaticList<T, CAPACITY> where T: Copy {
    /// Construct new LinkedList object.
    ///
    /// # Returns
    /// - New `LinkedList` object.
    pub fn new() -> Self {
        let empty_node: Node<T> = unsafe { mem::zeroed() };

        Self {
            nodes: [empty_node;CAPACITY],
            head:  None,
            tail:  None,
            size:  0,
        }
    }

    /// Get list length.
    ///
    /// # Returns
    /// - List length.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.size
    }

    /// Get list max size.
    ///
    /// # Returns
    /// - List capacity.
    #[inline(always)]
    pub fn capacity(&self) -> usize {
        CAPACITY
    }

    /// Get list head.
    ///
    /// # Returns
    /// - List head node.
    pub fn head(&mut self) -> Option<&mut Node<T>> {
        if let Some(head) = self.head {
            return Some(&mut self.nodes[head]);
        }

        None
    }

    /// Get list tail.
    ///
    /// # Returns
    /// - List tail node.
    pub fn tail(&mut self) -> Option<&mut Node<T>> {
        if let Some(tail) = self.tail {
            return Some(&mut self.nodes[tail]);
        }

        None
    }

    /// Check if list is empty.
    ///
    /// # Returns
    /// - `true`  - if list is empty.
    /// - `false` - otherwise.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Append new value.
    ///
    /// # Parameters
    /// - `value` - given value to push.
    ///
    /// # Result
    /// - `Ok`       - in case of success.
    /// - `Err(msg)` - error message - otherwise.
    pub fn push(&mut self, value: T) -> Result<(), &'static str> {
        if self.size == 0 {
                // Handle empty list.
                let node = Node { next: None, prev: None, value };
                self.nodes[0] = node;
                self.size += 1;

                self.head = Some(0);
                self.tail = Some(0);
        }
        else if self.size == CAPACITY {
            // Handle filled list.
            return Err("List is filled");
        }
        else {
            // Add new node in the end.
            if let Some(tail) = self.tail {
                let next_tail = tail + 1;

                // Make last node point on None.
                let next = if next_tail == CAPACITY - 1 {
                    None
                }
                else {
                    Some(next_tail)
                };

                let node = Node { next, prev: self.tail, value };

                self.nodes[next_tail] = node;
                self.size += 1;
                self.tail = Some(next_tail);
                self.nodes[tail].next = self.tail;
            }
            else {
                return Err("List tail is None");
            }
        }

        Ok(())
    }

    /// Remove value from the end of the list.
    ///
    /// # Result
    /// - `Ok`       - in case of success.
    /// - `Err(msg)` - error message - otherwise.
    pub fn pop(&mut self) -> Result<(), &'static str> {
        if self.size == 0 {
            return Err("Error to pop value from empty list");
        }
        else {
            // Check if list is not empty.
            if let Some(tail) = self.tail {
                // Pop previous node.
                if let Some(prev) = self.nodes[tail].prev {
                    self.nodes[prev].next = None;
                    self.size -= 1;
                    self.nodes[tail] = unsafe { mem::zeroed() };
                    self.tail = Some(prev);
                }
                else {
                    // Pop head node.
                    self.nodes[tail] = unsafe { mem::zeroed() };
                    self.head = None;
                    self.tail = None;
                    self.size -= 1;
                }
            }
            else {
                return Err("Error to pop value from empty list");
            }
        }

        Ok(())
    }
}

/// Default static list.
impl<T, const CAPACITY: usize> Default for StaticList<T, CAPACITY> where T: Copy {
    fn default() -> Self {
        let empty_node: Node<T> = unsafe { mem::zeroed() };

        Self {
            nodes: [empty_node;CAPACITY],
            head: None,
            tail: None,
            size: 0,
        }
    }
}