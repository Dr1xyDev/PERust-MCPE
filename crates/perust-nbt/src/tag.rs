use indexmap::IndexMap;

/// NBT Tag types as used in Minecraft Bedrock Edition.
#[derive(Debug, Clone, PartialEq)]
pub enum Tag {
    End,
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<i8>),
    String(String),
    List(Vec<Tag>),
    Compound(IndexMap<String, Tag>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

/// Tag type IDs matching Minecraft NBT format.
pub const TAG_END: u8 = 0;
pub const TAG_BYTE: u8 = 1;
pub const TAG_SHORT: u8 = 2;
pub const TAG_INT: u8 = 3;
pub const TAG_LONG: u8 = 4;
pub const TAG_FLOAT: u8 = 5;
pub const TAG_DOUBLE: u8 = 6;
pub const TAG_BYTE_ARRAY: u8 = 7;
pub const TAG_STRING: u8 = 8;
pub const TAG_LIST: u8 = 9;
pub const TAG_COMPOUND: u8 = 10;
pub const TAG_INT_ARRAY: u8 = 11;
pub const TAG_LONG_ARRAY: u8 = 12;

impl Tag {
    /// Returns the NBT tag type ID for this tag.
    pub fn tag_type(&self) -> u8 {
        match self {
            Tag::End => TAG_END,
            Tag::Byte(_) => TAG_BYTE,
            Tag::Short(_) => TAG_SHORT,
            Tag::Int(_) => TAG_INT,
            Tag::Long(_) => TAG_LONG,
            Tag::Float(_) => TAG_FLOAT,
            Tag::Double(_) => TAG_DOUBLE,
            Tag::ByteArray(_) => TAG_BYTE_ARRAY,
            Tag::String(_) => TAG_STRING,
            Tag::List(_) => TAG_LIST,
            Tag::Compound(_) => TAG_COMPOUND,
            Tag::IntArray(_) => TAG_INT_ARRAY,
            Tag::LongArray(_) => TAG_LONG_ARRAY,
        }
    }

    // === Get accessor methods (return owned value or error) ===

    pub fn get_byte(&self) -> Result<i8, crate::error::NbtError> {
        match self {
            Tag::Byte(v) => Ok(*v),
            _ => Err(crate::error::NbtError::UnexpectedTag),
        }
    }

    pub fn get_short(&self) -> Result<i16, crate::error::NbtError> {
        match self {
            Tag::Short(v) => Ok(*v),
            _ => Err(crate::error::NbtError::UnexpectedTag),
        }
    }

    pub fn get_int(&self) -> Result<i32, crate::error::NbtError> {
        match self {
            Tag::Int(v) => Ok(*v),
            _ => Err(crate::error::NbtError::UnexpectedTag),
        }
    }

    pub fn get_long(&self) -> Result<i64, crate::error::NbtError> {
        match self {
            Tag::Long(v) => Ok(*v),
            _ => Err(crate::error::NbtError::UnexpectedTag),
        }
    }

    pub fn get_float(&self) -> Result<f32, crate::error::NbtError> {
        match self {
            Tag::Float(v) => Ok(*v),
            _ => Err(crate::error::NbtError::UnexpectedTag),
        }
    }

    pub fn get_double(&self) -> Result<f64, crate::error::NbtError> {
        match self {
            Tag::Double(v) => Ok(*v),
            _ => Err(crate::error::NbtError::UnexpectedTag),
        }
    }

    pub fn get_string(&self) -> Result<&str, crate::error::NbtError> {
        match self {
            Tag::String(v) => Ok(v),
            _ => Err(crate::error::NbtError::UnexpectedTag),
        }
    }

    pub fn get_byte_array(&self) -> Result<&Vec<i8>, crate::error::NbtError> {
        match self {
            Tag::ByteArray(v) => Ok(v),
            _ => Err(crate::error::NbtError::UnexpectedTag),
        }
    }

    pub fn get_int_array(&self) -> Result<&Vec<i32>, crate::error::NbtError> {
        match self {
            Tag::IntArray(v) => Ok(v),
            _ => Err(crate::error::NbtError::UnexpectedTag),
        }
    }

    pub fn get_long_array(&self) -> Result<&Vec<i64>, crate::error::NbtError> {
        match self {
            Tag::LongArray(v) => Ok(v),
            _ => Err(crate::error::NbtError::UnexpectedTag),
        }
    }

    pub fn get_list(&self) -> Result<&Vec<Tag>, crate::error::NbtError> {
        match self {
            Tag::List(v) => Ok(v),
            _ => Err(crate::error::NbtError::UnexpectedTag),
        }
    }

    pub fn get_compound(&self) -> Result<&IndexMap<String, Tag>, crate::error::NbtError> {
        match self {
            Tag::Compound(v) => Ok(v),
            _ => Err(crate::error::NbtError::UnexpectedTag),
        }
    }

    // === as_* methods (return Option) ===

    pub fn as_byte(&self) -> Option<&i8> {
        match self {
            Tag::Byte(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_short(&self) -> Option<&i16> {
        match self {
            Tag::Short(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_int(&self) -> Option<&i32> {
        match self {
            Tag::Int(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_long(&self) -> Option<&i64> {
        match self {
            Tag::Long(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<&f32> {
        match self {
            Tag::Float(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_double(&self) -> Option<&f64> {
        match self {
            Tag::Double(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_string(&self) -> Option<&String> {
        match self {
            Tag::String(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_byte_array(&self) -> Option<&Vec<i8>> {
        match self {
            Tag::ByteArray(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_int_array(&self) -> Option<&Vec<i32>> {
        match self {
            Tag::IntArray(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_long_array(&self) -> Option<&Vec<i64>> {
        match self {
            Tag::LongArray(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_list(&self) -> Option<&Vec<Tag>> {
        match self {
            Tag::List(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_compound(&self) -> Option<&IndexMap<String, Tag>> {
        match self {
            Tag::Compound(v) => Some(v),
            _ => None,
        }
    }

    // === Compound convenience methods ===

    /// Insert a key-value pair into a Compound tag.
    pub fn insert(&mut self, key: String, value: Tag) -> Result<(), crate::error::NbtError> {
        match self {
            Tag::Compound(map) => {
                map.insert(key, value);
                Ok(())
            }
            _ => Err(crate::error::NbtError::UnexpectedTag),
        }
    }

    /// Get a tag from a Compound by key.
    pub fn get(&self, key: &str) -> Option<&Tag> {
        match self {
            Tag::Compound(map) => map.get(key),
            _ => None,
        }
    }

    /// Remove a tag from a Compound by key.
    pub fn remove(&mut self, key: &str) -> Option<Tag> {
        match self {
            Tag::Compound(map) => map.shift_remove(key),
            _ => None,
        }
    }

    /// Check if a Compound contains a key.
    pub fn contains(&self, key: &str) -> bool {
        match self {
            Tag::Compound(map) => map.contains_key(key),
            _ => false,
        }
    }
}

// === From conversions for primitive types ===

impl From<i8> for Tag {
    fn from(v: i8) -> Self {
        Tag::Byte(v)
    }
}

impl From<i16> for Tag {
    fn from(v: i16) -> Self {
        Tag::Short(v)
    }
}

impl From<i32> for Tag {
    fn from(v: i32) -> Self {
        Tag::Int(v)
    }
}

impl From<i64> for Tag {
    fn from(v: i64) -> Self {
        Tag::Long(v)
    }
}

impl From<f32> for Tag {
    fn from(v: f32) -> Self {
        Tag::Float(v)
    }
}

impl From<f64> for Tag {
    fn from(v: f64) -> Self {
        Tag::Double(v)
    }
}

impl From<String> for Tag {
    fn from(v: String) -> Self {
        Tag::String(v)
    }
}

impl From<&str> for Tag {
    fn from(v: &str) -> Self {
        Tag::String(v.to_string())
    }
}

impl From<Vec<i8>> for Tag {
    fn from(v: Vec<i8>) -> Self {
        Tag::ByteArray(v)
    }
}

impl From<Vec<i32>> for Tag {
    fn from(v: Vec<i32>) -> Self {
        Tag::IntArray(v)
    }
}

impl From<Vec<i64>> for Tag {
    fn from(v: Vec<i64>) -> Self {
        Tag::LongArray(v)
    }
}

impl From<Vec<Tag>> for Tag {
    fn from(v: Vec<Tag>) -> Self {
        Tag::List(v)
    }
}

impl From<IndexMap<String, Tag>> for Tag {
    fn from(v: IndexMap<String, Tag>) -> Self {
        Tag::Compound(v)
    }
}

/// A named NBT tag, used as the root structure for NBT data.
#[derive(Debug, Clone, PartialEq)]
pub struct NamedTag {
    pub name: String,
    pub tag: Tag,
}

impl NamedTag {
    pub fn new(name: impl Into<String>, tag: Tag) -> Self {
        NamedTag {
            name: name.into(),
            tag,
        }
    }
}

/// Returns true if the given tag type ID is valid.
pub fn is_valid_tag_type(tag_type: u8) -> bool {
    matches!(tag_type, TAG_END..=TAG_LONG_ARRAY)
}
