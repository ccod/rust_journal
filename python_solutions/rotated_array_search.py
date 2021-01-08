# ## Search in Rotated Sort Array

# Implemented the solution with an adjusted binary search. The first few passes
# contends with adjusting for the pivot point before operating as normal. The
# exception is a tie break when the value isn't present but gets confused at the
# pivot point.
# Time complexity: O(logn)
# Space complexity: O(n)

def rotated_array_search(input_list, number):
    """
    Find the index by searching in a rotated sorted array

    Args:
       input_list(array), number(int): Input array to search and the target
    Returns:
       int: Index or -1
    """
    if type(input_list) is not list or type(number) is not int:
        raise TypeError('rotated_array_search requires a list and a number as args')

    if not input_list:
        return -1

    first = input_list[0]
    high, low = len(input_list) -1, 0

    while high != low:
        mid = ((high - low) // 2) + low
        mid_value = input_list[mid]

        if mid_value == number:
            return mid

        if (high - low) == 1 and mid == low or mid == high:
            break

        if number < first and mid_value > first:
            low = mid
            continue

        if number > first and mid_value < first:
            high = mid
            continue

        if mid_value > number:
            high = mid
        else:
            low = mid
   
    return -1

def linear_search(input_list, number):
    for index, element in enumerate(input_list):
        if element == number:
            return index
    return -1

def test_function(test_case):
    input_list = test_case[0]
    number = test_case[1]
    if linear_search(input_list, number) == rotated_array_search(input_list, number):
        print("Pass")
    else:
        print("Fail")

try:
    rotated_array_search(dict(), 'foo')
    raise Exception()
except TypeError:
    print('Pass')
except:
    print('Fail')
# Pass

test_function([[], 8])
# Pass
test_function([[6, 7, 8, 9, 10, 1, 2, 3, 4], 6])
# Pass
test_function([[6, 7, 8, 9, 10, 1, 2, 3, 4], 1])
# Pass
test_function([[6, 7, 8, 1, 2, 3, 4], 8])
# Pass
test_function([[6, 7, 8, 1, 2, 3, 4], 1])
# Pass
test_function([[6, 7, 8, 1, 2, 3, 4], 10])
# Pass

