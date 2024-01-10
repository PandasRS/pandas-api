// main.rs
#[macro_use] extern crate rocket;

use rocket_okapi::{openapi_get_routes, swagger_ui::*};

mod config;
mod database;
mod modules;

use crate::config::AppConfig;
use crate::database::establish_mongo_connection;
use crate::modules::pandas::controller::{
    create_panda, 
    okapi_add_operation_for_create_panda_, 
    get_pandas, 
    okapi_add_operation_for_get_pandas_, 
    get_panda, 
    okapi_add_operation_for_get_panda_,
    update_panda,
    okapi_add_operation_for_update_panda_,
    delete_panda,
    okapi_add_operation_for_delete_panda_,
};

#[launch]
async fn rocket() -> _ {
    // Load application configuration
    let config = AppConfig::new();

    // Establish a connection to MongoDB
    let db = establish_mongo_connection(&config).await;

    // Build and launch the Rocket application
    rocket::build()
        .manage(db)
        .mount("/", openapi_get_routes![create_panda, get_pandas, get_panda, update_panda, delete_panda])
        .mount("/swagger-ui/", make_swagger_ui(&SwaggerUIConfig {
            url: "/openapi.json".to_owned(),
            ..Default::default()
        }))
}