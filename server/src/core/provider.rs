pub trait ProviderObject {}

#[derive(Debug)]
pub enum ProviderError {
    Other(&'static str),
}

pub trait ReadableProvider<O: ProviderObject> {
    fn get(&self, id: &str) -> Result<O, ProviderError>;
}

pub trait WriteableProvider<O: ProviderObject> {
    fn set(&self, id: &str, value: O) -> Result<(), ProviderError>;
}
