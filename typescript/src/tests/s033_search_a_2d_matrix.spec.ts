import { searchMatrix } from "../solutions/s033_search_a_2d_matrix";

describe(searchMatrix, () => {
  it("should pass", () => {
    const input = [
      [1, 3, 5, 7],
      [10, 11, 16, 20],
      [23, 30, 34, 60],
    ];

    expect(searchMatrix(input, 3)).toBe(true);
    expect(searchMatrix(input, 13)).toBe(false);
  });
});
