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
pub struct ApiPeriodV2010PeriodAccountPeriodRecording {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Recording resource.
    #[serde(rename = "account_sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<Option<String>>,
    /// The API version used during the recording.
    #[serde(rename = "api_version", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<Option<String>>,
    /// The SID of the [Call](https://www.twilio.com/docs/voice/api/call-resource) the Recording resource is associated with. This will always refer to the parent leg of a two-leg call.
    #[serde(rename = "call_sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub call_sid: Option<Option<String>>,
    /// The Conference SID that identifies the conference associated with the recording, if a conference recording.
    #[serde(rename = "conference_sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub conference_sid: Option<Option<String>>,
    /// The date and time in GMT that the resource was created specified in [RFC 2822](https://www.ietf.org/rfc/rfc2822.txt) format.
    #[serde(rename = "date_created", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<Option<String>>,
    /// The date and time in GMT that the resource was last updated specified in [RFC 2822](https://www.ietf.org/rfc/rfc2822.txt) format.
    #[serde(rename = "date_updated", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<Option<String>>,
    /// The start time of the recording in GMT and in [RFC 2822](https://www.php.net/manual/en/class.datetime.php#datetime.constants.rfc2822) format.
    #[serde(rename = "start_time", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<Option<String>>,
    /// The length of the recording in seconds.
    #[serde(rename = "duration", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub duration: Option<Option<String>>,
    /// The unique string that that we created to identify the Recording resource.
    #[serde(rename = "sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sid: Option<Option<String>>,
    /// The one-time cost of creating the recording in the `price_unit` currency.
    #[serde(rename = "price", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub price: Option<Option<String>>,
    /// The currency used in the `price` property. Example: `USD`.
    #[serde(rename = "price_unit", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub price_unit: Option<Option<String>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::RecordingEnumStatus>,
    /// The number of channels in the final recording file. Can be: `1` or `2`. You can split a call with two legs into two separate recording channels if you record using [TwiML Dial](https://www.twilio.com/docs/voice/twiml/dial#record) or the [Outbound Rest API](https://www.twilio.com/docs/voice/make-calls#manage-your-outbound-call).
    #[serde(rename = "channels", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub channels: Option<Option<i32>>,
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<crate::models::RecordingEnumSource>,
    /// The error code that describes why the recording is `absent`. The error code is described in our [Error Dictionary](https://www.twilio.com/docs/api/errors). This value is null if the recording `status` is not `absent`.
    #[serde(rename = "error_code", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub error_code: Option<Option<i32>>,
    /// The URI of the resource, relative to `https://api.twilio.com`.
    #[serde(rename = "uri", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub uri: Option<Option<String>>,
    /// How to decrypt the recording if it was encrypted using [Call Recording Encryption](https://www.twilio.com/docs/voice/tutorials/voice-recording-encryption) feature.
    #[serde(rename = "encryption_details", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub encryption_details: Option<Option<serde_json::Value>>,
    /// A list of related resources identified by their relative URIs.
    #[serde(rename = "subresource_uris", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub subresource_uris: Option<Option<serde_json::Value>>,
    /// The URL of the media file associated with this recording resource. When stored externally, this is the full URL location of the media file.
    #[serde(rename = "media_url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub media_url: Option<Option<String>>,
}

impl ApiPeriodV2010PeriodAccountPeriodRecording {
    pub fn new() -> ApiPeriodV2010PeriodAccountPeriodRecording {
        ApiPeriodV2010PeriodAccountPeriodRecording {
            account_sid: None,
            api_version: None,
            call_sid: None,
            conference_sid: None,
            date_created: None,
            date_updated: None,
            start_time: None,
            duration: None,
            sid: None,
            price: None,
            price_unit: None,
            status: None,
            channels: None,
            source: None,
            error_code: None,
            uri: None,
            encryption_details: None,
            subresource_uris: None,
            media_url: None,
        }
    }
}


