pub mod datetime {
    use chrono::{DateTime, Utc};
    use serde::{Deserializer, Deserialize};

    pub fn deserialize<'de, D: Deserializer<'de>>(de: D) -> Result<DateTime<Utc>, D::Error> {
        let st = String::deserialize(de)?;
        DateTime::parse_from_rfc3339(&st)
            .map(|d| d.with_timezone(&Utc))
            .map_err(serde::de::Error::custom)
    }

    pub fn deserialize_option<'de, D: Deserializer<'de>>(de: D) -> Result<Option<DateTime<Utc>>, D::Error> {
        let st = Option::<String>::deserialize(de)?;

        if let Some(st) = st {
            DateTime::parse_from_rfc3339(&st)
                .map(|d| d.with_timezone(&Utc))
                .map(Some)
                .map_err(serde::de::Error::custom)
        } else {
            Ok(None)
        }
    }
}