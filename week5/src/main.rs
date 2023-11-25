
// use serde::{Serialize,Deserialize};

// #[derive(Serialize,Deserialize,Debug)]
// #[serde(rename_all = "camelCase")]//convert the string to camelCase
// struct Person {
//     //#[serde(rename = "haha")]
//     name: String,
//     age: u8,
//     phone_number: Vec<String>,
// }


// fn main() {
//     let p = Person {
//         name: "Peter".to_string(),
//         age: 27,
//         phone_number: vec!["+886 974174956".to_string(), "+886 935525671".to_string()],
//     };

//     let p_json = serde_json::to_string(&p).unwrap();
//     let new_p = serde_json::from_str::<Person>(&p_json).unwrap();
//     println!{"{}",p_json};
//     println!("{:?}",new_p);
// }

//the above is about serde


use mini_redis::{client,Result};
use std::collections::HashMap;
use std::sync::{Arc,Mutex};
use tokio::sync::Mutex as AsyncMutex;
use axum::routing::{get,post};
use axum::extract::Query;


async fn print() {
    println!("hello");
}
async fn hello_world(Query(query): Query<HashMap<String,String>>) -> &'static str{
    println!("{:?}",query);
    "hello world from an app \n fuck"
}
#[tokio::main]
async fn main(){
    let x = 1;
    let y = x.clone();
    let f = print();
    let func = tokio::spawn(async move{//tokio::spawn takes a scope as arg
        println!("world {}",y);
    });
    f.await;
    let _ = func.await;
    //the above is about async functions;
    let mut client1 = client::connect("127.0.0.1:6379").await.unwrap();
    let data1 = client1.get("data1").await.unwrap();
    println!("get data from the server:{:?}",data1);

    client1.set("hello","world".into()).await.unwrap();//set the data named "hello" into "world";
    let client2: Arc<AsyncMutex<client::Client>> = Arc::new(AsyncMutex::new(client::connect("127.0.0.1:6379").await.unwrap()));
    tokio::spawn(async move{
        client2.lock().await.set("data2","100".into()).await.unwrap();
    });
    //the above is about interacting with the server
    let app = axum::Router::new()
        .route("/",post(hello_world));
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}