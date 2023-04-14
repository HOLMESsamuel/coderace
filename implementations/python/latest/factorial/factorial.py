def factorial(n):
    if n == 0:
        return 1
    else:
        return n * factorial(n - 1)

def main():
    n = int(input("Enter a number: "))
    result = factorial(n)
    print(f"The factorial of {n} is: {result}")

if __name__ == "__main__":
    main()
