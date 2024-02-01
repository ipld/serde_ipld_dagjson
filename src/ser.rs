//! Serialization.
use std::{fmt, io};

use cid::{multibase::Base, serde::CID_SERDE_PRIVATE_IDENTIFIER, Cid};
use serde::{ser, Serialize};

use crate::{
    error::EncodeError,
    shared::{ReservedKeyMap, ReservedKeyValue},
};

/// Serializes a value to a vector.
pub fn to_vec<T>(value: &T) -> Result<Vec<u8>, EncodeError>
where
    T: ser::Serialize + ?Sized,
{
    let mut writer = Vec::new();
    let mut json_serializer = serde_json::Serializer::new(&mut writer);
    let serializer = Serializer::new(&mut json_serializer);
    value.serialize(serializer)?;
    Ok(writer)
}

/// Serializes a value to a writer.
pub fn to_writer<W, T>(writer: W, value: &T) -> Result<(), EncodeError>
where
    W: io::Write,
    T: ser::Serialize,
{
    let mut json_serializer = serde_json::Serializer::new(writer);
    let serializer = Serializer::new(&mut json_serializer);
    Ok(value.serialize(serializer)?)
}

pub struct Serializer<S> {
    ser: S,
}

impl<S> Serializer<S> {
    pub fn new(serializer: S) -> Self {
        Self { ser: serializer }
    }
}

