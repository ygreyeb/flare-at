use cloudflare::{
    endpoints::dns::{
        DnsContent, DnsRecord, ListDnsRecords, ListDnsRecordsParams, UpdateDnsRecord,
        UpdateDnsRecordParams,
    },
    framework::{
        apiclient::ApiClient, auth::Credentials, Environment, HttpApiClient, HttpApiClientConfig,
    },
};
use core::panic;
use std::net::Ipv4Addr;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opts {
    /// API token
    #[structopt(short, long)]
    token: String,

    /// Zone ID
    #[structopt(short, long)]
    zone: String,

    /// DNS record name
    #[structopt(short, long)]
    name: String,

    /// The IP address
    ip: Ipv4Addr,
}

fn main() {
    let opts = Opts::from_args();

    let client = match HttpApiClient::new(
        Credentials::UserAuthToken { token: opts.token },
        HttpApiClientConfig::default(),
        Environment::Production,
    ) {
        Ok(c) => c,
        Err(e) => panic!("{:#}", e),
    };

    if let Some(record) = find_dns_record_by_name(&client, &opts.zone, &opts.name) {
        match client.request(&UpdateDnsRecord {
            zone_identifier: &record.zone_id,
            identifier: &record.id,
            params: UpdateDnsRecordParams {
                name: &record.name,
                ttl: Some(record.ttl),
                proxied: Some(record.proxied),
                content: DnsContent::A { content: opts.ip },
            },
        }) {
            Ok(success) => {
                if !success.errors.is_empty() {
                    eprint!(
                        "There were errors whilst updating the DNS record. {:?}",
                        success.errors
                    )
                }
                print!(
                    "DNS records updated successfully. New record is {:?}",
                    success.result
                )
            }

            Err(e) => panic!("DNS record update request failed. {:#}", e),
        }
    } else {
        panic!("Unable to find DNS record \"{}\".", opts.name)
    }
}

fn find_dns_record_by_name(
    client: &HttpApiClient,
    zone_id: &String,
    name: &String,
) -> Option<DnsRecord> {
    match client.request(&ListDnsRecords {
        zone_identifier: zone_id,
        params: ListDnsRecordsParams {
            name: Some(name.to_owned()),
            ..Default::default()
        },
    }) {
        Ok(success) => {
            if !success.errors.is_empty() {
                eprint!(
                    "There were errors whilst requesting DNS record list. {:?}",
                    success.errors
                )
            }
            for record in success.result {
                if name == &record.name && zone_id == &record.zone_id && record.is_a() {
                    return Some(record);
                }
            }
            None
        }

        Err(e) => panic!("DNS record list request failed. {:#}", e),
    }
}

impl IsA for DnsRecord {
    fn is_a(&self) -> bool {
        match self.content {
            DnsContent::A { content: _ } => true,
            _ => false,
        }
    }
}

trait IsA {
    fn is_a(&self) -> bool;
}
