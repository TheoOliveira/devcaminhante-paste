#[macro_use]
extern crate rocket;
use rocket::data::{Data, ToByteUnit};
use rocket::http::uri::Absolute;
use rocket::response::status::BadRequest;
use rocket::tokio::fs::File;
use rocket::tokio::io::AsyncReadExt;
use rocket::State;
use std::path::{Path, PathBuf};
mod paste_id;
use paste_id::PasteId;
use shuttle_persist::PersistInstance;

const ID_LENGTH: usize = 3;
const HOST: Absolute<'static> = uri!("http://localhost:8000");

struct MyState {
    persist: PersistInstance,
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_persist::Persist] persist: PersistInstance,
) -> shuttle_rocket::ShuttleRocket {
    let state = MyState { persist };
    let rocket = rocket::build()
        .mount("/", routes![index, retrieve, upload])
        .manage(state);

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
async fn retrieve(
    id: PasteId<'_>,
    state: &State<MyState>,
) -> Result<Option<File>, BadRequest<String>> {
    let str_id = id.to_string();
    let converted_id: &str = &str_id;
    let upload_dir: PathBuf = state
        .persist
        .load(converted_id)
        .map_err(|e| BadRequest(e.to_string()))?;
    let file_name = Path::new(&upload_dir);
    Ok(File::open(&file_name).await.ok())
}

#[post("/", data = "<paste>")]
async fn upload(paste: Data<'_>, state: &State<MyState>) -> std::io::Result<String> {
    let id = PasteId::new(ID_LENGTH);
    let str_id = id.to_string();
    let converted_id: &str = &str_id;
    // Read the file data into a string
    let mut buffer = String::new();
    let _ = paste
        .open(128.kibibytes())
        .read_to_string(&mut buffer)
        .await;
    let _ = state
        .persist
        .save(converted_id, &buffer)
        .map_err(|e| BadRequest(e.to_string()));
    Ok(uri!(HOST, retrieve(id)).to_string())
}