impl<S> ser::Serializer for Serializer<S>
where
    S: ser::Serializer,
{
    type Ok = S::Ok;
    type Error = S::Error;

    type SerializeSeq = Serializer<S::SerializeSeq>;
    type SerializeTuple = Serializer<S::SerializeTuple>;
    type SerializeTupleStruct = Serializer<S::SerializeTupleStruct>;
    //type SerializeTupleVariant = Serializer<S::SerializeTupleVariant>;
    //type SerializeTupleVariant = SerializeCid<S::SerializeTupleVariant, S>;
    //type SerializeTupleVariant = SerializeCid<Self>;
    //type SerializeTupleVariant = SerializeCid<S::SerializeTupleVariant, CidSerializer<'a, S>>;
    //type SerializeTupleVariant = SerializeCid<S::SerializeTupleVariant, CidSerializer<S>>;
    type SerializeTupleVariant = SerializeCid<S::SerializeTupleVariant, S>;
    type SerializeMap = Serializer<S::SerializeMap>;
    type SerializeStruct = Serializer<S::SerializeStruct>;
    type SerializeStructVariant = Serializer<S::SerializeStructVariant>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.ser.serialize_bool(v)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.ser.serialize_i8(v)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.ser.serialize_i16(v)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.ser.serialize_i32(v)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.ser.serialize_i64(v)
    }

    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        self.ser.serialize_i128(v)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.ser.serialize_u8(v)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.ser.serialize_u16(v)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.ser.serialize_u32(v)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.ser.serialize_u64(v)
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        self.ser.serialize_u128(v)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        // In DAG-JSON only finite floats are supported.
        if !v.is_finite() {
            Err(ser::Error::custom(
                "Float must be a finite number, not Infinity or NaN".to_string(),
            ))
        } else {
            self.ser.serialize_f32(v)
        }
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        // In DAG-JSON only finite floats are supported.
        if !v.is_finite() {
            Err(ser::Error::custom(
                "Float must be a finite number, not Infinity or NaN".to_string(),
            ))
        } else {
            self.ser.serialize_f64(v)
        }
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.ser.serialize_char(v)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.ser.serialize_str(v)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        let bytes = ReservedKeyMap {
            _slash: ReservedKeyValue::Bytes {
                bytes: Base::Base64.encode(v),
            },
        };
        bytes.serialize(self.ser)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.ser.serialize_none()
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        self.ser.serialize_some(&SerializeRef::new(value))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.ser.serialize_unit()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.ser.serialize_unit_struct(name)
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.ser
            .serialize_unit_variant(name, variant_index, variant)
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        //println!("vmx: ser: serialize_newtype_struct");
        //if name == CID_SERDE_PRIVATE_IDENTIFIER {
        //    //SerializeRef::new(value).serialize(CidSerializer(self.ser))
        //    value.serialize(CidSerializer(self.ser))
        //} else {
            self.ser
                .serialize_newtype_struct(name, &SerializeRef::new(value))
        //}
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        self.ser
            .serialize_newtype_variant(name, variant_index, variant, &SerializeRef::new(value))
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(Self::SerializeSeq::new(self.ser.serialize_seq(len)?))
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(Self::SerializeTuple::new(self.ser.serialize_tuple(len)?))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(Self::SerializeTupleStruct::new(
            self.ser.serialize_tuple_struct(name, len)?,
        ))
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        println!("vmx: ser: serialize tuple variant: name, variant_index, variant, len: {:?} {:?} {:?} {:?}", name, variant_index, variant, len);
        if name == CID_SERDE_PRIVATE_IDENTIFIER && variant_index == 0 && variant == CID_SERDE_PRIVATE_IDENTIFIER && len == 1 {
            println!("vmx: ser: serialize tuple variant: it's a CID");
            //Ok(SerializeCid::new(
            //        self.ser
            //        .serialize_tuple_variant(name, variant_index, variant, len)?,
            //))
            //Ok(Self::SerializeTupleVariant::new(CidSerializer(self.ser)))
            //Ok(Self::SerializeTupleVariant::new_cid(
            //        //self.ser
            //        //.serialize_tuple_variant(name, variant_index, variant, len)?,
            //        CidSerializer(self.ser)
            //))
            //Ok(Self::SerializeTupleVariant::new(
            //        self.ser
            //        .serialize_tuple_variant(name, variant_index, variant, len)?,
            //))
            //Ok(Self::SerializeTupleVariant::new(
            //        //self.ser
            //        //.serialize_tuple_variant(name, variant_index, variant, len)?,
            //        self.ser,
            //        //name, variant_index, variant, len,
            //))
            //Ok(Self::SerializeTupleVariant::new(CidSerializer(self.ser)))
            Ok(Self::SerializeTupleVariant::new_cid(
                    //CidSerializer::new(self.ser)
                   self.ser
            ))
        } else {
            Ok(Self::SerializeTupleVariant::new(
                    self.ser
                    .serialize_tuple_variant(name, variant_index, variant, len)?,
                    //self.ser,
                    //name, variant_index, variant, len,
            ))
        }
            //Ok(Self::SerializeTupleVariant::new(
            //        self.ser
            //        .serialize_tuple_variant(name, variant_index, variant, len)?,
            //))
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(Self::SerializeMap::new(self.ser.serialize_map(len)?))
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(Self::SerializeStruct::new(
            self.ser.serialize_struct(name, len)?,
        ))
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(Self::SerializeStructVariant::new(
            self.ser
                .serialize_struct_variant(name, variant_index, variant, len)?,
        ))
    }

    fn collect_seq<I>(self, iter: I) -> Result<Self::Ok, Self::Error>
    where
        I: IntoIterator,
        I::Item: ser::Serialize,
    {
        let iter = iter.into_iter().map(SerializeSized::new);
        self.ser.collect_seq(iter)
    }

    fn collect_map<K, V, I>(self, iter: I) -> Result<Self::Ok, Self::Error>
    where
        K: ser::Serialize,
        V: ser::Serialize,
        I: IntoIterator<Item = (K, V)>,
    {
        let iter = iter
            .into_iter()
            .map(|(k, v)| (SerializeSized::new(k), SerializeSized::new(v)));
        self.ser.collect_map(iter)
    }

    fn collect_str<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + fmt::Display,
    {
        self.ser.collect_str(value)
    }

    fn is_human_readable(&self) -> bool {
        self.ser.is_human_readable()
    }
}

struct SerializeRef<'a, T: ?Sized> {
    value: &'a T,
}

impl<'a, T: ?Sized> SerializeRef<'a, T> {
    fn new(value: &'a T) -> Self {
        Self { value }
    }
}

impl<'a, T> ser::Serialize for SerializeRef<'a, T>
where
    T: ?Sized + ser::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        ser::Serialize::serialize(self.value, Serializer::new(serializer))
    }
}

struct SerializeSized<T> {
    value: T,
}

impl<T> SerializeSized<T> {
    fn new(value: T) -> Self {
        SerializeSized { value }
    }
}

impl<T> ser::Serialize for SerializeSized<T>
where
    T: ser::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        ser::Serialize::serialize(&self.value, Serializer::new(serializer))
    }
}

impl<S> ser::SerializeSeq for Serializer<S>
where
    S: ser::SerializeSeq,
{
    type Ok = S::Ok;
    type Error = S::Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        self.ser.serialize_element(&SerializeRef::new(value))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.ser.end()
    }
}

