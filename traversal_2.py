from typing import List, Tuple

"""
[
 1, 2, 3
 4, 5, 6
 7, 8, 9
]

[1,2,3,6,9,8,7,4,5]

[
    1  2  3  4
    5  6  7  8
    9 10 11 12
    13 14 15 16
]

[1,2,3,4,8,12,16,15,14,13,9,5,6,7,11,10]

                      
"""


def traverse(
    mat: List[List[int]], upper_left: Tuple[int, int], bottom_right: Tuple[int, int]
):
    arr = []
    min_row, min_col = upper_left
    max_row, max_col = bottom_right
    # right
    for i in range(min_col, max_col + 1):
        arr.append(mat[min_row][i])
    # down
    for i in range(min_row + 1, max_row + 1):
        arr.append(mat[i][max_col])
    # left
    for i in range(max_col - 1, min_col - 1, -1):
        arr.append(mat[max_row][i])
    # up
    for i in range(max_row - 1, min_row, -1):
        arr.append(mat[i][min_col])
    return arr


def spiral_traverse(input):
    max_row = len(input)
    max_col = len(input[0])

    upper_left = (0, 0)
    bottom_right = (max_row - 1, max_col - 1)

    res = []

    while upper_left[0] <= bottom_right[0] and upper_left[1] <= bottom_right[1]:
        arr = traverse(input, upper_left, bottom_right)
        upper_left = (upper_left[0] + 1, upper_left[1] + 1)
        bottom_right = (bottom_right[0] - 1, bottom_right[1] - 1)
        res = res + arr

    return res


def main():
    # test_data = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
    test_data = [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12], [13, 14, 15, 16]]
    res = spiral_traverse(test_data)
    print(res)


if __name__ == "__main__":
    main()
