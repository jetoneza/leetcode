export function searchMatrix(matrix: number[][], target: number): boolean {
  let start = 0;
  let end = matrix.length - 1;

  while (start <= end) {
    const mid = Math.floor((start + end) / 2);
    const row = matrix[mid];

    const first = row[0];
    const last = row[row.length - 1];

    if (first == target || last == target) {
      return true;
    }

    if (target > first && target < last) {
      return searchRow(row, target)
    }

    if (target < first) {
      end = mid - 1;
    } else {
      start = mid + 1;
    }
  }

  return false
}

function searchRow(row: number[], target: number): boolean {
  let start = 0;
  let end = row.length - 1;

  while (start <= end) {
    const mid = Math.floor((start + end) / 2);
    const value = row[mid];

    if (value == target) {
      return true;
    }

    if (target < value) {
      end = mid - 1;
    } else {
      start = mid + 1;
    }
  }

  return false
}
