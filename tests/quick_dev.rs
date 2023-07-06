use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;

    hc.do_get("/hello2/melon").await?.print().await?;

    let req_login = hc.do_post(
        "/api/login",
        serde_json::json!({
            "username": "lemon",
            "pwd": "melon"
        }
        ),
    );

    req_login.await?.print().await?;

    let req_create_ticket = hc.do_post(
        "/api/tickets",
        serde_json::json!({
            "title": "test ticket",
        }),
    );

    req_create_ticket.await?.print().await?;

    // hc.do_delete("/api/tickets/1").await?.print().await?;

    hc.do_get("/api/tickets").await?.print().await?;

    Ok(())
}
