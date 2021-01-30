# InlineObject3

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**partner** | **String** | Name of your organization, please clarify the proper capitalization before submitting it. | [default to Partner]
**webhook_url** | Option<**String**> | Once the user's account gets approved, we'll send a webhook to this URL. Let us know if you want to receive our webhooks as they are optional. | [optional][default to https://yourwebsite.com/mercury-webhook]
**about** | [**crate::models::SubmitOnboardingDataAbout**](_submit_onboarding_data_about.md) |  | 
**business_contact_details** | [**crate::models::SubmitOnboardingDataBusinessContactDetails**](_submit_onboarding_data_businessContactDetails.md) |  | 
**formation_details** | [**crate::models::SubmitOnboardingDataFormationDetails**](_submit_onboarding_data_formationDetails.md) |  | 
**beneficial_owners** | [**Vec<crate::models::SubmitOnboardingDataBeneficialOwners>**](_submit_onboarding_data_beneficialOwners.md) | The first in the list will be considered the primary user, the person actually signing up for the bank account. Additional users should be: 1. People with > 25% ownership of the company 2. OR People with significant financial control over the company (e.g. CEOs, CFOs). | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


