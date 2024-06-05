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


class Tracker:
    def __init__(self, upper_bound, lower_bound=0, current=0):
        self.current = current
        self.upper_bound = upper_bound
        self.lower_bound = lower_bound

    def add_one(self):
        if self.current + 1 <= self.upper_bound:
            self.current = self.current + 1

    def less_one(self):
        if self.current - 1 >= self.lower_bound:
            self.current = self.current - 1


class FinalList:
    def __init__(self, total_count):
        self.lst = []
        self.remaining_elem = total_count

    def append(self, elem):
        self.lst.append(elem)
        self.remaining_elem -= 1


def spiral_traverse(input):
    max_row = len(input)
    max_col = len(input[0])

    total_elem = max_row * max_col

    final_list = FinalList(total_elem)

    max_row_indx = max_row - 1
    max_col_indx = max_col - 1
    min_row_indx = 0
    min_col_indx = 0

    row_indx = Tracker(max_row_indx)
    col_indx = Tracker(max_row_indx)

    while final_list.remaining_elem > 0:
        while col_indx.current < max_col_indx and final_list.remaining_elem > 0:
            print("a", row_indx.current, col_indx.current)
            final_list.append(input[row_indx.current][col_indx.current])
            col_indx.add_one()
        min_row_indx += 1

        while row_indx.current < max_row_indx and final_list.remaining_elem > 0:
            print("b", row_indx.current, col_indx.current)
            final_list.append(input[row_indx.current][col_indx.current])
            row_indx.add_one()
        max_col_indx -= 1

        while col_indx.current > min_col_indx and final_list.remaining_elem > 0:
            print("c", row_indx.current, col_indx.current)
            final_list.append(input[row_indx.current][col_indx.current])
            col_indx.less_one()
        max_row_indx -= 1

        while row_indx.current > min_row_indx and final_list.remaining_elem > 0:
            print("d", row_indx.current, col_indx.current)
            final_list.append(input[row_indx.current][col_indx.current])
            row_indx.less_one()
        min_col_indx += 1
        # print("bounds:", (min_row_indx, max_row_indx), (min_col_indx, max_col_indx))
        # print("current: ", row_indx.current, col_indx.current)
        # print("indx", col_indx.current, max_col_indx)
        # print("elem", final_list.remaining_elem)
        # break

    return final_list.lst


def main():
    # test_data = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
    test_data = [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12], [13, 14, 15, 16]]
    res = spiral_traverse(test_data)
    print(res)


if __name__ == "__main__":
    main()
