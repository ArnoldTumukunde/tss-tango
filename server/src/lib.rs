use actix_cors::Cors;
use actix_web::{
    error, get, http, post,
    web::{self},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use database::models::Token;
use database::{
    models::{ContractJson, TokenSwap},
    MongoRepo,
};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
// use std::sync::Mutex;
use lazy_static::lazy_static;

#[derive(Serialize, Deserialize, Debug)]
struct Event {
    log_block_number: i32,
    log_index: i32,
    log_name: String,
    from: String,
    to: String,
    tokens: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InputToken {
    pub token: String,
    pub token_address: String,
    pub api_token_endpoint: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct InputTokenSwap {
    pub token: String,
    pub swap_token: String,
    pub amount: i32,
}
#[derive(Debug, Deserialize)]
pub struct Params {
    id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct IdInterface {
    id: String,
}

const MAX_SIZE: usize = 262_144;
// #[get("/eventold")]
// async fn eventsOld() -> impl Responder {
//     // TODO get the event from db_conn
//     let event = Event {
//         log_block_number: 1,
//         log_index: 1,
//         log_name: String::from("Transfer"),
//         from: String::from("0xBA826fEc90CEFdf6706858E5FbaFcb27A290Fbe0"),
//         to: String::from("0x4aEE792A88eDDA29932254099b9d1e06D537883f"),
//         tokens: String::from("2863452144424379687066"),
//     };

//     let serialized = serde_json::to_string(&event).unwrap();
//     println!("serialized = {}", serialized);

//     match serde_json::to_string(&event) {
//         Ok(response_str) => response_str,
//         Err(_) => "data Error".to_string(),
//     }
// }

#[get("/event")]
async fn events(db_conn: web::Data<Arc<Mutex<database::MongoRepo>>>) -> impl Responder {
    // TODO get the event from db_conn
    let conn = db_conn.lock().await;
    let result = MongoRepo::get_event_data(&conn).await;
    let data = match result {
        Ok(response_str) => response_str,
        Err(_) => todo!(),
    };

    match serde_json::to_string(&data) {
        Ok(response_str) => response_str,
        Err(_) => "data Error".to_string(),
    }
}

#[get("/contracts")]
async fn get_contracts(db_conn: web::Data<Arc<Mutex<database::MongoRepo>>>) -> impl Responder {
    let conn = db_conn.lock().await;
    let result = MongoRepo::get_contract_json(&conn).await;
    let data = match result {
        Ok(response_str) => response_str,
        Err(_) => todo!(),
    };

    match serde_json::to_string(&data) {
        Ok(response_str) => response_str,
        Err(_) => "data Error".to_string(),
    }
}

#[get("/contract_events")]
async fn get_contract_events(
    db_conn: web::Data<Arc<Mutex<database::MongoRepo>>>,
    req: HttpRequest,
) -> impl Responder {
    let params = web::Query::<Params>::from_query(req.query_string()).unwrap();
    let conn = db_conn.lock().await;
    let result = MongoRepo::get_contract_event(&conn, params.id.to_string()).await;
    let data = match result {
        Ok(response_str) => response_str,
        Err(_) => todo!(),
    };

    match serde_json::to_string(&data) {
        Ok(response_str) => response_str,
        Err(_) => "data Error".to_string(),
    }
}

#[get("/tokenswap")]
async fn get_tokenswap(db_conn: web::Data<Arc<Mutex<database::MongoRepo>>>) -> impl Responder {
    let conn = db_conn.lock().await;
    let result = MongoRepo::get_swap_data(&conn).await;
    let data = match result {
        Ok(response_str) => response_str,
        Err(_) => todo!(),
    };

    match serde_json::to_string(&data) {
        Ok(response_str) => response_str,
        Err(_) => "data Error".to_string(),
    }
}

#[get("/swap_events")]
async fn get_swap_events(db_conn: web::Data<Arc<Mutex<database::MongoRepo>>>) -> impl Responder {
    let conn = db_conn.lock().await;

    let result = MongoRepo::get_swap_events(&conn).await;
    let data = match result {
        Ok(response_str) => response_str,
        Err(_) => todo!(),
    };

    match serde_json::to_string(&data) {
        Ok(response_str) => response_str,
        Err(_) => "data Error".to_string(),
    }
}

#[get("/tokens")]
async fn get_tokens(db_conn: web::Data<Arc<Mutex<database::MongoRepo>>>) -> impl Responder {
    let conn = db_conn.lock().await;
    let result = MongoRepo::get_tokens(&conn).await;
    let data = match result {
        Ok(response_str) => response_str,
        Err(_) => todo!(),
    };

    match serde_json::to_string(&data) {
        Ok(response_str) => response_str,
        Err(_) => "data Error".to_string(),
    }
}

#[post("/contractjson")]
async fn contractjson(
    db_conn: web::Data<Arc<Mutex<database::MongoRepo>>>,
    mut payload: web::Payload,
) -> impl Responder {
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    // body is loaded, now we can deserialize serde-json
    let obj = serde_json::from_slice::<Vec<ContractJson>>(&body)?;

    // TODO get the event from db_conn
    let conn = db_conn.lock().await;
    let result = MongoRepo::insert_contract_json(&conn, obj).await;
    let data = match result {
        Ok(response_str) => response_str,
        Err(_) => todo!(),
    };

    match serde_json::to_string(&data.inserted_ids) {
        Ok(response_str) => Ok(response_str),
        Err(_) => Ok("data Error".to_string()),
    }
    // Ok(data.inserted_ids)
}

#[post("/tokenswap")]
async fn tokenswap(
    db_conn: web::Data<Arc<Mutex<database::MongoRepo>>>,
    mut payload: web::Payload,
) -> impl Responder {
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    // body is loaded, now we can deserialize serde-json
    let obj = serde_json::from_slice::<TokenSwap>(&body)?;
    // TODO get the event from db_conn
    let conn = db_conn.lock().await;
    let exist = MongoRepo::if_exist_swap_data(&conn, obj).await.unwrap();
    let obj2 = serde_json::from_slice::<TokenSwap>(&body)?;
    log::info!("{}:{:?} data to insert ", exist, obj2);
    if !exist {
        let obj = serde_json::from_slice::<TokenSwap>(&body)?;
        let result = MongoRepo::insert_single_swap_data(&conn, obj).await;
        let data = match result {
            Ok(response_str) => response_str,
            Err(_) => todo!(),
        };

        match serde_json::to_string(&data.inserted_id) {
            Ok(response_str) => Ok(response_str),
            Err(_) => Ok("data Error".to_string()),
        }
    } else {
        Ok("data already exist".to_string())
    }

    // Ok(data.inserted_ids)
}

// #[post("/swap_event")]
// async fn swap_event(
//     db_conn: web::Data<Arc<Mutex<database::MongoRepo>>>,
//     mut payload: web::Payload,
// ) -> impl Responder {
//     let mut body = web::BytesMut::new();
//     while let Some(chunk) = payload.next().await {
//         let chunk = chunk?;
//         // limit max size of in-memory payload
//         if (body.len() + chunk.len()) > MAX_SIZE {
//             return Err(error::ErrorBadRequest("overflow"));
//         }
//         body.extend_from_slice(&chunk);
//     }

//     // body is loaded, now we can deserialize serde-json
//     let obj = serde_json::from_slice::<Vec<SwapEvent>>(&body)?;

//     // TODO get the event from db_conn
//     let conn = db_conn.lock().await;
//     let result = MongoRepo::insert_swap_event(&conn, obj).await;
//     let data = match result {
//         Ok(response_str) => response_str,
//         Err(_) => todo!(),
//     };

//     match serde_json::to_string(&data.inserted_ids) {
//         Ok(response_str) => Ok(response_str),
//         Err(_) => Ok("data Error".to_string()),
//     }
//     // Ok(data.inserted_ids)
// }

#[post("/tokens")]
async fn tokens(
    db_conn: web::Data<Arc<Mutex<database::MongoRepo>>>,
    mut payload: web::Payload,
) -> impl Responder {
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    // body is loaded, now we can deserialize serde-json
    let obj = serde_json::from_slice::<InputToken>(&body)?;
    // let token_endpoint = format!("https://api.etherscan.io/api?module=contract&action=getabi&address={}&apikey=1M6T2FCU18IEG2K7D8EWFM5Z8CH6QEUESM",obj.token_address.to_string());
    let input_token = Token::new(
        obj.token.to_string(),
        obj.token_address.to_string(),
        obj.api_token_endpoint,
    );
    // TODO get the event from db_conn
    let conn = db_conn.lock().await;
    let result = MongoRepo::insert_tokens(&conn, input_token).await;
    let data = match result {
        Ok(response_str) => response_str,
        Err(_) => todo!(),
    };

    match serde_json::to_string(&data.inserted_id) {
        Ok(response_str) => Ok(response_str),
        Err(_) => Ok("data Error".to_string()),
    }
    // Ok(data.inserted_ids)
}

#[post("/removeContract")]
async fn remove_contract(
    db_conn: web::Data<Arc<Mutex<database::MongoRepo>>>,
    mut payload: web::Payload,
) -> impl Responder {
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    // body is loaded, now we can deserialize serde-json
    let data = serde_json::from_slice::<IdInterface>(&body)?;

    // TODO get the event from db_conn
    let conn = db_conn.lock().await;
    let result = MongoRepo::delete_contract(&conn, data.id).await;
    let data = match result {
        Ok(response_str) => response_str,
        Err(_) => todo!(),
    };

    match serde_json::to_string(&data.deleted_count) {
        Ok(response_str) => Ok(response_str),
        Err(_) => Ok("data Error".to_string()),
    }
    // Ok(data.inserted_ids)
}

lazy_static! {
    static ref HASHMAP: Mutex<HashMap<std::string::String, std::string::String>> = {
        let mempool_cache = HashMap::new();
        Mutex::new(mempool_cache)
    };
}

#[post("/exchange_rates")]
async fn exchangerates(
    db_conn: web::Data<Arc<Mutex<database::MongoRepo>>>,
    mut payload: web::Payload,
) -> impl Responder {
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    // body is loaded, now we can deserialize serde-json
    let obj = serde_json::from_slice::<InputTokenSwap>(&body)?;
    let conn = db_conn.lock().await;
    let mempool_key = format!(
        "{}_to_{}",
        obj.token.to_string(),
        obj.swap_token.to_string()
    );
    let mut map = HASHMAP.lock().await;
    let cache_result = map.get(&mempool_key);
    if cache_result.is_some() {
        let result_data = cache_result.clone().unwrap();
        let str = String::from(result_data);
        return Ok(str);
    }
    let swap_events_record =
        MongoRepo::get_swap_pair_price(obj.token.to_string(), obj.swap_token.to_string(), &conn)
            .await
            .unwrap_or_default();

    // let value = TokenSwapCon::swap_handler_return(&swap_instance, swap_input)
    //     .await
    //     .unwrap();

    let result = match serde_json::to_string(&swap_events_record) {
        Ok(response_str) => response_str,
        Err(_) => "data Error".to_string(),
    };
    map.insert(String::from(mempool_key), String::from(result.to_string()));
    return Ok(result);
}

#[get("/")]
async fn echo() -> impl Responder {
    HttpResponse::Ok().body("server is live")
}

pub async fn start_server(
    db_conn: Arc<Mutex<database::MongoRepo>>,
    ip: String,
    port: u16,
    origin: String,
    workers_num: usize,
) -> Result<(), std::io::Error> {
    // let args = Args::parse();

    // log::info!("{}:{} node start up ", args.ip, args.port);
    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&origin)
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::ACCEPT,
                http::header::ACCESS_CONTROL_ALLOW_ORIGIN,
            ])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(db_conn.clone()))
            .service(events)
            .service(get_contracts)
            .service(contractjson)
            .service(tokenswap)
            // .service(swap_event)
            .service(exchangerates)
            .service(remove_contract)
            .service(get_contract_events)
            .service(get_tokenswap)
            .service(get_swap_events)
            .service(tokens)
            .service(get_tokens)
            .service(echo)
    })
    .workers(workers_num)
    .bind((ip, port))?
    .run();
    server.await.unwrap();
    Ok(())
}
