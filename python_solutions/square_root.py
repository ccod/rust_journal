# ## Finding a square root of an integer

# Wrote an implementation of the "Babylonian Method" described on wikipedia.
# Each successive approximation moves towards the answer, stopping when the next
# guess is the same integer as the last. I chose this because it was both easy to
# understand and satisfies the requirements posed.
# time complexity: O(logn)
# space complexity: O(1)

def sqrt(number):
    """
    Calculate the floored square root of a number

    Args:
       number(int): Number to find the floored squared root
    Returns:
       int: Floored Square Root
    """
    if type(number) is not int:
        raise TypeError("sqrt takes a positive integer")

    if number < 0:
        raise ValueError("sqrt applies to positive numbers")

    # seed value
    guess = 1.28
    next_guess = None

    while True:
        next_guess = 0.5 * (guess + (number / guess))

        if int(guess) == int(next_guess):
            return int(next_guess)

        guess = next_guess

try:
    sqrt('foo')
    raise Exception()
except TypeError:
    print('Pass')
except:
    print('Fail')
# Pass

try:
    sqrt(-25)
    raise Exception()
except ValueError:
    print('Pass')
except:
    print('Fail')
# Pass

print(sqrt(125348))
# 354 
print(sqrt(2))
# 1 
print(sqrt(9))
# 3 
print(sqrt(0))
# 0 
print(sqrt(16))
# 4 
print(sqrt(1))
# 1 
print(sqrt(27))
# 5 
