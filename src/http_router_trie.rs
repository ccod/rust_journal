use std::collections::HashMap;

fn split_path(route: &str) -> Vec<String> {
    let route_path: Vec<String> = route.split('/').map(|e| String::from(e)).collect();
    if route_path[route_path.len() - 1] == "" {
        return route_path[1..route_path.len() - 1].to_vec();
    } else {
        return route_path[1..].to_vec();
    }
}

struct TrieNode {
    handler: Option<String>,
    children: HashMap<String, usize>,
}

struct RouterTrie {
    nodes: Vec<TrieNode>,
}

impl RouterTrie {
    fn new(handler: String) -> Self {
        RouterTrie {
            nodes: vec![TrieNode {
                handler: Some(handler),
                children: HashMap::new(),
            }],
        }
    }

    fn new_node(&mut self) -> usize {
        self.nodes.push(TrieNode {
            handler: None,
            children: HashMap::new(),
        });
        return self.nodes.len() - 1;
    }

    fn add(&mut self, handler: String, path: &str) {
        let route_parts = split_path(path);
        let mut current = &self.nodes[0];

        for part in route_parts.iter() {
            let has_part = current.children.get(part);
            match has_part {
                Some(v) => current = &self.nodes[*v],
                None => {
                    let next = self.new_node();
                    current.children.insert(*part, next.clone());
                    current = &self.nodes[next];
                }
            }
        }
        current.handler = Some(handler);
    }
}

// struct Router {
//     root_handler: String,
//     page_missing: String,
//     trie: RouterTrie,
// }

// impl Router {
//     fn new(root: &str, missing: &str) -> Self {}
//     fn addHandler(&self, route: &str, handler: &str) {}
//     fn lookup(&self, route: &str) -> String {}
//     fn split_path(&self, route: &str) -> Vec<String> {}
// }

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn check_split_path() {
        assert_eq!(split_path("/about/me"), vec!["about", "me"]);
        assert_eq!(split_path("/home/page/"), vec!["home", "page"]);
        let x: Vec<String> = Vec::new();
        assert_eq!(split_path("/"), x);
    }

    #[test]
    fn check_router_trie() {
        let mut router = RouterTrie::new(String::from("root page"));
        // router should start with a root
        router.add(String::from("found home page"), "/");
    }
}
