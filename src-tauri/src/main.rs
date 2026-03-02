use musahih_pro::app::MusahihApp;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .without_time()
        .init();

    let mut app = MusahihApp::bootstrap().await?;
    app.run().await
}
