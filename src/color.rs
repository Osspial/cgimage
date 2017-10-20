use cgmath::Array;
use std::convert::AsRef;
use std::ops::{Index, IndexMut, Add, Mul, Deref, DerefMut};

macro_rules! impl_add {
    ($self:ident. $s:ident $(, $c:ident)*) => {
        $self.$s $(+ $self.$c)*
    }
}

macro_rules! impl_product {
    ($self:ident. $s:ident $(, $c:ident)*) => {
        $self.$s $(* $self.$c)*
    }
}

macro_rules! color {
    ($(
        pub struct $name:ident<S: Copy, L = $len:tt> {
            $($c:ident),+
        }
    )*) => {$(
        #[repr(C)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $name<S: Copy> {
            $(pub $c: S),+
        }

        impl<S: Copy> $name<S> {
            #[inline]
            pub fn new($($c: S),+) -> $name<S> {
                $name{ $($c),+ }
            }
        }

        impl<S: Copy> Array for $name<S> {
            type Element = S;
            #[inline]
            fn len() -> usize {$len}
            #[inline]
            fn from_value(value: S) -> $name<S> {
                $name {
                    $($c: value),+
                }
            }
            #[inline]
            fn sum(self) -> S
                where S: Add<Output=S>
            {
                impl_add!(self. $($c),+)
            }
            #[inline]
            fn product(self) -> S
                where S: Mul<Output=S>
            {
                impl_product!(self. $($c),+)
            }
        }

        impl<S: Copy> Index<usize> for $name<S> {
            type Output = S;
            #[inline]
            fn index(&self, index: usize) -> &S {
                &self.as_ref()[index]
            }
        }

        impl<S: Copy> IndexMut<usize> for $name<S> {
            #[inline]
            fn index_mut(&mut self, index: usize) -> &mut S {
                &mut self.as_mut()[index]
            }
        }

        impl<S: Copy> AsRef<[S; 4]> for $name<S> {
            #[inline]
            fn as_ref(&self) -> &[S; 4] {
                &*self
            }
        }

        impl<S: Copy> AsMut<[S; 4]> for $name<S> {
            #[inline]
            fn as_mut(&mut self) -> &mut [S; 4] {
                &mut *self
            }
        }

        impl<S: Copy> Deref for $name<S> {
            type Target = [S; 4];
            #[inline]
            fn deref(&self) -> &[S; 4] {
                unsafe{ &*(&self.r as *const _ as *const [S; 4]) }
            }
        }

        impl<S: Copy> DerefMut for $name<S> {
            #[inline]
            fn deref_mut(&mut self) -> &mut [S; 4] {
                unsafe{ &mut *(&mut self.r as *mut _ as *mut [S; 4]) }
            }
        }
    )*}
}

color!{
    pub struct Rgba<S: Copy, L = 4> {
        r, g, b, a
    }
    pub struct Rgb<S: Copy, L = 3> {
        r, g, b
    }
    pub struct Rg<S: Copy, L = 2> {
        r, g
    }
    pub struct Red<S: Copy, L = 1> {
        r
    }
}
