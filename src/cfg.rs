use {
    crate::{
        err::*,
        json::{Body, DnsRecord, DnsRecordResult, PublicIpAPI},
    },
    clap::Parser,
    reqwest::{
        blocking::{Client, RequestBuilder},
        header::CONTENT_TYPE,
        Method,
    },
    std::net::IpAddr,
};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Config {
    #[arg(short, long)]
    email: String,

    #[arg(short, long)]
    api_token: String,

    #[arg(short, long)]
    zone_id: String,

    #[arg(short, long)]
    id: String,
}

pub struct BaseClient {
    client: Client,
    body: Body,
    builder_fn: Box<dyn Fn(Method, &Client) -> RequestBuilder>,
}

impl BaseClient {
    const PUBLIC_IP_API: [PublicIpAPI; 3] = [
        PublicIpAPI::Ipify,
        PublicIpAPI::OpenDNS,
        PublicIpAPI::Ifconfig,
    ];

    pub fn setup() -> Result<Self> {
        let Config {
            email,
            api_token,
            zone_id,
            id,
        } = Config::parse();
        let client = Client::default();
        let builder_fn = Box::new(move |method: Method, client: &Client| -> RequestBuilder {
            client
                .request(
                    method,
                    format!(
                        "https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}",
                        &zone_id, &id
                    ),
                )
                .header("X-Auth-Email", &email)
                .header("X-Auth-Key", &api_token)
                .header(CONTENT_TYPE, "application/json")
        });
        let body = Self::request(Method::GET, &client, builder_fn.as_ref(), None)?.try_into()?;

        Ok(Self {
            client,
            body,
            builder_fn,
        })
    }

    pub fn get(&self) -> Result<IpAddr> {
        Self::request(Method::GET, &self.client, self.builder_fn.as_ref(), None).map(|r| r.addr())
    }

    pub fn patch(&mut self, addr: IpAddr) -> Result<()> {
        self.body.update(addr);

        Self::request(
            Method::PATCH,
            &self.client,
            self.builder_fn.as_ref(),
            Some(&self.body),
        )
        .map(|_| ())
    }

    pub fn public_ip(&self) -> Result<IpAddr> {
        Self::PUBLIC_IP_API
            .into_iter()
            .find_map(|api| api.try_get(&self.client).ok())
            .ok_or(Error::API)
    }

    fn request(
        method: Method,
        client: &Client,
        builder_fn: &dyn Fn(Method, &Client) -> RequestBuilder,
        body: Option<&Body>,
    ) -> Result<DnsRecordResult> {
        let builder = builder_fn(method.clone(), client);

        let res = if let Some(raw) = body {
            builder.json(raw)
        } else {
            builder
        }
        .send()?;

        res.json::<DnsRecord>()?.try_into()
    }
}
