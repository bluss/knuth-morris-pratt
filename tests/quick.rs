
#![allow(dead_code)]

extern crate knuth_morris_pratt;

extern crate quickcheck;
extern crate odds;
#[macro_use] extern crate macro_attr;
#[macro_use] extern crate newtype_derive;


use knuth_morris_pratt::knuth_morris_pratt;
use std::ops::Deref;

use odds::string::StrExt;

use quickcheck as qc;
use quickcheck::TestResult;
use quickcheck::Arbitrary;
use quickcheck::quickcheck;

#[derive(Copy, Clone, Debug)]
/// quickcheck Arbitrary adaptor - half the size of `T` on average
struct Short<T>(T);

impl<T> Deref for Short<T> {
    type Target = T;
    fn deref(&self) -> &T { &self.0 }
}

impl<T> Arbitrary for Short<T>
    where T: Arbitrary
{
    fn arbitrary<G: qc::Gen>(g: &mut G) -> Self {
        let sz = g.size() / 2;
        Short(T::arbitrary(&mut qc::StdGen::new(g, sz)))
    }

    fn shrink(&self) -> Box<Iterator<Item=Self>> {
        Box::new((**self).shrink().map(Short))
    }
}

macro_attr! {
    #[derive(Clone, Debug, NewtypeDeref!)]
    struct Text(String);
}

static ALPHABET: &'static str = "abñòαβ\u{3c72}";
static SIMPLEALPHABET: &'static str = "ab";

impl Arbitrary for Text {
    fn arbitrary<G: qc::Gen>(g: &mut G) -> Self {
        let len = u16::arbitrary(g);
        let mut s = String::with_capacity(len as usize);
        let alpha_len = ALPHABET.chars().count();
        for _ in 0..len {
            let i = usize::arbitrary(g);
            let i = i % alpha_len;
            s.push(ALPHABET.chars().nth(i).unwrap());
        }
        Text(s)
    }
    fn shrink(&self) -> Box<Iterator<Item=Self>> {
        Box::new(self.0.shrink().map(Text))
    }
}

/// Text from an alphabet of only two letters
macro_attr! {
    #[derive(Clone, Debug, NewtypeDeref!)]
    struct SimpleText(String);
}

impl Arbitrary for SimpleText {
    fn arbitrary<G: qc::Gen>(g: &mut G) -> Self {
        let len = u16::arbitrary(g);
        let mut s = String::with_capacity(len as usize);
        let alpha_len = SIMPLEALPHABET.chars().count();
        for _ in 0..len {
            let i = usize::arbitrary(g);
            let i = i % alpha_len;
            s.push(SIMPLEALPHABET.chars().nth(i).unwrap());
        }
        SimpleText(s)
    }
    fn shrink(&self) -> Box<Iterator<Item=Self>> {
        Box::new(self.0.shrink().map(SimpleText))
    }
}

#[derive(Clone, Debug)]
struct ShortText(String);
// Half the length of Text on average
impl Arbitrary for ShortText {
    fn arbitrary<G: qc::Gen>(g: &mut G) -> Self {
        let len = u16::arbitrary(g) / 2;
        let mut s = String::with_capacity(len as usize);
        let alpha_len = ALPHABET.chars().count();
        for _ in 0..len {
            let i = usize::arbitrary(g);
            let i = i % alpha_len;
            s.push(ALPHABET.chars().nth(i).unwrap());
        }
        ShortText(s)
    }
    fn shrink(&self) -> Box<Iterator<Item=Self>> {
        Box::new(self.0.shrink().map(ShortText))
    }
}

pub fn contains(hay: &str, n: &str) -> bool {
    knuth_morris_pratt(hay.as_bytes(), n.as_bytes()).is_some()
}

pub fn find(hay: &str, n: &str) -> Option<usize> {
    knuth_morris_pratt(hay.as_bytes(), n.as_bytes())
}

pub fn contains_rev(hay: &str, n: &str) -> bool {
    let _ = (hay, n);
    unimplemented!()
}

pub fn rfind(hay: &str, n: &str) -> Option<usize> {
    let _ = (hay, n);
    unimplemented!()
}

#[test]
fn test_contains() {
    fn prop(a: Text, b: Short<Text>) -> TestResult {
        let a = &a.0;
        let b = &b[..];
        let truth = a.contains(b);
        TestResult::from_bool(contains(&a, &b) == truth)
    }
    quickcheck(prop as fn(_, _) -> _);
}

#[test]
fn test_find_str() {
    fn prop(a: Text, b: Short<Text>) -> TestResult {
        let a = &a.0;
        let b = &b[..];
        let truth = a.find(b);
        TestResult::from_bool(find(&a, &b) == truth)
    }
    quickcheck(prop as fn(_, _) -> _);
}

#[test]
fn test_contains_plus() {
    fn prop(a: Text, b: Short<Text>) -> TestResult {
        let a = &a.0;
        let b = &b[..];
        //let b = &b.0;
        if b.len() == 0 { return TestResult::discard() }
        let truth = a.contains(b);
        TestResult::from_bool(contains(&a, &b) == truth &&
            (!truth || b.substrings().all(|sub| contains(&a, sub))))
    }
    quickcheck(prop as fn(_, _) -> _);
}

#[test]
fn test_contains_substrings() {
    fn prop(s: (char, char, char, char)) -> bool {
        let mut ss = String::new();
        ss.push(s.0);
        ss.push(s.1);
        ss.push(s.2);
        ss.push(s.3);
        let a = &ss;
        for sub in a.substrings() {
            assert!(a.contains(sub));
            if !contains(a, sub) {
                return false;
            }
        }
        true
    }
    quickcheck(prop as fn(_) -> _);
}

#[ignore]
#[test]
fn test_contains_substrings_rev() {
    fn prop(s: (char, char, char, char)) -> bool {
        let mut ss = String::new();
        ss.push(s.0);
        ss.push(s.1);
        ss.push(s.2);
        ss.push(s.3);
        let a = &ss;
        for sub in a.substrings() {
            assert!(a.contains(sub));
            if !contains_rev(a, sub) {
                return false;
            }
        }
        true
    }
    quickcheck(prop as fn(_) -> _);
}

#[test]
fn test_find_period() {
    fn prop(a: SimpleText, b: Short<SimpleText>) -> TestResult {
        let a = &a.0;
        let b = &b[..];
        let pat = [b, b].concat();
        let truth = a.find(&pat);
        TestResult::from_bool(find(a, &pat) == truth)
    }
    quickcheck(prop as fn(_, _) -> _);
}
