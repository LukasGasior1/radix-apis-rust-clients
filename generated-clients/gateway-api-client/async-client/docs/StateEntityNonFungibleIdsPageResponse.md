# StateEntityNonFungibleIdsPageResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ledger_state** | [**models::LedgerState**](LedgerState.md) |  | 
**next_cursor** | Option<**String**> | If specified, contains a cursor to query next page of the `items` collection. | [optional]
**total_count** | Option<**i64**> | Total number of items in underlying collection, fragment of which is available in `items` collection. | [optional]
**items** | **Vec<String>** |  | 
**address** | **String** | Bech32m-encoded human readable version of the address. | 
**resource_address** | **String** | Bech32m-encoded human readable version of the address. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