impl<S> ser::SerializeTuple for Serializer<S>
where
    S: ser::SerializeTuple,
{
    type Ok = S::Ok;
    type Error = S::Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        self.ser.serialize_element(&SerializeRef::new(value))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.ser.end()
    }
}

impl<S> ser::SerializeTupleStruct for Serializer<S>
where
    S: ser::SerializeTupleStruct,
{
    type Ok = S::Ok;
    type Error = S::Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        self.ser.serialize_field(&SerializeRef::new(value))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.ser.end()
    }
}

impl<S> ser::SerializeTupleVariant for Serializer<S>
where
    S: ser::SerializeTupleVariant
{
    type Ok = S::Ok;
    type Error = S::Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        println!("vmx: ser: serialize tuple variant");
        self.ser.serialize_field(&SerializeRef::new(value))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.ser.end()
    }
}

impl<S> ser::SerializeMap for Serializer<S>
where
    S: ser::SerializeMap,
{
    type Ok = S::Ok;
    type Error = S::Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        self.ser.serialize_key(&SerializeRef::new(key))
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        self.ser.serialize_value(&SerializeRef::new(value))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.ser.end()
    }

    fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> Result<(), Self::Error>
    where
        K: ?Sized + ser::Serialize,
        V: ?Sized + ser::Serialize,
    {
        self.ser
            .serialize_entry(&SerializeRef::new(key), &SerializeRef::new(value))
    }
}

impl<S> ser::SerializeStruct for Serializer<S>
where
    S: ser::SerializeStruct,
{
    type Ok = S::Ok;
    type Error = S::Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        self.ser.serialize_field(key, &SerializeRef::new(value))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.ser.end()
    }

    fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error> {
        self.ser.skip_field(key)
    }
}

impl<S> ser::SerializeStructVariant for Serializer<S>
where
    S: ser::SerializeStructVariant,
{
    type Ok = S::Ok;
    type Error = S::Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        self.ser.serialize_field(key, &SerializeRef::new(value))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.ser.end()
    }

    fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error> {
        self.ser.skip_field(key)
    }
}


//pub struct SerializeCid<U>
//where
//    //S: ser::Serializer + ser::SerializeTupleVariant,
//    U: ser::Serializer,
//{
//    //tuple_ser: S,
//    tuple_ser: Option<U::SerializeTupleVariant>,
//    is_cid: bool,
//    json_ser: U,
//    //cid_serializer: Option<CidSerializer<S>>,
//    //cid: Option<<S as ser::Serializer>::SerializeTupleVariant::Ok>
//    //cid: Option<<S as ser::SerializeTupleVariant>::Ok>
//}

pub enum SerializeCid<S, U> {
    TupleVariant(S),
    Cid(U),
}

impl<S, U> SerializeCid<S, U>
//where
    //S: ser::SerializeTupleVariant,
    //U: ser::Serializer,
{
    //pub fn new(tuple_ser: S, json_ser: U) -> Self {
    //    //Self { ser: Some(serializer), cid_serializer: None, cid: None }
    //    Self { tuple_ser, json_ser, is_cid: false }
    //}
    //
    //pub fn new_cid(tuple_ser: S, json_ser: U) -> Self {
    //    //Self { ser: None, cid_serializer: Some(CidSerializer(serializer)), cid: None }
    //    Self { tuple_ser, json_ser, is_cid: false }
    //}
    //pub fn new(json_ser: U,
    //    name: &'static str,
    //    variant_index: u32,
    //    variant: &'static str,
    //    len: usize,
    //
    //    ) -> Self {
    //    //Self { ser: Some(serializer), cid_serializer: None, cid: None }
    //    //let tuple_ser = json_ser.serialize_tuple_variant(name, variant_index, variant, len).unwrap();
    //    Self { json_ser, is_cid: false, tuple_ser: None }
    //}
    //
    //pub fn new_cid(json_ser: U,
    //    name: &'static str,
    //    variant_index: u32,
    //    variant: &'static str,
    //    len: usize,
    //    ) -> Self {
    //    //Self { ser: None, cid_serializer: Some(CidSerializer(serializer)), cid: None }
    //    //let tuple_ser = json_ser.serialize_tuple_variant(name, variant_index, variant, len).unwrap();
    //    Self { json_ser, is_cid: false, tuple_ser: None}
    //}
    pub fn new(tuple_serializer: S) -> Self {
        Self::TupleVariant(tuple_serializer)
    }
    pub fn new_cid(json_serializer: U) -> Self {
        Self::Cid(json_serializer)
    }
}


