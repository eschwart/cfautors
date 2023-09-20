use {
    crate::{BaseClient, Result},
    std::{net::IpAddr, thread::sleep, time::Duration},
};

pub const DELAY: Duration = Duration::from_secs(300);

pub fn routine(client: &mut BaseClient) -> Result<Option<(IpAddr, IpAddr)>> {
    let current = client.public_ip()?;
    let record = client.get()?;

    Ok(if current != record {
        client.patch(current)?;
        Some((record, current))
    } else {
        None
    })
}

pub fn delay() {
    sleep(DELAY)
}
