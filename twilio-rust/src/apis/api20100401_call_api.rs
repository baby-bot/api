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


/// struct for typed errors of method [`create_call`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateCallError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_call`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteCallError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`fetch_call`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchCallError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_call`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListCallError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_call`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateCallError {
    UnknownValue(serde_json::Value),
}


/// Create a new outgoing call to phones, SIP-enabled endpoints or Twilio Client connections
pub async fn create_call(configuration: &configuration::Configuration, account_sid: &str, to: &str, from: &str, method: Option<&str>, fallback_url: Option<&str>, fallback_method: Option<&str>, status_callback: Option<&str>, status_callback_event: Option<Vec<String>>, status_callback_method: Option<&str>, send_digits: Option<&str>, timeout: Option<i32>, record: Option<bool>, recording_channels: Option<&str>, recording_status_callback: Option<&str>, recording_status_callback_method: Option<&str>, sip_auth_username: Option<&str>, sip_auth_password: Option<&str>, machine_detection: Option<&str>, machine_detection_timeout: Option<i32>, recording_status_callback_event: Option<Vec<String>>, trim: Option<&str>, caller_id: Option<&str>, machine_detection_speech_threshold: Option<i32>, machine_detection_speech_end_threshold: Option<i32>, machine_detection_silence_timeout: Option<i32>, async_amd: Option<&str>, async_amd_status_callback: Option<&str>, async_amd_status_callback_method: Option<&str>, byoc: Option<&str>, call_reason: Option<&str>, call_token: Option<&str>, recording_track: Option<&str>, time_limit: Option<i32>, url: Option<&str>, twiml: Option<&str>, application_sid: Option<&str>) -> Result<crate::models::ApiPeriodV2010PeriodAccountPeriodCall, Error<CreateCallError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/2010-04-01/Accounts/{AccountSid}/Calls.json", local_var_configuration.base_path, AccountSid=crate::apis::urlencode(account_sid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    local_var_form_params.insert("To", to.to_string());
    local_var_form_params.insert("From", from.to_string());
    if let Some(local_var_param_value) = method {
        local_var_form_params.insert("Method", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = fallback_url {
        local_var_form_params.insert("FallbackUrl", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = fallback_method {
        local_var_form_params.insert("FallbackMethod", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = status_callback {
        local_var_form_params.insert("StatusCallback", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = status_callback_event {
        local_var_form_params.insert("StatusCallbackEvent", local_var_param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string());
    }
    if let Some(local_var_param_value) = status_callback_method {
        local_var_form_params.insert("StatusCallbackMethod", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = send_digits {
        local_var_form_params.insert("SendDigits", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = timeout {
        local_var_form_params.insert("Timeout", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = record {
        local_var_form_params.insert("Record", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = recording_channels {
        local_var_form_params.insert("RecordingChannels", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = recording_status_callback {
        local_var_form_params.insert("RecordingStatusCallback", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = recording_status_callback_method {
        local_var_form_params.insert("RecordingStatusCallbackMethod", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = sip_auth_username {
        local_var_form_params.insert("SipAuthUsername", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = sip_auth_password {
        local_var_form_params.insert("SipAuthPassword", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = machine_detection {
        local_var_form_params.insert("MachineDetection", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = machine_detection_timeout {
        local_var_form_params.insert("MachineDetectionTimeout", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = recording_status_callback_event {
        local_var_form_params.insert("RecordingStatusCallbackEvent", local_var_param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string());
    }
    if let Some(local_var_param_value) = trim {
        local_var_form_params.insert("Trim", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = caller_id {
        local_var_form_params.insert("CallerId", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = machine_detection_speech_threshold {
        local_var_form_params.insert("MachineDetectionSpeechThreshold", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = machine_detection_speech_end_threshold {
        local_var_form_params.insert("MachineDetectionSpeechEndThreshold", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = machine_detection_silence_timeout {
        local_var_form_params.insert("MachineDetectionSilenceTimeout", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = async_amd {
        local_var_form_params.insert("AsyncAmd", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = async_amd_status_callback {
        local_var_form_params.insert("AsyncAmdStatusCallback", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = async_amd_status_callback_method {
        local_var_form_params.insert("AsyncAmdStatusCallbackMethod", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = byoc {
        local_var_form_params.insert("Byoc", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = call_reason {
        local_var_form_params.insert("CallReason", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = call_token {
        local_var_form_params.insert("CallToken", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = recording_track {
        local_var_form_params.insert("RecordingTrack", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = time_limit {
        local_var_form_params.insert("TimeLimit", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = url {
        local_var_form_params.insert("Url", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = twiml {
        local_var_form_params.insert("Twiml", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = application_sid {
        local_var_form_params.insert("ApplicationSid", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateCallError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete a Call record from your account. Once the record is deleted, it will no longer appear in the API and Account Portal logs.
pub async fn delete_call(configuration: &configuration::Configuration, account_sid: &str, sid: &str) -> Result<(), Error<DeleteCallError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/2010-04-01/Accounts/{AccountSid}/Calls/{Sid}.json", local_var_configuration.base_path, AccountSid=crate::apis::urlencode(account_sid), Sid=crate::apis::urlencode(sid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DeleteCallError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Fetch the call specified by the provided Call SID
pub async fn fetch_call(configuration: &configuration::Configuration, account_sid: &str, sid: &str) -> Result<crate::models::ApiPeriodV2010PeriodAccountPeriodCall, Error<FetchCallError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/2010-04-01/Accounts/{AccountSid}/Calls/{Sid}.json", local_var_configuration.base_path, AccountSid=crate::apis::urlencode(account_sid), Sid=crate::apis::urlencode(sid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<FetchCallError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieves a collection of calls made to and from your account
pub async fn list_call(configuration: &configuration::Configuration, account_sid: &str, to: Option<&str>, from: Option<&str>, parent_call_sid: Option<&str>, status: Option<&str>, start_time: Option<String>, start_time_less_than: Option<String>, start_time_greater_than: Option<String>, end_time: Option<String>, end_time_less_than: Option<String>, end_time_greater_than: Option<String>, page_size: Option<i32>, page: Option<i32>, page_token: Option<&str>) -> Result<crate::models::ListCallResponse, Error<ListCallError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/2010-04-01/Accounts/{AccountSid}/Calls.json", local_var_configuration.base_path, AccountSid=crate::apis::urlencode(account_sid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = to {
        local_var_req_builder = local_var_req_builder.query(&[("To", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = from {
        local_var_req_builder = local_var_req_builder.query(&[("From", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = parent_call_sid {
        local_var_req_builder = local_var_req_builder.query(&[("ParentCallSid", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = status {
        local_var_req_builder = local_var_req_builder.query(&[("Status", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = start_time {
        local_var_req_builder = local_var_req_builder.query(&[("StartTime", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = start_time_less_than {
        local_var_req_builder = local_var_req_builder.query(&[("StartTime<", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = start_time_greater_than {
        local_var_req_builder = local_var_req_builder.query(&[("StartTime>", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = end_time {
        local_var_req_builder = local_var_req_builder.query(&[("EndTime", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = end_time_less_than {
        local_var_req_builder = local_var_req_builder.query(&[("EndTime<", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = end_time_greater_than {
        local_var_req_builder = local_var_req_builder.query(&[("EndTime>", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_size {
        local_var_req_builder = local_var_req_builder.query(&[("PageSize", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("Page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_token {
        local_var_req_builder = local_var_req_builder.query(&[("PageToken", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ListCallError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Initiates a call redirect or terminates a call
pub async fn update_call(configuration: &configuration::Configuration, account_sid: &str, sid: &str, url: Option<&str>, method: Option<&str>, status: Option<&str>, fallback_url: Option<&str>, fallback_method: Option<&str>, status_callback: Option<&str>, status_callback_method: Option<&str>, twiml: Option<&str>, time_limit: Option<i32>) -> Result<crate::models::ApiPeriodV2010PeriodAccountPeriodCall, Error<UpdateCallError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/2010-04-01/Accounts/{AccountSid}/Calls/{Sid}.json", local_var_configuration.base_path, AccountSid=crate::apis::urlencode(account_sid), Sid=crate::apis::urlencode(sid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    if let Some(local_var_param_value) = url {
        local_var_form_params.insert("Url", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = method {
        local_var_form_params.insert("Method", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = status {
        local_var_form_params.insert("Status", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = fallback_url {
        local_var_form_params.insert("FallbackUrl", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = fallback_method {
        local_var_form_params.insert("FallbackMethod", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = status_callback {
        local_var_form_params.insert("StatusCallback", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = status_callback_method {
        local_var_form_params.insert("StatusCallbackMethod", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = twiml {
        local_var_form_params.insert("Twiml", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = time_limit {
        local_var_form_params.insert("TimeLimit", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateCallError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}
