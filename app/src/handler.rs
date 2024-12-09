use axum::{Extension, Form};
use serde::Deserialize;

use crate::ExtState;

#[derive(Deserialize, Debug)]
pub struct WebhookData {
    pub ChannelPrefix: Option<String>,
    pub ApiVersion: Option<String>,
    pub MessageStatus: Option<String>,
    pub SmsSid: Option<String>,
    pub SmsStatus: Option<String>,
    pub ChannelInstallSid: Option<String>,
    pub To: Option<String>,
    pub From: Option<String>,
    pub MessageSid: Option<String>,
    pub StructuredMessage: Option<bool>,
    pub AccountSid: Option<String>,
    pub ChannelToAddress: Option<String>,
}

pub async fn handle_twilio_webhook_status(
    Extension(app_state): ExtState,
    Form(form): Form<WebhookData>,
) {
    let db = app_state.db;
    println!("{:?}", form);
}

#[derive(Deserialize, Debug)]
pub struct WebhookPayload {
    pub SmsMessageSid: String,
    pub NumMedia: String,
    pub ProfileName: String,
    pub MessageType: String,
    pub SmsSid: String,
    pub WaId: String,
    pub SmsStatus: String,
    pub Body: String,
    pub ButtonText: String,
    pub To: String,
    pub ButtonPayload: String,
    pub NumSegments: String,
    pub ReferralNumMedia: String,
    pub MessageSid: String,
    pub AccountSid: String,
    pub From: String,
    pub ApiVersion: String,
}

pub async fn handle_twilio_webhook_payload(
    Extension(app_state): ExtState,
    Form(form): Form<WebhookPayload>,
) {
    println!("{:?}", form);
}
