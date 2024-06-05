def spiralTraverse(array):
    rows = len(array)
    cols = len(array[0])

    left_top_bound = [0, 0]
    right_bottom_bound = [rows - 1, cols - 1]

    tracked_array = []
    while (
        left_top_bound[0] < right_bottom_bound[0]
        or left_top_bound[1] < right_bottom_bound[1]
    ):
        # move right
        for i in range(left_top_bound[1], right_bottom_bound[1] + 1):
            tracked_array.append(array[left_top_bound[0]][i])

        # move down
        for i in range(left_top_bound[0] + 1, right_bottom_bound[0] + 1):
            tracked_array.append(array[i][right_bottom_bound[1]])

        # move left
        for i in range(right_bottom_bound[1] - 1, left_top_bound[1] - 1, -1):
            tracked_array.append(array[right_bottom_bound[0]][i])

        # move up
        for i in range(right_bottom_bound[0] - 1, left_top_bound[0], -1):
            tracked_array.append(array[i][left_top_bound[1]])

        left_top_bound = [left_top_bound[0] + 1, left_top_bound[1] + 1]
        right_bottom_bound = [right_bottom_bound[0] - 1, right_bottom_bound[1] - 1]

        if any(el < 0 for el in [])
            break

    if (
        left_top_bound[0] == right_bottom_bound[0]
        and right_bottom_bound[1] == right_bottom_bound[1]
    ):
        tracked_array.append(array[left_top_bound[0]][left_top_bound[1]])

    print(tracked_array)
    return tracked_array


#  [
#     [ 1,  2,  3,  4, 1],
#     [ 5,  6,  7,  8, 1],
#     [ 9, 10, 11, 12, 1],
#     [13, 14, 15, 16, 1],
#  ]

if __name__ == "__main__":
    test_data = [
        # [1, 2, 3, 4, 1],
        # [5, 6, 7, 8, 1],
        # [9, 10, 11, 12, 1],
        # [13, 14, 15, 16, 1],
        # [17, 18, 19, 20, 1],
        # ----
        # [4, 2, 3, 6, 7, 8, 1, 9, 5, 10],
        # [12, 19, 15, 16, 20, 18, 13, 17, 11, 14],
        # ---
        # [1, 2, 3],
        # [8, 9, 4],
        # [7, 6, 5],
        # ----
        # [1, 2, 3, 4],
        # [10, 11, 12, 5],
        # [9, 8, 7, 6],
        # ------
        [1],
        [3],
        [2],
        [5],
        [4],
        [7],
        [6],
    ]
    spiralTraverse(test_data)
