use std::convert::TryInto;

use byteorder::{BigEndian, ReadBytesExt};
use serde::de::{self, IntoDeserializer};
use serde::forward_to_deserialize_any;

use crate::error::{Error, Result};
use crate::{de::Deserializer, Tag};

enum ArrWrapStage {
    Tag,
    Data,
    Done,
}

pub(crate) struct ArrayWrapperAccess<'a, 'de> {
    de: &'a mut Deserializer<'de>,
    stage: ArrWrapStage,
    tag: Tag,
    size: i32,
}

impl<'a, 'de> ArrayWrapperAccess<'a, 'de> {
    pub(crate) fn new(de: &'a mut Deserializer<'de>, size: i32, tag: Tag) -> Self {
        Self {
            de,
            tag,
            size,
            stage: ArrWrapStage::Tag,
        }
    }
}

impl<'a, 'de> de::MapAccess<'de> for ArrayWrapperAccess<'a, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: de::DeserializeSeed<'de>,
    {
        match self.stage {
            ArrWrapStage::Tag => seed.deserialize("tag".into_deserializer()).map(Some),
            ArrWrapStage::Data => seed.deserialize("data".into_deserializer()).map(Some),
            ArrWrapStage::Done => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: de::DeserializeSeed<'de>,
    {
        match self.stage {
            ArrWrapStage::Tag => {
                self.stage = ArrWrapStage::Data;
                let t: u8 = self.tag.into();
                seed.deserialize(t.into_deserializer())
            }
            ArrWrapStage::Data => {
                // Can we just read directly from the input and return the
                // value? Need to make a deserializer...
                self.stage = ArrWrapStage::Done;
                seed.deserialize(ArrayDeserializer {
                    de: &mut *self.de,
                    size: self.size,
                })
            }
            ArrWrapStage::Done => panic!("extra key"),
        }
    }
}

struct ArrayAccess<'a, 'de> {
    de: &'a mut Deserializer<'de>,
    hint: i32,
    remaining: i32,
}

impl<'a, 'de> ArrayAccess<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>, size: i32) -> Self {
        Self {
            de,
            hint: size,
            remaining: size,
        }
    }
}

impl<'a, 'de> de::SeqAccess<'de> for ArrayAccess<'a, 'de> {
    type Error = Error;

    fn size_hint(&self) -> Option<usize> {
        self.hint.try_into().ok()
    }

    #[inline]
    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        if self.remaining > 0 {
            self.remaining = self.remaining - 1;
            let val = seed.deserialize(ArrayDeserializer {
                de: self.de,
                size: 0, // not important here. Maybe split ArrayDeserializer into two types.
            })?;
            Ok(Some(val))
        } else {
            Ok(None)
        }
    }
}

pub(crate) struct ArrayDeserializer<'a, 'de> {
    pub(crate) de: &'a mut Deserializer<'de>,
    pub(crate) size: i32,
}

// Job is to start deserializing a Seq which is a *Array type, and to actually
// deserialize the elements.
impl<'a, 'de> serde::Deserializer<'de> for ArrayDeserializer<'a, 'de> {
    type Error = Error;

    forward_to_deserialize_any! {
        bool i16 i128 u16  u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct tuple
        tuple_struct map struct enum identifier ignored_any
    }

    fn deserialize_any<V>(self, _: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::bespoke(
            "fastnbt issue: unexpected any in ArrayDeserializer".into(),
        ))
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_seq(ArrayAccess::new(self.de, self.size)) // TOOD: size
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let val = self.de.input.0.read_i8()?;
        visitor.visit_i8(val)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let val = self.de.input.0.read_u8()?;
        visitor.visit_u8(val)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let val = self.de.input.0.read_i32::<BigEndian>()?;
        visitor.visit_i32(val)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let val = self.de.input.0.read_u32::<BigEndian>()?;
        visitor.visit_u32(val)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let val = self.de.input.0.read_i64::<BigEndian>()?;
        visitor.visit_i64(val)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let val = self.de.input.0.read_u64::<BigEndian>()?;
        visitor.visit_u64(val)
    }
}
