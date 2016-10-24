use alloc::boxed::Box;

use collections::string::String;


#[derive(Debug)]
pub enum AttributeValue {
    U8(Box<[u8]>),
    U16(Box<[u16]>),
    U32(Box<[u32]>),
    U64(Box<[u64]>),

    I8(Box<[i8]>),
    I16(Box<[i16]>),
    I32(Box<[i32]>),
    I64(Box<[i64]>),

    F32(Box<[f32]>),
    F64(Box<[f64]>),
}


#[derive(Debug)]
pub struct Attribute {
    pub name: String,
    pub value: AttributeValue,
    pub item_size: usize,
    pub dynamic: bool,
}

macro_rules! create_new_fn {
    ($f: ident, $n: ident, $k: ty) => (
        pub fn $f(name: &str, value: Box<[$k]>, item_size: usize, dynamic: bool) -> Self {
            Attribute {
                name: String::from(name),
                value: AttributeValue::$n(value),
                item_size: item_size,
                dynamic: dynamic,
            }
        }
    )
}

impl Attribute {
    create_new_fn!(new_i8, I8, i8);
    create_new_fn!(new_i16, I16, i16);
    create_new_fn!(new_i32, I32, i32);
    create_new_fn!(new_i64, I64, i64);

    create_new_fn!(new_u8, U8, u8);
    create_new_fn!(new_u16, U16, u16);
    create_new_fn!(new_u32, U32, u32);
    create_new_fn!(new_u64, U64, u64);

    create_new_fn!(new_f32, F32, f32);
    create_new_fn!(new_f64, F64, f64);

    pub fn is_static(&self) -> bool {!self.dynamic}
    pub fn is_dynamic(&self) -> bool {self.dynamic}

    pub fn len(&self) -> usize {
        match self.value {
            AttributeValue::U8(ref v) => v.len(),
            AttributeValue::U16(ref v) => v.len(),
            AttributeValue::U32(ref v) => v.len(),
            AttributeValue::U64(ref v) => v.len(),

            AttributeValue::I8(ref v) => v.len(),
            AttributeValue::I16(ref v) => v.len(),
            AttributeValue::I32(ref v) => v.len(),
            AttributeValue::I64(ref v) => v.len(),

            AttributeValue::F32(ref v) => v.len(),
            AttributeValue::F64(ref v) => v.len(),
        }
    }
}

impl PartialEq<Attribute> for Attribute {
    fn eq(&self, other: &Attribute) -> bool {
        (self as *const _) == (other as *const _)
    }
}
impl Eq for Attribute {}
