# CommittedTransactionInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**affected_global_entities** | Option<**Vec<String>**> |  | [optional]
**balance_changes** | Option<[**models::TransactionBalanceChanges**](TransactionBalanceChanges.md)> |  | [optional]
**confirmed_at** | Option<**String**> |  | [optional]
**epoch** | **i64** |  | 
**error_message** | Option<**String**> |  | [optional]
**fee_paid** | Option<**String**> | String-encoded decimal representing the amount of a related fungible resource. | [optional]
**intent_hash** | Option<**String**> | Bech32m-encoded hash. | [optional]
**manifest_classes** | Option<[**Vec<models::ManifestClass>**](ManifestClass.md)> | A collection of zero or more manifest classes ordered from the most specific class to the least specific one. This field will be present only for user transactions.  | [optional]
**manifest_instructions** | Option<**String**> | A text-representation of a transaction manifest. This field will be present only for user transactions and when explicitly opted-in using `manifest_instructions` flag.  | [optional]
**message** | Option<[**serde_json::Value**](.md)> | The optional transaction message. This type is defined in the Core API as `TransactionMessage`. See the Core API documentation for more details.  | [optional]
**payload_hash** | Option<**String**> | Bech32m-encoded hash. | [optional]
**raw_hex** | Option<**String**> | Hex-encoded binary blob. | [optional]
**receipt** | Option<[**models::TransactionReceipt**](TransactionReceipt.md)> |  | [optional]
**round** | **i64** |  | 
**round_timestamp** | **String** |  | 
**state_version** | **i64** |  | 
**transaction_status** | [**models::TransactionStatus**](TransactionStatus.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


