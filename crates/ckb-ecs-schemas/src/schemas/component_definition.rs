// Generated by Molecule 0.7.5

use super::blockchain::*;
use molecule::prelude::*;
#[derive(Clone)]
pub struct String(molecule::bytes::Bytes);
impl ::core::fmt::LowerHex for String {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl ::core::fmt::Debug for String {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl ::core::fmt::Display for String {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        let raw_data = hex_string(&self.raw_data());
        write!(f, "{}(0x{})", Self::NAME, raw_data)
    }
}
impl ::core::default::Default for String {
    fn default() -> Self {
        let v = molecule::bytes::Bytes::from_static(&Self::DEFAULT_VALUE);
        String::new_unchecked(v)
    }
}
impl String {
    const DEFAULT_VALUE: [u8; 4] = [0, 0, 0, 0];
    pub const ITEM_SIZE: usize = 1;
    pub fn total_size(&self) -> usize {
        molecule::NUMBER_SIZE + Self::ITEM_SIZE * self.item_count()
    }
    pub fn item_count(&self) -> usize {
        molecule::unpack_number(self.as_slice()) as usize
    }
    pub fn len(&self) -> usize {
        self.item_count()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn get(&self, idx: usize) -> Option<Byte> {
        if idx >= self.len() {
            None
        } else {
            Some(self.get_unchecked(idx))
        }
    }
    pub fn get_unchecked(&self, idx: usize) -> Byte {
        let start = molecule::NUMBER_SIZE + Self::ITEM_SIZE * idx;
        let end = start + Self::ITEM_SIZE;
        Byte::new_unchecked(self.0.slice(start..end))
    }
    pub fn raw_data(&self) -> molecule::bytes::Bytes {
        self.0.slice(molecule::NUMBER_SIZE..)
    }
    pub fn as_reader<'r>(&'r self) -> StringReader<'r> {
        StringReader::new_unchecked(self.as_slice())
    }
}
impl molecule::prelude::Entity for String {
    type Builder = StringBuilder;
    const NAME: &'static str = "String";
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        String(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        StringReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn from_compatible_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        StringReader::from_compatible_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::core::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder().extend(self.into_iter())
    }
}
#[derive(Clone, Copy)]
pub struct StringReader<'r>(&'r [u8]);
impl<'r> ::core::fmt::LowerHex for StringReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl<'r> ::core::fmt::Debug for StringReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl<'r> ::core::fmt::Display for StringReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        let raw_data = hex_string(&self.raw_data());
        write!(f, "{}(0x{})", Self::NAME, raw_data)
    }
}
impl<'r> StringReader<'r> {
    pub const ITEM_SIZE: usize = 1;
    pub fn total_size(&self) -> usize {
        molecule::NUMBER_SIZE + Self::ITEM_SIZE * self.item_count()
    }
    pub fn item_count(&self) -> usize {
        molecule::unpack_number(self.as_slice()) as usize
    }
    pub fn len(&self) -> usize {
        self.item_count()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn get(&self, idx: usize) -> Option<ByteReader<'r>> {
        if idx >= self.len() {
            None
        } else {
            Some(self.get_unchecked(idx))
        }
    }
    pub fn get_unchecked(&self, idx: usize) -> ByteReader<'r> {
        let start = molecule::NUMBER_SIZE + Self::ITEM_SIZE * idx;
        let end = start + Self::ITEM_SIZE;
        ByteReader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn raw_data(&self) -> &'r [u8] {
        &self.as_slice()[molecule::NUMBER_SIZE..]
    }
}
impl<'r> molecule::prelude::Reader<'r> for StringReader<'r> {
    type Entity = String;
    const NAME: &'static str = "StringReader";
    fn to_entity(&self) -> Self::Entity {
        Self::Entity::new_unchecked(self.as_slice().to_owned().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        StringReader(slice)
    }
    fn as_slice(&self) -> &'r [u8] {
        self.0
    }
    fn verify(slice: &[u8], _compatible: bool) -> molecule::error::VerificationResult<()> {
        use molecule::verification_error as ve;
        let slice_len = slice.len();
        if slice_len < molecule::NUMBER_SIZE {
            return ve!(Self, HeaderIsBroken, molecule::NUMBER_SIZE, slice_len);
        }
        let item_count = molecule::unpack_number(slice) as usize;
        if item_count == 0 {
            if slice_len != molecule::NUMBER_SIZE {
                return ve!(Self, TotalSizeNotMatch, molecule::NUMBER_SIZE, slice_len);
            }
            return Ok(());
        }
        let total_size = molecule::NUMBER_SIZE + Self::ITEM_SIZE * item_count;
        if slice_len != total_size {
            return ve!(Self, TotalSizeNotMatch, total_size, slice_len);
        }
        Ok(())
    }
}
#[derive(Debug, Default)]
pub struct StringBuilder(pub(crate) Vec<Byte>);
impl StringBuilder {
    pub const ITEM_SIZE: usize = 1;
    pub fn set(mut self, v: Vec<Byte>) -> Self {
        self.0 = v;
        self
    }
    pub fn push(mut self, v: Byte) -> Self {
        self.0.push(v);
        self
    }
    pub fn extend<T: ::core::iter::IntoIterator<Item = Byte>>(mut self, iter: T) -> Self {
        for elem in iter {
            self.0.push(elem);
        }
        self
    }
    pub fn replace(&mut self, index: usize, v: Byte) -> Option<Byte> {
        self.0
            .get_mut(index)
            .map(|item| ::core::mem::replace(item, v))
    }
}
impl molecule::prelude::Builder for StringBuilder {
    type Entity = String;
    const NAME: &'static str = "StringBuilder";
    fn expected_length(&self) -> usize {
        molecule::NUMBER_SIZE + Self::ITEM_SIZE * self.0.len()
    }
    fn write<W: molecule::io::Write>(&self, writer: &mut W) -> molecule::io::Result<()> {
        writer.write_all(&molecule::pack_number(self.0.len() as molecule::Number))?;
        for inner in &self.0[..] {
            writer.write_all(inner.as_slice())?;
        }
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner)
            .unwrap_or_else(|_| panic!("{} build should be ok", Self::NAME));
        String::new_unchecked(inner.into())
    }
}
pub struct StringIterator(String, usize, usize);
impl ::core::iter::Iterator for StringIterator {
    type Item = Byte;
    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= self.2 {
            None
        } else {
            let ret = self.0.get_unchecked(self.1);
            self.1 += 1;
            Some(ret)
        }
    }
}
impl ::core::iter::ExactSizeIterator for StringIterator {
    fn len(&self) -> usize {
        self.2 - self.1
    }
}
impl ::core::iter::IntoIterator for String {
    type Item = Byte;
    type IntoIter = StringIterator;
    fn into_iter(self) -> Self::IntoIter {
        let len = self.len();
        StringIterator(self, 0, len)
    }
}
#[derive(Clone)]
pub struct ComponentDefinitionV1(molecule::bytes::Bytes);
impl ::core::fmt::LowerHex for ComponentDefinitionV1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl ::core::fmt::Debug for ComponentDefinitionV1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl ::core::fmt::Display for ComponentDefinitionV1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "component_name", self.component_name())?;
        write!(f, ", {}: {}", "info_hash", self.info_hash())?;
        write!(f, ", {}: {}", "delegate", self.delegate())?;
        let extra_count = self.count_extra_fields();
        if extra_count != 0 {
            write!(f, ", .. ({} fields)", extra_count)?;
        }
        write!(f, " }}")
    }
}
impl ::core::default::Default for ComponentDefinitionV1 {
    fn default() -> Self {
        let v = molecule::bytes::Bytes::from_static(&Self::DEFAULT_VALUE);
        ComponentDefinitionV1::new_unchecked(v)
    }
}
impl ComponentDefinitionV1 {
    const DEFAULT_VALUE: [u8; 105] = [
        105, 0, 0, 0, 16, 0, 0, 0, 20, 0, 0, 0, 52, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 53, 0, 0, 0, 16, 0, 0,
        0, 48, 0, 0, 0, 49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    pub const FIELD_COUNT: usize = 3;
    pub fn total_size(&self) -> usize {
        molecule::unpack_number(self.as_slice()) as usize
    }
    pub fn field_count(&self) -> usize {
        if self.total_size() == molecule::NUMBER_SIZE {
            0
        } else {
            (molecule::unpack_number(&self.as_slice()[molecule::NUMBER_SIZE..]) as usize / 4) - 1
        }
    }
    pub fn count_extra_fields(&self) -> usize {
        self.field_count() - Self::FIELD_COUNT
    }
    pub fn has_extra_fields(&self) -> bool {
        Self::FIELD_COUNT != self.field_count()
    }
    pub fn component_name(&self) -> String {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[4..]) as usize;
        let end = molecule::unpack_number(&slice[8..]) as usize;
        String::new_unchecked(self.0.slice(start..end))
    }
    pub fn info_hash(&self) -> Byte32 {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[8..]) as usize;
        let end = molecule::unpack_number(&slice[12..]) as usize;
        Byte32::new_unchecked(self.0.slice(start..end))
    }
    pub fn delegate(&self) -> Script {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[12..]) as usize;
        if self.has_extra_fields() {
            let end = molecule::unpack_number(&slice[16..]) as usize;
            Script::new_unchecked(self.0.slice(start..end))
        } else {
            Script::new_unchecked(self.0.slice(start..))
        }
    }
    pub fn as_reader<'r>(&'r self) -> ComponentDefinitionV1Reader<'r> {
        ComponentDefinitionV1Reader::new_unchecked(self.as_slice())
    }
}
impl molecule::prelude::Entity for ComponentDefinitionV1 {
    type Builder = ComponentDefinitionV1Builder;
    const NAME: &'static str = "ComponentDefinitionV1";
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        ComponentDefinitionV1(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        ComponentDefinitionV1Reader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn from_compatible_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        ComponentDefinitionV1Reader::from_compatible_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::core::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .component_name(self.component_name())
            .info_hash(self.info_hash())
            .delegate(self.delegate())
    }
}
#[derive(Clone, Copy)]
pub struct ComponentDefinitionV1Reader<'r>(&'r [u8]);
impl<'r> ::core::fmt::LowerHex for ComponentDefinitionV1Reader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl<'r> ::core::fmt::Debug for ComponentDefinitionV1Reader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl<'r> ::core::fmt::Display for ComponentDefinitionV1Reader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "component_name", self.component_name())?;
        write!(f, ", {}: {}", "info_hash", self.info_hash())?;
        write!(f, ", {}: {}", "delegate", self.delegate())?;
        let extra_count = self.count_extra_fields();
        if extra_count != 0 {
            write!(f, ", .. ({} fields)", extra_count)?;
        }
        write!(f, " }}")
    }
}
impl<'r> ComponentDefinitionV1Reader<'r> {
    pub const FIELD_COUNT: usize = 3;
    pub fn total_size(&self) -> usize {
        molecule::unpack_number(self.as_slice()) as usize
    }
    pub fn field_count(&self) -> usize {
        if self.total_size() == molecule::NUMBER_SIZE {
            0
        } else {
            (molecule::unpack_number(&self.as_slice()[molecule::NUMBER_SIZE..]) as usize / 4) - 1
        }
    }
    pub fn count_extra_fields(&self) -> usize {
        self.field_count() - Self::FIELD_COUNT
    }
    pub fn has_extra_fields(&self) -> bool {
        Self::FIELD_COUNT != self.field_count()
    }
    pub fn component_name(&self) -> StringReader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[4..]) as usize;
        let end = molecule::unpack_number(&slice[8..]) as usize;
        StringReader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn info_hash(&self) -> Byte32Reader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[8..]) as usize;
        let end = molecule::unpack_number(&slice[12..]) as usize;
        Byte32Reader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn delegate(&self) -> ScriptReader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[12..]) as usize;
        if self.has_extra_fields() {
            let end = molecule::unpack_number(&slice[16..]) as usize;
            ScriptReader::new_unchecked(&self.as_slice()[start..end])
        } else {
            ScriptReader::new_unchecked(&self.as_slice()[start..])
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for ComponentDefinitionV1Reader<'r> {
    type Entity = ComponentDefinitionV1;
    const NAME: &'static str = "ComponentDefinitionV1Reader";
    fn to_entity(&self) -> Self::Entity {
        Self::Entity::new_unchecked(self.as_slice().to_owned().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        ComponentDefinitionV1Reader(slice)
    }
    fn as_slice(&self) -> &'r [u8] {
        self.0
    }
    fn verify(slice: &[u8], compatible: bool) -> molecule::error::VerificationResult<()> {
        use molecule::verification_error as ve;
        let slice_len = slice.len();
        if slice_len < molecule::NUMBER_SIZE {
            return ve!(Self, HeaderIsBroken, molecule::NUMBER_SIZE, slice_len);
        }
        let total_size = molecule::unpack_number(slice) as usize;
        if slice_len != total_size {
            return ve!(Self, TotalSizeNotMatch, total_size, slice_len);
        }
        if slice_len < molecule::NUMBER_SIZE * 2 {
            return ve!(Self, HeaderIsBroken, molecule::NUMBER_SIZE * 2, slice_len);
        }
        let offset_first = molecule::unpack_number(&slice[molecule::NUMBER_SIZE..]) as usize;
        if offset_first % molecule::NUMBER_SIZE != 0 || offset_first < molecule::NUMBER_SIZE * 2 {
            return ve!(Self, OffsetsNotMatch);
        }
        if slice_len < offset_first {
            return ve!(Self, HeaderIsBroken, offset_first, slice_len);
        }
        let field_count = offset_first / molecule::NUMBER_SIZE - 1;
        if field_count < Self::FIELD_COUNT {
            return ve!(Self, FieldCountNotMatch, Self::FIELD_COUNT, field_count);
        } else if !compatible && field_count > Self::FIELD_COUNT {
            return ve!(Self, FieldCountNotMatch, Self::FIELD_COUNT, field_count);
        };
        let mut offsets: Vec<usize> = slice[molecule::NUMBER_SIZE..offset_first]
            .chunks_exact(molecule::NUMBER_SIZE)
            .map(|x| molecule::unpack_number(x) as usize)
            .collect();
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            return ve!(Self, OffsetsNotMatch);
        }
        StringReader::verify(&slice[offsets[0]..offsets[1]], compatible)?;
        Byte32Reader::verify(&slice[offsets[1]..offsets[2]], compatible)?;
        ScriptReader::verify(&slice[offsets[2]..offsets[3]], compatible)?;
        Ok(())
    }
}
#[derive(Debug, Default)]
pub struct ComponentDefinitionV1Builder {
    pub(crate) component_name: String,
    pub(crate) info_hash: Byte32,
    pub(crate) delegate: Script,
}
impl ComponentDefinitionV1Builder {
    pub const FIELD_COUNT: usize = 3;
    pub fn component_name(mut self, v: String) -> Self {
        self.component_name = v;
        self
    }
    pub fn info_hash(mut self, v: Byte32) -> Self {
        self.info_hash = v;
        self
    }
    pub fn delegate(mut self, v: Script) -> Self {
        self.delegate = v;
        self
    }
}
impl molecule::prelude::Builder for ComponentDefinitionV1Builder {
    type Entity = ComponentDefinitionV1;
    const NAME: &'static str = "ComponentDefinitionV1Builder";
    fn expected_length(&self) -> usize {
        molecule::NUMBER_SIZE * (Self::FIELD_COUNT + 1)
            + self.component_name.as_slice().len()
            + self.info_hash.as_slice().len()
            + self.delegate.as_slice().len()
    }
    fn write<W: molecule::io::Write>(&self, writer: &mut W) -> molecule::io::Result<()> {
        let mut total_size = molecule::NUMBER_SIZE * (Self::FIELD_COUNT + 1);
        let mut offsets = Vec::with_capacity(Self::FIELD_COUNT);
        offsets.push(total_size);
        total_size += self.component_name.as_slice().len();
        offsets.push(total_size);
        total_size += self.info_hash.as_slice().len();
        offsets.push(total_size);
        total_size += self.delegate.as_slice().len();
        writer.write_all(&molecule::pack_number(total_size as molecule::Number))?;
        for offset in offsets.into_iter() {
            writer.write_all(&molecule::pack_number(offset as molecule::Number))?;
        }
        writer.write_all(self.component_name.as_slice())?;
        writer.write_all(self.info_hash.as_slice())?;
        writer.write_all(self.delegate.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner)
            .unwrap_or_else(|_| panic!("{} build should be ok", Self::NAME));
        ComponentDefinitionV1::new_unchecked(inner.into())
    }
}
#[derive(Clone)]
pub struct ComponentDefinition(molecule::bytes::Bytes);
impl ::core::fmt::LowerHex for ComponentDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl ::core::fmt::Debug for ComponentDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl ::core::fmt::Display for ComponentDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}(", Self::NAME)?;
        self.to_enum().display_inner(f)?;
        write!(f, ")")
    }
}
impl ::core::default::Default for ComponentDefinition {
    fn default() -> Self {
        let v = molecule::bytes::Bytes::from_static(&Self::DEFAULT_VALUE);
        ComponentDefinition::new_unchecked(v)
    }
}
impl ComponentDefinition {
    const DEFAULT_VALUE: [u8; 109] = [
        0, 0, 0, 0, 105, 0, 0, 0, 16, 0, 0, 0, 20, 0, 0, 0, 52, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 53, 0, 0,
        0, 16, 0, 0, 0, 48, 0, 0, 0, 49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    pub const ITEMS_COUNT: usize = 1;
    pub fn item_id(&self) -> molecule::Number {
        molecule::unpack_number(self.as_slice())
    }
    pub fn to_enum(&self) -> ComponentDefinitionUnion {
        let inner = self.0.slice(molecule::NUMBER_SIZE..);
        match self.item_id() {
            0 => ComponentDefinitionV1::new_unchecked(inner).into(),
            _ => panic!("{}: invalid data", Self::NAME),
        }
    }
    pub fn as_reader<'r>(&'r self) -> ComponentDefinitionReader<'r> {
        ComponentDefinitionReader::new_unchecked(self.as_slice())
    }
}
impl molecule::prelude::Entity for ComponentDefinition {
    type Builder = ComponentDefinitionBuilder;
    const NAME: &'static str = "ComponentDefinition";
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        ComponentDefinition(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        ComponentDefinitionReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn from_compatible_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        ComponentDefinitionReader::from_compatible_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::core::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder().set(self.to_enum())
    }
}
#[derive(Clone, Copy)]
pub struct ComponentDefinitionReader<'r>(&'r [u8]);
impl<'r> ::core::fmt::LowerHex for ComponentDefinitionReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl<'r> ::core::fmt::Debug for ComponentDefinitionReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl<'r> ::core::fmt::Display for ComponentDefinitionReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}(", Self::NAME)?;
        self.to_enum().display_inner(f)?;
        write!(f, ")")
    }
}
impl<'r> ComponentDefinitionReader<'r> {
    pub const ITEMS_COUNT: usize = 1;
    pub fn item_id(&self) -> molecule::Number {
        molecule::unpack_number(self.as_slice())
    }
    pub fn to_enum(&self) -> ComponentDefinitionUnionReader<'r> {
        let inner = &self.as_slice()[molecule::NUMBER_SIZE..];
        match self.item_id() {
            0 => ComponentDefinitionV1Reader::new_unchecked(inner).into(),
            _ => panic!("{}: invalid data", Self::NAME),
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for ComponentDefinitionReader<'r> {
    type Entity = ComponentDefinition;
    const NAME: &'static str = "ComponentDefinitionReader";
    fn to_entity(&self) -> Self::Entity {
        Self::Entity::new_unchecked(self.as_slice().to_owned().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        ComponentDefinitionReader(slice)
    }
    fn as_slice(&self) -> &'r [u8] {
        self.0
    }
    fn verify(slice: &[u8], compatible: bool) -> molecule::error::VerificationResult<()> {
        use molecule::verification_error as ve;
        let slice_len = slice.len();
        if slice_len < molecule::NUMBER_SIZE {
            return ve!(Self, HeaderIsBroken, molecule::NUMBER_SIZE, slice_len);
        }
        let item_id = molecule::unpack_number(slice);
        let inner_slice = &slice[molecule::NUMBER_SIZE..];
        match item_id {
            0 => ComponentDefinitionV1Reader::verify(inner_slice, compatible),
            _ => ve!(Self, UnknownItem, Self::ITEMS_COUNT, item_id),
        }?;
        Ok(())
    }
}
#[derive(Debug, Default)]
pub struct ComponentDefinitionBuilder(pub(crate) ComponentDefinitionUnion);
impl ComponentDefinitionBuilder {
    pub const ITEMS_COUNT: usize = 1;
    pub fn set<I>(mut self, v: I) -> Self
    where
        I: ::core::convert::Into<ComponentDefinitionUnion>,
    {
        self.0 = v.into();
        self
    }
}
impl molecule::prelude::Builder for ComponentDefinitionBuilder {
    type Entity = ComponentDefinition;
    const NAME: &'static str = "ComponentDefinitionBuilder";
    fn expected_length(&self) -> usize {
        molecule::NUMBER_SIZE + self.0.as_slice().len()
    }
    fn write<W: molecule::io::Write>(&self, writer: &mut W) -> molecule::io::Result<()> {
        writer.write_all(&molecule::pack_number(self.0.item_id()))?;
        writer.write_all(self.0.as_slice())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner)
            .unwrap_or_else(|_| panic!("{} build should be ok", Self::NAME));
        ComponentDefinition::new_unchecked(inner.into())
    }
}
#[derive(Debug, Clone)]
pub enum ComponentDefinitionUnion {
    ComponentDefinitionV1(ComponentDefinitionV1),
}
#[derive(Debug, Clone, Copy)]
pub enum ComponentDefinitionUnionReader<'r> {
    ComponentDefinitionV1(ComponentDefinitionV1Reader<'r>),
}
impl ::core::default::Default for ComponentDefinitionUnion {
    fn default() -> Self {
        ComponentDefinitionUnion::ComponentDefinitionV1(::core::default::Default::default())
    }
}
impl ::core::fmt::Display for ComponentDefinitionUnion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            ComponentDefinitionUnion::ComponentDefinitionV1(ref item) => {
                write!(
                    f,
                    "{}::{}({})",
                    Self::NAME,
                    ComponentDefinitionV1::NAME,
                    item
                )
            }
        }
    }
}
impl<'r> ::core::fmt::Display for ComponentDefinitionUnionReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            ComponentDefinitionUnionReader::ComponentDefinitionV1(ref item) => {
                write!(
                    f,
                    "{}::{}({})",
                    Self::NAME,
                    ComponentDefinitionV1::NAME,
                    item
                )
            }
        }
    }
}
impl ComponentDefinitionUnion {
    pub(crate) fn display_inner(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            ComponentDefinitionUnion::ComponentDefinitionV1(ref item) => write!(f, "{}", item),
        }
    }
}
impl<'r> ComponentDefinitionUnionReader<'r> {
    pub(crate) fn display_inner(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            ComponentDefinitionUnionReader::ComponentDefinitionV1(ref item) => {
                write!(f, "{}", item)
            }
        }
    }
}
impl ::core::convert::From<ComponentDefinitionV1> for ComponentDefinitionUnion {
    fn from(item: ComponentDefinitionV1) -> Self {
        ComponentDefinitionUnion::ComponentDefinitionV1(item)
    }
}
impl<'r> ::core::convert::From<ComponentDefinitionV1Reader<'r>>
    for ComponentDefinitionUnionReader<'r>
{
    fn from(item: ComponentDefinitionV1Reader<'r>) -> Self {
        ComponentDefinitionUnionReader::ComponentDefinitionV1(item)
    }
}
impl ComponentDefinitionUnion {
    pub const NAME: &'static str = "ComponentDefinitionUnion";
    pub fn as_bytes(&self) -> molecule::bytes::Bytes {
        match self {
            ComponentDefinitionUnion::ComponentDefinitionV1(item) => item.as_bytes(),
        }
    }
    pub fn as_slice(&self) -> &[u8] {
        match self {
            ComponentDefinitionUnion::ComponentDefinitionV1(item) => item.as_slice(),
        }
    }
    pub fn item_id(&self) -> molecule::Number {
        match self {
            ComponentDefinitionUnion::ComponentDefinitionV1(_) => 0,
        }
    }
    pub fn item_name(&self) -> &str {
        match self {
            ComponentDefinitionUnion::ComponentDefinitionV1(_) => "ComponentDefinitionV1",
        }
    }
    pub fn as_reader<'r>(&'r self) -> ComponentDefinitionUnionReader<'r> {
        match self {
            ComponentDefinitionUnion::ComponentDefinitionV1(item) => item.as_reader().into(),
        }
    }
}
impl<'r> ComponentDefinitionUnionReader<'r> {
    pub const NAME: &'r str = "ComponentDefinitionUnionReader";
    pub fn as_slice(&self) -> &'r [u8] {
        match self {
            ComponentDefinitionUnionReader::ComponentDefinitionV1(item) => item.as_slice(),
        }
    }
    pub fn item_id(&self) -> molecule::Number {
        match self {
            ComponentDefinitionUnionReader::ComponentDefinitionV1(_) => 0,
        }
    }
    pub fn item_name(&self) -> &str {
        match self {
            ComponentDefinitionUnionReader::ComponentDefinitionV1(_) => "ComponentDefinitionV1",
        }
    }
}
