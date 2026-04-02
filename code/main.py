from time import time

def get_degree(num: int, divisor: int, degree: int) -> int:
    if num % divisor == 0:
        num //= divisor
        num = get_degree(num, divisor, degree + 1)
    else: print(f" ^{degree}")

    return num


def get_divisors(num: int):
    if num < 4:
        print(f"{num} ^1")
        return

    if num % 2 == 0:
        num //= 2
        print(2, end = "")
        num = get_degree(num, 2, 1)

    if num % 3 == 0:
        num //= 3
        print(3, end = "")
        num = get_degree(num, 3, 1)

    divisor = 5
    while divisor * divisor < num + 1:
        if num % divisor == 0:
            num //= divisor
            print(divisor, end = "")
            num = get_degree(num, divisor, 1)

        if num % (divisor + 2) == 0:
            num //= divisor + 2
            print(divisor + 2, end = "")
            num = get_degree(num, divisor + 2, 1)

        divisor += 6

    if num != 1: print(f"{num} ^1")

print("This program returns divisors of a number.\nEnter the number:")

num = int(input())
print("----------")

now = time()
get_divisors(num)
print(f"----------\nFinished: {time() - now}s")
