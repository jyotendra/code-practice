def numberOfWaysToMakeChange(n, denoms):

    def exec(n, denoms, cache={}):
        ways = 0
        if n == 0:
            ways += 1
            return ways
        if n < 0:
            return 0

        for d in denoms:
            res = n - d
            if res not in cache:
                cache[res] = exec(res, denoms, cache)
                ways += cache[res]
            else:
                ways += 1

        return ways

    return exec(n, denoms)


if __name__ == "__main__":
    test_data = [25, [1, 5, 10, 25]]
    print(numberOfWaysToMakeChange(*test_data))
