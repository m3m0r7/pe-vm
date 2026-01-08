//! COM argument and return value helpers.

// Supported COM argument types for dispatch calls.
#[derive(Debug, Clone)]
pub enum ComArg {
    I4(i32),
    U32(u32),
    BStr(String),
    Ansi(String),
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

    pub fn as_ansi(&self) -> Option<&str> {
        match self {
            ComArg::Ansi(value) => Some(value.as_str()),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_com_arg_as_i4() {
        let arg = ComArg::I4(42);
        assert_eq!(arg.as_i4(), Some(42));
        assert_eq!(arg.as_u32(), None);
        assert_eq!(arg.as_bstr(), None);
    }

    #[test]
    fn test_com_arg_as_u32() {
        let arg = ComArg::U32(100);
        assert_eq!(arg.as_u32(), Some(100));
        assert_eq!(arg.as_i4(), None);
        assert_eq!(arg.as_bstr(), None);
    }

    #[test]
    fn test_com_arg_as_bstr() {
        let arg = ComArg::BStr("hello".to_string());
        assert_eq!(arg.as_bstr(), Some("hello"));
        assert_eq!(arg.as_i4(), None);
        assert_eq!(arg.as_u32(), None);
    }

    #[test]
    fn test_com_value_i4() {
        let value = ComValue::I4(123);
        if let ComValue::I4(v) = value {
            assert_eq!(v, 123);
        } else {
            panic!("Expected I4");
        }
    }

    #[test]
    fn test_com_value_bstr() {
        let value = ComValue::BStr("test".to_string());
        if let ComValue::BStr(s) = value {
            assert_eq!(s, "test");
        } else {
            panic!("Expected BStr");
        }
    }

    #[test]
    fn test_com_value_void() {
        let value = ComValue::Void;
        assert!(matches!(value, ComValue::Void));
    }
}
