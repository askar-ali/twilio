-- 
CREATE TABLE twilio_webhook_logs (
    id SERIAL PRIMARY KEY,
    sms_sid VARCHAR(50) UNIQUE NOT NULL,
    from_phone VARCHAR(20) NOT NULL,
    to_phone VARCHAR(20) NOT NULL, 
    account_sid VARCHAR(50) NOT NULL,
    latest_status VARCHAR(20),
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE twilio_message_status (
    id SERIAL PRIMARY KEY,
    sms_sid VARCHAR(50) NOT NULL REFERENCES twilio_webhook_logs(sms_sid) ON DELETE CASCADE,
    message_status VARCHAR(20) NOT NULL,
    status_updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
