extern crate reqwest;
extern crate serde_json;

use crate::error::Error;
use crate::Location;
use std::collections::HashMap;
use std::io::Read;

const LOCATION_URL: &str = "https://www.google.com/maps/preview/locationsharing/read";
// https://github.com/costastf/locationsharinglib/blob/fded96e1a6b0752174b17755ba31cd04779c5653/locationsharinglib/locationsharinglib.py#L103-L110";
const PB_LOCATION: &str = "!1m7!8m6!1m3!1i14!2i8413!3i5385!2i6!3x4095!2m3!1e0!2sm!3i407105169!3m7!2sen!5e1105!12m4!1e68!2m2!1sset!2sRoadmap!4e1!5m4!1e4!8m2!1e0!1e1!6m9!1e12!2i2!26m1!4b1!30m1!1f1.3953487873077393!39b1!44e1!50e0!23i4111425";

pub fn get_locations(cookie: &str) -> Result<Vec<Location>, Error> {
    let qs: HashMap<&str, &str> = vec![
        ("authuser", "0"),
        ("hl", "en"),
        ("gl", "en"),
        ("pb", PB_LOCATION),
    ]
    .iter()
    .cloned()
    .collect();

    let client = reqwest::Client::new();

    let mut response = client
        .get(LOCATION_URL)
        .query(&qs)
        .header("cookie", cookie)
        .send()?;

    let mut buffer = String::new();
    response.read_to_string(&mut buffer).map_err(Error::from)?;
    let (_, json) = buffer.split_at(4); // Cut off initial )]}'
    let arr: Vec<serde_json::Value> = serde_json::from_str(json)?;
    let locations = arr
        .get(0)
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| Error::Malformed(format!("Missing location array: {:?}", arr)))?;
    let mut vec = Vec::new();
    for elem in locations {
        vec.push(Location::from_array(elem)?);
    }
    Ok(vec)
}
