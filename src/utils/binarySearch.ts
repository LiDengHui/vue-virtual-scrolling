export default function binarySearch(arr: number[], target: number) {
  let left = 0;
  let right = arr.length - 1;

  while (left <= right) {
    let mid = Math.floor((left + right) / 2);
    if (arr[mid] === target) {
      return mid;
    } else if (arr[mid] < target) {
      left = mid + 1;
    } else {
      right = mid - 1;
    }
  }
  // 目标值不在数组中，确定最接近的下标
  if (left === arr.length) {
    return right;
  } else if (right < 0) {
    return left;
  } else {
    let diffLeft = Math.abs(arr[left] - target);
    let diffRight = Math.abs(arr[right] - target);
    return diffLeft < diffRight? left : right;
  }
}
