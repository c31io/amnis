pub enum Variable {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    Str(String),
    Bytes(Box<[u8]>),

    I32Array(Box<[i32]>),
    I64Array(Box<[i64]>),
    F32Array(Box<[f32]>),
    F64Array(Box<[f64]>),
    StrArray(Box<[String]>),
    BytesArray(Box<[Box<[u8]>]>),
}
