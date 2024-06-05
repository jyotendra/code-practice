def calc_monthly_compounding(prinicpal, rate_monthly, months, recurring):
    investments = [prinicpal]
    interests = []
    for i in range(months - 1):
        investments.append(investments[i - 1] + recurring)
    for i in range(months):
        interest = investments[i] * (rate_monthly / 100) * (months - i)
        interests.append(interest)

    total_interest = sum(interests)
    sum_in_end = investments[-1] + total_interest
    print("sum in end: ", "{:.2f}".format(sum_in_end))
    print("interests by period", ["{:.2f}".format(val) for val in interests])
    # return interests


if __name__ == "__main__":
    principal = 5040000
    rate = 1.08
    months = 34
    recurring = 100000
    calc_monthly_compounding(principal, rate, months, recurring)
    # print("{:2f}".format(val))
