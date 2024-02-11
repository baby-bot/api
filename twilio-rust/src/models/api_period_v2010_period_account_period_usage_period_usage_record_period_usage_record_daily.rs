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
pub struct ApiPeriodV2010PeriodAccountPeriodUsagePeriodUsageRecordPeriodUsageRecordDaily {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that accrued the usage.
    #[serde(rename = "account_sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<Option<String>>,
    /// The API version used to create the resource.
    #[serde(rename = "api_version", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<Option<String>>,
    /// Usage records up to date as of this timestamp, formatted as YYYY-MM-DDTHH:MM:SS+00:00. All timestamps are in GMT
    #[serde(rename = "as_of", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub as_of: Option<Option<String>>,
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: Option<crate::models::UsageRecordDailyEnumCategory>,
    /// The number of usage events, such as the number of calls.
    #[serde(rename = "count", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub count: Option<Option<String>>,
    /// The units in which `count` is measured, such as `calls` for calls or `messages` for SMS.
    #[serde(rename = "count_unit", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub count_unit: Option<Option<String>>,
    /// A plain-language description of the usage category.
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    /// The last date for which usage is included in the UsageRecord. The date is specified in GMT and formatted as `YYYY-MM-DD`.
    #[serde(rename = "end_date", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<Option<String>>,
    /// The total price of the usage in the currency specified in `price_unit` and associated with the account.
    #[serde(rename = "price", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub price: Option<Option<f32>>,
    /// The currency in which `price` is measured, in [ISO 4127](https://www.iso.org/iso/home/standards/currency_codes.htm) format, such as `usd`, `eur`, and `jpy`.
    #[serde(rename = "price_unit", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub price_unit: Option<Option<String>>,
    /// The first date for which usage is included in this UsageRecord. The date is specified in GMT and formatted as `YYYY-MM-DD`.
    #[serde(rename = "start_date", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<Option<String>>,
    /// A list of related resources identified by their URIs. For more information, see [List Subresources](https://www.twilio.com/docs/usage/api/usage-record#list-subresources).
    #[serde(rename = "subresource_uris", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub subresource_uris: Option<Option<serde_json::Value>>,
    /// The URI of the resource, relative to `https://api.twilio.com`.
    #[serde(rename = "uri", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub uri: Option<Option<String>>,
    /// The amount used to bill usage and measured in units described in `usage_unit`.
    #[serde(rename = "usage", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub usage: Option<Option<String>>,
    /// The units in which `usage` is measured, such as `minutes` for calls or `messages` for SMS.
    #[serde(rename = "usage_unit", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub usage_unit: Option<Option<String>>,
}

impl ApiPeriodV2010PeriodAccountPeriodUsagePeriodUsageRecordPeriodUsageRecordDaily {
    pub fn new() -> ApiPeriodV2010PeriodAccountPeriodUsagePeriodUsageRecordPeriodUsageRecordDaily {
        ApiPeriodV2010PeriodAccountPeriodUsagePeriodUsageRecordPeriodUsageRecordDaily {
            account_sid: None,
            api_version: None,
            as_of: None,
            category: None,
            count: None,
            count_unit: None,
            description: None,
            end_date: None,
            price: None,
            price_unit: None,
            start_date: None,
            subresource_uris: None,
            uri: None,
            usage: None,
            usage_unit: None,
        }
    }
}


