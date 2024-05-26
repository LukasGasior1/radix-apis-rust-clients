# TransactionReceiptRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**intent_hash** | **String** | The intent hash for a user transaction, also known as the transaction id. This hash identifies the core content \"intent\" of the transaction. Each intent can only be committed once. This hash gets signed by any signatories on the transaction, to create the signed intent. Either hex or Bech32m-encoded strings are supported.  | 
**network** | **String** | The logical name of the network | 
**transaction_format_options** | Option<[**models::TransactionFormatOptions**](TransactionFormatOptions.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


