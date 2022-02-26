use serde::de::{self, Deserialize, Deserializer};

pub mod db;
pub mod health;
pub mod meter;

#[derive(Debug)]
pub enum Identifier {
    Id(i32),
    Name(String),
}

#[derive(Debug)]
pub enum IdentifierPath {
    Id(i32),
    Name(String),
}

impl From<IdentifierPath> for Identifier {
    fn from(identifier: IdentifierPath) -> Self {
        match identifier {
            IdentifierPath::Id(id) => Self::Id(id),
            IdentifierPath::Name(name) => Self::Name(name),
        }
    }
}

impl<'de> Deserialize<'de> for Identifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let input: serde_json::value::Value = Deserialize::deserialize(deserializer)?;

        let identifier = match input {
            serde_json::Value::Number(number) => {
                let x = match i32::try_from(
                    number
                        .as_i64()
                        .ok_or_else(|| de::Error::custom("number not a i32"))?,
                ) {
                    Ok(n) => n,
                    Err(_) => return Err(de::Error::custom("number not an i32")),
                };

                Self::Id(x)
            }
            serde_json::Value::String(name) => Self::Name(name),
            _ => return Err(de::Error::custom("Invalid type")),
        };

        Ok(identifier)
    }
}

impl<'de> Deserialize<'de> for IdentifierPath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let input: String = Deserialize::deserialize(deserializer)?;

        let identifier: Self = match input.parse::<i32>() {
            Ok(num) => Self::Id(num),
            Err(_) => Self::Name(input),
        };

        Ok(identifier)
    }
}
