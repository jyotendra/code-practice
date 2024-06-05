from enum import Enum


class Direction(Enum):
    up = 1
    straight = 0
    down = -1


def eval_direction(i, j):
    if i < j:
        return Direction.up
    if i > j:
        return Direction.down
    else:
        return Direction.straight


def longestPeak(array):
    # Write your code here.

    peaks = []
    # 1,2,3,3,6,3,5,3,5,4,3,2,1
    i = 0
    j = 1

    if len(array) < 3:
        return 0

    current_dir = Direction.straight
    current_peak = [array[i]]

    while j <= len(array) - 1:
        if (
            eval_direction(array[i], array[j]) == Direction.down
            and current_dir == Direction.straight
        ):
            i += 1
            j += 1
            continue

        elif (
            eval_direction(array[i], array[j]) == Direction.up
            and current_dir == Direction.up
        ):
            current_peak.append(array[j])
            i += 1
            j += 1

        elif (
            eval_direction(array[i], array[j]) == Direction.down
            and current_dir == Direction.up
        ):
            current_dir = Direction.down
            current_peak.append(array[j])
            i += 1
            j += 1

        elif (
            eval_direction(array[i], array[j]) == Direction.down
            and current_dir == Direction.down
        ):
            current_peak.append(array[j])
            i += 1
            j += 1

        elif (
            eval_direction(array[i], array[j]) == Direction.up
            and current_dir == Direction.down
        ):
            peaks.append(current_peak)
            current_peak = [array[i]]
            current_dir = Direction.up

        else:
            if len(current_peak) > 2:
                peaks.append(current_peak)
            i = j
            j = i + 1
            current_peak = [array[i]]
            current_dir = Direction.up

    peak_len = [len(p) for p in peaks]
    return max(peak_len or [0])


def main():
    # test_data = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
    test_data = [5, 4, 3, 2, 1, 2, 1]
    res = longestPeak(test_data)
    print(res)


if __name__ == "__main__":
    main()
