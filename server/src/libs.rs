use edgedb_tokio::*;

pub async fn get_db_connection() -> anyhow::Result<edgedb_tokio::Client> {
    /*
    let config = Builder::new()
        .database("edgedb")
        .unwrap()
        .instance("nothing2wear")
        .unwrap()
        .build_env()
        .await?;
        */
    let config = Builder::new()
        .database("edgedb")
        .unwrap()
        .host("0.0.0.0")
        .unwrap()
        .port(5656)
        .unwrap()
        .user("edgedb")
        .unwrap()
        .password("secret")
        .build_env()
        .await?;
    let conn = edgedb_tokio::Client::new(&config);
    conn.ensure_connected().await?;

    Ok(conn)
}
