# InlineObject2

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**recipient_id** | **String** | Recipient ID from the [/recipients](https://mercurytechnologies.readme.io/reference#recipients-1) endpoint. | [default to <recipient_id>]
**amount** | **f64** | Amount of USD you want to send, must be a positive number. | [default to 10]
**payment_method** | **String** | Payment method to use, currently only supports \"ach\" | [default to ach]
**note** | Option<**String**> | Optional note | [optional]
**external_memo** | Option<**String**> | Optional external memo | [optional]
**idempotency_key** | **String** | Unique string identifying the transaction | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


