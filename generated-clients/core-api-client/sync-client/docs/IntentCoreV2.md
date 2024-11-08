# IntentCoreV2

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**blobs_hex** | Option<**std::collections::HashMap<String, String>**> | A map of the hex-encoded blob hash, to hex-encoded blob content. Only returned if enabled in `TransactionFormatOptions` on your request. | [optional]
**children_specifiers** | **Vec<String>** |  | 
**instructions** | Option<**String**> | The decompiled transaction manifest instructions. Only returned if enabled in `TransactionFormatOptions` on your request. | [optional]
**intent_header** | [**models::IntentHeaderV2**](IntentHeaderV2.md) |  | 
**message** | Option<[**models::TransactionMessage**](TransactionMessage.md)> | The optional transaction message. Only returned if present and enabled in `TransactionFormatOptions` on your request. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


