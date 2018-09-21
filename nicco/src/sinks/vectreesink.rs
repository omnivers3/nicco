use super::tokenizer::*;
use sink::{ ISink };
use std::cell::RefCell;

#[derive(Debug)]
pub enum Error {
    Overflow,
}

pub struct VecSink<TInput>
where
    TInput: Clone,
{
    data: RefCell<Vec<TInput>>,
}

impl<TInput> VecSink<TInput>
where
    TInput: Clone,
{
    pub fn new() -> Self {
        VecSink {
            data: RefCell::new(vec![]),
        }
    }

    pub fn data(&self) -> Vec<TInput> {
        let data = self.data.borrow();
        data.clone()
    }

    pub fn push(&self, input: TInput) -> Result<usize, Error> {
        let mut data = self.data.borrow_mut();
        (*data).push(input);
        Ok (data.len())
    }
}

impl<TInput> ISink for VecSink<TInput>
where
    TInput: Clone,
{
    type TInput = TInput;
    type TResult = usize;
    type TError = Error;

    fn send(&self, input: Self::TInput) -> Result<Self::TResult, Self::TError> {
        self.push(input)
    }
}

impl<'a, TInput> ISink for &'a VecSink<TInput>
where
    TInput: Clone,
{
    type TInput = TInput;
    type TResult = usize;
    type TError = Error;

    fn send(&self, input: Self::TInput) -> Result<Self::TResult, Self::TError> {
        self.push(input)
    }
}

pub type VecTreeSink = VecSink<TreeEvents>;

impl ITreeSink for VecTreeSink {
    type TResult = usize;
    type TError = Error;

    fn handle(&self, input: TreeEvents) -> Result<Self::TResult, Self::TError> {
        self.push(input)
    }
}

impl<'a> ITreeSink for &'a VecTreeSink {
    type TResult = usize;
    type TError = Error;

    fn handle(&self, input: TreeEvents) -> Result<Self::TResult, Self::TError> {
        self.push(input)
    }
}