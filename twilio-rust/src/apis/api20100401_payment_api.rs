/*
 * Twilio - Api
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.54.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`create_payments`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreatePaymentsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_payments`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdatePaymentsError {
    UnknownValue(serde_json::Value),
}


/// create an instance of payments. This will start a new payments session
pub async fn create_payments(configuration: &configuration::Configuration, account_sid: &str, call_sid: &str, idempotency_key: &str, status_callback: &str, bank_account_type: Option<&str>, charge_amount: Option<f32>, currency: Option<&str>, description: Option<&str>, input: Option<&str>, min_postal_code_length: Option<i32>, parameter: Option<serde_json::Value>, payment_connector: Option<&str>, payment_method: Option<&str>, postal_code: Option<bool>, security_code: Option<bool>, timeout: Option<i32>, token_type: Option<&str>, valid_card_types: Option<&str>) -> Result<crate::models::ApiPeriodV2010PeriodAccountPeriodCallPeriodPayments, Error<CreatePaymentsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Payments.json", local_var_configuration.base_path, AccountSid=crate::apis::urlencode(account_sid), CallSid=crate::apis::urlencode(call_sid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    local_var_form_params.insert("IdempotencyKey", idempotency_key.to_string());
    local_var_form_params.insert("StatusCallback", status_callback.to_string());
    if let Some(local_var_param_value) = bank_account_type {
        local_var_form_params.insert("BankAccountType", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = charge_amount {
        local_var_form_params.insert("ChargeAmount", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = currency {
        local_var_form_params.insert("Currency", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = description {
        local_var_form_params.insert("Description", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = input {
        local_var_form_params.insert("Input", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = min_postal_code_length {
        local_var_form_params.insert("MinPostalCodeLength", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = parameter {
        local_var_form_params.insert("Parameter", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = payment_connector {
        local_var_form_params.insert("PaymentConnector", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = payment_method {
        local_var_form_params.insert("PaymentMethod", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = postal_code {
        local_var_form_params.insert("PostalCode", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = security_code {
        local_var_form_params.insert("SecurityCode", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = timeout {
        local_var_form_params.insert("Timeout", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = token_type {
        local_var_form_params.insert("TokenType", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = valid_card_types {
        local_var_form_params.insert("ValidCardTypes", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreatePaymentsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// update an instance of payments with different phases of payment flows.
pub async fn update_payments(configuration: &configuration::Configuration, account_sid: &str, call_sid: &str, sid: &str, idempotency_key: &str, status_callback: &str, capture: Option<&str>, status: Option<&str>) -> Result<crate::models::ApiPeriodV2010PeriodAccountPeriodCallPeriodPayments, Error<UpdatePaymentsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Payments/{Sid}.json", local_var_configuration.base_path, AccountSid=crate::apis::urlencode(account_sid), CallSid=crate::apis::urlencode(call_sid), Sid=crate::apis::urlencode(sid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    local_var_form_params.insert("IdempotencyKey", idempotency_key.to_string());
    local_var_form_params.insert("StatusCallback", status_callback.to_string());
    if let Some(local_var_param_value) = capture {
        local_var_form_params.insert("Capture", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = status {
        local_var_form_params.insert("Status", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdatePaymentsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}
