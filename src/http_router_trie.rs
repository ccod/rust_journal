use std::collections::HashMap;

struct TrieNode {
    handler: Option<String>,
    children: HashMap<String, usize>,
}

impl TrieNode {
    fn new(handler: Option<String>) -> TrieNode {
        TrieNode {
            handler,
            children: HashMap::new(),
        }
    }

    fn change_handler(&mut self, handler: String) {
        self.handler = Some(handler);
    }

    fn add_path(&mut self, sub_path: &String, index: &usize) {
        match self.children.get(sub_path) {
            Some(_) => {
                return;
            }
            None => {
                self.children.insert(sub_path.to_owned(), *index);
            }
        }
    }
}

pub struct Trie {
    nodes: Vec<TrieNode>,
}

impl Trie {
    pub fn new() -> Trie {
        Trie {
            nodes: vec![TrieNode::new(None)],
        }
    }

    pub fn add_root_handler(&mut self, handler: String) {
        self.nodes[0].change_handler(handler);
    }

    pub fn add_route(&mut self, path: Vec<String>, handler: String) {
        let mut current_idx: usize = 0;
        for sub_path in path.iter() {
            match self.nodes[current_idx].children.get(sub_path) {
                Some(idx) => current_idx = *idx,
                None => {
                    self.nodes.push(TrieNode::new(None));
                    let next = &(self.nodes.len() - 1);
                    self.nodes[current_idx].add_path(sub_path, &next);
                    current_idx = *next;
                }
            }
        }
        self.nodes[current_idx].change_handler(handler);
    }

    pub fn get_route(&self, path: Vec<String>) -> Option<String> {
        let mut current_idx: usize = 0;
        for sub_path in path.iter() {
            match self.nodes[current_idx].children.get(sub_path) {
                Some(idx) => current_idx = *idx,
                None => {
                    return None;
                }
            }
        }
        return self.nodes[current_idx].handler.clone();
    }
}

pub struct HttpRouter {
    page_missing: Option<String>,
    routes: Trie,
}

impl HttpRouter {
    pub fn new(root_handler: Option<String>, page_missing: Option<String>) -> Self {
        let mut router = HttpRouter {
            page_missing,
            routes: Trie::new(),
        };

        match root_handler {
            Some(v) => router.routes.add_root_handler(v),
            _ => (),
        }

        return router;
    }

    pub fn add_handler(&mut self, path: String, handler: String) {
        let route_path = HttpRouter::split_path(&path);
        self.routes.add_route(route_path, handler);
    }

    pub fn lookup(&self, route: String) -> String {
        let route_path = HttpRouter::split_path(&route);
        match self.routes.get_route(route_path) {
            Some(v) => v,
            None => match self.page_missing.clone() {
                Some(v) => v,
                None => "default route missing".to_owned(),
            },
        }
    }

    fn split_path(route: &str) -> Vec<String> {
        let route_path: Vec<String> = route.split('/').map(|e| String::from(e)).collect();
        if route_path[route_path.len() - 1] == "" {
            return route_path[1..route_path.len() - 1].to_vec();
        } else {
            return route_path[1..].to_vec();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_change_root_handler() {
        let mut trie = Trie::new();
        trie.add_root_handler("index handler".to_owned());
        assert_eq!(trie.nodes[0].handler, Some("index handler".to_owned()));
    }

    #[test]
    fn check_add_children() {
        let mut trie = Trie::new();
        trie.add_route(
            vec!["product".to_owned(), "listings".to_owned()],
            "product listings".to_owned(),
        );
        assert_eq!(trie.nodes.len(), 3);

        trie.add_route(
            vec!["product".to_owned(), "about".to_owned()],
            "product about".to_owned(),
        );
        assert_eq!(trie.nodes.len(), 4);
    }

    #[test]
    fn check_get_route() {
        let mut trie = Trie::new();
        trie.add_route(
            vec!["product".to_owned(), "listings".to_owned()],
            "product listings handler".to_owned(),
        );

        assert_eq!(
            trie.get_route(vec!["product".to_owned(), "listings".to_owned()]),
            Some("product listings handler".to_owned())
        )
    }

    #[test]
    fn check_http_router_page_missing() {
        let router = HttpRouter::new(None, Some("test 404".to_owned()));
        assert_eq!(router.lookup("/home".to_owned()), "test 404".to_owned());
    }

    #[test]
    fn check_http_router_index() {
        let router = HttpRouter::new(Some("basic index".to_owned()), None);
        assert_eq!(router.lookup("/".to_owned()), "basic index".to_owned());
    }

    #[test]
    fn check_http_router_product_list() {
        let mut router = HttpRouter::new(None, None);
        router.add_handler(
            "/product/listing".to_owned(),
            "product listing page".to_owned(),
        );
        assert_eq!(
            router.lookup("/product/listing".to_owned()),
            "product listing page".to_owned()
        );
    }
}
