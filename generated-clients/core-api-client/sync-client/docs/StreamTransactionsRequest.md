# StreamTransactionsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**from_state_version** | **u64** |  | 
**include_proofs** | Option<**bool**> | Whether to include LedgerProofs (default false) | [optional]
**limit** | **i32** | The maximum number of transactions that will be returned. | 
**network** | **String** | The logical name of the network | 
**sbor_format_options** | Option<[**models::SborFormatOptions**](SborFormatOptions.md)> |  | [optional]
**substate_format_options** | Option<[**models::SubstateFormatOptions**](SubstateFormatOptions.md)> |  | [optional]
**transaction_format_options** | Option<[**models::TransactionFormatOptions**](TransactionFormatOptions.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


