import { ListNode } from "../types";
import has_cycle from "../solutions/s025_linked_list_cycle";

function create_list(input: number[], pos: number): ListNode {
  const reversed = input.reverse();

  let last = null;
  let head = null;
  let cp = null; // Cycle pointer

  for (let i = 0; i < reversed.length; ++i) {
    const val = reversed[i];

    const node = new ListNode(val, null);

    if (!head) {
      head = last = node;
      continue;
    }

    if (reversed.length - 1 - i == pos) {
      cp = node;
    }

    node.next = head;
    head = node;
  }

  if (cp) {
    last.next = cp;
  }

  return head;
}

describe(has_cycle, () => {
  it("should pass", () => {
    const list = create_list([3, 2, 0, -4], 1);
    const list2 = create_list([1, 2], 0);
    const list3 = create_list([1], -1);

    expect(has_cycle(list)).toBe(true);
    expect(has_cycle(list2)).toBe(true);
    expect(has_cycle(list3)).toBe(false);
  });
});
