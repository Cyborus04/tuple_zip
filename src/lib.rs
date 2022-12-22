//! A crate for converting a tuple of iterators into an iterator of tuples
//! 
//! ```
//! use tuple_zip::TupleZip;
//! 
//! let a = [1, 2, 3, 4];
//! let b = [5, 6, 7];
//! let c = ["x", "y", "z"];
//! 
//! let zipped = (a, b, c).tuple_zip();
//! 
//! let expected = [(1, 5, "x"), (2, 6, "y"), (3, 7, "z")];
//! assert!(zipped.eq(expected));
//! ```

macro_rules! types {
    ($(struct $name:ident ($($t:ident $item:ident $into:ident $n:ident),*);)*) => {
        $(
            #[doc(hidden)]
            pub struct $name<$($t),*>($($t,)*);

            impl<$($t,)* $($item),*> std::iter::Iterator for $name<$($t),*> where
                $($t: std::iter::Iterator<Item = $item>),* {
                type Item = ($($item,)*);
                fn next(&mut self) -> Option<Self::Item> {
                    let $name($($n,)*) = self;
                    Some(($(<$t as std::iter::Iterator>::next($n)?,)*))
                }
            }

            impl<$($t,)* $($item,)* $($into),*> $crate::TupleZip for ($($into,)*) where
                $($into: std::iter::IntoIterator<Item = $item, IntoIter = $t>,)*
                $($t: std::iter::Iterator<Item = $item>),* {
                type Iter = $name<$($t,)*>;
                fn tuple_zip(self) -> $name<$($t,)*> {
                    let ($($n,)*) = self;
                    $name($(<$into as std::iter::IntoIterator>::into_iter($n),)*)
                }
            }
        )*
    };
}

types! {
    struct Tuple01(A A2 A3 a);
    struct Tuple02(A A2 A3 a, B B2 B3 b);
    struct Tuple03(A A2 A3 a, B B2 B3 b, C C2 C3 c);
    struct Tuple04(A A2 A3 a, B B2 B3 b, C C2 C3 c, D D2 D3 d);
    struct Tuple05(A A2 A3 a, B B2 B3 b, C C2 C3 c, D D2 D3 d, E E2 E3 e);
    struct Tuple06(A A2 A3 a, B B2 B3 b, C C2 C3 c, D D2 D3 d, E E2 E3 e, F F2 F3 f);
    struct Tuple07(A A2 A3 a, B B2 B3 b, C C2 C3 c, D D2 D3 d, E E2 E3 e, F F2 F3 f, G G2 G3 g);
    struct Tuple08(A A2 A3 a, B B2 B3 b, C C2 C3 c, D D2 D3 d, E E2 E3 e, F F2 F3 f, G G2 G3 g, H H2 H3 h);
    struct Tuple09(A A2 A3 a, B B2 B3 b, C C2 C3 c, D D2 D3 d, E E2 E3 e, F F2 F3 f, G G2 G3 g, H H2 H3 h, I I2 I3 i);
    struct Tuple10(A A2 A3 a, B B2 B3 b, C C2 C3 c, D D2 D3 d, E E2 E3 e, F F2 F3 f, G G2 G3 g, H H2 H3 h, I I2 I3 i, J J2 J3 j);
    struct Tuple11(A A2 A3 a, B B2 B3 b, C C2 C3 c, D D2 D3 d, E E2 E3 e, F F2 F3 f, G G2 G3 g, H H2 H3 h, I I2 I3 i, J J2 J3 j, K K2 K3 k);
    struct Tuple12(A A2 A3 a, B B2 B3 b, C C2 C3 c, D D2 D3 d, E E2 E3 e, F F2 F3 f, G G2 G3 g, H H2 H3 h, I I2 I3 i, J J2 J3 j, K K2 K3 k, L L2 L3 l);
    struct Tuple13(A A2 A3 a, B B2 B3 b, C C2 C3 c, D D2 D3 d, E E2 E3 e, F F2 F3 f, G G2 G3 g, H H2 H3 h, I I2 I3 i, J J2 J3 j, K K2 K3 k, L L2 L3 l, M M2 M3 m);
    struct Tuple14(A A2 A3 a, B B2 B3 b, C C2 C3 c, D D2 D3 d, E E2 E3 e, F F2 F3 f, G G2 G3 g, H H2 H3 h, I I2 I3 i, J J2 J3 j, K K2 K3 k, L L2 L3 l, M M2 M3 m, N N2 N3 n);
    struct Tuple15(A A2 A3 a, B B2 B3 b, C C2 C3 c, D D2 D3 d, E E2 E3 e, F F2 F3 f, G G2 G3 g, H H2 H3 h, I I2 I3 i, J J2 J3 j, K K2 K3 k, L L2 L3 l, M M2 M3 m, N N2 N3 n, O O2 O3 o);
    struct Tuple16(A A2 A3 a, B B2 B3 b, C C2 C3 c, D D2 D3 d, E E2 E3 e, F F2 F3 f, G G2 G3 g, H H2 H3 h, I I2 I3 i, J J2 J3 j, K K2 K3 k, L L2 L3 l, M M2 M3 m, N N2 N3 n, O O2 O3 o, P P2 P3 p);
}

pub trait TupleZip {
    type Iter: std::iter::Iterator;

    /// Converts a tuple of iterators into an iterator of tuples (max 16-tuple)
    /// 
    /// The returned iterator will return `None` as soon as any of its component
    /// iterators does
    fn tuple_zip(self) -> Self::Iter;
}