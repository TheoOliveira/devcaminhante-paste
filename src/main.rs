#[macro_use] extern crate rocket;
use rocket::tokio::fs::File;
use rocket::data::{Data, ToByteUnit};
use rocket::http::uri::Absolute;
mod paste_id;
use paste_id::PasteId;

#[derive(UrlDisplayPath)]
pub struct PasteId<'a>(Cow<'a, str>);

const ID_LENGTH: usize = 3;
const HOST: Absolute<'static> = uri!("http://localhost:8000");
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
async fn retrieve(id: PasteId<'_>) -> Option<File> {
        File::open(id.file_path()).await.ok()        
    }

#[post("/", data = "<paste>")]
async fn upload(paste: Data<'_>) -> std::io::Result<String> {
    let id = PasteId::new(ID_LENGTH);
    paste.open(128.kibibytes()).into_file(id.file_path()).await?;
    Ok(uri!(HOST, retrieve(id)).to_string())
}
