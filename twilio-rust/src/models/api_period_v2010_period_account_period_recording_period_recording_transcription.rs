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
pub struct ApiPeriodV2010PeriodAccountPeriodRecordingPeriodRecordingTranscription {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Transcription resource.
    #[serde(rename = "account_sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<Option<String>>,
    /// The API version used to create the transcription.
    #[serde(rename = "api_version", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<Option<String>>,
    /// The date and time in GMT that the resource was created specified in [RFC 2822](https://www.ietf.org/rfc/rfc2822.txt) format.
    #[serde(rename = "date_created", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<Option<String>>,
    /// The date and time in GMT that the resource was last updated specified in [RFC 2822](https://www.ietf.org/rfc/rfc2822.txt) format.
    #[serde(rename = "date_updated", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<Option<String>>,
    /// The duration of the transcribed audio in seconds.
    #[serde(rename = "duration", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub duration: Option<Option<String>>,
    /// The charge for the transcript in the currency associated with the account. This value is populated after the transcript is complete so it may not be available immediately.
    #[serde(rename = "price", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub price: Option<Option<f32>>,
    /// The currency in which `price` is measured, in [ISO 4127](https://www.iso.org/iso/home/standards/currency_codes.htm) format (e.g. `usd`, `eur`, `jpy`).
    #[serde(rename = "price_unit", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub price_unit: Option<Option<String>>,
    /// The SID of the [Recording](https://www.twilio.com/docs/voice/api/recording) from which the transcription was created.
    #[serde(rename = "recording_sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub recording_sid: Option<Option<String>>,
    /// The unique string that that we created to identify the Transcription resource.
    #[serde(rename = "sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sid: Option<Option<String>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::RecordingTranscriptionEnumStatus>,
    /// The text content of the transcription.
    #[serde(rename = "transcription_text", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub transcription_text: Option<Option<String>>,
    /// The transcription type.
    #[serde(rename = "type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Option<String>>,
    /// The URI of the resource, relative to `https://api.twilio.com`.
    #[serde(rename = "uri", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub uri: Option<Option<String>>,
}

impl ApiPeriodV2010PeriodAccountPeriodRecordingPeriodRecordingTranscription {
    pub fn new() -> ApiPeriodV2010PeriodAccountPeriodRecordingPeriodRecordingTranscription {
        ApiPeriodV2010PeriodAccountPeriodRecordingPeriodRecordingTranscription {
            account_sid: None,
            api_version: None,
            date_created: None,
            date_updated: None,
            duration: None,
            price: None,
            price_unit: None,
            recording_sid: None,
            sid: None,
            status: None,
            transcription_text: None,
            r#type: None,
            uri: None,
        }
    }
}


