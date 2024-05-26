# TransactionIntent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**blobs_hex** | Option<**std::collections::HashMap<String, String>**> | A map of the hex-encoded blob hash, to hex-encoded blob content. Only returned if enabled in `TransactionFormatOptions` on your request. | [optional]
**hash** | **String** | The hex-encoded intent hash for a user transaction, also known as the transaction id. This hash identifies the core content \"intent\" of the transaction. Each intent can only be committed once. This hash gets signed by any signatories on the transaction, to create the signed intent.  | 
**hash_bech32m** | **String** | The Bech32m-encoded human readable `IntentHash`. | 
**header** | [**models::TransactionHeader**](TransactionHeader.md) |  | 
**instructions** | Option<**String**> | The decompiled transaction manifest instructions. Only returned if enabled in `TransactionFormatOptions` on your request. | [optional]
**message** | Option<[**models::TransactionMessage**](TransactionMessage.md)> | The optional transaction message. Only returned if present and enabled in `TransactionFormatOptions` on your request. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


