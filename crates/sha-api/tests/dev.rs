use anyhow::Result;

#[tokio::test]
async fn check_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:4000")?;

    hc.do_get("/").await?.print().await?;

    Ok(())
}