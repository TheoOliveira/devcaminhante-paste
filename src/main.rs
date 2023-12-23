#[macro_use] extern crate rocket;
use std::path::Path;
use rocket::tokio::fs::File;

mod paste_id;

use paste_id::PasteId;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, retrieve])
}

#[get("/")]
fn index() -> &'static str {
    "
    Uso

    POST /
            aceita arquivos raw no corpo do request e responde com  uma url da página contendo o conteudo do body
    
    GET /id
            retorna o conteúdo do paste com o id
    "
}
#[get("/<id>")]
async fn retrieve(id: &str) -> Option<File> {
        let upload_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/", "upload");
        let filename = Path::new(upload_dir).join(id);
        File::open(&filename).await.ok();
    }

