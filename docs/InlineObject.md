# InlineObject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**address** | Option<[**crate::models::RecipientsAddress**](_recipients_address.md)> |  | [optional]
**emails** | **Vec<String>** |  | [default to [null]]
**payment_method** | **String** | \"check\" | \"electronic\" | \"domesticWire\" | \"internationalWire\" | 
**electronic_routing_info** | Option<[**crate::models::RecipientsElectronicRoutingInfo**](_recipients_electronicRoutingInfo.md)> |  | [optional]
**domestic_wire_routing_info** | Option<[**serde_json::Value**](.md)> | Note that all fields in this object are required | [optional]
**international_wire_routing_info** | Option<[**crate::models::RecipientsInternationalWireRoutingInfo**](_recipients_internationalWireRoutingInfo.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


