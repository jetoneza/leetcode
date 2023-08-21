/**
 * Definition for singly-linked list.
 * class ListNode {
 *     val: number
 *     next: ListNode | null
 *     constructor(val?: number, next?: ListNode | null) {
 *         this.val = (val===undefined ? 0 : val)
 *         this.next = (next===undefined ? null : next)
 *     }
 * }
 */

import { ListNode } from "../types";

function has_cycle(head: ListNode | null): boolean {
  const visited = new Set();
  let curr = head;

  if (!curr) {
    return false;
  }

  while (curr) {
    if (visited.has(curr)) {
      return true;
    }

    visited.add(curr);
    curr = curr.next;
  }

  return false;
}

export default has_cycle;
