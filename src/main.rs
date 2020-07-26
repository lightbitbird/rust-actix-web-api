use actix_web::{App, client::Client, Error, HttpResponse, HttpServer, web};
// use futures::Future;
use serde_derive::Deserialize;

async fn index() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().content_type("text/plain").body("Hello World!"))
}

// fn get_wiki_resources() -> impl Future<Item=HttpResponse, Error=actix_web::Error> {
//     let name = "Tablet";
//     let url = "http://wikipedia.simpleapi.net/api?keyword=name&output=json".replace("name", name);
//
//     let mut response = Client::new()
//         .get(url)
//         // .timeout(std::time::Duration::from_secs(20))
//         .send()
//         .map_err(move |e| {
//             println!("error while getting url: {:?}. e: {}", url, e);
//             Error::from(e)
//         })
//         .and_then(|mut res| match res.text() {
//             Ok(body) => HttpResponse::Ok()
//                 .content_type("application/json")
//                 .body(body),
//             Err(error) => {
//                 println!("error while getting payload. e: {}", error);
//                 HttpResponse::InternalServerError()
//                     .content_type("application/json")
//                     .body(format!("{{\"error\": \"Error getting response text.\"}}"))
//             }
//         });
//     response
// }

#[derive(Deserialize)]
pub struct Info {
    name: String,
}

async fn get_wiki_resources_async(info: web::Query<Info>) -> Result<HttpResponse, Error> {
    let client = Client::new();
    let url = "http://wikipedia.simpleapi.net/api?keyword=name&output=json".replace("name", &info.name);
    // let url = format!("{}{}{}", "http://wikipedia.simpleapi.net/api?keyword=", &info.name, "&output=json");

    let mut res = client
        .get(url)
        .header("User-Agent", "Actix-web")
        .send()
        .await.map_err(Error::from)?;
    let mut client_res = HttpResponse::build(res.status());
    let body = res.body().limit(usize::max_value()).await?;

    Ok(client_res.content_type("application/json").body(body).await?)
}

// fn main() {
//     HttpServer::new(|| {
//         App::new()
//             .service(
//                 web::scope("/app")
//                     .service(web::resource("/").to(index))
//                     // .service(web::resource("/wiki").route(web::get().to(get_wiki_resources_async)))
//                     .service(web::resource("/wiki_future").route(web::get().to_async(get_wiki_resources)))
//             )
//     }).bind("localhost:3000")
//         .unwrap()
//         .run();
// }

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/app")
                    .service(web::resource("/").to(index))
                    .service(web::resource("/wiki").route(web::get().to(get_wiki_resources_async)))
            )
    }).bind("localhost:3000")
        .unwrap()
        .run()
        .await
}


