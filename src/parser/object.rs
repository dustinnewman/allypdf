use std::borrow::Borrow;
use std::collections::BTreeMap;
use std::convert::TryFrom;

use crate::error::{PdfError, Result};
use crate::operators::Operator;

use super::cid_parser::CIDOperator;
use super::lexer::Header;

#[derive(Debug, PartialEq, Eq)]
pub struct CrossReference {
    pub offset: u64,
    pub generation_number: u32,
    pub in_use: bool,
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord)]
pub struct IndirectReference {
    pub object_number: u32,
    pub generation_number: u32,
}

#[derive(Debug, PartialEq)]
pub struct Stream {
    pub dict: Dictionary,
    pub content: Vec<u8>,
}

#[derive(Debug, PartialEq)]
pub struct IndirectObject {
    pub object_number: u32,
    pub generation_number: u32,
    pub object: Box<Object>,
}

#[derive(Debug, PartialEq)]
pub struct XrefSubsection {
    pub start_number: u32,
    pub subsection_length: u32,
    pub references: Vec<CrossReference>,
}

#[derive(Debug, PartialEq)]
pub struct XrefSection {
    pub subsections: Vec<XrefSubsection>,
}

#[derive(Debug, PartialEq)]
pub struct Trailer {
    pub size: u64,
    pub root: IndirectReference,
    pub dictionary: Dictionary,
}

#[derive(Debug, PartialEq)]
pub enum ObjectKind {
    Boolean(bool),
    Integer(i64),
    Real(f64),
    Name(Name),
    String(Vec<u8>),
    Array(Vec<Object>),
    Dictionary(Dictionary),
    IndirectReference(IndirectReference),
    CrossReference(CrossReference),
    IndirectObject(IndirectObject),
    Stream(Stream),
    Header(Header),
    Trailer(Trailer),
    Xref(XrefSection),
    StartXref(u64),
    Operator(Operator),
    CIDOperator(CIDOperator),
    Null,
}

impl TryFrom<&Object> for bool {
    type Error = PdfError;

    fn try_from(value: &Object) -> Result<Self> {
        match &value.kind {
            ObjectKind::Boolean(x) => Ok(*x),
            _ => Err(PdfError::ObjectNotDictionary),
        }
    }
}

impl<'a> TryFrom<&'a Object> for &'a Name {
    type Error = PdfError;

    fn try_from(value: &'a Object) -> Result<Self> {
        match &value.kind {
            ObjectKind::Name(x) => Ok(x),
            _ => Err(PdfError::ObjectNotDictionary),
        }
    }
}

impl<'a> TryFrom<&'a Object> for &'a Vec<u8> {
    type Error = PdfError;

    fn try_from(value: &'a Object) -> Result<Self> {
        match &value.kind {
            ObjectKind::String(x) => Ok(x),
            _ => Err(PdfError::ObjectNotDictionary),
        }
    }
}

impl<'a> TryFrom<&'a Object> for &'a Vec<Object> {
    type Error = PdfError;

    fn try_from(value: &'a Object) -> Result<Self> {
        match &value.kind {
            ObjectKind::Array(x) => Ok(x),
            _ => Err(PdfError::ObjectNotDictionary),
        }
    }
}

impl<'a> TryFrom<&'a Object> for &'a Dictionary {
    type Error = PdfError;

    fn try_from(value: &'a Object) -> Result<Self> {
        match &value.kind {
            ObjectKind::Dictionary(x) => Ok(x),
            _ => Err(PdfError::ObjectNotDictionary),
        }
    }
}

impl<'a> TryFrom<&'a Object> for &'a IndirectReference {
    type Error = PdfError;

    fn try_from(value: &'a Object) -> Result<Self> {
        match &value.kind {
            ObjectKind::IndirectReference(x) => Ok(x),
            _ => Err(PdfError::ObjectNotDictionary),
        }
    }
}

impl<'a> TryFrom<&'a Object> for &'a CrossReference {
    type Error = PdfError;

    fn try_from(value: &'a Object) -> Result<Self> {
        match &value.kind {
            ObjectKind::CrossReference(x) => Ok(x),
            _ => Err(PdfError::ObjectNotDictionary),
        }
    }
}