//impl<U> ser::SerializeTupleVariant for SerializeCid<U>
impl<S, U> ser::SerializeTupleVariant for SerializeCid<S, U>
where
    S: ser::SerializeTupleVariant,
    //for<'a> &'a mut U: ser::Serializer,
    U: ser::Serializer,
//    //S: ser::SerializeTupleVariant,
//    //S: ser::Serializer,
{
    //type Ok = <S as ser::Serialize>::SerializeTupleVariant::Ok;
    //type Error = <S as ser::Serialize>::SerializeTupleVariant::Error;
    //type Ok = ();
    //type Error = EncodeError;
    //type Ok = <S as ser::SerializeTupleVariant>::Ok;
    //type Error = <S as ser::SerializeTupleVariant>::Error;
    //type Ok = <<U as ser::Serializer>::SerializeTupleVariant as ser::SerializeTupleVariant>::Ok;
    //type Error = <<U as ser::Serializer>::SerializeTupleVariant as ser::SerializeTupleVariant>::Error;
    type Ok = S::Ok;
    type Error = S::Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        println!("vmx: ser: serialize tuple variant2");
        //if self.is_cid {
        //    //self.cid = Some(cid_serializer.serialize_bytes(value));
        //    //self.cid = Some(value.serialize(cid_serializer).unwrap());
        //    value.serialize(CidSerializer(self.json_ser)).unwrap();
        //    return Ok(())
        //} else {
        //    let tuple_ser = self.json_ser.serialize_tuple_variant("df", 0 , "da", 1).unwrap();
        //    //self.tuple_ser.serialize_field(&SerializeRef::new(value))
        //    tuple_ser.serialize_field(&SerializeRef::new(value))
        //}
        match self {
            Self::TupleVariant(serializer) => {
                serializer.serialize_field(&SerializeRef::new(value));
                Ok(())
            },
            Self::Cid(serializer) => {
                //let cid: Cid = value.serialize(serializer)?;
                //serializer.serialize(value);
                //serializer.serialize_field(&SerializeRef::new(value));



                // NOTE vmx 2024-02-02: commenting this line out makes things compile. But how can
                // I get this serialization work? The idea would then to store the result of the
                // deserialization temporarily within this impl and then return it on the `end()`
                // call.
                value.serialize(CidSerializer::new(serializer));



                //value.serialize(SerializeRef::new(value));
                //ser::Serialize::serialize(value, serializer);
                Ok(())
            }
        }
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        //if let Some(cid) = self.cid.take() {
        //    cid.into()
        //} else {
        //self.ser.as_mut().unwrap().end()
        //}
        match self {
            Self::TupleVariant(serializer) => {
                serializer.end()
            },
            Self::Cid(serializer) => {
                todo!();
            }
        }
    }
}

//pub struct SerializeCid<S, U>
//where
//    U: ser::Serializer,
//{
//    ser: Option<S>,
//    //is_cid: bool,
//    cid_serializer: Option<CidSerializer<U>>,
//    cid: Option<U::Ok>
//}
//
//impl<S, U> SerializeCid<S, U>
//where
//    U: ser::Serializer,
//{
//    pub fn new(serializer: S) -> Self {
//        Self { ser: Some(serializer), cid_serializer: None, cid: None }
//    }
//
//    pub fn new_cid(cid_serializer: CidSerializer<U>) -> Self {
//        Self { ser: None, cid_serializer: Some(cid_serializer), cid: None }
//    }
//}
//
//impl<S, U> ser::SerializeTupleVariant for SerializeCid<S, U>
//where
//    //S: ser::SerializeTupleVariant + ser::Serializer,
//    S: ser::SerializeTupleVariant,
//    U: ser::Serializer,
//{
//    type Ok = <S as ser::SerializeTupleVariant>::Ok;
//    type Error = <S as ser::SerializeTupleVariant>::Error;
//
//    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
//    where
//        T: ?Sized + ser::Serialize,
//    {
//        println!("vmx: ser: serialize tuple variant2");
//        if let Some(cid_serializer) = self.cid_serializer.take() {
//            //self.cid = Some(cid_serializer.serialize_bytes(value));
//            self.cid = Some(value.serialize(cid_serializer).unwrap());
//            return Ok(())
//        } else {
//            self.ser.as_mut().unwrap().serialize_field(&SerializeRef::new(value))
//        }
//    }
//
//    fn end(mut self) -> Result<Self::Ok, Self::Error> {
//        if let Some(cid) = self.cid.take() {
//            cid.into()
//        } else {
//        //self.ser.as_mut().unwrap().end()
//        todo!();
//        //self.ser.as_mut().unwrap().end()
//        }
//    }
//}
//


