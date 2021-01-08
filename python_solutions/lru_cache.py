## LRU Cache

# I used a combination fifo queue implemented with a doubly linked list, and a hash table
# pointing to node object references in the queue. The queue helps me determine
# which key to remove new elements after reaching capacity. The doubly linked list
# allows me to move a node the front simply without needing to shift later
# elements like with an array. the hash table saves me the trouble of looking for the
# node in the queue, side stepping traversing the list each time for the match.

# The time complexity for get and set is O(1).
# The space efficiency is O(n) where n is capacity. 

class Node(object):
    def __init__(self, key, value):
        self.key = key
        self.value = value
        self.next = None
        self.prev = None
    
    def __repr__(self):
        next = self.next and self.next.value
        prev = self.prev and self.prev.value
        fmt_str = "Node(key={}, value={}, next_value={}, prev_value={})"
        return fmt_str.format(self.key, self.value, next, prev)


class Queue(object):
    def __init__(self, capacity=5):
        self.head = None
        self.tail = None
        self.capacity = capacity
        self.size = 0

    def enqueue(self, node):
        if not self.head:
            self.head = self.tail = node
            self.size += 1
            return

        temp = self.head
        self.head, node.next, temp.prev = node, temp, node

        if self.size + 1 > self.capacity:
            self.dequeue()
        else:
            self.size += 1

    def dequeue(self):
        self.tail = self.tail.prev
        self.tail.next = None

    def move_to_front(self, node):
        if node == self.head:
            return

        if node == self.tail:
            self.tail = node.prev 
            node.prev.next, node.prev = None, None
        else:
            next, prev = node.next, node.prev
            prev.next, next.prev = next, prev
            node.prev = node.next = None

        self.size -= 1
        self.enqueue(node)

    def __repr__(self):
        start, elements, finish = '[\n',  '', ']'
        current = self.head

        while current:
            elements += '\t' + repr(current) + '\n'
            current = current.next

        return start + elements + finish 


class LRU_Cache(object):
    def __init__(self, capacity):
        self.queue = Queue(capacity)
        self.cache = dict()

    def get(self, key):
        if key in self.cache:
            node = self.cache[key]
            self.queue.move_to_front(node)
            return node.value
        else:
            return -1

    def set(self, key, value):
        if key in self.cache:
            node = self.cache[key]
            node.value = value
            self.queue.move_to_front(node)
        else:
            node = Node(key, value)
            if self.queue.size == self.queue.capacity:
                del self.cache[self.queue.tail.key]
                self.cache[key] = node
                self.queue.enqueue(node)
            else:
                self.cache[key] = node
                self.queue.enqueue(node)


# Test calls

cache = LRU_Cache(5)

cache.set(1, 1);
cache.set(2, 2);
cache.set(3, 3);
cache.set(4, 4);

print(cache.get(1))
# returns 1

print(cache.get(2))
# returns 2

print(cache.get(9))
# returns -1 because 9 is not present in the cache

cache.set(5, 5) 
cache.set(6, 6)

print(cache.get(3))
# returns -1 because 3 was dropped as an the most infrequently used value 
