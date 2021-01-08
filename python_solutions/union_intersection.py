
### Union and Intersection

# Union and intersection both return a new list. In both cases I use a dictionary 
# for the purposes of caching. Since the resulting lists are created from
# dictionaries, ordering isn't a guarantee, but that is also true of sets.

# time complexity
#   union: O(n) where n is number of elements in list_1, list_2, and uniq elements
#       of new_list
#   intersection: O(n) where n is number of elements in list_1, list_2, and uniq elements
#       of new_list

# space complexity
#   union: O(n) where n is number of elements in list_1, list_2, and uniq elements
#       of new_list
#   intersection: O(n) where n is number of elements in list_1, list_2, and uniq elements
#       of new_list

class Node:
    def __init__(self, value):
        self.value = value
        self.next = None

    def __repr__(self):
        return str(self.value)


class LinkedList:
    def __init__(self):
        self.head = None

    def __str__(self):
        cur_head = self.head
        out_string = ""
        while cur_head:
            out_string += str(cur_head.value) + " -> "
            cur_head = cur_head.next
        return out_string


    def append(self, value):

        if self.head is None:
            self.head = Node(value)
            return

        node = self.head
        while node.next:
            node = node.next

        node.next = Node(value)

    def size(self):
        size = 0
        node = self.head
        while node:
            size += 1
            node = node.next

        return size

    def as_list(self):
        collection = []
        current = self.head

        while current:
            collection.append(current.value)
            current = current.next

        return collection

def union(llist_1, llist_2):
    if llist_1 == None or llist_2 == None:
        return None

    check = dict()
    current_1 = llist_1.head
    current_2 = llist_2.head

    while current_1: 
        check[current_1.value] = True
        current_1 = current_1.next

    while current_2: 
        check[current_2.value] = True
        current_2 = current_2.next

    new_list = LinkedList()
    for value in check.keys():
        new_list.append(value)

    return new_list

def intersection(llist_1, llist_2):
    if llist_1 == None or llist_2 == None:
        return None

    check = dict()
    in_common = dict()

    current_1 = llist_1.head
    current_2 = llist_2.head

    while current_1: 
        check[current_1.value] = True
        current_1 = current_1.next

    while current_2: 
        if current_2.value in check:
            in_common[current_2.value] = True
        current_2 = current_2.next

    new_list = LinkedList()
    for value in in_common.keys():
        new_list.append(value)

    return new_list

####
# Test Case 1
####
linked_list_1 = LinkedList()
linked_list_2 = LinkedList()

element_1 = [3,2,4,35,6,65,6,4,3,21]
element_2 = [6,32,4,9,6,1,11,21,1]

for i in element_1:
    linked_list_1.append(i)

for i in element_2:
    linked_list_2.append(i)


print(union(linked_list_1,linked_list_2))
# 3 -> 2 -> 4 -> 35 -> 6 -> 65 -> 21 -> 32 -> 9 -> 1 -> 11 ->
print(intersection(linked_list_1,linked_list_2))
# 6 -> 4 -> 21 ->

####
# Test Case 2
####
linked_list_3 = LinkedList()
linked_list_4 = LinkedList()

element_1 = [3,2,4,35,6,65,6,4,3,23]
element_2 = [1,7,8,9,11,21,1]

for i in element_1:
    linked_list_3.append(i)

for i in element_2:
    linked_list_4.append(i)

print (union(linked_list_3,linked_list_4))
# 3 -> 2 -> 4 -> 35 -> 6 -> 65 -> 23 -> 1 -> 7 -> 8 -> 9 -> 11 -> 21 ->
print(intersection(linked_list_3,linked_list_4))
# ""
# There was no common elements
# I figure an empty linked list is still an appropriate return in this case.

####
# Test Case 3
####
print(union(linked_list_3, None))
# None
print(intersection(None, linked_list_3))
# None

####
# Test Case 4
####
linked_list_5 = LinkedList()
linked_list_6 = LinkedList()

element_1 = []
element_2 = [6,32,4,9,6,1,11,21,1]

for i in element_1:
    linked_list_5.append(i)

for i in element_2:
    linked_list_6.append(i)

print(union(linked_list_5, linked_list_6))
# 6 -> 32 -> 4 -> 9 -> 1 -> 11 -> 21 ->
print(intersection(linked_list_5, linked_list_6))
# ""
