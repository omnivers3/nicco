use proc_macro2::{ Delimiter, Ident, TokenTree, TokenStream };
// use proc_macro2::{ Group, Ident, TokenStream };
// use syn::parse::{ Parse, ParseStream };
// use syn::{ parse2 };
use std::fmt;

mod messages;

pub use self::messages::*;

use sink::{ ISink };

pub static EMPTY_TEMPLATE: &str = "Template is empty";
pub static AMBIGUOUS_EMPTY_LIST: &str = "Ambiguous empty list. This can be ommitted, or you can try adding attributes or children";
pub static EMPTY_LISTS_OPTIONAL: &str = "Empty trailing lists are optional following an element";
pub static URECOGNIZED_NODE_TYPE: &str = "Unrecognized node type";
pub static UNEXPECTED_LITERAL: &str = "Unexpected literal";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Container {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Element {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Attribute {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParseEvents {
    ParsedContainer(Container),
    ParsedElement(Element),
    ParsedAttribute(Attribute),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TreeEvents {
    Parse (ParseEvents),
    Message (Message),
}

pub trait ITreeSink {
    type TResult;
    type TError: fmt::Debug;

    fn handle(&self, input: TreeEvents) -> Result<Self::TResult, Self::TError>;

    fn info(&self, message: &str) {
        self.handle(TreeEvents::Message(Info::call_site(message))).unwrap();
    }

    fn warning(&self, message: &str) {
        self.handle(TreeEvents::Message(Warning::call_site(message))).unwrap();
    }

    fn error(&self, message: &str) {
        self.handle(TreeEvents::Message(Error::call_site(message))).unwrap();
    }

    fn notify(&self, strategy: &NotifyStrategies, message: &str) {
        match strategy {
            NotifyStrategies::Ignore => {},
            NotifyStrategies::Warning => self.warning(message),
            NotifyStrategies::Error => self.error(message),
        }
    }

    fn element(&self, element: Element) -> Result<Self::TResult, Self::TError> {
        self.handle(TreeEvents::Parse(ParseEvents::ParsedElement(element)))
    }
}

impl<TResult, TError> ISink for ITreeSink<TResult=TResult, TError=TError>
where
    TError: fmt::Debug,
{
    type TInput = TreeEvents;
    type TResult = TResult;
    type TError = TError;

    fn send(&self, input: Self::TInput) -> Result<Self::TResult, Self::TError> {
        (self as &ITreeSink<TResult=TResult, TError=TError>).handle(input)
    }
}

impl<TResult, TError> ITreeSink for ISink<TInput=TreeEvents, TResult=TResult, TError=TError>
where
    TError: fmt::Debug,
{
    type TResult = TResult;
    type TError = TError;

    fn handle(&self, input: TreeEvents) -> Result<Self::TResult, Self::TError> {
        (self as &ISink<TInput=TreeEvents, TResult=TResult, TError=TError>).send(input)
    }
}

fn check_kind<TSink>(options: &Options, sink: TSink, kind: &Ident)
where
    TSink: ITreeSink + Clone,
{
    let kind: &str = &kind.to_string();
    match kind {
        "html" => (),
        _ => {
            sink.notify(&options.unrecognized_node_type, URECOGNIZED_NODE_TYPE);
        },
    }
}

pub fn parse<TSink>(options: Option<Options>, sink: TSink, input: TokenStream)
where
    TSink: ITreeSink + Clone,
{
    let options = &options.unwrap_or_default();
    if input.is_empty() {
        sink.notify(&options.empty_template, EMPTY_TEMPLATE);
        return;
    }
    let state = parse_impl(options, sink, input);
    println!("Final State: {:?}", state);
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum ParseModes {
    Empty,
    Element (Ident),
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ParseState {
    mode: ParseModes,
}

impl Default for ParseState {
    fn default() -> Self {
        ParseState {
            mode: ParseModes::Empty,
        }
    }
}

fn parse_impl<TSink>(options: &Options, sink: TSink, input: TokenStream) -> ParseState
where
    TSink: ITreeSink + Clone,
{
    println!("Input: {:?}", input);
    let state = input
        .into_iter()
        .fold(ParseState::default(), | mut s, node | {
            match node {
                TokenTree::Ident (ident) => {
                    println!("Identity: {:?}", ident);
                    s.mode = ParseModes::Element(ident.clone());
                    check_kind(options, sink.clone(), &ident);
                },
                TokenTree::Group (group) => {
                    println!("Group: {:?}", group);
                    if group.delimiter() == Delimiter::Bracket {
                        println!("Group Found");
                    }
                    sink.error("Unexpected delimiter");
                },
                TokenTree::Punct (_punct) => {
                    sink.error("Unexpected punctuation");
                },
                TokenTree::Literal (_literal) => {
                    sink.error(UNEXPECTED_LITERAL);
                },
            }
            println!("State: {:?}", s);
            s
        });
    state
}

pub trait ISource {
    type TOutput;
    type THandle = Self;

    fn bind(self, sink: impl ISink) -> Self::THandle;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NotifyStrategies {
    Ignore,
    Warning,
    Error,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Options {
    pub empty_template: NotifyStrategies,
    pub unrecognized_node_type: NotifyStrategies,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            empty_template: NotifyStrategies::Warning,
            unrecognized_node_type: NotifyStrategies::Warning,
        }
    }
}

pub trait ITreeSource<TSink>
where
    TSink: ITreeSink,
{
    type THandle = Self;
    type TResult;
    type TError;

    fn bind(self, options: Option<Options>, sink: TSink) -> Self::THandle;
}

#[derive(Clone, Debug)]
pub enum TokenTreeSources {
    Group (Vec<TokenTreeSources>),
    TokenStream (TokenStream),
}

impl TokenTreeSources {
    pub fn from_token_stream(input: TokenStream) -> Self {
        TokenTreeSources::TokenStream(input)
    }
}

impl<TSink> ITreeSource<TSink> for TokenTreeSources
where
    TSink: ITreeSink + Clone,
{
    type THandle = ();
    type TResult = ();
    type TError = ();

    fn bind(self, options: Option<Options>, sink: TSink) -> Self::THandle {
        match self {
            TokenTreeSources::Group(children) => {
                for child in children {
                    child.bind(options.to_owned(), sink.clone());
                }
            },
            TokenTreeSources::TokenStream(input) => parse(options, sink, input),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    use sinks::*;
    
    #[test]
    fn should_provide_warning_for_empty_template() {
        let expected_events = vec![
            TreeEvents::Message(Warning::call_site(EMPTY_TEMPLATE)),
        ];
        let input = quote! { };
        let sink = &VecTreeSink::new();
        parse(None, sink, input);
        assert_eq!(expected_events, sink.data());
    }

    #[test]
    fn should_provide_error_for_unexpected_literal_at_root() {
        let expected_events = vec![
            TreeEvents::Message(Error::call_site(UNEXPECTED_LITERAL)),
        ];
        let input = quote! { 10 };
        let sink = &VecTreeSink::new();
        parse(None, sink, input);
        assert_eq!(expected_events, sink.data());
    }

    #[test]
    fn should_parse_stand_alone_element() {
        let expected_events = vec![
            TreeEvents::Parse(ParseEvents::ParsedElement(Element{})),
        ];
        let expected_messages = Messages::empty();
        let input = quote! { html };
        let sink = &VecTreeSink::new();
        parse(None, sink, input);
        assert_eq!(expected_events, sink.data());
    }

    #[test]
    fn should_parse_stand_alone_element_with_warning_on_ambiguous_empty_list() {
        let expected_events = vec![
            TreeEvents::Message(Warning::call_site(AMBIGUOUS_EMPTY_LIST)),
        ];
        let expected_messages = Messages::empty();
        let input = quote! { html [] };
        let sink = &VecTreeSink::new();
        parse(None, sink, input);
        assert_eq!(expected_events, sink.data());
    }

    #[test]
    fn should_parse_stand_alone_element_with_warning_on_ambiguous_empty_lists() {
        let expected_events = vec![
            TreeEvents::Message(Warning::call_site(EMPTY_LISTS_OPTIONAL)),
        ];
        let expected_messages = Messages::empty();
        let input = quote! { html [][] };
        let sink = &VecTreeSink::new();
        parse(None, sink, input);
        assert_eq!(expected_events, sink.data());
    }
}

// use syn::{ Expr };
// use syn::

// pub struct TokenizerStates {

// }

// #[derive(Clone, Debug, Eq, PartialEq)]
// pub enum Attributes {
//     Custom { key: String, value: String },
// }

// impl Parse for Attributes {
//     fn parse(_input: ParseStream) -> syn::parse::Result<Self> {
//         Ok (Attributes::Custom { key: "key".to_owned(), value: "value".to_owned() })
//     }
// }

// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct Element {
//     kind: Ident,
//     attributes: Vec<Attribute>,
//     messages: Vec<Message>,
// }

// impl IValidated for Element {
//     fn get_messages(&self) -> Vec<Message> {
//         self.messages.clone()
//     }
// }

// impl Parse for Element {
//     fn parse(input: ParseStream) -> Result<Self> {
//         input
//             .parse()
//             .map(|kind: Ident| {
//                 Element {
//                     kind: kind.to_owned(),
//                     attributes: vec![], // TODO: need to parse attrs
//                     messages: vec![
//                         Info::call_site(&format!("Parsing Element Kind: {:?}", kind))
//                     ],
//                 }
//             })
//     }
// }

// pub struct ElementList {
//     elements: Vec<Element>,
// }

// impl Parse for ElementList {
//     fn parse(input: ParseStream) -> Result<Self> {
//         input
//             .parse()
//             .map(|elements: Group| {
//                 println!("ElementList: {:?}", elements.stream());
//                     // .foldr(|mut list, element| {
//                     //     list.push(element);
//                     //     list
//                     // }, vec![])
//                     // .map(|elements| ElementList { elements })
//                 ElementList { elements: vec![] }
//             })
//     }
// }

// fn checked_parse_element(input: ParseStream) -> Option<Result<Element>> {
//     let lookahead = input.lookahead1();
//     if !lookahead.peek(syn::Ident) {
//         return None
//     }
//     // println!("Peek ident found");
//     return Some(input.parse())
//         // .map(|element: Element| {
//         //     element
//         //     // Node::new(None, None, Some(vec![
//         //     //     Info::call_site(&format!("Element: {:?}", element)),
//         //     // ]))
//         // })
//     // )
// }

// fn checked_parse_elements(input: ParseStream) -> Option<Result<ElementList>> {
//     let lookahead = input.lookahead1();
//     if !lookahead.peek(syn::Lit) {
//         println!("No literal found");
//         return None
//     }
//     println!("Literal found");
//     return Some(input.parse())
// }

// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct Node {
//     element: Element,
//     children: Vec<Node>,
// }

// impl Node {
//     pub fn new(element: Element, children: Vec<Node>) -> Self {
//         Node {
//             element,
//             children: children,
//         }
//     }
// }

// impl Parse for Node {
//     fn parse(input: ParseStream) -> syn::parse::Result<Self> {
//         let lookahead = input.lookahead1();
//         if lookahead.peek(syn::Ident) { // Element not list

//             return Ok (Node::new(Element{}, vec![]))
            
//         }
//         Ok (Node::new(Element{}, vec![]))
//     }
// }
    //     if input.is_empty() {
    //         return Ok (Node::new(None, None, Some(vec![
    //             Warning::call_site(EMPTY_TEMPLATE),
    //         ])))
    //     }
    //     println!("Begin parsing elements");
    //     let elements: Vec<Node> = vec![];
    //     // Root node is defined as an element so try to parse all found
    //     if let Some (element_result) = checked_parse_element(input) {
    //         println!("Parsed element");
    //         if let Ok (element) = element_result {
    //             elements.push(Node::new(Some(element), None, None));
    //             loop { // Grab all the elements that can be found
    //                 if let Some (elements) = checked_parse_element(input) {
    //                     println!("Parsed elements: {:?}", elements);
    //                     // elements.append(elements);
    //                     continue;
    //                 }
    //                 break;
    //             }
    //             // let nodes = elements.into_iter().map(|element| {
    //             //     Node::new(Some(element), None, None)
    //             // });
    //             return Ok (Node::new(None, Some(elements), Some(vec![
    //                 Info::call_site(&format!("Found [{:?}] elements", elements.len())),
    //             ])))
    //         }
    //     }
    //     println!("Finished parsing elements: {:?}", elements);
    //     if let Some (elements) = checked_parse_elements(input) {

    //     }
    //     // let lookahead = input.lookahead1();
    //     // if lookahead.peek(syn::Ident) {
    //     //     // println!("Peek ident found");
    //     //     return input
    //     //         .parse()
    //     //         .map(|element: Element| {
    //     //             // println!("Element: {:?}", element);
    //     //             Node::new(None, None, Some(vec![
    //     //                 Info::call_site(&format!("Element: {:?}", element)),
    //     //             ]))
    //     //         })
    //     //     // // Recursively parse nodes from the next group section if found
    //     //     // if !input.is_empty() {
                
    //     //     // }
    //     // }
    //     // if lookahead.peek(syn::UseTree::Group) {
    //     //     println!("Peek group found");
    //     // }
    //     input
    //         .parse()
    //         .map(|expr: Group| {
    //             Node::new(None, None, Some(vec![
    //                 Info::call_site(&format!("Top Level Group: {:?}", expr)),
    //             ]))
    //         })
    //         // .map(|expr: TokenStream| {
    //         //     println!("Second expr: {:?}", expr);
    //         //     Node::new(None)
    //         // })
    //         .or_else(|err| {
    //             let root_not_list_warning = Warning::call_site(&format!("Root wasn't a list: {:?}", err));
    //             // Span::call_site().unstable().warning(format!("Root wasn't a list: {:?}", err)).emit();
    //             // println!("Fall back to top level element");
    //             input
    //                 .parse()
    //                 // .map(|element: Element| {
    //                 .map(|element: TokenStream| {
    //                     // println!("Element: {:?}", element);
    //                     // Node::new(Some(element))
    //                     Node::new(None, None, Some(vec![
    //                         root_not_list_warning,
    //                         Info::call_site(&format!("Element: {:?}", element)),
    //                     ]))
    //                 })
    //         })
    //         .or_else(|err| {
    //             Ok (
    //                 Node::new(None, None, Some(vec![
    //                     Warning::call_site(&format!("Root also wasn't an element: {:?}", err)),
    //                 ]))
    //             )
    //         })
    //         // .map_err(|err| {
    //         //     Node::new(None, None, Some(vec![
    //         //         Warning::call_site(&format!("Root also wasn't an element: {:?}", err)),
    //         //     ]))
    //         //     // Span::call_site().unstable().warning(format!("Root also wasn't an element: {:?}", err)).emit();
    //         //     // err
    //         // })
    // }
    // // input
    // //     .parse()
    // //     .map(|_root: Ident| {
    // //         println!("Got root children");
    // //         Node {
    // //             element: None,
    // //             children: vec![],
    // //         }
    // //     })
// }

// fn is_next_ident(input: ParseStream) -> bool {
//     let lookahead = input.lookahead1();
//     lookahead.peek(syn::Ident)
// }

// pub static EMPTY_TEMPLATE: &str = "Template is empty";


//     // #[test]
//     fn should_parse_template_with_empty_top_level_list() {
//         let expected = Node::new(None, None, None);
//         let input = quote! {[]};
//         let actual = parse(input);
//         assert_eq!(expected, actual.trim_messages());
//     }

//     #[test]
//     fn should_parse_single_element() {
//         let expected = Node::new(None, None, None);
//         let input = quote! {html};
//         let actual = parse(input);
//         let messages = actual.messages.clone();
//         assert_eq!(expected, actual.trim_messages());
//         assert_eq!(0, messages.len(), "{:?}", messages);
//     }

//     // #[test]
//     fn should_parse_invlid_with_correct_messages() {
//         let expected = Node::new(None, None, None);
//         let input = quote! {
//             [] html
//         };
//         let actual = parse(input);
//         assert_eq!(expected, actual.trim_messages());
//         // TODO: Validate messages
//     }

//     // #[test]
//     fn should_parse_elemnt_with_empty_attributes() {
//         let expected = Node::new(None, None, None);
//         let input = quote! {
//             html []
//         };
//         let actual = parse(input);
//         assert_eq!(expected, actual.trim_messages());
//     }

//     // #[test]
//     fn should_parse_multiple_elements() {
//         let expected = Node::new(None, Some(vec![
//             Node::new(None, None, None),
//             Node::new(None, None, None),
//             Node::new(None, None, None),
//         ]), None);
//         let input = quote! {
//             html html html
//         };
//         let actual = parse(input);
//         assert_eq!(expected, actual.trim_messages());
//     }
// }
