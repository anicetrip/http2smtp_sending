use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use secrecy::Secret;

use crate::{configuration::get_configuration, service::email_send::send_email, email_content::{EmailContent, Content}};
///restful api start place
#[post("/")]
pub async fn email_api(request: HttpRequest, content: web::Json<Content>) -> impl Responder {
    let req_headers = request.headers();

    let pass_head = req_headers
        .get(get_configuration().unwrap().pass_header)
        .unwrap()
        .to_str()
        .unwrap();
    let content = content.into_inner();
    let email_content = EmailContent{
        From:content.From,
        password: Secret::new(pass_head.to_string()),
        Subject:content.Subject,
        To:content.To,
        TextBody:content.TextBody,
        HtmlBody:content.HtmlBody,
    };
    let a = send_email(email_content).await;
    // Ok(web::Json(obj))
    HttpResponse::Ok().json(web::Json(a))
}

// This is a reminder if I need to scan what's in the head.
// let key_list = req_headers.keys();
// for key in key_list{
//     let a = key.as_str();
//     println!("{}:{}",a,req_headers.get(a).unwrap().to_str().unwrap());
// }
