use {
    crate::{BaseClient, Result},
    std::{
        io::{Write, stdout},
        net::IpAddr,
    },
};

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

pub fn dbg<T, F: FnOnce() -> Result<T>>(msg: &'static str, f: F) -> Result<T> {
    let mut stdout = stdout();
    stdout.write_fmt(format_args!("{}...", msg))?;
    stdout.flush()?;

    let result = f();

    stdout.write_fmt(format_args!(
        " {}\n",
        if result.is_ok() { "Done" } else { "Failed" }
    ))?;

    stdout.flush()?;
    result
}
