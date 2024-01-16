mod cfg;
mod err;
mod json;
mod util;

use {cfg::*, err::*, util::*};

fn main() -> Result<()> {
    let mut client = dbg("Initializing client", || BaseClient::setup())?;

    loop {
        match dbg("Doing routine", || routine(&mut client)) {
            Ok(patch) => {
                if let Some((old, new)) = patch {
                    println!("PATCH: {} => {}", old, new)
                }
            }
            Err(e) => eprintln!("{}", e),
        }
        dbg("Delaying", || Ok(client.delay()))?;
    }
}
