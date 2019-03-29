extern crate locationsharing;

fn main() -> Result<(), locationsharing::error::Error> {
    let val = std::env::var("GOOGLE_COOKIE")?;

    let locations = locationsharing::get_locations(&val)?;
    for location in locations {
        println!("{:#?}", location)
    }
    Ok(())
}
