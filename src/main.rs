use boteco::{client::Client, errors::BotecoError};
use futures::future::try_join_all;

#[tokio::main]
async fn main() -> Result<(), BotecoError> {
    env_logger::init();
    let app = Client::new()?;
    try_join_all(
        app.rules()
            .await?
            .into_iter()
            .map(|rule| app.redirect(rule)),
    )
    .await?;
    Ok(())
}
