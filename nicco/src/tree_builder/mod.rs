use std::fmt::{ Debug };
// use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash};//, Hasher};

// use sink::ISink;

/// `IEventType` captures the set of required traits expected to be implemented
/// on any type being used to represent VDom event payloads.
///
/// Events in this system are deterministic and are expected to be represented
/// statically in the built VDom such that all events simply trigger an emit
/// of the event through the program's main update loop.
pub trait IEventPayload: Clone + Eq + Hash + PartialEq + Debug {}

/// Auto-implement IEvent for types which share the required traits
impl<TTarget> IEventPayload for TTarget
where
    TTarget: Clone + Eq + Hash + PartialEq + Debug,
{}

/// `EventKinds` represent the set of triggering actions that can be observed
/// via the VDom elements.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum EventKinds {
    /// `OnClick` is triggered when the specified VDom element is clicked on.
    OnClick,
}

/// `Attribute` represents a property of a VDom element.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Attributes<'a, TEvents>
where
    TEvents: IEventPayload,
{
    /// `Custom` attributes allow you to provide your own key=value attribute for
    /// a VDom element
    Custom { key: &'a str, value: &'a str },

    /// `Event` represents a triggering type along with the payload which should
    /// be routed through the main program's update loop when that event fires.
    Event { kind: EventKinds, payload: TEvents },
}

pub enum TreeSinkEvents {
    Element ()
}

// pub trait ITreeSink {
//     type THandle: Clone;
//     type TError;

//     fn send(&self, event: TreeSinkEvents) -> Result<Self::THandle, Self::TError>;
// }

// impl<T> ISink<
//     TInput=TreeSinkEvents,
//     TResult=(),
//     TError=()
// > for T
// where
//     T: ITreeSink<
//     >
// {}

/// Methods used by Parser implementations to create a VDom.
///
/// Using associate types could be used to enable a parser to implement
/// multiple VDoms
pub trait TreeSink {
    /// `Handle` references a VDom node.
    ///
    /// Must implement Clone to enable shared references to the same Node.
    type Handle: Clone;

    /// `Output` specifies the expected result of the parse execution.
    type Output = Self;

    /// Consume the sink and build the desired output
    fn finish(self) -> Self::Output;

    /// Get a handle to the root VDom node.
    fn get_root(&mut self) -> Self::Handle;

    /// Register a VDom element with the Parser and return it's Handle
    fn create_element(&mut self, name: &str) -> Self::Handle;
}

pub trait Tracer {
    type Handle;

    fn trace_handle(&self, node: &Self::Handle);
}