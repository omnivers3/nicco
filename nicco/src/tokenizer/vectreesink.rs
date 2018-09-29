use super::*;

use sink::{ ISink };
use sink::vecsink::{ Error, VecSink };

pub type VecTreeSink = VecSink<TreeEvents>;

impl ITreeSink for VecTreeSink {
    type TResult = usize;
    type TError = Error;

    fn handle(&self, input: TreeEvents) -> Result<Self::TResult, Self::TError> {
        (self as &ISink<TInput=TreeEvents, TResult=Self::TResult, TError=Self::TError>).handle(input)
    }
}

impl<'a> ITreeSink for &'a VecTreeSink {
    type TResult = usize;
    type TError = Error;

    fn handle(&self, input: TreeEvents) -> Result<Self::TResult, Self::TError> {
        (self as &ISink<TInput=TreeEvents, TResult=Self::TResult, TError=Self::TError>).handle(input)
    }
}
