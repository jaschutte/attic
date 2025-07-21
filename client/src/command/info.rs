use anyhow::{anyhow, Result};
use clap::Parser;

use crate::api::ApiClient;
use crate::cache::CacheRef;
use crate::cli::Opts;
use crate::config::Config;

#[derive(Debug, Parser)]
pub struct Info {
    /// The cache to configure.
    ///
    /// This can be either `servername:cachename` or `cachename`
    /// when using the default server.
    cache: CacheRef,
    /// Print out the substituter to this cache (printed 1st)
    #[clap(long, short, action)]
    substituter: bool,
    /// Print out the public to this cache (printed 2nd)
    #[clap(long, short, action)]
    public_key: bool,
    /// Print out the token to this cache (if there is no token, "NONE" will be printed) (printed
    /// 3rd)
    #[clap(long, short, action)]
    token: bool,
}

pub async fn run(opts: Opts) -> Result<()> {
    let sub = opts.command.as_info().unwrap();
    let config = Config::load()?;

    let (_server_name, server, cache) = config.resolve_cache(&sub.cache)?;

    let api = ApiClient::from_server_config(server.clone())?;
    let cache_config = api.get_cache_config(cache).await?;

    let substituter = cache_config
        .substituter_endpoint
        .ok_or_else(|| anyhow!("The server did not tell us where the binary cache endpoint is."))?;
    let public_key = cache_config.public_key
        .ok_or_else(|| anyhow!("The server did not tell us which public key it uses. Is signing managed by the client?"))?;

    if sub.substituter {
        println!("{}", substituter);
    }
    if sub.public_key {
        println!("{}", public_key);
    }
    if sub.token {
        match server.token()? {
            Some(token) => println!("{}", token),
            None => println!("NONE"),
        }
    }

    Ok(())
}
