use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://127.0.0.1:8000")?;
    hc.do_get("/hello?name=Sagar").await?.print().await?;
    hc.do_get("/hello/Sagar").await?.print().await?;
    hc.do_get("/src/main.rs").await?.print().await?;

    Ok(())
}
