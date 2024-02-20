mod cfg;
mod err;
mod json;
mod util;

use {cfg::*, err::*, util::*};

fn main() -> Result<()> {
    let cfg = Config::default();
    let mut client = dbg("Initializing client", || BaseClient::setup(cfg))?;

    loop {
        match dbg("Doing routine", || routine(&mut client)) {
            Ok(patch) => {
                if let Some((old, new)) = patch {
                    println!("PATCH: {} => {}", old, new)
                }
            }
            Err(e) => eprintln!("{}", e),
        }
        dbg("Delaying", || {
            client.delay();
            Ok(())
        })?;
    }
}
