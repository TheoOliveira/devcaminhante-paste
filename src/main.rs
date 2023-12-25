#[macro_use] extern crate rocket;
use rocket::tokio::fs::File;
use rocket::data::{Data, ToByteUnit};
use rocket::http::uri::Absolute;
mod paste_id;
use paste_id::PasteId;


const ID_LENGTH: usize = 3;
const HOST: Absolute<'static> = uri!("http://localhost:8000");
#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket =  rocket::build().mount("/", routes![index, retrieve, upload]);

    Ok(rocket.into())
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
async fn retrieve(id: PasteId<'_>) -> Option<File> {
        File::open(id.file_path()).await.ok()        
    }

#[post("/", data = "<paste>")]
async fn upload(paste: Data<'_>) -> std::io::Result<String> {
    let id = PasteId::new(ID_LENGTH);
    paste.open(128.kibibytes()).into_file(id.file_path()).await?;
    Ok(uri!(HOST, retrieve(id)).to_string())
}
