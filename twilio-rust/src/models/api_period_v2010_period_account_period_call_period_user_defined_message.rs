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
pub struct ApiPeriodV2010PeriodAccountPeriodCallPeriodUserDefinedMessage {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created User Defined Message.
    #[serde(rename = "account_sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<Option<String>>,
    /// The SID of the [Call](https://www.twilio.com/docs/voice/api/call-resource) the User Defined Message is associated with.
    #[serde(rename = "call_sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub call_sid: Option<Option<String>>,
    /// The SID that uniquely identifies this User Defined Message.
    #[serde(rename = "sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sid: Option<Option<String>>,
    /// The date that this User Defined Message was created, given in RFC 2822 format.
    #[serde(rename = "date_created", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<Option<String>>,
}

impl ApiPeriodV2010PeriodAccountPeriodCallPeriodUserDefinedMessage {
    pub fn new() -> ApiPeriodV2010PeriodAccountPeriodCallPeriodUserDefinedMessage {
        ApiPeriodV2010PeriodAccountPeriodCallPeriodUserDefinedMessage {
            account_sid: None,
            call_sid: None,
            sid: None,
            date_created: None,
        }
    }
}


