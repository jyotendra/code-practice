def traverse(arr):
    el_count = 0
    all_el = len(arr) * len(arr[0])
    final_arr = []

    min_R, min_C = (0, 0)
    max_R, max_C = (len(arr) - 1, len(arr[0]) - 1)
    while (min_R - max_R <= 0) and (min_C - max_C <= 0):
        for i in range(min_C, max_C + 1):
            curr_el = arr[min_R][i]
            final_arr.append(curr_el)
            el_count += 1
        if el_count >= all_el:
            break
        for i in range(min_R + 1, max_R + 1):
            curr_el = arr[i][max_C]
            final_arr.append(curr_el)
            el_count += 1
        if el_count >= all_el:
            break
        for i in range(max_C - 1, min_C - 1, -1):
            curr_el = arr[max_R][i]
            final_arr.append(curr_el)
            el_count += 1
        if el_count >= all_el:
            break
        for i in range(max_R - 1, min_R, -1):
            curr_el = arr[i][min_C]
            final_arr.append(curr_el)
            el_count += 1
        if el_count >= all_el:
            break
        min_R, min_C = (min_R + 1, min_C + 1)
        max_R, max_C = (max_R - 1, max_C - 1)
    return final_arr


def spiralTraverse(array):
    # Write your code here.
    return traverse(array)


if __name__ == "__main__":
    test_data = [[1, 2, 3, 4], [10, 11, 12, 5], [9, 8, 7, 6]]
    res = spiralTraverse(test_data)
    print(res)
