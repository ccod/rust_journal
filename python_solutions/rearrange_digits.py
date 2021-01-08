# ## Rearange Array Elements

# implement quicksort as my efficient sorting function.
# Once I have a sorted list from greatest to least, I can build my resulting
# numbers by alternating by even indexes.

# time complexity:
#   O(nlogn) for sorting the list
#   O(n) for building the two numbers, where n size of input_list
# space complexity:
#   O(n) where n is the input_list

def quicksort(input_list, low, high):
    checker, pivot = low, high

    while checker + 1 != pivot:
        if input_list[checker] < input_list[pivot]:
            input_list[pivot], input_list[pivot -1] = input_list[pivot -1], input_list[pivot]
            input_list[checker], input_list[pivot] = input_list[pivot], input_list[checker]
            pivot -= 1
        else:
            checker += 1

    if input_list[checker] < input_list[pivot]:
        input_list[checker], input_list[pivot] = input_list[pivot], input_list[checker]
        pivot -= 1

    if not pivot + 1 >= high:
        quicksort(input_list, pivot + 1, high)
    if not pivot - 1 <= low:
        quicksort(input_list, low, pivot - 1)

    return input_list

def rearrange_digits(input_list):
    """
    Rearrange Array Elements so as to form two number such that their sum is maximum.

    Args:
       input_list(list): Input List
    Returns:
       (int),(int): Two maximum sums
    """
    if type(input_list) is not list:
        raise TypeError('rearrange_digits require a list type as an argument')

    if len(input_list) < 2:
        raise ValueError('rearrange_digits requires at least 2 digits to operate')

    sorted_list = quicksort(input_list, 0, len(input_list) -1)
    first = second = ''

    for idx, number in enumerate(sorted_list):
        if idx % 2 == 0:
            first += str(number)
        else:
            second += str(number)

    return [int(first), int(second)]

try:
    rearrange_digits(dict())
    raise Exception
except TypeError:
    print("Pass")
except:
    print('Fail')
# Pass

try:
    rearrange_digits([1])
    raise Exception
except ValueError:
    print("Pass")
except:
    print("Fail")
# Pass    

# leveraging comparison operators in quicksort to raise this error
try:
    rearrange_digits([2, 'foo', 12])
    raise Exception
except TypeError:
    print("Pass")
except:
    print("Fail")
# Pass

def test_function(test_case):
    output = rearrange_digits(test_case[0])
    solution = test_case[1]
    if sum(output) == sum(solution):
        print("Pass")
    else:
        print("Fail")

test_function([[1, 2, 3, 4, 5], [542, 31]])
# Pass
test_function([[4, 6, 2, 5, 9, 8], [964, 852]])
# Pass
