use futures::future::try_join_all;

use boteco::cloud_flare::CloudFlare;
use boteco::errors::BotecoError;
use boteco::improvmx::ImprovMx;

#[tokio::main]
async fn main() -> Result<(), BotecoError> {
    env_logger::init();

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        return Err(BotecoError::MissingUrl);
    }

    let cloud_flare = CloudFlare::new(args[1].clone())?;
    try_join_all(
        cloud_flare
            .rules()
            .await?
            .into_iter()
            .map(|rule| cloud_flare.redirect(rule)),
    )
    .await?;

    let improv_mx = ImprovMx::new(args.get(2).cloned())?;
    improv_mx.redirect().await?;

    Ok(())
}
