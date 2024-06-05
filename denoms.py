import math
from typing import List


# table
#           num1(1)    num2(2)    num3(3)    num4..
# coin1(1)     1        2           3           4
# coin2(5)     6        12
# coin3


def numberOfWaysToMakeChange(n: int, denoms: List[int]):
    # Write your code here.
    min_coin = min(denoms)
    max_possible_cases = math.ceil(n / min_coin)
    all_cases = [[0 for _el in range(max_possible_cases)] for i in denoms]
    total_cases = 0
    for row, denom in enumerate(denoms):
        for col in range(max_possible_cases):
            num_coins = col + 1
            current_val = denom * num_coins + all_cases[row - 1][col] if row > 1 else 0
            if current_val > n:
                current_val = 0
            if current_val == n:
                total_cases += 1
            all_cases[row][col] = current_val


if __name__ == "__main__":
    target_amount = 6
    coins = [1, 5]
    print(numberOfWaysToMakeChange(target_amount, coins))  # 2
