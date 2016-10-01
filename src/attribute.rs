use collections::string::String;


#[derive(Debug)]
pub enum AttributeValue<'a> {
    U8(&'a [u8]),
    U16(&'a [u16]),
    U32(&'a [u32]),
    U64(&'a [u64]),

    I8(&'a [i8]),
    I16(&'a [i16]),
    I32(&'a [i32]),
    I64(&'a [i64]),

    F32(&'a [f32]),
    F64(&'a [f64]),
}


#[derive(Debug)]
pub struct Attribute<'a> {
    pub name: String,
    pub value: AttributeValue<'a>,
    pub item_size: usize,
    pub dynamic: bool,
}

macro_rules! create_new_fn {
    ($f: ident, $n: ident, $k: ty) => (
        pub fn $f(name: &str, value: &'a [$k], item_size: usize, dynamic: bool) -> Self {
            Attribute {
                name: String::from(name),
                value: AttributeValue::$n(value),
                item_size: item_size,
                dynamic: dynamic,
            }
        }
    )
}

impl<'a> Attribute<'a> {
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
            AttributeValue::U8(v) => v.len(),
            AttributeValue::U16(v) => v.len(),
            AttributeValue::U32(v) => v.len(),
            AttributeValue::U64(v) => v.len(),

            AttributeValue::I8(v) => v.len(),
            AttributeValue::I16(v) => v.len(),
            AttributeValue::I32(v) => v.len(),
            AttributeValue::I64(v) => v.len(),

            AttributeValue::F32(v) => v.len(),
            AttributeValue::F64(v) => v.len(),
        }
    }
}
