class MinHash {
  public length: number;
  public data: number[][];

  constructor() {
    this.length = 0;
    this.data = [];
  }

  insert(point: number[]) {
    this.data.push(point);

    if (this.length > 0) {
      this.hepifyUp(this.length);
    }

    this.length++;
  }

  delete(): number[] {
    if (this.length == 0) {
      throw new Error("Unable to delete an empty hash");
    }

    this.length--;

    const point = this.data[0];

    if (this.length == 0) {
      this.data = [];

      return point;
    }

    this.data[0] = this.data[this.length];
    this.data.pop();

    this.hepifyDown(0);

    return point;
  }

  private hepifyUp(index: number) {
    if (index == 0) {
      return;
    }

    const parentIdx = this.getParentIdx(index);

    const parentPoint = this.data[parentIdx];
    const point = this.data[index];

    if (this.getLength(parentPoint) > this.getLength(point)) {
      this.data[index] = parentPoint;
      this.data[parentIdx] = point;

      this.hepifyUp(parentIdx);
    }
  }

  private hepifyDown(index: number) {
    const lIdx = this.getLeftIdx(index);
    const rIdx = this.getRightIdx(index);

    if (!this.data[lIdx]) {
      return;
    }

    const point = this.data[index];
    const pointDistance = this.getLength(point);

    if (!this.data[rIdx]) {
      if (this.getLength(this.data[lIdx]) < pointDistance) {
        this.data[index] = this.data[lIdx];
        this.data[lIdx] = point;

        this.hepifyDown(lIdx);
      }

      return;
    }

    const lDistance = this.getLength(this.data[lIdx]);
    const rDistance = this.getLength(this.data[rIdx]);

    const minIdx = lDistance < rDistance ? lIdx : rIdx;

    if (this.getLength(this.data[minIdx]) < pointDistance) {
      this.data[index] = this.data[minIdx];
      this.data[minIdx] = point;

      this.hepifyDown(minIdx);
    }
  }

  private getParentIdx(index: number): number {
    return Math.floor((index - 1) / 2);
  }

  private getLeftIdx(index: number): number {
    return 2 * index + 1;
  }

  private getRightIdx(index: number): number {
    return 2 * index + 2;
  }

  private getLength(point: number[]): number {
    const x = point[0] - 0;
    const y = point[1] - 0;

    return Math.sqrt(x * x + y * y);
  }
}

export function kClosest(points: number[][], k: number): number[][] {
  const minHash = new MinHash();

  points.forEach((point) => {
    minHash.insert(point);
  });

  const result: number[][] = [];

  for (let i = 0; i < k; i++) {
    result.push(minHash.delete());
  }

  return result;
}
