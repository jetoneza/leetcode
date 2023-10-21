import { kClosest } from "../solutions/s032_k_closest_point_to_origin";

describe(kClosest, () => {
  it("should pass", () => {
    expect(
      kClosest(
        [
          [1, 3],
          [-2, 2],
        ],
        1,
      ),
    ).toEqual([[-2, 2]]);

    expect(
      kClosest(
        [
          [3, 3],
          [5, -1],
          [-2, 4],
        ],
        2,
      ),
    ).toEqual([
      [3, 3],
      [-2, 4],
    ]);

    expect(
      kClosest(
        [
          [6, 10],
          [-3, 3],
          [-2, 5],
          [0, 2],
        ],
        3,
      ),
    ).toEqual([
      [0, 2],
      [-3, 3],
      [-2, 5],
    ]);
  });
});
