pub mod terra_datetime_format {
    use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.f";
    const FORMAT_SHORT: &str = "%Y-%m-%dT%H:%M:%SZ";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    #[allow(dead_code)]
    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        let len = s.len();
        let slice_len = if s.contains('.') {
            len.saturating_sub(4)
        } else {
            len
        };

        // match Utc.datetime_from_str(&s, FORMAT) {
        let sliced = &s[0..slice_len];
        match NaiveDateTime::parse_from_str(sliced, FORMAT) {
            Err(_e) => match NaiveDateTime::parse_from_str(sliced, FORMAT_SHORT) {
                Err(_e2) => {
                    eprintln!("DateTime Fail {} {:#?}", sliced, _e);
                    Err(serde::de::Error::custom(_e))
                }
                Ok(dt) => Ok(Utc.from_utc_datetime(&dt)),
            },
            Ok(dt) => Ok(Utc.from_utc_datetime(&dt)),
        }
    }
}

pub mod terra_u64_format {
    use serde::{self, Deserialize, Deserializer, Serializer};

    // convert a number in string format into a regular u64
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    #[allow(dead_code)]
    pub fn serialize<S>(val: &u64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        //  let s = format!("{}", val);
        serializer.serialize_str(&val.to_string())
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        match s.parse::<u64>() {
            Err(_e) => {
                eprintln!("u64 Fail {} {:#?}", s, _e);
                Err(serde::de::Error::custom(_e))
            }
            Ok(val) => Ok(val),
        }
    }
}

pub mod terra_f64_format {
    use serde::{self, Deserialize, Deserializer, Serializer};

    // convert a number in string format into a regular u64
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    #[allow(dead_code)]
    pub fn serialize<S>(val: f64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        //  let s = format!("{}", val);
        serializer.serialize_str(&val.to_string())
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;

        match s.parse::<f64>() {
            Err(_e) => {
                eprintln!("f64 Fail {} {:#?}", s, _e);
                Err(serde::de::Error::custom(_e))
            }
            Ok(val) => Ok(val),
        }
    }
}
