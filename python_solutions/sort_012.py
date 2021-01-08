# ## Dutch National Flag Problem

# I maintain a moving index for 0 and 2 on each side of the list, increment and
# decrementing respectively, swapping values to either side as I come across them.
# The pass terminates once I reach the "two" index, so I would argue checking those
# indexes qualifies as being part of "one pass".

# Time complexity: O(n)
# Space complexity: O(n)

def next_non_two(input_list, idx):
    if input_list[idx] != 2:
        return idx 
    else:
        return next_non_two(input_list, idx - 1)

def next_non_zero(input_list, idx):
    if input_list[idx] != 0:
        return idx 
    else:
        return next_non_two(input_list, idx + 1)

def move_zero_forward(input_list, idx, zero):
    zero = next_non_zero(input_list, zero)

    if idx > zero:
        input_list[zero], input_list[idx] = input_list[idx], input_list[zero]
        zero += 1

    return zero

def sort_012(input_list):
    """
    Given an input array consisting on only 0, 1, and 2, sort the array in a single traversal.

    Args:
       input_list(list): List to be sorted
    """
    if type(input_list) is not list:
        raise TypeError("sort_012 requires a list of 0's 1's and 2's")

    if len(input_list) < 3:
        raise ValueError("sort_012 should have at least 3 values, of either 0, 1, and 2")

    zero, two = 0, len(input_list) -1

    for idx, number in enumerate(input_list):
        if number == 0:
            zero = move_zero_forward(input_list, idx, zero)

        if number == 2:
            two = next_non_two(input_list, two)

            if idx >= two:
                break

            input_list[two], input_list[idx] = input_list[idx], input_list[two]
            two -= 1

            if input_list[idx] == 0:
                zero = move_zero_forward(input_list, idx, zero)

    return input_list

try:
    sort_012('foo')
    raise Exception
except TypeError:
    print("Pass")
except:
    print("Fail")
# Pass

try:
    sort_012([])
    raise Exception
except ValueError:
    print("Pass")
except:
    print("Fail")
# Pass

def test_function(test_case):
    sorted_array = sort_012(test_case)
    if sorted_array == sorted(test_case):
        print("Pass")
    else:
        print("Fail")

test_function([0, 0, 2, 2, 2, 1, 1, 1, 2, 0, 2])
# Pass
test_function([2, 1, 2, 0, 0, 2, 1, 0, 1, 0, 0, 2, 2, 2, 1, 2, 0, 0, 0, 2, 1, 0, 2, 0, 0, 1])
# Pass
test_function([0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2])
# Pass
