#![allow(dead_code)]

use crate::util::integer::*;

use std::marker::PhantomData;
use std::str::Bytes;

pub struct ParseUnsigned<'a, T> {
    bytes: Bytes<'a>,
    phantom: PhantomData<T>,
}

pub struct ParseSigned<'a, T> {
    bytes: Bytes<'a>,
    phantom: PhantomData<T>,
}

impl<T: Unsigned<T>> Iterator for ParseUnsigned<'_, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        try_unsigned(&mut self.bytes)
    }
}

impl<T: Signed<T>> Iterator for ParseSigned<'_, T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        try_signed(&mut self.bytes)
    }
}

pub trait ParseOps {
    fn unsigned<T: Unsigned<T>>(&self) -> T;
    fn signed<T: Signed<T>>(&self) -> T;
    fn iter_unsigned<T: Unsigned<T>>(&self) -> ParseUnsigned<'_, T>;
    fn iter_signed<T: Signed<T>>(&self) -> ParseSigned<'_, T>;
}

impl<S: AsRef<str>> ParseOps for S {
    fn unsigned<T: Unsigned<T>>(&self) -> T {
        let str = self.as_ref();
        try_unsigned(&mut str.bytes()).expect(format!("Unable to parse \"{str}\"").as_str())
    }

    fn signed<T: Signed<T>>(&self) -> T {
        let str = self.as_ref();
        try_signed(&mut str.bytes()).expect(format!("Unable to parse \"{str}\"").as_str())
    }

    fn iter_unsigned<T: Unsigned<T>>(&self) -> ParseUnsigned<'_, T> {
        ParseUnsigned {
            bytes: self.as_ref().bytes(),
            phantom: PhantomData,
        }
    }

    fn iter_signed<T: Signed<T>>(&self) -> ParseSigned<'_, T> {
        ParseSigned {
            bytes: self.as_ref().bytes(),
            phantom: PhantomData,
        }
    }
}

fn try_unsigned<T: Unsigned<T>>(bytes: &mut Bytes) -> Option<T> {
    let mut n = loop {
        let digit = bytes.next()?.wrapping_sub(b'0');
        if digit < 10 {
            break T::from(digit);
        }
    };

    for byte in bytes {
        let digit = byte.wrapping_sub(b'0');
        if digit >= 10 {
            break;
        }
        n = T::TEN * n + T::from(digit);
    }

    Some(n)
}

fn try_signed<T: Signed<T>>(bytes: &mut Bytes<'_>) -> Option<T> {
    let (mut n, negative) = loop {
        let digit = bytes.next()?.wrapping_sub(b'0');
        if digit == 253 {
            break (T::ZERO, true);
        }
        if digit < 10 {
            break (T::from(digit), false);
        }
    };

    for byte in bytes {
        let digit = byte.wrapping_sub(b'0');
        if digit >= 10 {
            break;
        }
        n = T::TEN * n + T::from(digit);
    }

    Some(if negative { -n } else { n })
}
