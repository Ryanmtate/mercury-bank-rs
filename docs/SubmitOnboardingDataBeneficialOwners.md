# SubmitOnboardingDataBeneficialOwners

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**first_name** | Option<**String**> | First name. | [optional][default to Immad]
**last_name** | Option<**String**> | Last name. | [optional][default to Akhund]
**job_title** | Option<**String**> | This must be one of: \"ChiefExecutiveOfficer\", \"ChiefOperatingOfficer\", \"ChiefTechnologyOfficer\", \"ChiefFinancialOfficer\", \"Founder\", \"President\", \"GeneralPartner\", \"Other\". | [optional][default to CEO]
**other_job_title** | Option<**String**> | Free-form job title. Only valid if jobTitle is Other. | [optional][default to ]
**email** | Option<**String**> | Should match the HTML5 valid email regexp. | [optional][default to test@example.com]
**percent_ownership** | Option<**i32**> | Number between 0 and 100. | [optional]
**citizenship_status** | Option<**String**> | This must be one of: \"USCitizen\", \"USResident\", \"NonResident\". | [optional][default to USCitizen]
**identification_type** | Option<**String**> | Type of identification document, depends on 'citizenshipStatus' value. Valid identification types per citizenship status are described below. For \"USCitizen\": \"DriversLicense\", \"Passport\", \"StateID\". For \"USResident\": \"DriversLicense\", \"Passport\", \"StateID\", \"AlienRegistrationCard\", \"EmployeeAuthorizationDocument\". For \"NonResident\": \"Passport\". | [optional][default to Passport]
**identification_blob** | Option<**String**> | PDF file with this Beneficial Owner's ID, base64-encoded (using strict mode). | [optional][default to ]
**address1** | Option<**String**> | This Beneficial Owner's address line 1. | [optional][default to 149 Laidley St]
**address2** | Option<**String**> | This Beneficial Owner's address line 2. | [optional][default to ]
**city** | Option<**String**> |  | [optional][default to San Francisco]
**state** | Option<**String**> | If US, 2-character US state code, otherwise freeform String for the region (e.g. \"Jalisco\"). | [optional][default to CA]
**country** | Option<**String**> | 2-character country code per ISO 3166-1 Alpha2, e.g. \"AT\" for Austria. | [optional][default to US]
**postal_code** | Option<**String**> | Zip code. | [optional][default to 94103]
**phone_number** | Option<**String**> | This Beneficial Owner's phone number. | [optional][default to +18596300123]
**is_pep** | Option<**String**> | Whether this Beneficial Owner is a politically exposed person, possible values: \"IsNotPep\", \"IsPep\". | [optional][default to IsNotPep]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


