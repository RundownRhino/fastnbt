use std::convert::TryInto;

use super::super::*;

pub struct Builder {
    payload: Vec<u8>,
}

impl Builder {
    pub fn new() -> Self {
        Builder {
            payload: Vec::new(),
        }
    }

    pub fn tag(mut self, t: Tag) -> Self {
        self.payload.push(t as u8);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        let len_bytes = &(name.len() as u16).to_be_bytes()[..];
        self.payload.extend_from_slice(len_bytes);
        self.payload.extend_from_slice(name.as_bytes());
        self
    }

    pub fn start_compound(self, name: &str) -> Self {
        self.tag(Tag::Compound).name(name)
    }

    pub fn end_compound(self) -> Self {
        self.tag(Tag::End)
    }

    pub fn start_list(self, name: &str, element_tag: Tag, size: i32) -> Self {
        self.tag(Tag::List)
            .name(name)
            .tag(element_tag)
            .int_payload(size)
    }

    pub fn byte(self, name: &str, b: i8) -> Self {
        self.tag(Tag::Byte).name(name).byte_payload(b)
    }

    pub fn short(self, name: &str, b: i16) -> Self {
        self.tag(Tag::Short).name(name).short_payload(b)
    }

    pub fn int(self, name: &str, b: i32) -> Self {
        self.tag(Tag::Int).name(name).int_payload(b)
    }

    pub fn long(self, name: &str, b: i64) -> Self {
        self.tag(Tag::Long).name(name).long_payload(b)
    }

    pub fn string(self, name: &str, s: &str) -> Self {
        self.tag(Tag::String).name(name).string_payload(s)
    }

    pub fn float(self, name: &str, n: f32) -> Self {
        self.tag(Tag::Float).name(name).float_payload(n)
    }

    pub fn double(self, name: &str, n: f64) -> Self {
        self.tag(Tag::Double).name(name).double_payload(n)
    }

    pub fn byte_array(self, name: &str, bs: &[i8]) -> Self {
        self.tag(Tag::ByteArray)
            .name(name)
            .int_payload(bs.len().try_into().unwrap())
            .byte_array_payload(bs)
    }

    pub fn string_payload(self, s: &str) -> Self {
        self.name(s)
    }

    pub fn byte_payload(mut self, b: i8) -> Self {
        self.payload.push(b as u8);
        self
    }

    pub fn byte_array_payload(mut self, bs: &[i8]) -> Self {
        for b in bs {
            self.payload.push(*b as u8);
        }
        self
    }

    pub fn short_payload(mut self, i: i16) -> Self {
        self.payload.extend_from_slice(&i.to_be_bytes()[..]);
        self
    }

    pub fn int_payload(mut self, i: i32) -> Self {
        self.payload.extend_from_slice(&i.to_be_bytes()[..]);
        self
    }

    pub fn int_array_payload(mut self, is: &[i32]) -> Self {
        for i in is {
            self = self.int_payload(*i);
        }
        self
    }

    pub fn long_payload(mut self, i: i64) -> Self {
        self.payload.extend_from_slice(&i.to_be_bytes()[..]);
        self
    }

    pub fn long_array_payload(mut self, is: &[i64]) -> Self {
        for i in is {
            self = self.long_payload(*i);
        }
        self
    }

    pub fn float_payload(mut self, f: f32) -> Self {
        self.payload.extend_from_slice(&f.to_be_bytes()[..]);
        self
    }

    pub fn double_payload(mut self, f: f64) -> Self {
        self.payload.extend_from_slice(&f.to_be_bytes()[..]);
        self
    }

    pub fn build(self) -> Vec<u8> {
        self.payload
    }
}
