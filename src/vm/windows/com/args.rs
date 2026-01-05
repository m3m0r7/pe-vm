//! COM argument and return value helpers.

// Supported COM argument types for dispatch calls.
#[derive(Debug, Clone)]
pub enum ComArg {
    I4(i32),
    U32(u32),
    BStr(String),
}

impl ComArg {
    pub fn as_i4(&self) -> Option<i32> {
        match self {
            ComArg::I4(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_u32(&self) -> Option<u32> {
        match self {
            ComArg::U32(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_bstr(&self) -> Option<&str> {
        match self {
            ComArg::BStr(value) => Some(value.as_str()),
            _ => None,
        }
    }
}

// Supported COM return values from dispatch calls.
#[derive(Debug, Clone)]
pub enum ComValue {
    I4(i32),
    BStr(String),
    Void,
}
