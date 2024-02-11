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


/// struct for typed errors of method [`create_incoming_phone_number_mobile`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateIncomingPhoneNumberMobileError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_available_phone_number_mobile`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAvailablePhoneNumberMobileError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_incoming_phone_number_mobile`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListIncomingPhoneNumberMobileError {
    UnknownValue(serde_json::Value),
}


/// 
pub async fn create_incoming_phone_number_mobile(configuration: &configuration::Configuration, account_sid: &str, phone_number: &str, api_version: Option<&str>, friendly_name: Option<&str>, sms_application_sid: Option<&str>, sms_fallback_method: Option<&str>, sms_fallback_url: Option<&str>, sms_method: Option<&str>, sms_url: Option<&str>, status_callback: Option<&str>, status_callback_method: Option<&str>, voice_application_sid: Option<&str>, voice_caller_id_lookup: Option<bool>, voice_fallback_method: Option<&str>, voice_fallback_url: Option<&str>, voice_method: Option<&str>, voice_url: Option<&str>, identity_sid: Option<&str>, address_sid: Option<&str>, emergency_status: Option<&str>, emergency_address_sid: Option<&str>, trunk_sid: Option<&str>, voice_receive_mode: Option<&str>, bundle_sid: Option<&str>) -> Result<crate::models::ApiPeriodV2010PeriodAccountPeriodIncomingPhoneNumberPeriodIncomingPhoneNumberMobile, Error<CreateIncomingPhoneNumberMobileError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/Mobile.json", local_var_configuration.base_path, AccountSid=crate::apis::urlencode(account_sid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    local_var_form_params.insert("PhoneNumber", phone_number.to_string());
    if let Some(local_var_param_value) = api_version {
        local_var_form_params.insert("ApiVersion", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = friendly_name {
        local_var_form_params.insert("FriendlyName", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = sms_application_sid {
        local_var_form_params.insert("SmsApplicationSid", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = sms_fallback_method {
        local_var_form_params.insert("SmsFallbackMethod", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = sms_fallback_url {
        local_var_form_params.insert("SmsFallbackUrl", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = sms_method {
        local_var_form_params.insert("SmsMethod", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = sms_url {
        local_var_form_params.insert("SmsUrl", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = status_callback {
        local_var_form_params.insert("StatusCallback", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = status_callback_method {
        local_var_form_params.insert("StatusCallbackMethod", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = voice_application_sid {
        local_var_form_params.insert("VoiceApplicationSid", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = voice_caller_id_lookup {
        local_var_form_params.insert("VoiceCallerIdLookup", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = voice_fallback_method {
        local_var_form_params.insert("VoiceFallbackMethod", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = voice_fallback_url {
        local_var_form_params.insert("VoiceFallbackUrl", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = voice_method {
        local_var_form_params.insert("VoiceMethod", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = voice_url {
        local_var_form_params.insert("VoiceUrl", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = identity_sid {
        local_var_form_params.insert("IdentitySid", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = address_sid {
        local_var_form_params.insert("AddressSid", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = emergency_status {
        local_var_form_params.insert("EmergencyStatus", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = emergency_address_sid {
        local_var_form_params.insert("EmergencyAddressSid", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = trunk_sid {
        local_var_form_params.insert("TrunkSid", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = voice_receive_mode {
        local_var_form_params.insert("VoiceReceiveMode", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = bundle_sid {
        local_var_form_params.insert("BundleSid", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateIncomingPhoneNumberMobileError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// 
pub async fn list_available_phone_number_mobile(configuration: &configuration::Configuration, account_sid: &str, country_code: &str, area_code: Option<i32>, contains: Option<&str>, sms_enabled: Option<bool>, mms_enabled: Option<bool>, voice_enabled: Option<bool>, exclude_all_address_required: Option<bool>, exclude_local_address_required: Option<bool>, exclude_foreign_address_required: Option<bool>, beta: Option<bool>, near_number: Option<&str>, near_lat_long: Option<&str>, distance: Option<i32>, in_postal_code: Option<&str>, in_region: Option<&str>, in_rate_center: Option<&str>, in_lata: Option<&str>, in_locality: Option<&str>, fax_enabled: Option<bool>, page_size: Option<i32>, page: Option<i32>, page_token: Option<&str>) -> Result<crate::models::ListAvailablePhoneNumberMobileResponse, Error<ListAvailablePhoneNumberMobileError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/2010-04-01/Accounts/{AccountSid}/AvailablePhoneNumbers/{CountryCode}/Mobile.json", local_var_configuration.base_path, AccountSid=crate::apis::urlencode(account_sid), CountryCode=crate::apis::urlencode(country_code));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = area_code {
        local_var_req_builder = local_var_req_builder.query(&[("AreaCode", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = contains {
        local_var_req_builder = local_var_req_builder.query(&[("Contains", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sms_enabled {
        local_var_req_builder = local_var_req_builder.query(&[("SmsEnabled", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = mms_enabled {
        local_var_req_builder = local_var_req_builder.query(&[("MmsEnabled", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = voice_enabled {
        local_var_req_builder = local_var_req_builder.query(&[("VoiceEnabled", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = exclude_all_address_required {
        local_var_req_builder = local_var_req_builder.query(&[("ExcludeAllAddressRequired", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = exclude_local_address_required {
        local_var_req_builder = local_var_req_builder.query(&[("ExcludeLocalAddressRequired", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = exclude_foreign_address_required {
        local_var_req_builder = local_var_req_builder.query(&[("ExcludeForeignAddressRequired", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = beta {
        local_var_req_builder = local_var_req_builder.query(&[("Beta", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = near_number {
        local_var_req_builder = local_var_req_builder.query(&[("NearNumber", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = near_lat_long {
        local_var_req_builder = local_var_req_builder.query(&[("NearLatLong", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = distance {
        local_var_req_builder = local_var_req_builder.query(&[("Distance", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = in_postal_code {
        local_var_req_builder = local_var_req_builder.query(&[("InPostalCode", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = in_region {
        local_var_req_builder = local_var_req_builder.query(&[("InRegion", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = in_rate_center {
        local_var_req_builder = local_var_req_builder.query(&[("InRateCenter", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = in_lata {
        local_var_req_builder = local_var_req_builder.query(&[("InLata", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = in_locality {
        local_var_req_builder = local_var_req_builder.query(&[("InLocality", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = fax_enabled {
        local_var_req_builder = local_var_req_builder.query(&[("FaxEnabled", &local_var_str.to_string())]);
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
        let local_var_entity: Option<ListAvailablePhoneNumberMobileError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// 
pub async fn list_incoming_phone_number_mobile(configuration: &configuration::Configuration, account_sid: &str, beta: Option<bool>, friendly_name: Option<&str>, phone_number: Option<&str>, origin: Option<&str>, page_size: Option<i32>, page: Option<i32>, page_token: Option<&str>) -> Result<crate::models::ListIncomingPhoneNumberMobileResponse, Error<ListIncomingPhoneNumberMobileError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/Mobile.json", local_var_configuration.base_path, AccountSid=crate::apis::urlencode(account_sid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = beta {
        local_var_req_builder = local_var_req_builder.query(&[("Beta", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = friendly_name {
        local_var_req_builder = local_var_req_builder.query(&[("FriendlyName", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = phone_number {
        local_var_req_builder = local_var_req_builder.query(&[("PhoneNumber", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = origin {
        local_var_req_builder = local_var_req_builder.query(&[("Origin", &local_var_str.to_string())]);
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
        let local_var_entity: Option<ListIncomingPhoneNumberMobileError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}
