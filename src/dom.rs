struct Node {
  children: Vec<Node>,
  node_type: NodeType,
}

enum NodeType {
  Text(string),
  Element(ElementData),
}

struct ElementData {
  tag_name: String,
  attributes: AttrMap,
}

type AttrMap = HashMap<String, String>;

fn text(data: String) -> Node {
  Node { children: vec![], node_type: Text(data) }
}

fn elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
  Node {
    children: children,
    node_type: Element(ElementData {
      tagt_type: name,
      attributes: attrs
    })
  }
}
