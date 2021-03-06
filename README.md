# Rust API client for openapi

No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)

## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 1
- Package version: 1.0.0
- Build package: org.openapitools.codegen.languages.RustClientCodegen

## Installation

Put the package under your project folder and add the following to `Cargo.toml` under `[dependencies]`:

```
    openapi = { path = "./generated" }
```

## Documentation for API Endpoints

All URIs are relative to *https://backend.mercury.com/api/v1*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*DefaultApi* | [**accountidcards**](docs/DefaultApi.md#accountidcards) | **get** /account/{id}/cards | /account/:id/cards
*DefaultApi* | [**accountidstatement**](docs/DefaultApi.md#accountidstatement) | **get** /account/{id}/statements | /account/:id/statements (experimental)
*DefaultApi* | [**accountidtransactiontransactionid**](docs/DefaultApi.md#accountidtransactiontransactionid) | **get** /account/{id}/transaction/{transactionId} | /account/:id/transaction/:transactionId
*DefaultApi* | [**accounts**](docs/DefaultApi.md#accounts) | **get** /accounts | /accounts
*DefaultApi* | [**accountsid**](docs/DefaultApi.md#accountsid) | **get** /account/{id} | /account/:id
*DefaultApi* | [**recipientid**](docs/DefaultApi.md#recipientid) | **post** /recipient/{id} | /recipient/:id
*DefaultApi* | [**recipients1**](docs/DefaultApi.md#recipients1) | **get** /recipients | /recipients
*DefaultApi* | [**recipients2**](docs/DefaultApi.md#recipients2) | **post** /recipients | /recipients
*DefaultApi* | [**recipientsid**](docs/DefaultApi.md#recipientsid) | **get** /recipient/{id} | /recipient/:id
*DefaultApi* | [**submit_onboarding_data**](docs/DefaultApi.md#submit_onboarding_data) | **post** /submit-onboarding-data | /submit-onboarding-data
*DefaultApi* | [**transactions1**](docs/DefaultApi.md#transactions1) | **get** /account/{id}/transactions | /account/:id/transactions
*DefaultApi* | [**transactions2**](docs/DefaultApi.md#transactions2) | **post** /account/{id}/transactions | /account/:id/transactions


## Documentation For Models

 - [About](docs/About.md)
 - [AddressWithoutName](docs/AddressWithoutName.md)
 - [BeneficialOwner](docs/BeneficialOwner.md)
 - [BusinessContactDetails](docs/BusinessContactDetails.md)
 - [DomesticWireRoutingInfoRaw](docs/DomesticWireRoutingInfoRaw.md)
 - [ElectronicRoutingInfoRaw](docs/ElectronicRoutingInfoRaw.md)
 - [FormationDetails](docs/FormationDetails.md)
 - [InlineObject](docs/InlineObject.md)
 - [InlineObject1](docs/InlineObject1.md)
 - [InlineObject2](docs/InlineObject2.md)
 - [InlineObject3](docs/InlineObject3.md)
 - [InternationalWireRoutingInfoRaw](docs/InternationalWireRoutingInfoRaw.md)
 - [RecipientIdDomesticWireRoutingInfo](docs/RecipientIdDomesticWireRoutingInfo.md)
 - [RecipientIdElectronicRoutingInfo](docs/RecipientIdElectronicRoutingInfo.md)
 - [RecipientIdInternationalWireRoutingInfo](docs/RecipientIdInternationalWireRoutingInfo.md)
 - [RecipientsAddress](docs/RecipientsAddress.md)
 - [RecipientsElectronicRoutingInfo](docs/RecipientsElectronicRoutingInfo.md)
 - [RecipientsInternationalWireRoutingInfo](docs/RecipientsInternationalWireRoutingInfo.md)
 - [SubmitOnboardingDataAbout](docs/SubmitOnboardingDataAbout.md)
 - [SubmitOnboardingDataBeneficialOwners](docs/SubmitOnboardingDataBeneficialOwners.md)
 - [SubmitOnboardingDataBusinessContactDetails](docs/SubmitOnboardingDataBusinessContactDetails.md)
 - [SubmitOnboardingDataFormationDetails](docs/SubmitOnboardingDataFormationDetails.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author



