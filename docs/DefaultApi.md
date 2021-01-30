# \DefaultApi

All URIs are relative to *https://backend.mercury.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**accountidcards**](DefaultApi.md#accountidcards) | **get** /account/{id}/cards | /account/:id/cards
[**accountidstatement**](DefaultApi.md#accountidstatement) | **get** /account/{id}/statements | /account/:id/statements (experimental)
[**accountidtransactiontransactionid**](DefaultApi.md#accountidtransactiontransactionid) | **get** /account/{id}/transaction/{transactionId} | /account/:id/transaction/:transactionId
[**accounts**](DefaultApi.md#accounts) | **get** /accounts | /accounts
[**accountsid**](DefaultApi.md#accountsid) | **get** /account/{id} | /account/:id
[**recipientid**](DefaultApi.md#recipientid) | **post** /recipient/{id} | /recipient/:id
[**recipients1**](DefaultApi.md#recipients1) | **get** /recipients | /recipients
[**recipients2**](DefaultApi.md#recipients2) | **post** /recipients | /recipients
[**recipientsid**](DefaultApi.md#recipientsid) | **get** /recipient/{id} | /recipient/:id
[**submit_onboarding_data**](DefaultApi.md#submit_onboarding_data) | **post** /submit-onboarding-data | /submit-onboarding-data
[**transactions1**](DefaultApi.md#transactions1) | **get** /account/{id}/transactions | /account/:id/transactions
[**transactions2**](DefaultApi.md#transactions2) | **post** /account/{id}/transactions | /account/:id/transactions



## accountidcards

> accountidcards(id)
/account/:id/cards

Retrieve information about cards associated with a specific account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Your 36-character account uuid. | [required] |

### Return type

 (empty response body)

### Authorization

[sec0](../README.md#sec0)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## accountidstatement

> accountidstatement(id, start, end)
/account/:id/statements (experimental)

Retrieve information about a statement for a given time period for this account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Your 36-character account UUID. | [required] |
**start** | Option<**String**> | Filter the statements so that their startDate is equal to or later than this date. Format: YYYY-MM-DD. |  |
**end** | Option<**String**> | Filter the statements so that their endDate is less than or equal to this date. Format: YYYY-MM-DD. |  |

### Return type

 (empty response body)

### Authorization

[sec0](../README.md#sec0)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## accountidtransactiontransactionid

> accountidtransactiontransactionid(id, transaction_id)
/account/:id/transaction/:transactionId

Retrieve information about a specific transaction for a specific account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Your 36-character account uuid. | [required] |
**transaction_id** | **String** | Your 36-character transaction uuid. | [required] |

### Return type

 (empty response body)

### Authorization

[sec0](../README.md#sec0)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## accounts

> accounts()
/accounts

Retrieve information about all your bank accounts

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[sec0](../README.md#sec0)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## accountsid

> accountsid(id)
/account/:id

Retrieve information about a specific account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Your 36-character account UUID. | [required] |

### Return type

 (empty response body)

### Authorization

[sec0](../README.md#sec0)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## recipientid

> recipientid(id, inline_object1)
/recipient/:id

Edit information about a specific recipient

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | 36-character recipient uuid. | [required] |
**inline_object1** | Option<[**InlineObject1**](InlineObject1.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[sec0](../README.md#sec0)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## recipients1

> recipients1()
/recipients

Retrieve information about all of your recipients

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[sec0](../README.md#sec0)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## recipients2

> recipients2(inline_object)
/recipients

Add a new recipient

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object** | Option<[**InlineObject**](InlineObject.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[sec0](../README.md#sec0)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## recipientsid

> recipientsid(id)
/recipient/:id

Retrieve information about a specific recipient

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | 36-character recipient uuid. | [required] |

### Return type

 (empty response body)

### Authorization

[sec0](../README.md#sec0)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submit_onboarding_data

> submit_onboarding_data(inline_object3)
/submit-onboarding-data

Provide onboarding data for a new account. Note: This endpoint is separate from the rest of the Mercury API, any previous documentation pages do not apply here.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object3** | Option<[**InlineObject3**](InlineObject3.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transactions1

> transactions1(id, limit, offset, status, start, end, search)
/account/:id/transactions

Retrieve information about your transactions for a specific account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Your 36-character account uuid. | [required] |
**limit** | Option<**i32**> | Limit how many transactions you want to retrieve. |  |[default to 500]
**offset** | Option<**i32**> | The number of most recent transactions you want to omit. |  |[default to 0]
**status** | Option<**String**> | \"pending\" | \"sent\" | \"cancelled\" | \"failed\" |  |
**start** | Option<**String**> | Earliest createdAt date to filter for. If it's not provided, it defaults to 30 days ago. Format: YYYY-MM-DD. Please note that your Mercury transactions on your Dashboard might have their postedAt date displayed, as opposed to createdAt |  |
**end** | Option<**String**> | Latest createdAt date fo filter for. If it's not provided, it defaults to current day. Format: YYYY-MM-DD. Please note that your Mercury transactions on your Dashboard might have their postedAt date displayed, as opposed to createdAt |  |
**search** | Option<**String**> | Search term to look for in transaction descriptions. |  |

### Return type

 (empty response body)

### Authorization

[sec0](../README.md#sec0)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transactions2

> transactions2(id, inline_object2)
/account/:id/transactions

Create a new transaction, currently only supporting ACH payments

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Your 36-character account uuid. | [required] |
**inline_object2** | Option<[**InlineObject2**](InlineObject2.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[sec0](../README.md#sec0)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

