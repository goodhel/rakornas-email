use tide::{Request, Response};
use sqlx::PgPool;
use lettre::transport::smtp::{authentication::Credentials};
use lettre::{Message, SmtpTransport, Transport};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Email {
    nama: String,
    message: String,
    email: String,
    notif: String
}

pub async fn send_email(mut req: Request<PgPool>) -> tide::Result<Response> {
    // let pool = req.state();
    let data: Email = req.body_json().await?;
    let to = format!("{} <{}>",data.nama,data.email);
    let email = Message::builder()
    .from("Virtual IAI <virtualiai2020@gmail.com>".parse().unwrap())
    .to(to.parse().unwrap())
    .subject(data.notif)
    .body(data.message)
    .unwrap();

    let creds = Credentials::new("virtualiai2020@gmail.com".to_string(), "Virtualiai-2020".to_string());
    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
    .unwrap()
    .credentials(creds)
    .build();
    // Tunnel Connection SMTP
    // let mailer = SmtpTransport::relay("127.0.0.1")
    // .unwrap()
    // .credentials(creds)
    // .port(10587)
    // .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => {
            println!("Email sent successfully!");
            crate::ws_response("OK", "Email berhasil dikirim")
        },
        Err(e) => {
            panic!("Could not send email: {:?}", e);
            // crate::ws_response("Error", "Email gagal dikirim")
        },
    }
}