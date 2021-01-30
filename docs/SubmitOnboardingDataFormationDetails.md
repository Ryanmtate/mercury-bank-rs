# SubmitOnboardingDataFormationDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**company_structure** | Option<**String**> | This must be one of: \"CCorp\", \"LLC\", \"LLP\", \"NonProfit\", \"Partnership\", \"ProfessionalCorporation\", \"SCorp\", \"SoleProprietorship\" | [optional][default to CCorp]
**federal_ein** | Option<**String**> | Format: 2 digits, optional hyphen, 7 digits. | [optional][default to 81-9292920]
**formation_document_type** | Option<**String**> | This must be one of: \"ArticlesOfIncorporation\", \"ArticlesOfOrganization\", \"AssumedNameCertificate\", \"BusinessLicense\", \"CertificateOfGoodStanding\", \"CertificateOfLimitedPartnership\", \"LettersTestamentary\", \"OtherBusinessDocumentation\", \"PartnershipAgreement\", \"TrustAgreement\", \"CertificateOfFormation\" | [optional][default to ArticlesOfIncorporation]
**formation_document_file_blob** | Option<[**std::path::PathBuf**](std::path::PathBuf.md)> | Base64-encoded (using strict mode) pdf file for state-stamped formation documents. | [optional]
**e_in_document_file_blob** | Option<[**std::path::PathBuf**](std::path::PathBuf.md)> | PDF for SS-4 document to verify EIN. Base64-encoded (using strict mode). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


