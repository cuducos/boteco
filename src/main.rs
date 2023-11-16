use std::process::exit;

use tokio::try_join;

use boteco::cloud_flare::CloudFlare;
use boteco::errors::BotecoError;
use boteco::improvmx::ImprovMx;

async fn run() -> Result<(), BotecoError> {
    env_logger::init();

    let args: Vec<String> = std::env::args().collect();
    let (url, host) = match args.len() {
        2..=3 => (args[1].clone(), args.get(2).cloned()),
        _ => return Err(BotecoError::CliArgumentsError),
    };

    let cloud_flare = CloudFlare::new(url)?;
    let improv_mx = ImprovMx::new(host)?;
    try_join!(cloud_flare.run(), improv_mx.run())?;

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        eprintln!("{err}");
        exit(1);
    }
}
