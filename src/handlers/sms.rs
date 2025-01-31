use crate::models::response::ApiResponse;
use crate::models::sms::{SMSRequest, SMSTemplate};
use crate::models::user::User;
use actix_web::{web, HttpResponse, Responder};
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use rand::Rng;
use chrono::Utc;

pub async fn generate_otp(
    db: web::Data<Surreal<Client>>,
    request: web::Json<SMSRequest>,
) -> impl Responder {
    // Extract values with detailed logging
    let phone_number = &request.phone_number;
    let template_id = request.template_id.replace("sms_template:", "");
    let user_id = request.user_id.clone(); // Don't remove prefix for storage

    log::info!(
        "Processing OTP generation: user_id={}, phone_number={}, template_id={}",
        user_id, phone_number, request.template_id
    );

    // Step 1: Fetch Template First
    let template_query = "SELECT * FROM type::thing('sms_template', $id)";
    let template_result = db
        .query(template_query)
        .bind(("id", template_id.clone()))
        .await;

    log::info!("Template query executed: {}", template_query);

    let template = match template_result {
        Ok(mut response) => {
            let template = response.take::<Option<SMSTemplate>>(0).unwrap_or(None);
            log::info!("Template found: {:?}", template);
            template
        }
        Err(e) => {
            log::error!("Error fetching SMS template: {}", e);
            return HttpResponse::InternalServerError().json(ApiResponse::<()> {
                status: "error".to_string(),
                message: format!("Failed to fetch SMS template: {}", e),
                data: None,
            });
        }
    };

    if template.is_none() {
        println!("masuk 1");
        log::warn!("Template not found for ID: {}", template_id);
        return HttpResponse::BadRequest().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: "Template not found".to_string(),
            data: None,
        });
    }

    let template = template.unwrap();

    // Step 2: Generate OTP
    let otp_code: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Uniform::new(0, 10))
        .take(6)
        .map(|d| d.to_string())
        .collect();

    log::info!("Generated OTP code: {}", otp_code);
    println!("masuk 2");
    // Replace OTP in content
    let content = template.content.replace("{{otp}}", &otp_code);
    let now = Utc::now();
    let created_at = now.to_rfc3339();

    // Log for debugging user_id binding
    log::info!("Binding user_id: {}", user_id);

    // Step 3: Try direct insert into inbox_message with user_id correctly bound
    let inbox_query = r#"
        CREATE inbox_message SET
            content = $content,
            content_type = 'text/plain',
            created_at = time::now(),
            created_by = $created_by,
            judul = $judul,
            updated_at = time::now(),
            user_id = $user_id
        RETURN *
    "#;

    log::info!("Executing inbox query: {}", inbox_query);
    log::info!("With parameters: content={}, created_by={}, judul={}, user_id={}", 
        content, template.judul, user_id, template.created_by);

        println!("masuk 3");
    // Try to create inbox message directly first
    let inbox_result = db
        .query(inbox_query)
        .bind(("content", content.clone()))
        .bind(("judul", template.judul.clone()))
        .bind(("created_by", template.created_by.clone()))
        .bind(("user_id", user_id.clone()))
        .await;

    match &inbox_result {
        Ok(response) => {
            log::info!("Inbox creation response: {:?}", response);
        }
        Err(e) => {
            log::error!("Failed to create inbox message. Error details: {:?}", e);
        }
    }

    // Step 4: Create OTP in Database
    let otp_query = r#"
        CREATE otp SET 
            user_id = $user_id,
            code = $code,
            expires_at = time::now() + 5m,
            is_used = false,
            created_at = time::now()
        RETURN *
    "#;

    let otp_result = db
        .query(otp_query)
        .bind(("user_id", user_id.clone()))
        .bind(("code", otp_code.clone()))
        .await;

    match &otp_result {
        Ok(response) => {
            log::info!("OTP creation response: {:?}", response);
        }
        Err(e) => {
            log::error!("Failed to create OTP. Error details: {:?}", e);
        }
    }

    // Step 5: Verify that the inbox_message is correctly stored with user_id
    let verify_query = "SELECT * FROM inbox_message WHERE user_id = $user_id ORDER BY created_at DESC LIMIT 1";
    
    match db
        .query(verify_query)
        .bind(("user_id", user_id.clone()))
        .await
    {
        Ok(mut result) => {
            log::info!("Verification found messages: {:?}", result.take::<Vec<serde_json::Value>>(0));
        }
        Err(e) => {
            log::error!("Verification query failed: {}", e);
        }
    }

    // Step 6: Fetch all inbox messages for debugging
    let all_messages = db.query("SELECT * FROM inbox_message").await;
    match all_messages {
        Ok(mut result) => {
            log::info!("All inbox messages: {:?}", result.take::<Vec<serde_json::Value>>(0));
        }
        Err(e) => {
            log::error!("Failed to fetch all messages: {}", e);
        }
    }

    HttpResponse::Ok().json(ApiResponse::<()> {
        status: "success".to_string(),
        message: "OTP generated and sent to inbox successfully".to_string(),
        data: None,
    })
}
