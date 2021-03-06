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
pub struct FormationDetails {
    #[serde(rename = "companyStructure", skip_serializing_if = "Option::is_none")]
    pub company_structure: Option<String>,
    #[serde(rename = "federalEin", skip_serializing_if = "Option::is_none")]
    pub federal_ein: Option<String>,
    #[serde(
        rename = "formationDocumentType",
        skip_serializing_if = "Option::is_none"
    )]
    pub formation_document_type: Option<String>,
    #[serde(
        rename = "formationDocumentFileBlob",
        skip_serializing_if = "Option::is_none"
    )]
    pub formation_document_file_blob: Option<std::path::PathBuf>,
    #[serde(
        rename = "eINDocumentFileBlob",
        skip_serializing_if = "Option::is_none"
    )]
    pub e_in_document_file_blob: Option<std::path::PathBuf>,
}

impl FormationDetails {
    pub fn new() -> FormationDetails {
        FormationDetails {
            company_structure: None,
            federal_ein: None,
            formation_document_type: None,
            formation_document_file_blob: None,
            e_in_document_file_blob: None,
        }
    }
}
