impl<'de> Visitor<'de> for ObjectIdVisitor {
    type Value = ObjectId;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a string containing at least {} bytes", 1)
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let value = ObjectId::from_hex(&s.to_owned()).unwrap();
        Ok(ObjectId::from_hex(&s.to_owned()).unwrap())
    }

    fn visit_string<E>(self, s: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let value = ObjectId::from_hex(&s).unwrap();
        Ok(ObjectId::from_hex(&s).unwrap())
    }
}