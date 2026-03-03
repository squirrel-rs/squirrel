def is_palindrome(n: int) -> bool:
    original = n
    reversed_num = 0

    while n > 0:
        reversed_num = reversed_num * 10 + (n % 10)
        n //= 10

    return original == reversed_num


def main():
    ans = 0

    a = 100
    while a < 1000:
        b = 100
        while b < 1000:
            product = a * b

            if is_palindrome(product):
                if product > ans:
                    ans = product

            b += 1
        a += 1

    print(ans)


if __name__ == "__main__":
    main()