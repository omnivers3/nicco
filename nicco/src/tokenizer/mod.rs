use proc_macro2::{ Delimiter, Group, Ident, Span, TokenTree, TokenStream };
// use proc_macro2::{ Group, Ident, TokenStream };
// use syn::parse::{ Parse, ParseStream };
// use syn::{ parse2 };
use std::fmt;

mod messages;
mod vectreesink;

pub use self::messages::*;

use sink::{ ISink };

pub static EMPTY_TEMPLATE: &str = "Template is empty";
pub static AMBIGUOUS_EMPTY_LIST: &str = "Ambiguous empty list. This can be ommitted, or you can try adding attributes or children";
pub static EMPTY_ROOT_LIST_OPTIONAL: &str = "Template is empty, root list is optional";
pub static EMPTY_LISTS_OPTIONAL: &str = "Empty trailing lists are optional following an element";
pub static URECOGNIZED_NODE_TYPE: &str = "Unrecognized node type";
pub static UNEXPECTED_LITERAL: &str = "Unexpected literal";
pub static ELEMENT_FOLLOWING_ROOT_GROUP: &str = "Unexpected element following root group";
pub static INVALID_NESTED_GROUPS: &str = "Invalid group nesting";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Container {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Element {
    kind: Ident,
    attributes: Vec<Attribute>,
}

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

    fn handle(&self, input: Self::TInput) -> Result<Self::TResult, Self::TError> {
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
        (self as &ISink<TInput=TreeEvents, TResult=TResult, TError=TError>).handle(input)
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
pub enum DomTokenStream {
    None,
    Element (Element),
    Elements (Vec<Element>),
    Attribute (Attribute),
    Attributes (Vec<Attribute>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParseState<'a> {
    Key (&'a Ident),
}

// #[derive(Clone, Debug, Eq, PartialEq)]
// struct ParseState {
//     // mode: ParseModes,
// }

// impl Default for ParseState {
//     fn default() -> Self {
//         ParseState {
//             // mode: ParseModes::Empty,
//         }
//     }
// }


fn parse_impl<TSink>(options: &Options, sink: TSink, input: TokenStream) -> DomTokenStream
where
    TSink: ITreeSink + Clone,
{
    DomTokenStream::None
    // let mut stack: [Option<&TokenTree>; 2] = [None; 2];
    // let (_, _, result) = input
    //     .into_iter()
    //     .fold((None, None, DomTokenStream::None), | stack, token | {
    //         match token {
    //             // Group of elements or attributes
    //             TokenTree::Group (group) => {
    //                 match ( stack.0, stack.1 ) {
    //                     ( None, _ ) => {
    //                         println!("Return content for group\n{:?}", group);
    //                         ( Some(&token), None, DomTokenStream::None )
    //                     },
    //                     ( Some ( prev ), None ) => {
    //                         println!("Group\n{:?}\nwith prev\n{:?}", group, prev);
    //                         ( Some(&token), None, DomTokenStream::None )
    //                     },
    //                     ( Some ( prev ), Some ( group ) ) => {
    //                         println!("Some, Some ...");
    //                         ( Some(&token), None, DomTokenStream::None )
    //                     }
    //                 }
    //             },
    //             TokenTree::Ident (ref ident) => {
    //                 match ( stack.0, stack.1 ) {
    //                     ( None, _ ) => {
    //                         ( Some(&token), None, DomTokenStream::None )
    //                     },
    //                     ( Some (prev), None ) => {
    //                         println!("Multiple ident in a row");
    //                         ( Some(&token), Some(&token), DomTokenStream::None )
    //                     },
    //                     ( Some (prev), Some (group) ) => {
    //                         println!("Ident following ident+group");
    //                         ( Some(&token), None, DomTokenStream::None )
    //                     }
    //                 }
    //             },
    //             TokenTree::Literal (literal) => {
    //                 println!("Literal");
    //                 ( Some(&token), None, DomTokenStream::None )
    //             },
    //             TokenTree::Punct (punct) => {
    //                 println!("Punct");
    //                 ( Some(&token), None, DomTokenStream::None )
    //             },
    //         }
    //         // ( None, None, DomTokenStream::None )
    //     });
    // result
    // let input: Vec<TokenTree> = input.into_iter().collect();
    // for token in input {
    //     match token {
    //         // Group of elements or attributes
    //         TokenTree::Group (group) => {
    //             match ( stack[0], stack[1] ) {
    //                 ( None, _ ) => {
    //                     println!("Return content for group\n{:?}", group);
    //                 },
    //                 ( Some ( prev ), None ) => {
    //                     println!("Group\n{:?}\nwith prev\n{:?}", group, prev);
    //                 },
    //                 ( Some ( prev ), Some ( group ) ) => {
    //                     println!("Some, Some ...");
    //                 }
    //             }
    //         },
    //         TokenTree::Ident (ref ident) => {
    //             match ( stack[0], stack[1] ) {
    //                 ( None, _ ) => {
    //                     stack[0] = Some(&token);
    //                 },
    //                 ( Some (prev), None ) => {
    //                     println!("Multiple ident in a row");
    //                 },
    //                 ( Some (prev), Some (group) ) => {
    //                     println!("Ident following ident+group");
    //                 }
    //             }
    //         },
    //         TokenTree::Literal (literal) => {
    //             println!("Literal");
    //         },
    //         TokenTree::Punct (punct) => {
    //             println!("Punct");
    //         },
    //     }
    //     // match (token, stack[0], stack[1], stack[2]) {
    //     //     (TokenTree::Ident (ident), _, _, _) => println!("ident"),
    //     //     (TokenTree::Group (ident), _, _, _) => println!("group"),
    //     //     (TokenTree::Punct (ident), _, _, _) => println!("punct"),
    //     //     (TokenTree::Literal (ident), _, _, _) => println!("literal"),
    //     // }
    // }
    // DomTokenStream::None
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
    
    use self::vectreesink::{ VecTreeSink };
    
    #[test]
    fn should_provide_warning_for_empty_template() {
        let sink = &VecTreeSink::new();
        let expected_events = vec![
            TreeEvents::Message(Warning::call_site(EMPTY_TEMPLATE)),
        ];
        let input = quote! { };
        
        parse(None, sink, input);
        assert_eq!(expected_events, sink.data());
    }

    #[test]
    fn should_provide_error_for_unexpected_literal_at_root() {
        let sink = &VecTreeSink::new();
        let expected_events = vec![
            TreeEvents::Message(Error::call_site(UNEXPECTED_LITERAL)),
        ];
        let input = quote! { 10 };
        parse(None, sink, input);
        assert_eq!(expected_events, sink.data());
    }

    #[test]
    fn should_provide_error_for_element_following_list() {
        let sink = &VecTreeSink::new();
        let expected_events = vec![
            TreeEvents::Message(Error::call_site(ELEMENT_FOLLOWING_ROOT_GROUP)),
        ];
        let input = quote! { [] html };
        parse(None, sink, input);
        assert_eq!(expected_events, sink.data());
    }

    #[test]
    fn should_parse_empty_list_of_elements() {
        let sink = &VecTreeSink::new();
        let expected_events = vec![
            TreeEvents::Message(Warning::call_site(EMPTY_ROOT_LIST_OPTIONAL)),
        ];
        let input = quote! { [] };
        parse(None, sink, input);
        assert_eq!(expected_events, sink.data());
    }

    #[test]
    fn should_parse_root_list_of_elements() {
        let sink = &VecTreeSink::new();
        let expected_events: Vec<TreeEvents> = vec![
            // TreeEvents::Message(Warning::call_site(EMPTY_LISTS_OPTIONAL)),
        ];
        let input = quote! { [ html html ] };
        
        parse(None, sink, input);
        assert_eq!(expected_events, sink.data());
    }

    // #[test]
    fn should_parse_stand_alone_element() {
        let sink = &VecTreeSink::new();
        let expected_events = vec![
            TreeEvents::Parse(ParseEvents::ParsedElement(Element { kind: Ident::new("html", Span::call_site()), attributes: vec![] })),
        ];
        let input = quote! { html };
        parse(None, sink, input);
        assert_eq!(expected_events, sink.data());
    }

    // #[test]
    fn should_parse_stand_alone_element_with_warning_on_ambiguous_empty_list() {
        let sink = &VecTreeSink::new();
        let expected_events = vec![
            TreeEvents::Message(Warning::call_site(AMBIGUOUS_EMPTY_LIST)),
        ];
        let input = quote! { html [] };
        parse(None, sink, input);
        assert_eq!(expected_events, sink.data());
    }

    // #[test]
    fn should_parse_stand_alone_element_with_warning_on_ambiguous_empty_lists() {
        let sink = &VecTreeSink::new();
        let expected_events = vec![
            TreeEvents::Message(Warning::call_site(EMPTY_LISTS_OPTIONAL)),
        ];
        let input = quote! { html [][] };
        parse(None, sink, input);
        assert_eq!(expected_events, sink.data());
    }

    // #[test]
    fn should_parse_adjacent_elements() {
        let sink = &VecTreeSink::new();
        let expected_events: Vec<TreeEvents> = vec![
            // TreeEvents::Message(Warning::call_site(EMPTY_LISTS_OPTIONAL)),
        ];
        let input = quote! { html html };
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


// fn parse_impl<TSink>(options: &Options, sink: TSink, input: TokenStream) -> ParseState
// where
//     TSink: ITreeSink + Clone,
// {
//     println!("Input: {:?}", input);
//     let state = input
//         .into_iter()
//         .fold(ParseState::default(), | mut state, node | {
//             let mode = state.mode.clone();
//             match node {
//                 TokenTree::Ident (ident) => {
//                     // match state.mode.clone() {
//                     match mode {
//                         ParseModes::Error => {
//                             //
//                         },
//                         ParseModes::Empty => { // Not yet parsing anything
//                             println!("Identity: {:?}", ident);
//                             state.mode = ParseModes::Element(ident.clone());
//                             check_kind(options, sink.clone(), &ident);
//                         },
//                         ParseModes::RootGroup (group) => { // Found an identity following a root group node
//                             println!("Invalid identity after root group: {:?}", group);
//                             // TODO: Bridge the span in the error to cover both the group and the element tags?
//                             sink.error(ELEMENT_FOLLOWING_ROOT_GROUP);
//                             state.mode = ParseModes::Element(ident.clone());
//                         },
//                         ParseModes::Element (prev_element) => { // Found an adjacent element to one in flight
//                             println!("Neighbor element: {:?}", ident);
//                             sink.element(Element { kind: prev_element.clone(), attributes: vec![] });
//                             state.mode = ParseModes::Element(ident.clone());
//                             check_kind(options, sink.clone(), &ident);
//                         },
//                         ParseModes::ElementGroup (_group) => {
//                             //
//                         },
//                         ParseModes::ElementGroups (_group) => {
//                             //
//                         },
//                     }
                    
//                 },
//                 TokenTree::Group (group) => {
//                     println!("Group: {:?}", group);
//                     if group.delimiter() == Delimiter::Bracket {
//                         println!("Group Found");
//                         match mode {
//                             ParseModes::Error => {
//                                 //
//                             },
//                             ParseModes::Empty => {
//                                 let sink = sink.clone();
//                                 let inner_state = parse_impl(options, sink, group.stream());
//                                 println!("Inner State: {:?}", inner_state);
//                                 state.mode = ParseModes::RootGroup (group);
//                             },
//                             ParseModes::RootGroup (_group) => {
//                                 // Shouldn't allow group directly inside another group
//                                 sink.error(INVALID_NESTED_GROUPS);
//                                 state.mode = ParseModes::Error;
//                             },
//                             ParseModes::Element (_prev_element) => {
//                                 // capture prev state as nested child
//                                 state.mode = ParseModes::ElementGroup (group);
//                             },
//                             ParseModes::ElementGroup (_prev_group) => {
//                                 // capture prev state as nested child
//                                 state.mode = ParseModes::ElementGroups (group);
//                             },
//                             ParseModes::ElementGroups (_prev_group) => {
//                                 //
//                             },
//                         }
//                     } else {
//                         sink.error("Unexpected delimiter");
//                     }
//                 },
//                 TokenTree::Punct (_punct) => {
//                     sink.error("Unexpected punctuation");
//                 },
//                 TokenTree::Literal (_literal) => {
//                     sink.error(UNEXPECTED_LITERAL);
//                 },
//             }
//             println!("State: {:?}", state);
//             state
//         });
//     let mode = state.mode.clone();
//     match mode {
//         ParseModes::RootGroup (_) => {
//             sink.warning(EMPTY_ROOT_LIST_OPTIONAL);
//         },
//         _ => {
//             // TODO: evaluate other ending states for validity
//         }
//     }
//     state
// }

// #[derive(Clone, Debug)]
// enum ParseModes {
//     None,
    
//     Error,
//     Empty,
//     RootGroup (Group),
//     Element (Ident),
//     ElementGroup (Group),
//     ElementGroups (Group),
// }

// impl PartialEq for ParseModes {
//     fn eq(&self, other: &ParseModes) -> bool {
//         match self {
//             ParseModes::Error => {
//                 match other {
//                     ParseModes::Error => true,
//                     _ => false,
//                 }
//             }
//             ParseModes::Empty => {
//                 match other {
//                     ParseModes::Empty => true,
//                     _ => false,
//                 }
//             },
//             ParseModes::Element (element) => {
//                 match other {
//                     ParseModes::Element (other) => {
//                         element == other
//                     },
//                     _ => false,
//                 }
//             },
//             ParseModes::RootGroup (_group) => {
//                 match other {
//                     ParseModes::RootGroup (_group) => {
//                         true
//                     },
//                     _ => false,
//                 }
//             },
//             ParseModes::ElementGroup (_group) => {
//                 match other {
//                     ParseModes::ElementGroup (_group) => {
//                         true
//                     },
//                     _ => false,
//                 }
//             },
//             ParseModes::ElementGroups (_group) => {
//                 match other {
//                     ParseModes::ElementGroups (_group) => {
//                         true
//                     },
//                     _ => false,
//                 }
//             },
//         }
//     }
// }

// impl Eq for ParseModes {}

// let mut index = 0;
// let mut stack = [&proc_macro2::TokenTree; 3];

// for token in input {
//     stack.push(token);
//     match &token {
//         TokenTree::Ident (ident) => {
//         },
//         TokenTree::Group (group) => {
//         },
//         TokenTree::Punct (_punct) => {
//         },
//         TokenTree::Literal (_literal) => {
//         },
//     }
// }

// match input.into() {
//     [ TokenTree::Ident (ident) ] => {}
// }
// println!("Input: {:?}", input);

// let state = input
//     .into_iter()
//     .fold(ParseState::default(), | mut state, node | {
//         // let mode = state.mode.clone();
//         match node {
//             TokenTree::Ident (ident) => {
//             },
//             TokenTree::Group (group) => {
//             },
//             TokenTree::Punct (_punct) => {
//             },
//             TokenTree::Literal (_literal) => {
//             },
//         }
//         println!("State: {:?}", state);
//         state
//     });
// let mode = state.mode.clone();
// match mode {
//     ParseModes::RootGroup (_) => {
//         sink.warning(EMPTY_ROOT_LIST_OPTIONAL);
//     },
//     _ => {
//         // TODO: evaluate other ending states for validity
//     }
// }
// state
