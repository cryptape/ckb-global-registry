// Generated by Molecule 0.7.5

use molecule::prelude::*;
#[derive(Clone)]
pub struct Bytes(molecule::bytes::Bytes);
impl ::core::fmt::LowerHex for Bytes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl ::core::fmt::Debug for Bytes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl ::core::fmt::Display for Bytes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        let raw_data = hex_string(&self.raw_data());
        write!(f, "{}(0x{})", Self::NAME, raw_data)
    }
}
impl ::core::default::Default for Bytes {
    fn default() -> Self {
        let v = molecule::bytes::Bytes::from_static(&Self::DEFAULT_VALUE);
        Bytes::new_unchecked(v)
    }
}
impl Bytes {
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
    pub fn as_reader<'r>(&'r self) -> BytesReader<'r> {
        BytesReader::new_unchecked(self.as_slice())
    }
}
impl molecule::prelude::Entity for Bytes {
    type Builder = BytesBuilder;
    const NAME: &'static str = "Bytes";
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        Bytes(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        BytesReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn from_compatible_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        BytesReader::from_compatible_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::core::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder().extend(self.into_iter())
    }
}
#[derive(Clone, Copy)]
pub struct BytesReader<'r>(&'r [u8]);
impl<'r> ::core::fmt::LowerHex for BytesReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl<'r> ::core::fmt::Debug for BytesReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl<'r> ::core::fmt::Display for BytesReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        let raw_data = hex_string(&self.raw_data());
        write!(f, "{}(0x{})", Self::NAME, raw_data)
    }
}
impl<'r> BytesReader<'r> {
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
impl<'r> molecule::prelude::Reader<'r> for BytesReader<'r> {
    type Entity = Bytes;
    const NAME: &'static str = "BytesReader";
    fn to_entity(&self) -> Self::Entity {
        Self::Entity::new_unchecked(self.as_slice().to_owned().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        BytesReader(slice)
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
pub struct BytesBuilder(pub(crate) Vec<Byte>);
impl BytesBuilder {
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
impl molecule::prelude::Builder for BytesBuilder {
    type Entity = Bytes;
    const NAME: &'static str = "BytesBuilder";
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
        Bytes::new_unchecked(inner.into())
    }
}
pub struct BytesIterator(Bytes, usize, usize);
impl ::core::iter::Iterator for BytesIterator {
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
impl ::core::iter::ExactSizeIterator for BytesIterator {
    fn len(&self) -> usize {
        self.2 - self.1
    }
}
impl ::core::iter::IntoIterator for Bytes {
    type Item = Byte;
    type IntoIter = BytesIterator;
    fn into_iter(self) -> Self::IntoIter {
        let len = self.len();
        BytesIterator(self, 0, len)
    }
}
#[derive(Clone)]
pub struct DemoData(molecule::bytes::Bytes);
impl ::core::fmt::LowerHex for DemoData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl ::core::fmt::Debug for DemoData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl ::core::fmt::Display for DemoData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "demo", self.demo())?;
        write!(f, ", {}: {}", "curr", self.curr())?;
        write!(f, ", {}: {}", "next", self.next())?;
        let extra_count = self.count_extra_fields();
        if extra_count != 0 {
            write!(f, ", .. ({} fields)", extra_count)?;
        }
        write!(f, " }}")
    }
}
impl ::core::default::Default for DemoData {
    fn default() -> Self {
        let v = molecule::bytes::Bytes::from_static(&Self::DEFAULT_VALUE);
        DemoData::new_unchecked(v)
    }
}
impl DemoData {
    const DEFAULT_VALUE: [u8; 28] = [
        28, 0, 0, 0, 16, 0, 0, 0, 20, 0, 0, 0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
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
    pub fn demo(&self) -> Bytes {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[4..]) as usize;
        let end = molecule::unpack_number(&slice[8..]) as usize;
        Bytes::new_unchecked(self.0.slice(start..end))
    }
    pub fn curr(&self) -> Bytes {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[8..]) as usize;
        let end = molecule::unpack_number(&slice[12..]) as usize;
        Bytes::new_unchecked(self.0.slice(start..end))
    }
    pub fn next(&self) -> Bytes {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[12..]) as usize;
        if self.has_extra_fields() {
            let end = molecule::unpack_number(&slice[16..]) as usize;
            Bytes::new_unchecked(self.0.slice(start..end))
        } else {
            Bytes::new_unchecked(self.0.slice(start..))
        }
    }
    pub fn as_reader<'r>(&'r self) -> DemoDataReader<'r> {
        DemoDataReader::new_unchecked(self.as_slice())
    }
}
impl molecule::prelude::Entity for DemoData {
    type Builder = DemoDataBuilder;
    const NAME: &'static str = "DemoData";
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        DemoData(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        DemoDataReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn from_compatible_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        DemoDataReader::from_compatible_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::core::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .demo(self.demo())
            .curr(self.curr())
            .next(self.next())
    }
}
#[derive(Clone, Copy)]
pub struct DemoDataReader<'r>(&'r [u8]);
impl<'r> ::core::fmt::LowerHex for DemoDataReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl<'r> ::core::fmt::Debug for DemoDataReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl<'r> ::core::fmt::Display for DemoDataReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "demo", self.demo())?;
        write!(f, ", {}: {}", "curr", self.curr())?;
        write!(f, ", {}: {}", "next", self.next())?;
        let extra_count = self.count_extra_fields();
        if extra_count != 0 {
            write!(f, ", .. ({} fields)", extra_count)?;
        }
        write!(f, " }}")
    }
}
impl<'r> DemoDataReader<'r> {
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
    pub fn demo(&self) -> BytesReader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[4..]) as usize;
        let end = molecule::unpack_number(&slice[8..]) as usize;
        BytesReader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn curr(&self) -> BytesReader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[8..]) as usize;
        let end = molecule::unpack_number(&slice[12..]) as usize;
        BytesReader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn next(&self) -> BytesReader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[12..]) as usize;
        if self.has_extra_fields() {
            let end = molecule::unpack_number(&slice[16..]) as usize;
            BytesReader::new_unchecked(&self.as_slice()[start..end])
        } else {
            BytesReader::new_unchecked(&self.as_slice()[start..])
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for DemoDataReader<'r> {
    type Entity = DemoData;
    const NAME: &'static str = "DemoDataReader";
    fn to_entity(&self) -> Self::Entity {
        Self::Entity::new_unchecked(self.as_slice().to_owned().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        DemoDataReader(slice)
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
        BytesReader::verify(&slice[offsets[0]..offsets[1]], compatible)?;
        BytesReader::verify(&slice[offsets[1]..offsets[2]], compatible)?;
        BytesReader::verify(&slice[offsets[2]..offsets[3]], compatible)?;
        Ok(())
    }
}
#[derive(Debug, Default)]
pub struct DemoDataBuilder {
    pub(crate) demo: Bytes,
    pub(crate) curr: Bytes,
    pub(crate) next: Bytes,
}
impl DemoDataBuilder {
    pub const FIELD_COUNT: usize = 3;
    pub fn demo(mut self, v: Bytes) -> Self {
        self.demo = v;
        self
    }
    pub fn curr(mut self, v: Bytes) -> Self {
        self.curr = v;
        self
    }
    pub fn next(mut self, v: Bytes) -> Self {
        self.next = v;
        self
    }
}
impl molecule::prelude::Builder for DemoDataBuilder {
    type Entity = DemoData;
    const NAME: &'static str = "DemoDataBuilder";
    fn expected_length(&self) -> usize {
        molecule::NUMBER_SIZE * (Self::FIELD_COUNT + 1)
            + self.demo.as_slice().len()
            + self.curr.as_slice().len()
            + self.next.as_slice().len()
    }
    fn write<W: molecule::io::Write>(&self, writer: &mut W) -> molecule::io::Result<()> {
        let mut total_size = molecule::NUMBER_SIZE * (Self::FIELD_COUNT + 1);
        let mut offsets = Vec::with_capacity(Self::FIELD_COUNT);
        offsets.push(total_size);
        total_size += self.demo.as_slice().len();
        offsets.push(total_size);
        total_size += self.curr.as_slice().len();
        offsets.push(total_size);
        total_size += self.next.as_slice().len();
        writer.write_all(&molecule::pack_number(total_size as molecule::Number))?;
        for offset in offsets.into_iter() {
            writer.write_all(&molecule::pack_number(offset as molecule::Number))?;
        }
        writer.write_all(self.demo.as_slice())?;
        writer.write_all(self.curr.as_slice())?;
        writer.write_all(self.next.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner)
            .unwrap_or_else(|_| panic!("{} build should be ok", Self::NAME));
        DemoData::new_unchecked(inner.into())
    }
}