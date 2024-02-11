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
pub struct ApiPeriodV2010PeriodAccountPeriodIncomingPhoneNumberPeriodIncomingPhoneNumberAssignedAddOn {
    /// The unique string that that we created to identify the resource.
    #[serde(rename = "sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sid: Option<Option<String>>,
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the resource.
    #[serde(rename = "account_sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<Option<String>>,
    /// The SID of the Phone Number to which the Add-on is assigned.
    #[serde(rename = "resource_sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub resource_sid: Option<Option<String>>,
    /// The string that you assigned to describe the resource.
    #[serde(rename = "friendly_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<Option<String>>,
    /// A short description of the functionality that the Add-on provides.
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    /// A JSON string that represents the current configuration of this Add-on installation.
    #[serde(rename = "configuration", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub configuration: Option<Option<serde_json::Value>>,
    /// An application-defined string that uniquely identifies the resource. It can be used in place of the resource's `sid` in the URL to address the resource.
    #[serde(rename = "unique_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub unique_name: Option<Option<String>>,
    /// The date and time in GMT that the resource was created specified in [RFC 2822](https://www.ietf.org/rfc/rfc2822.txt) format.
    #[serde(rename = "date_created", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<Option<String>>,
    /// The date and time in GMT that the resource was last updated specified in [RFC 2822](https://www.ietf.org/rfc/rfc2822.txt) format.
    #[serde(rename = "date_updated", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<Option<String>>,
    /// The URI of the resource, relative to `https://api.twilio.com`.
    #[serde(rename = "uri", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub uri: Option<Option<String>>,
    /// A list of related resources identified by their relative URIs.
    #[serde(rename = "subresource_uris", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub subresource_uris: Option<Option<serde_json::Value>>,
}

impl ApiPeriodV2010PeriodAccountPeriodIncomingPhoneNumberPeriodIncomingPhoneNumberAssignedAddOn {
    pub fn new() -> ApiPeriodV2010PeriodAccountPeriodIncomingPhoneNumberPeriodIncomingPhoneNumberAssignedAddOn {
        ApiPeriodV2010PeriodAccountPeriodIncomingPhoneNumberPeriodIncomingPhoneNumberAssignedAddOn {
            sid: None,
            account_sid: None,
            resource_sid: None,
            friendly_name: None,
            description: None,
            configuration: None,
            unique_name: None,
            date_created: None,
            date_updated: None,
            uri: None,
            subresource_uris: None,
        }
    }
}


