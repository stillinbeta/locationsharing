//! This module contains the data types and the logic for parsing them.

use chrono::{DateTime, TimeZone, Utc};
use serde_json::Value;

use crate::error::Error;

#[derive(Default, Debug, PartialEq)]
/// A user sharing a location with you
pub struct Person {
    /// An Opaque identifier
    pub id: String,
    /// A URL pointing to an avatar
    pub picture_url: Option<String>,
    /// The user's full display name
    pub full_name: String,
    /// The user's shorter nickname
    pub nickname: Option<String>,
}

#[derive(Default, Debug, PartialEq)]
/// One shared location
pub struct Location {
    /// The person represented by this location
    pub person: Person,

    pub latitude: f64,
    pub longitude: f64,
    /// The best guess as to the person's address
    pub address: Option<String>,
    /// When the location was last updated
    pub timestamp: Option<DateTime<Utc>>,
    /// The user's phone's battery, from 0 to 100
    pub battery: Option<u8>,
}

macro_rules! get_vec {
    ($arr : expr, $i : expr) => {
        $arr.get($i).ok_or(Error::Malformed(format!(
            "Vector {:?} missing [{}]",
            $arr, $i
        )))?
    };
    ($arr : expr, $i : expr, $j : expr) => {
        get_vec!($arr, $i).get($j).ok_or(Error::Malformed(format!(
            "Vector {:?} missing [{}][{}]",
            $arr, $i, $j
        )))?
    };
    ($arr : expr, $i : expr, $j : expr, $k : expr) => {
        get_vec!($arr, $i, $j)
            .get($k)
            .ok_or(Error::Malformed(format!(
                "Vector {:?} missing [{}][{}][{}]",
                $arr, $i, $j, $k
            )))?
    };
}

macro_rules! get_str {
    ($arr : expr, $i : expr, $j : expr) => {{
        let v = get_vec!($arr, $i, $j);
        v.as_str()
            .map(String::from)
            .ok_or(Error::Malformed(format!("{:?} was not a string", v)))?
    }};
}

macro_rules! get_float {
    ($arr : expr, $i : expr, $j : expr, $k : expr) => {{
        let v = get_vec!($arr, $i, $j, $k);
        v.as_f64()
            .ok_or(Error::Malformed(format!("{:?} was not a string", v)))?
    }};
}

impl Location {
    // https://github.com/costastf/locationsharinglib/blob/fded96e1a6b0752174b17755ba31cd04779c5653/locationsharinglib/locationsharinglib.py#L222-L233
    // def _populate(self, data):
    // try:
    //     self._id = data[6][0]
    //     self._picture_url = data[6][1]
    //     self._full_name = data[6][2]
    //     self._nickname = data[6][3]
    //     self._latitude = data[1][1][2]
    //     self._longitude = data[1][1][1]
    //     self._timestamp = data[1][2]
    //     self._accuracy = data[1][3]
    //     self._address = data[1][4]
    //     self._country_code = data[1][6]
    //     try:
    //         self._charging = data[13][0]
    //     except TypeError:
    //         self._charging = None
    //     try:
    //         self._battery_level = data[13][1]
    //     except TypeError:
    //         self._battery_level = None
    // except (IndexError, TypeError):
    /// Populates a Location from the proprietary Google representation
    pub(crate) fn from_array(val: &Value) -> Result<Self, Error> {
        let arr = match val {
            Value::Array(v) => v,
            _ => return Err(Error::Malformed("Value should be array".into())),
        };

        Ok(Location {
            person: Person {
                id: get_str!(arr, 6, 0),
                picture_url: get_vec!(arr, 6, 1).as_str().map(String::from),
                full_name: get_str!(arr, 6, 2),
                nickname: get_vec!(arr, 6, 3).as_str().map(String::from),
            },
            latitude: get_float!(arr, 1, 1, 2),
            longitude: get_float!(arr, 1, 1, 1),
            timestamp: Some(
                get_vec!(arr, 1, 2)
                    .as_u64()
                    .map(|v| Utc.timestamp((v / 1000) as i64, 0))
                    .ok_or_else(|| Error::Malformed("Failed to parse datetime".into()))?,
            ),
            address: get_vec!(arr, 1, 4).as_str().map(|v| v.into()),
            battery: get_vec!(arr, 13, 1).as_u64().map(|v| v as u8),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    const EXAMPLE_LOCATION: &str = include_str!("testdata/example.json");

    #[test]
    fn decode() {
        let expected = Location{
            person: Person{
                id: "3Jef8UhKEnoX92I68LSNuaTKiCTZCpzQav".into(),
                picture_url: Some("https://lh3.googleusercontent.com/-XdUIqdMkCWA/AAAAAAAAAAI/AAAAAAAAAAA/4252rscbv5MWrETbApjU_W4ck_MpC5JqwIhkKAUQQ____________ARibr4X4______8B/w80-h80/photo.jpg".into()),
                full_name: "Twilight Sparkle".into(),
                nickname: Some("Twilight".into()),
            },

            latitude: 49.266_488_6,
            longitude: -123.112_358_9,
            address: Some("380 W 5th Ave, Vancouver, BC V5Y 1J5, Canada".into()),
            timestamp: Some(Utc.ymd(2019, 3, 28).and_hms(19, 54, 45)),
            battery: Some(89),
        };

        let v: Value = serde_json::from_str(EXAMPLE_LOCATION).expect("failed to decode JSON");
        let location = Location::from_array(&v).expect("failed to unserialize array");
        assert_eq!(expected, location);
    }
}
