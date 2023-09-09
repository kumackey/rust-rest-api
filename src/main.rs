use actix_web::{App, get, HttpResponse, HttpServer, post, Responder, web};
use tokio_postgres::NoTls;

#[get("/")]
async fn hello() -> impl Responder {
    match query().await {
        Ok(s) => HttpResponse::Ok().body(s),
        Err(e) => HttpResponse::Ok().body("failed"),
    }
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

async fn query() -> Result<String, tokio_postgres::Error> {
    let (client, connection) = tokio_postgres::connect(
        "postgresql://postgres:mysecretpassword@127.0.0.1:15432/rust_rest_api",
        NoTls,
    )
        .await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let rows = client.query("SELECT id, name FROM users", &[]).await?;

    if let Some(row) = rows.get(0) {
        let id: i32 = row.get(0);
        let name: String = row.get(1);
        println!("found person: {} {}", id, name);
        return Ok(format!("found person: {} {}", id, name));
    }

    Ok("No results found".to_string())
}