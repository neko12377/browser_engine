use browser_engine::dom::{element, text, Node, NodeType};
use std::collections::HashMap;

fn main() {
    let t = text(String::from("hello"));
    get_node(t);
    let e = element("world".to_string(), HashMap::new(), Vec::new());
    get_node(e);
}

fn get_node(n: Node) -> String {
    let t = match n.node_type {
        NodeType::Text(t) => {
            println!("{t:?}");
            t
        }
        NodeType::Element(e) => {
            let t = e.tag_name;
            println!("{t:?}");
            t
        }
    };
    t
}

#[cfg(test)]
mod tests {
    use crate::get_node;
    use browser_engine::dom::{element, text, Node, NodeType};

    #[test]
    fn node_got() {
        let mock_node = Node {
            children: vec![],
            node_type: NodeType::Text("Hello".to_string()),
        };
        let result = get_node(mock_node);
        assert_eq!(result, "Hello".to_string())
    }
}
