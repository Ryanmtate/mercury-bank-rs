/*
 * Mercury API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineObject3 {
    /// Name of your organization, please clarify the proper capitalization before submitting it.
    #[serde(rename = "partner")]
    pub partner: String,
    /// Once the user's account gets approved, we'll send a webhook to this URL. Let us know if you want to receive our webhooks as they are optional.
    #[serde(rename = "webhookURL", skip_serializing_if = "Option::is_none")]
    pub webhook_url: Option<String>,
    #[serde(rename = "about")]
    pub about: crate::models::SubmitOnboardingDataAbout,
    #[serde(rename = "businessContactDetails")]
    pub business_contact_details: crate::models::SubmitOnboardingDataBusinessContactDetails,
    #[serde(rename = "formationDetails")]
    pub formation_details: crate::models::SubmitOnboardingDataFormationDetails,
    /// The first in the list will be considered the primary user, the person actually signing up for the bank account. Additional users should be: 1. People with > 25% ownership of the company 2. OR People with significant financial control over the company (e.g. CEOs, CFOs).
    #[serde(rename = "beneficialOwners")]
    pub beneficial_owners: Vec<crate::models::SubmitOnboardingDataBeneficialOwners>,
}

impl InlineObject3 {
    pub fn new(
        partner: String,
        about: crate::models::SubmitOnboardingDataAbout,
        business_contact_details: crate::models::SubmitOnboardingDataBusinessContactDetails,
        formation_details: crate::models::SubmitOnboardingDataFormationDetails,
        beneficial_owners: Vec<crate::models::SubmitOnboardingDataBeneficialOwners>,
    ) -> InlineObject3 {
        InlineObject3 {
            partner,
            webhook_url: None,
            about,
            business_contact_details,
            formation_details,
            beneficial_owners,
        }
    }
}
