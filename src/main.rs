use anyhow::{anyhow, Result};
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3 as s3;
use aws_sdk_s3::presigning::config::PresigningConfig;
use aws_smithy_http::endpoint::Endpoint;
use axum::{
    extract::{Extension, Multipart, Path},
    http::Method,
    http::StatusCode,
    http::Uri,
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};
use s3::{types::ByteStream, Client};
use serde::Deserialize;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tower_http::cors::{Any, CorsLayer};

use std::collections::BTreeMap;
use surrealdb::{
    sql::{Object, Value},
    Response,
};
use surrealdb::{Datastore, Session};

type DB = (Datastore, Session);

#[derive(Clone)]
pub struct AwsInfo {
    client: Client,
    bucket_name: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let db: Arc<DB> = Arc::new((
        // file://temp.db memory
        Datastore::new("file://temp.db").await?,
        Session::for_db("oort_hackathon", "oort_storage"),
    ));

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any)
        .allow_origin(Any);

    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env()
        .region(region_provider)
        .endpoint_resolver(Endpoint::immutable(Uri::from_static(
            "https://s3-storj.oortech.com",
        )))
        .load()
        .await;

    let client = s3::Client::new(&config);
    let new_bucket_name = "oort-hackathon";

    let life_time_client = Arc::new(AwsInfo {
        client,
        bucket_name: new_bucket_name.to_string(),
    });

    let api_routes = Router::new()
        .route("/health_check", get(health_check))
        .route("/user_nft_items/:address", get(get_address_info))
        .route("/list_all", post(list_objects))
        .route("/object_uri", post(get_object_uri))
        .route("/create", post(item_upload))
        .route("/delete_item", delete(delete_upload))
        .layer(Extension(life_time_client))
        .layer(Extension(db))
        .layer(cors);

    let app = Router::new().nest("/api/v1", api_routes);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, Json("All healthy here!"))
}

#[derive(Deserialize)]
pub struct ListInfo {
    name: String,
}

async fn list_objects(
    Json(payload): Json<ListInfo>,
    Extension(awsinfo): Extension<Arc<AwsInfo>>,
) -> impl IntoResponse {
    let res = awsinfo
        .client
        .list_objects_v2()
        .prefix(payload.name)
        .delimiter("/")
        .bucket(awsinfo.bucket_name.clone())
        .send()
        .await
        .unwrap();

    let keys = res.contents().unwrap_or_default();

    let keys = keys
        .iter()
        .filter_map(|o| o.key.as_ref())
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    match keys.is_empty() {
        true => (StatusCode::NOT_FOUND, Json(keys)),
        false => (StatusCode::OK, Json(keys)),
    }
}

// #[axum_macros::debug_handler]
async fn get_address_info(
    Path(address): Path<String>,
    Extension(db): Extension<Arc<DB>>,
) -> impl IntoResponse {
    let ds = &db.0;
    let ses = &db.1;

    let sql = "SELECT * FROM nftMarketItem WHERE address = $address";
    let vars: BTreeMap<String, Value> = [("address".into(), address.into())].into();

    let res = ds
        .execute(sql, ses, Some(vars), false)
        .await
        .expect("Error trying to retrive data");

    let mut res_obj = Vec::new();

    for object in into_iter_objects(res).unwrap() {
        res_obj.push(object.unwrap())
    }

    (StatusCode::OK, Json(res_obj))
}

fn into_iter_objects(ress: Vec<Response>) -> Result<impl Iterator<Item = Result<Object>>> {
    let res = ress.into_iter().next().map(|rp| rp.result).transpose()?;

    match res {
        Some(Value::Array(arr)) => {
            let it = arr.into_iter().map(|v| match v {
                Value::Object(object) => Ok(object),
                _ => Err(anyhow!("A error with the record")),
            });
            Ok(it)
        }
        _ => Err(anyhow!("No record was found")),
    }
}

#[derive(Deserialize)]
pub struct UrlInfo {
    address: String,
    contract_address: String,
    metadata: BTreeMap<String, Value>,
    project_name: String,
    object_name: String,
}

async fn get_object_uri(
    Json(payload): Json<UrlInfo>,
    Extension(awsinfo): Extension<Arc<AwsInfo>>,
    Extension(db): Extension<Arc<DB>>,
) -> impl IntoResponse {
    let ds = &db.0;
    let ses = &db.1;

    let key_name = format!("{}/{}", payload.project_name, payload.object_name);
    let expries_in = Duration::from_secs(36000);

    let presigned_request = &awsinfo
        .client
        .get_object()
        .bucket(&awsinfo.bucket_name.clone())
        .key(key_name)
        .presigned(PresigningConfig::expires_in(expries_in).expect("Problem with the expires time"))
        .await;

    match presigned_request {
        Ok(presigned) => {
            let time = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap();

            let sql = "CREATE nftMarketItem CONTENT $data";

            let data: BTreeMap<String, Value> = [
                ("address".into(), payload.address.into()),
                ("contract_address".into(), payload.contract_address.into()),
                ("metadata".into(), payload.metadata.into()),
                ("blocktime".into(), time.as_secs().to_string().into()),
                ("file".into(), presigned.uri().to_string().into()),
            ]
            .into();

            let vars: BTreeMap<String, Value> = [("data".into(), data.into())].into();

            let ress = ds
                .execute(sql, ses, Some(vars), false)
                .await
                .expect("Did not store in db");

            println!("{:?}", ress);

            (StatusCode::OK, Json(presigned.uri().to_string())).into_response()
        }
        Err(err) => (StatusCode::NOT_FOUND, Json(err.to_string())).into_response(),
    }
}

async fn item_upload(
    mut multipart: Multipart,
    Extension(awsinfo): Extension<Arc<AwsInfo>>,
) -> impl IntoResponse {
    let mut project_name: String = awsinfo.bucket_name.clone();

    let mut msg_and_name: Vec<String> = Vec::new();

    while let Some(field) = multipart.next_field().await.unwrap() {
        if field.file_name().is_some() {
            let file_name = field.file_name().unwrap().to_string();
            let data = field.bytes().await.unwrap();
            let file_name_for_err = file_name.clone();
            let body: ByteStream = data.into();

            let upload_to_oort = &awsinfo
                .client
                .put_object()
                .bucket(&project_name)
                .key(file_name)
                .body(body)
                .send()
                .await;

            match upload_to_oort {
                Ok(upload_success) => msg_and_name.push(format!(
                    "{} file was successfully uploaded!",
                    upload_success.e_tag().unwrap_or_default().to_string()
                )),
                Err(upload_err) => msg_and_name.push(format!(
                    "{} error uploading file | Error: {}",
                    file_name_for_err, upload_err
                )),
            }
        } else {
            let folder_name = format!("/{}", &field.text().await.unwrap().to_string());
            project_name.push_str(&folder_name);
        }
    }

    (StatusCode::CREATED, Json(msg_and_name))
}

#[derive(Deserialize)]
pub struct DeleteItem {
    project_name: String,
    item_name: String,
}

async fn delete_upload(
    Json(payload): Json<DeleteItem>,
    Extension(awsinfo): Extension<Arc<AwsInfo>>,
) -> impl IntoResponse {
    let project_name = format!("{}/{}", awsinfo.bucket_name, payload.project_name);

    let _delete_item = awsinfo
        .client
        .delete_object()
        .bucket(project_name)
        .key(payload.item_name)
        .send()
        .await
        .unwrap();

    (StatusCode::OK, Json("Success deleting item"))
}
