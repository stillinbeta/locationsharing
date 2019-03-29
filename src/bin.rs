extern crate locationsharing;

use std::io::Result;

fn main() -> Result<()> {
    let val = std::env::var("GOOGLE_COOKIE")?;

    let locations = locationsharing::fetch_locations(val);
    for location in locations {
        println!("{:#?}", location)
    }
}
