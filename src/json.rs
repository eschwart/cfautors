use {
    crate::err::*,
    reqwest::blocking::{Client, Response},
    serde::de::DeserializeOwned,
    serde_derive::{Deserialize, Serialize},
    std::net::IpAddr,
};

#[derive(Deserialize, Debug)]
pub struct DnsRecordResult {
    name: String,
    r#type: String,
    content: IpAddr,
}

impl DnsRecordResult {
    pub fn addr(self) -> IpAddr {
        self.content
    }
}

#[derive(Debug, Deserialize)]
pub struct DnsRecord {
    result: DnsRecordResult,
    success: bool,
}

impl TryInto<DnsRecordResult> for DnsRecord {
    type Error = Error;

    fn try_into(self) -> Result<DnsRecordResult> {
        self.success.then_some(self.result).ok_or(Error::Cloudflare)
    }
}

trait GetPublicIp {
    fn public_ip(self) -> IpAddr;
}

pub enum PublicIpAPI {
    Ipify,
    OpenDNS,
    Ifconfig,
}

impl PublicIpAPI {
    pub fn try_get(&self, client: &Client) -> Result<IpAddr> {
        match self {
            Self::Ipify => self.get::<Ipify>("https://api.ipify.org?format=json", client),
            Self::OpenDNS => self.get::<OpenDNS>("https://myipv4.p1.opendns.com/get_my_ip", client),
            Self::Ifconfig => self.get::<Ifconfig>("https://ifconfig.me/all.json", client),
        }
    }

    fn send(url: &'static str, client: &Client) -> Result<Response> {
        client.get(url).send().map_err(Into::into)
    }

    fn get<T: DeserializeOwned + GetPublicIp>(
        &self,
        url: &'static str,
        client: &Client,
    ) -> Result<IpAddr> {
        Self::send(url, client)?
            .json::<T>()
            .map(GetPublicIp::public_ip)
            .map_err(Into::into)
    }
}

#[derive(Deserialize, Debug)]
pub struct Ipify {
    ip: IpAddr,
}

impl GetPublicIp for Ipify {
    fn public_ip(self) -> IpAddr {
        self.ip
    }
}

#[derive(Deserialize, Debug)]
pub struct OpenDNS {
    ip: IpAddr,
}

impl GetPublicIp for OpenDNS {
    fn public_ip(self) -> IpAddr {
        self.ip
    }
}

#[derive(Deserialize, Debug)]
pub struct Ifconfig {
    ip_addr: IpAddr,
}

impl GetPublicIp for Ifconfig {
    fn public_ip(self) -> IpAddr {
        self.ip_addr
    }
}

#[derive(Serialize)]
pub struct Body {
    content: IpAddr,
    name: String,
    r#type: String,
}

impl TryInto<Body> for DnsRecordResult {
    type Error = Error;

    fn try_into(self) -> Result<Body> {
        Ok(Body {
            content: self.content,
            name: self
                .name
                .split_once('.')
                .ok_or(Error::Invalid)?
                .0
                .to_string(),
            r#type: self.r#type,
        })
    }
}

impl Body {
    pub fn update(&mut self, content: IpAddr) {
        self.content = content
    }
}
