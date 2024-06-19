from typing import List, Optional
from math import ceil, floor


def bestSeat(seats: List[int]):
    # obtain the max series of zeroes - index wise.
    possible_seats: List[Optional[List[int]]] = []
    for i in range(len(seats)):
        if seats[i] == 1:
            possible_seats.append(None)
        if seats[i] == 0:
            last_arr = possible_seats[-1]
            if not isinstance(last_arr, list):
                new_seat_arr = [i]
                possible_seats.append(new_seat_arr)
            else:
                last_arr.append(i)
    # find the biggest length array
    possible_seats: List[List[int]] = list(
        filter(lambda x: x is not None, possible_seats)
    )
    if possible_seats:
        possible_seats_len = list(map(len, possible_seats))
        max_possible_seat_val = max(possible_seats_len)
        for ps in possible_seats:
            if len(ps) == max_possible_seat_val:
                # obtain its middle
                middle_seat_indx = ceil((len(ps) / 2) - 1)
                return ps[middle_seat_indx]
    return -1


if __name__ == "__main__":
    t1 = [1, 0, 1]
    seat_index = bestSeat(t1)
    assert seat_index == 1
    # ------------------
    t2 = [1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1]
    seat_index = bestSeat(t2)
    assert seat_index == 3
