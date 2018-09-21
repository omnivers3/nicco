

fn main() {

}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Attributes<'a> {
    Custom { key: &'a str, value: &'a str },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Element<'a> {
    kind: &'a str,
    attributes: &'a [&'a Attributes<'a>],
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Node<'a> {
    element: Option<Element<'a>>,
    children: &'a[&'a Node<'a>],
}

impl<'a> Node<'a> {
    pub fn empty() -> Self {
        Node {
            element: None,
            children: &[],
        }
    }
//     pub fn new(element: Element, children: Vec<Node>, messages: Vec<Message>) -> Self {
//         Node {
//             element,
//             children: children,
//             messages: messages,
//         }
//     }

//     pub fn trim_messages(self) -> Self {
//         Node {
//             element: self.element,
//             children: self.children,
//             messages: vec![],
//         }
//     }
}