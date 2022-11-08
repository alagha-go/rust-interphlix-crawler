#[derive(Debug, Clone)]
pub enum Document {
    Dom(Dom),
    Element(Element),
    Text(String)
}

#[derive(Debug, Clone)]
enum Action<T>  {
    Call(T),
    Handle(T)
}


#[derive(Debug, Clone)]
pub struct Attribute {
    key: String,
    value: Option<String>
}

#[derive(Debug, Clone)]
pub struct Error(String);

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for Error {}