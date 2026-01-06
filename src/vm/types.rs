use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub enum Value {
    Env(BTreeMap<String, String>),
    U32(u32),
    U64(u64),
    String(String),
}

// Captures COM out parameters for the most recent IDispatch/ITypeInfo call.
#[derive(Debug, Clone)]
pub struct ComOutParam {
    pub index: usize,
    pub vt: u16,
    pub flags: u32,
    pub ptr: u32,
}

#[derive(Debug, Default, Clone)]
pub struct ExecuteOptions {
    env: Option<BTreeMap<String, String>>,
}

impl ExecuteOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn env(self, env: BTreeMap<String, String>) -> Self {
        let mut options = self;
        options.env = Some(env);
        options
    }

    pub(crate) fn env_ref(&self) -> Option<&BTreeMap<String, String>> {
        self.env.as_ref()
    }
}
