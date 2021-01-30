/*
 * Mercury API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1
 *
 * Generated by: https://openapi-generator.tech
 */

/// SubmitOnboardingDataBusinessContactDetails : Business contact information.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubmitOnboardingDataBusinessContactDetails {
    /// Address line 1.
    #[serde(rename = "address1", skip_serializing_if = "Option::is_none")]
    pub address1: Option<String>,
    /// Address line 2.
    #[serde(rename = "address2", skip_serializing_if = "Option::is_none")]
    pub address2: Option<String>,
    /// City.
    #[serde(rename = "city", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// 2-character state code e.g. \"CA\" for California.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Zip code.
    #[serde(rename = "postalCode", skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// Phone number.
    #[serde(rename = "phoneNumber", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
}

impl SubmitOnboardingDataBusinessContactDetails {
    /// Business contact information.
    pub fn new() -> SubmitOnboardingDataBusinessContactDetails {
        SubmitOnboardingDataBusinessContactDetails {
            address1: None,
            address2: None,
            city: None,
            state: None,
            postal_code: None,
            phone_number: None,
        }
    }
}
