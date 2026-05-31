use clap::{Parser, ValueEnum};
use colored::Colorize;
use hickory_resolver::Resolver;
use hickory_resolver::config::{CLOUDFLARE, GOOGLE, QUAD9, ResolverConfig};
use hickory_resolver::net::runtime::TokioRuntimeProvider;
use hickory_resolver::proto::rr::{RData, RecordType};
use std::error::Error;
use std::str::FromStr;
use std::time::Duration;

#[derive(Parser)]
#[command(version, about = "DNS query tool")]
struct Cli {
    /// DNS resolver to use
    #[arg(long, value_enum, default_value_t = ResolverChoice::System)]
    resolver: ResolverChoice,

    /// The domain to query
    domain: String,

    /// The DNS record type (A, AAAA, NS, MX, TXT, CNAME, etc.)
    #[arg(value_parser = parse_record_type)]
    record_type: RecordType,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
enum ResolverChoice {
    /// Use the operating system DNS configuration
    System,

    /// Use Google Public DNS over HTTPS
    Google,

    /// Use Cloudflare 1.1.1.1 DNS over HTTPS
    Cloudflare,

    /// Use Quad9 DNS over HTTPS
    Quad9,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let resolver = build_resolver(cli.resolver)?;

    let response = resolver.lookup(cli.domain, cli.record_type).await?;

    for record in response.answers() {
        let ttl = format_ttl(record.ttl);
        let record_type_name = format!("{:?}", record.record_type());
        let name = record.name.to_string();

        match &record.data {
            RData::A(addr) => {
                println!(
                    "{} {} {}   {}",
                    record_type_name.green(),
                    name.blue(),
                    ttl,
                    addr
                );
            }
            RData::AAAA(addr) => {
                println!(
                    "{} {} {}   {}",
                    record_type_name.green(),
                    name.blue(),
                    ttl,
                    addr
                );
            }
            RData::NS(ns) => {
                println!(
                    "{} {} {}   \"{}\"",
                    record_type_name.red(),
                    name.blue(),
                    ttl,
                    ns
                );
            }
            RData::CNAME(cname) => {
                println!(
                    "{} {} {}   \"{}\"",
                    record_type_name.green(),
                    name.blue(),
                    ttl,
                    cname
                );
            }
            RData::MX(mx) => {
                println!(
                    "{} {} {}   {} \"{}\"",
                    record_type_name.green(),
                    name.blue(),
                    ttl,
                    mx.preference,
                    mx.exchange
                );
            }
            RData::TXT(txt) => {
                let txt_data = txt
                    .txt_data
                    .iter()
                    .map(|bytes| String::from_utf8_lossy(bytes))
                    .collect::<Vec<_>>()
                    .join("");
                println!(
                    "{} {} {}   \"{}\"",
                    record_type_name.green(),
                    name.blue(),
                    ttl,
                    txt_data
                );
            }
            RData::SOA(soa) => {
                println!(
                    "{} {} {}   \"{}\" \"{}\" {} {} {} {} {}",
                    record_type_name.purple(),
                    name.blue(),
                    ttl,
                    soa.mname,
                    soa.rname,
                    soa.serial,
                    soa.refresh,
                    soa.retry,
                    soa.expire,
                    soa.minimum
                );
            }
            RData::PTR(ptr) => {
                println!(
                    "{} {} {}   \"{}\"",
                    record_type_name.green(),
                    name.blue(),
                    ttl,
                    ptr
                );
            }
            other => {
                println!(
                    "{} {} {}   {:?}",
                    record_type_name.red(),
                    name.blue(),
                    ttl,
                    other
                );
            }
        }
    }

    Ok(())
}

fn build_resolver(
    resolver: ResolverChoice,
) -> Result<Resolver<TokioRuntimeProvider>, Box<dyn Error>> {
    let resolver = match resolver {
        ResolverChoice::System => Resolver::builder_tokio()?.build()?,
        ResolverChoice::Google => public_resolver(&GOOGLE)?,
        ResolverChoice::Cloudflare => public_resolver(&CLOUDFLARE)?,
        ResolverChoice::Quad9 => public_resolver(&QUAD9)?,
    };

    Ok(resolver)
}

fn public_resolver(
    config: &hickory_resolver::config::ServerGroup<'_>,
) -> Result<Resolver<TokioRuntimeProvider>, Box<dyn Error>> {
    Ok(Resolver::builder_with_config(
        ResolverConfig::https(config),
        TokioRuntimeProvider::default(),
    )
    .build()?)
}

fn parse_record_type(record_type: &str) -> Result<RecordType, String> {
    RecordType::from_str(record_type)
        .map_err(|_| format!("invalid DNS record type '{record_type}'"))
}

fn format_ttl(ttl: u32) -> String {
    let duration = Duration::from_secs(ttl as u64);
    let total_seconds = duration.as_secs();

    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    if hours > 0 {
        format!("{}h{:02}m{:02}s", hours, minutes, seconds)
    } else if minutes > 0 {
        format!("{}m{:02}s", minutes, seconds)
    } else {
        format!("{}s", seconds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_seconds_only_ttl() {
        assert_eq!(format_ttl(42), "42s");
    }

    #[test]
    fn formats_minutes_and_seconds_ttl() {
        assert_eq!(format_ttl(181), "3m01s");
    }

    #[test]
    fn formats_hours_minutes_and_seconds_ttl() {
        assert_eq!(format_ttl(5445), "1h30m45s");
    }

    #[test]
    fn rejects_unknown_record_type() {
        assert!(parse_record_type("NOPE").is_err());
    }
}