impl<'a> TryFrom<&'a Object> for &'a IndirectObject {
    type Error = PdfError;

    fn try_from(value: &'a Object) -> Result<Self> {
        match &value.kind {
            ObjectKind::IndirectObject(x) => Ok(x),
            _ => Err(PdfError::ObjectNotDictionary),
        }
    }
}

impl<'a> TryFrom<&'a Object> for &'a Stream {
    type Error = PdfError;

    fn try_from(value: &'a Object) -> Result<Self> {
        match &value.kind {
            ObjectKind::Stream(x) => Ok(x),
            _ => Err(PdfError::ObjectNotDictionary),
        }
    }
}

impl<'a> TryFrom<&'a Object> for &'a Header {
    type Error = PdfError;

    fn try_from(value: &'a Object) -> Result<Self> {
        match &value.kind {
            ObjectKind::Header(x) => Ok(x),
            _ => Err(PdfError::ObjectNotDictionary),
        }
    }
}

impl<'a> TryFrom<&'a Object> for &'a Trailer {
    type Error = PdfError;

    fn try_from(value: &'a Object) -> Result<Self> {
        match &value.kind {
            ObjectKind::Trailer(x) => Ok(x),
            _ => Err(PdfError::ObjectNotDictionary),
        }
    }
}

#[derive(Debug)]
// When not in test mode, use default equality impl (consider different
// offsets to be different objects)
#[cfg_attr(not(test), derive(PartialEq))]
pub struct Object {
    pub offset: u64,
    pub kind: ObjectKind,
}

impl TryFrom<&Object> for f64 {
    type Error = PdfError;
    fn try_from(object: &Object) -> Result<Self> {
        match object {
            Object {
                kind: ObjectKind::Integer(i),
                ..
            } => Ok(*i as f64),
            Object {
                kind: ObjectKind::Real(r),
                ..
            } => Ok(*r),
            _ => Err(PdfError::ParseF64Error),
        }
    }
}

impl TryFrom<&Object> for u32 {
    type Error = PdfError;
    fn try_from(object: &Object) -> Result<Self> {
        match object {
            Object {
                kind: ObjectKind::Integer(i),
                ..
            } => Ok(*i as u32),
            Object {
                kind: ObjectKind::Real(r),
                ..
            } => Ok(*r as u32),
            _ => Err(PdfError::ParseU32Error),
        }
    }
}

impl TryFrom<&Object> for i64 {
    type Error = PdfError;
    fn try_from(object: &Object) -> Result<Self> {
        match object {
            Object {
                kind: ObjectKind::Integer(i),
                ..
            } => Ok(*i),
            Object {
                kind: ObjectKind::Real(r),
                ..
            } => Ok(*r as i64),
            _ => Err(PdfError::ParseU32Error),
        }
    }
}

// We do not want to test if the offsets are equal during testing so we don't
// to specify the offsets everywhere when they are not relevant.
#[cfg(test)]
impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

#[cfg(test)]
impl PartialEq<ObjectKind> for Object {
    fn eq(&self, other: &ObjectKind) -> bool {
        self.kind == *other
    }
}

#[derive(Debug, Eq, PartialOrd, Ord, Default, Clone)]
pub struct Name(pub Vec<u8>);

impl From<Vec<u8>> for Name {
    fn from(vec: Vec<u8>) -> Name {
        Name(vec)
    }
}

impl From<&str> for Name {
    fn from(str: &str) -> Name {
        Name(str.as_bytes().to_vec())
    }
}

impl From<Name> for Vec<u8> {
    fn from(name: Name) -> Vec<u8> {
        name.0
    }
}

impl PartialEq for Name {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> PartialEq<T> for Name
where
    Vec<u8>: PartialEq<T>,
{
    fn eq(&self, other: &T) -> bool {
        self.0 == *other
    }
}

#[derive(Debug, PartialEq)]
pub struct Dictionary(pub BTreeMap<Vec<u8>, Object>);

impl Dictionary {
    pub const fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn insert(&mut self, key: Name, object: Object) {
        self.0.insert(key.0, object);
    }

    pub fn get<'a, K>(&'a self, key: &'a K) -> Option<&'a Object>
    where
        K: Ord + ?Sized,
        Vec<u8>: Borrow<K>,
    {
        self.0.get(key)
    }
}

impl<const N: usize> From<[(Vec<u8>, Object); N]> for Dictionary {
    fn from(arr: [(Vec<u8>, Object); N]) -> Self {
        Self(BTreeMap::from(arr))
    }
}
