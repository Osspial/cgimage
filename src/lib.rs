extern crate cgmath;
extern crate cgmath_geometry;

pub mod color;

use std::borrow::Borrow;
use std::marker::PhantomData;
use cgmath_geometry::DimsRect;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Image<P: Copy, B: Borrow<[P]>> {
    buffer: B,
    img_deref: ImgMut<'static, P>
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Img<'a, P: Copy> {
    tlo: *const P,
    stride: usize,
    dims: DimsRect<u32>,
    _marker: PhantomData<&'a [P]>
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub struct ImgMut<'a, P: Copy> {
    tlo: *mut P,
    stride: usize,
    dims: DimsRect<u32>,
    _marker: PhantomData<&'a mut [P]>
}

pub trait Buffer: Borrow<[<Self as Buffer>::Item]> {
    type Item: Clone;
    fn alloc(len: usize, value: Self::Item) -> Self;
}

pub trait BufferResize: Buffer {
    fn resize(&mut self, new_len: usize, value: Self::Item);
}

impl<P: Copy, B: Buffer<Item=P>> Image<P, B> {
    pub fn new(init_color: P, dims: DimsRect<u32>) -> Image<P, B> {
        unimplemented!()
        // Image {
        //     buffer: B::alloc(init_color, dims.width() as usize * dims.height() as usize)
        // }
    }
}
