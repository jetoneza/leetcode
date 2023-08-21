import hasCycle from "../solutions/s025_linked_list_cycle";

describe(hasCycle, () => {
  it("should pass example 1", () => {
    expect(hasCycle(null)).toBe(false);
  });
});
