use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::process::id;
use std::sync::Mutex;

pub struct DataHolder {
    my_data: Mutex<Vec<CreateEntryData>>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct CreateEntryData {
    bhajan_nepali: String,
    bhajan_english:String,
    id:i32
}
#[post("/api")]
async fn create_entry(
    data: web::Data<DataHolder>,
    param_obj: web::Json<Vec<CreateEntryData>>,
) -> impl Responder {
    let mut my_data = data.my_data.lock().unwrap();
    
    for entry in param_obj.into_inner() {
        my_data.push(entry);
    }
    HttpResponse::Ok().json(my_data.to_vec())
   
    
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = web::Data::new(DataHolder {
        my_data: Mutex::new(vec![]),
    });
    HttpServer::new(move || App::new().app_data(app_data.clone()).service(create_entry))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
