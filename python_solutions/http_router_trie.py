# ## HTTP Router using a Trie

# The router leverages the Trie data structure, which is a tree with possibly many
# branches. It has the advantage of very fast insert and find speed, at the
# expense of larger memory footprint. 
# Time complexity:
# RouteTrie.find: O(n) where n is route length, so it is O(1) ish normally
# RouteTrie.insert: the same as above
# Space complexity:
# RouteTrie: O(n * p) where n is the number of routes, and p is the number of
#   route parts that varies in impact based on ordering and uniqueness 

# A RouteTrie will store our routes and their associated handlers
class RouteTrie:
    def __init__(self, root_handler=None): 
        # Initialize the trie with an root node and a handler, this is the root path or home page node
        self.root = RouteTrieNode()
        self.root.handler = root_handler

    def insert(self, route_parts, handler): 
        # Similar to our previous example you will want to recursively add nodes
        # Make sure you assign the handler to only the leaf (deepest) node of this path
        current = self.root
        for part in route_parts:
            current = current.insert(part)

        current.handler = handler

    def find(self, route_parts):
        # Starting at the root, navigate the Trie to find a match for this path
        # Return the handler for a match, or None for no match
        current = self.root

        for part in route_parts:
            current = current.sub_routes.get(part)
            if not current:
                return None

        return current.handler


# A RouteTrieNode will be similar to our autocomplete TrieNode... with one additional element, a handler.
class RouteTrieNode:
    def __init__(self):
        # Initialize the node with children as before, plus a handler
        self.handler = None
        self.sub_routes = dict()

    def insert(self, route_part):
        # Insert the node as before
        sub_route = self.sub_routes.get(route_part)
        if sub_route:
            return sub_route

        new_node = RouteTrieNode()
        self.sub_routes[route_part] = new_node

        return new_node

# The Router class will wrap the Trie and handle 
class Router:
    def __init__(self, root_handler, page_missing='basic 404'):
        # Create a new RouteTrie for holding our routes
        # You could also add a handler for 404 page not found responses as well!
        self.page_missing = page_missing
        self.routes = RouteTrie(root_handler)

    def add_handler(self, route, handler): 
        # Add a handler for a path
        # You will need to split the path and pass the pass parts
        # as a list to the RouteTrie
        self.routes.insert(self.split_path(route), handler)


    def lookup(self, route): 
        # lookup path (by parts) and return the associated handler
        # you can return None if it's not found or
        # return the "not found" handler if you added one
        # bonus points if a path works with and without a trailing slash
        # e.g. /about and /about/ both return the /about handler

        route_parts = self.split_path(route)
        handler = self.routes.find(route_parts)
        if not handler:
            return self.page_missing

        return handler

    def split_path(self, route):
        # you need to split the path into parts for 
        # both the add_handler and loopup functions,
        # so it should be placed in a function here

        if type(route) is not str:
            raise TypeError('lookup requires a string')

        if route[0] != '/':
            raise ValueError("route should start with '/'")

        raw_parts = route.split('/')
        # trailing backslashes are optional
        if raw_parts[-1] == '':
            return raw_parts[1:-1]
        else:
            return raw_parts[1:]

# Here are some test cases and expected outputs you can use to test your implementation

# create the router and add a route
router = Router("root handler", "not found handler") # remove the 'not found handler' if you did not implement this
router.add_handler("/home/about", "about handler")  # add a route

# # some lookups with the expected output
print(router.lookup("/")) 
# 'root handler'
print(router.lookup("/home")) 
# 'not found handler'
print(router.lookup("/home/about")) 
# 'about handler'
print(router.lookup("/home/about/")) 
# 'about handler'
print(router.lookup("/home/about/me")) 
# 'not found handler'

try:
    router.lookup('foobar/baz')
    raise Exception
except ValueError:
    print("Pass")
except:
    print("Fail")
# Pass

try:
    router.lookup(dict())
    raise Exception
except TypeError:
    print("Pass")
except:
    print("Fail")
# Pass

try:
    router.add_handler('not/doing/it', 'fail route')
    raise Exception
except ValueError:
    print("Pass")
except:
    print("Fail")
# Pass
