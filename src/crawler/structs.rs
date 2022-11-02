#[derive(Debug, Clone)]
pub enum Document {
    Dom(Dom),
    Element(Element),
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