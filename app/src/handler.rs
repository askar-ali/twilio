use axum::{
    response::IntoResponse,
    http::StatusCode,
    Extension,
    Form
};
use serde::Deserialize;
use sqlx::PgPool;
use tracing::{info, debug};

use crate::ExtState;

#[derive(Deserialize, Debug)]
pub struct WebhookData {
    pub MessageStatus: Option<String>,
    pub SmsSid: Option<String>,
    pub To: Option<String>,
    pub From: Option<String>,
    pub AccountSid: Option<String>,
}

// #[derive(Deserialize, Debug)]
// pub struct WebhookPayload {
//     pub SmsMessageSid: String,
//     pub NumMedia: String,
//     pub ProfileName: String,
//     pub MessageType: String,
//     pub SmsSid: String,
//     pub WaId: String,
//     pub SmsStatus: String,
//     pub Body: String,
//     pub ButtonText: String,
//     pub To: String,
//     pub ButtonPayload: String,
//     pub NumSegments: String,
//     pub ReferralNumMedia: String,
//     pub MessageSid: String,
//     pub AccountSid: String,
//     pub From: String,
//     pub ApiVersion: String,
// }

// pub async fn handle_twilio_webhook_payload(
//     Extension(app_state): ExtState,
//     Form(form): Form<WebhookPayload>,
// ) {
//     println!("{:?}", form);
// }


pub async fn handle_twilio_webhook_status(
    Extension(app_state): Extension<ExtState>, // Correct the type for Extension
    Form(form): Form<WebhookData>,
) -> Result<impl IntoResponse, StatusCode> {
    let db: &PgPool = &app_state.db;
    let WebhookData {
        SmsSid,
        From,
        To,
        AccountSid,
        MessageStatus,
        ..
    } = form;

    info!("Received webhook data: SmsSid: {:?}, MessageStatus: {:?}", SmsSid, MessageStatus);

    // Validate required fields
    if SmsSid.is_none() || MessageStatus.is_none() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let query_main = r#"
        INSERT INTO twilio_webhook_logs (
            sms_sid, from_phone, to_phone, account_sid, latest_status
        ) VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (sms_sid)
        DO UPDATE SET
            latest_status = EXCLUDED.latest_status,
            updated_at = CURRENT_TIMESTAMP
    "#;

    info!("Inserting/Updating twilio_webhook_logs for SmsSid: {:?}", SmsSid);

    sqlx::query(query_main)
        .bind(SmsSid.as_deref())
        .bind(From.as_deref())
        .bind(To.as_deref())
        .bind(AccountSid.as_deref())
        .bind(MessageStatus.as_deref())
        .execute(db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let query_status = r#"
        INSERT INTO twilio_message_status (sms_sid, message_status)
        VALUES ($1, $2)
    "#;

    info!("Inserting message status for SmsSid: {:?}, MessageStatus: {:?}", SmsSid, MessageStatus);

    sqlx::query(query_status)
        .bind(SmsSid.as_deref())
        .bind(MessageStatus.as_deref())
        .execute(db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    debug!("Successfully logged message status for SmsSid: {:?}", SmsSid);

    Ok((StatusCode::OK, "Message status logged"))
}