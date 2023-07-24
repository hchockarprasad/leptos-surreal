#![allow(non_snake_case)]

use leptos::*;

use surrealdb::engine::remote::ws::Client as SurrealClient;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

pub static DB: Surreal<SurrealClient> = Surreal::init();

async fn connect_db(_: Scope) -> String {
    DB.connect::<Ws>("localhost:8000")
        .await
        .expect("Error connecting to database");
    DB.signin(Root {
        username: "root",
        password: "root",
    })
    .await
    .unwrap();
    DB.use_ns("localdb").await.unwrap();
    DB.use_db("localdb").await.unwrap();
    DB.version().await.unwrap().to_string()
}

pub fn main() {
    log!("csr mode - mounting to body");

    mount_to_body(|cx| {
        view! { cx,
            <Await
                future=|cx| connect_db(cx)
                bind:data
                blocking=true
            >
                <p>"Surreal version " {data} </p>
            </Await>
        }
    });
}
