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
pub enum MessageEnumTrafficType {
    #[serde(rename = "free")]
    Free,

}

impl ToString for MessageEnumTrafficType {
    fn to_string(&self) -> String {
        match self {
            Self::Free => String::from("free"),
        }
    }
}

impl Default for MessageEnumTrafficType {
    fn default() -> MessageEnumTrafficType {
        Self::Free
    }
}




