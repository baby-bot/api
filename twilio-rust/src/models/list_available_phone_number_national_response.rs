/*
 * Twilio - Api
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.54.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListAvailablePhoneNumberNationalResponse {
    #[serde(rename = "available_phone_numbers", skip_serializing_if = "Option::is_none")]
    pub available_phone_numbers: Option<Vec<crate::models::ApiPeriodV2010PeriodAccountPeriodAvailablePhoneNumberCountryPeriodAvailablePhoneNumberNational>>,
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: Option<i32>,
    #[serde(rename = "first_page_uri", skip_serializing_if = "Option::is_none")]
    pub first_page_uri: Option<String>,
    #[serde(rename = "next_page_uri", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub next_page_uri: Option<Option<String>>,
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(rename = "page_size", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "previous_page_uri", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub previous_page_uri: Option<Option<String>>,
    #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
    pub start: Option<i32>,
    #[serde(rename = "uri", skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

impl ListAvailablePhoneNumberNationalResponse {
    pub fn new() -> ListAvailablePhoneNumberNationalResponse {
        ListAvailablePhoneNumberNationalResponse {
            available_phone_numbers: None,
            end: None,
            first_page_uri: None,
            next_page_uri: None,
            page: None,
            page_size: None,
            previous_page_uri: None,
            start: None,
            uri: None,
        }
    }
}


