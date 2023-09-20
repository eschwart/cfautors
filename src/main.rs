mod cfg;
mod err;
mod json;
mod util;

use {cfg::*, err::*, util::*};

fn main() -> Result<()> {
    let mut client = BaseClient::setup()?;

    loop {
        match routine(&mut client) {
            Ok(patch) => {
                if let Some((old, new)) = patch {
                    println!("PATCH: {} => {}", old, new)
                }
            }
            Err(e) => eprintln!("{}", e),
        }
        delay()
    }
}
