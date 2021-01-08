# ## Huffman encoding and decoding 

# I used a binary tree based min-heap to implement the priority queue. The tree
# based queue allows me to efficiently build up the ordered queue, and mantain it
# when enqueueing new values. the queue data structure is contained in a list,
# making adding leaf nodes a straight forward as "list.append". As each element in
# the queue is a node, it becomes straight forward to build up the tree and
# enqueue the head node back into the priority queue.

# The priority queue also affords more predictable efficiency;
# Another method might be to sort the list, create a
# linked list from the sorted list, and doing a sorted insert starting from head
# of the list to build the huffman tree. 

# time complexity of encoding the huffman tree
# O(n) to count frequency
# O(nlogn) sort frequencies in priority queue where n is unique letters
# O(n) build up the tree from queue where n is unique letters

# overal time complexity for encoding O(n)
# space complexity is also O(n)

# time complexity of decoding the huffman tree
# O(n) where n is the size of encoded string

# space complexity is also O(n) 
#   where n is the encoded string, tree structure, and resulting message

class Node(object):
    def __init__(self, value, letter=None):
        self.letter = letter
        self.value = value 
        self.left = None
        self.right = None

    def __repr__(self):
        fmt = "Node(value={}, letter={})"
        return fmt.format(self.value, self.letter)

class PriorityQueue(object):
    def __init__(self):
        self._queue = []

    def size(self):
        return len(self._queue)

    def _left(self, idx):
        return (2 * idx) + 1

    def _right(self, idx):
        return (2 * idx) + 2
    
    def _swap(self, idx1, idx2):
        node1, node2 = self._queue[idx1], self._queue[idx2]
        self._queue[idx1], self._queue[idx2] = node2, node1 

    def _parent_idx(self, idx):
        return idx // 2

    def _check_parent(self, idx):
        current = self._queue[idx]
        parent = self._queue[self._parent_idx(idx)]

        if current == parent:
            return

        if current.value < parent.value:
            self._queue[idx], self._queue[self._parent_idx(idx)] = parent, current
            self._check_parent(self._parent_idx(idx))

    def _value_at(self, idx):
        return self._queue[idx].value

    def _check_children(self, idx):
        if self._left(idx) > self.size() - 1:
            return

        if self._value_at(idx) > self._value_at(self._left(idx)):
            self._swap(idx, self._left(idx))
            self._check_children(self._left(idx))
            return

        if self._right(idx) > self.size() - 1:
            return

        if self._value_at(idx) > self._value_at(self._right(idx)):
            self._swap(idx, self._right(idx))
            self._check_children(self._right(idx))
            return
            
    def enqueue(self, node):
        self._queue.append(node)
        self._check_parent(self.size() -1)

    def dequeue(self):
        if not self._queue:
            return None

        if self.size() == 1:
            return self._queue.pop()

        self._swap(0, -1)
        # self._queue[-1], self._queue[0] = self._queue[0], self._queue[-1]
        node = self._queue.pop()

        self._check_children(0)
        return node

    def __repr__(self):
        return str(self._queue)

def huffman_mappings(coll, code, node):
    if node.letter:
        coll[node.letter] = code
    else:
        if node.left:
            huffman_mappings(coll, code + '0', node.left)
        if node.right:
            huffman_mappings(coll, code + '1', node.right)

        return coll

def huffman_encoding(data):
    if data == None or data == '':
        return None, None

    queue = PriorityQueue()
    counter = dict()
    encoded_str = ''
    tree = None

    for element in data:
        if element in counter:
            counter[element] += 1
        else:
            counter[element] = 1

    for key, count in counter.items():
        queue.enqueue(Node(count, key))

    while queue.size() != 1:
        left, right = queue.dequeue(), queue.dequeue()
        node = Node(left.value + right.value)
        node.left, node.right = left, right
        queue.enqueue(node)

    tree = queue.dequeue()
    mappings = huffman_mappings(dict(), '', tree)

    for element in data:
        encoded_str += mappings[element]

    return encoded_str, tree

def huffman_decoding(data, tree):
    if data == None or tree == None:
        return None

    decoded_message = ''
    root = tree
    for bit in data:
        if bit == '1':
            tree = tree.right
        else:
            tree = tree.left

        if tree.letter:
            decoded_message += tree.letter
            tree = root

    return decoded_message


## Test calls

data, tree = huffman_encoding("AAAAAAABBBCCCCCCCDDEEEEEE")
print(data)
# 1010101010101000100100111111111111111000000010101010101

print(huffman_decoding(data, tree))
# AAAAAAABBBCCCCCCCDDEEEEEE

data, tree = huffman_encoding("ANDBPEMEMEPEN")
print(data)
# 01101000111010101110011001110111100

print(huffman_decoding(data, tree))
# ANDBPEMEMEPEN

data, tree = huffman_encoding("")
print(data)
# None

print(huffman_decoding(data, tree))
# None

data, tree = huffman_encoding(None)
print(data)
# None

print(huffman_decoding(data, tree))
# None
