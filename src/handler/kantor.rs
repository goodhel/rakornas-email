use tide::{Request, Response};
use sqlx::PgPool;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Jabatan {
    id: i32,
    nama: String
}

pub async fn list(req: Request<PgPool>) -> tide::Result<Response> {
    let pool = req.state();

    let list_jabatan = sqlx::query_as!( Jabatan,
        "SELECT id, nama FROM jabatan" )
        .fetch_all(pool) .await?;

    crate::to_json(&list_jabatan)
}

// http://127.0.0.1:8181/pengaturan/jabatan
pub async fn tambah(mut req: Request<PgPool>) -> tide::Result<Response> {
    let jabatan: Jabatan = req.body_json().await?;
    let pool = req.state();
 
    let _jabatan = sqlx::query!(
        "INSERT INTO jabatan (nama) VALUES ($1)", jabatan.nama)
        .execute(pool) .await?;
 
    crate::ws_response("OK", "Data telah tersimpan")
 }