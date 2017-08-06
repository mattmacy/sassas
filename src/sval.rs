#[derive(Clone, Debug)]
pub enum SVal {
    Bool(bool),
    Int(i32),
    Float(f32),
    Data(Vec<u8>),
    DataL(Vec<u64>),
    Str(Vec<String>),
    Required,
}

impl From<f32> for SVal {
    fn from(input: f32) -> Self {
        SVal::Float(input)
    }
}
impl From<i32> for SVal {
    fn from(input: i32) -> Self {
        SVal::Int(input)
    }
}
impl From<bool> for SVal {
    fn from(input: bool) -> Self {
        SVal::Bool(input)
    }
}
impl From<Vec<u8>> for SVal {
    fn from(input: Vec<u8>) -> Self {
        SVal::Data(input)
    }
}
impl From<Vec<u64>> for SVal {
    fn from(input: Vec<u64>) -> Self {
        SVal::DataL(input)
    }
}
impl From<Vec<String>> for SVal {
    fn from(input: Vec<String>) -> Self {
        SVal::Str(input)
    }
}
impl From<SVal> for bool {
    fn from(input: SVal) -> Self {
        match input {
            self::SVal::Bool(x) => x.clone(),
            _ => unimplemented!(),
        }
    }
}
impl From<SVal> for f32 {
    fn from(input: SVal) -> Self {
        match input {
            self::SVal::Float(x) => x.clone(),
            _ => unimplemented!(),
        }
    }
}
impl From<SVal> for Vec<u8> {
    fn from(input: SVal) -> Self {
        match input {
            self::SVal::Data(x) => x.clone(),
            _ => unimplemented!(),
        }
    }
}
impl From<SVal> for Vec<u64> {
    fn from(input: SVal) -> Self {
        match input {
            self::SVal::DataL(x) => x.clone(),
            _ => unimplemented!(),
        }
    }
}
impl From<SVal> for Vec<String> {
    fn from(input: SVal) -> Self {
        match input {
            self::SVal::Str(x) => x.clone(),
            _ => unimplemented!(),
        }
    }
}