/*
 * Twilio - Api
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.54.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CallFeedbackEnumIssues {
    #[serde(rename = "audio-latency")]
    AudioLatency,
    #[serde(rename = "digits-not-captured")]
    DigitsNotCaptured,
    #[serde(rename = "dropped-call")]
    DroppedCall,
    #[serde(rename = "imperfect-audio")]
    ImperfectAudio,
    #[serde(rename = "incorrect-caller-id")]
    IncorrectCallerId,
    #[serde(rename = "one-way-audio")]
    OneWayAudio,
    #[serde(rename = "post-dial-delay")]
    PostDialDelay,
    #[serde(rename = "unsolicited-call")]
    UnsolicitedCall,

}

impl ToString for CallFeedbackEnumIssues {
    fn to_string(&self) -> String {
        match self {
            Self::AudioLatency => String::from("audio-latency"),
            Self::DigitsNotCaptured => String::from("digits-not-captured"),
            Self::DroppedCall => String::from("dropped-call"),
            Self::ImperfectAudio => String::from("imperfect-audio"),
            Self::IncorrectCallerId => String::from("incorrect-caller-id"),
            Self::OneWayAudio => String::from("one-way-audio"),
            Self::PostDialDelay => String::from("post-dial-delay"),
            Self::UnsolicitedCall => String::from("unsolicited-call"),
        }
    }
}

impl Default for CallFeedbackEnumIssues {
    fn default() -> CallFeedbackEnumIssues {
        Self::AudioLatency
    }
}