/// Serializing a CID correctly as DAG-JSON.
//struct CidSerializer<S>(S);
pub struct CidSerializer<S> {
//struct CidSerializer<'a, S> {
    //ser: &'a mut S
    ser: S
}

impl<S> CidSerializer<S> {
//impl<'a, S> CidSerializer<'a, S> {
    //pub fn new(ser: &'a mut S) -> Self {
    pub fn new(ser: S) -> Self {
        Self { ser }
    }
}

//impl<S> ser::SerializeTupleVariant for CidSerializer<S>
//where
//    S: ser::SerializeTupleVariant,
//{
//    type Ok = S::Ok;
//    type Error = S::Error;
//
//    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
//    where
//        T: ?Sized + ser::Serialize,
//    {
//        self.0.serialize_field(&SerializeRef::new(value))
//    }
//
//    fn end(self) -> Result<Self::Ok, Self::Error> {
//        self.0.end()
//    }
//}
//

//impl<'a, S> ser::Serializer for &mut CidSerializer<'a, S>
//impl<S> ser::Serializer for &mut CidSerializer<S>
impl<S> ser::Serializer for CidSerializer<S>
where
    S: ser::Serializer,
{
    type Ok = S::Ok;
    type Error = S::Error;

    type SerializeSeq = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeMap = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = ser::Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, _value: bool) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("unreachable"))
    }
    fn serialize_i8(self, _value: i8) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("unreachable"))
    }
    fn serialize_i16(self, _value: i16) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("unreachable"))
    }
    fn serialize_i32(self, _value: i32) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("unreachable"))
    }
    fn serialize_i64(self, _value: i64) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("unreachable"))
    }
    fn serialize_u8(self, _value: u8) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("unreachable"))
    }
    fn serialize_u16(self, _value: u16) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("unreachable"))
    }
    fn serialize_u32(self, _value: u32) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("unreachable"))
    }
    fn serialize_u64(self, _value: u64) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("unreachable"))
    }
    fn serialize_f32(self, _value: f32) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("unreachable"))
    }
    fn serialize_f64(self, _value: f64) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("unreachable"))
    }
    fn serialize_char(self, _value: char) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("unreachable"))
    }
    fn serialize_str(self, _value: &str) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("unreachable"))
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok, Self::Error> {
        let cid = Cid::try_from(value).map_err(|_| ser::Error::custom("Invalid CID"))?;
        let cid_json = ReservedKeyMap {
            _slash: ReservedKeyValue::Cid(cid.to_string()),
        };
        //SerializeSized::new(cid_json).serialize(self.0)
        SerializeSized::new(cid_json).serialize(self.ser)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("unreachable"))
    }
    fn serialize_some<T: ?Sized + ser::Serialize>(
        self,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("unreachable"))
    }
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("unreachable"))
    }
    fn serialize_unit_struct(self, _name: &str) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("unreachable"))
    }
    fn serialize_unit_variant(
        self,
        _name: &str,
        _variant_index: u32,
        _variant: &str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("unreachable"))
    }

    fn serialize_newtype_struct<T: ?Sized + ser::Serialize>(
        self,
        _name: &str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("unreachable"))
    }
    fn serialize_newtype_variant<T: ?Sized + ser::Serialize>(
        self,
        _name: &str,
        _variant_index: u32,
        _variant: &str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("unreachable"))
    }
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(ser::Error::custom("unreachable"))
    }
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(ser::Error::custom("unreachable"))
    }
    fn serialize_tuple_struct(
        self,
        _name: &str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(ser::Error::custom("unreachable"))
    }
    fn serialize_tuple_variant(
        self,
        _name: &str,
        _variant_index: u32,
        _variant: &str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(ser::Error::custom("unreachable"))
    }
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(ser::Error::custom("unreachable"))
    }
    fn serialize_struct(
        self,
        _name: &str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(ser::Error::custom("unreachable"))
    }
    fn serialize_struct_variant(
        self,
        _name: &str,
        _variant_index: u32,
        _variant: &str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(ser::Error::custom("unreachable"))
    }
}
