# StateEntityFungiblesPageRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**at_ledger_state** | Option<[**models::LedgerStateSelector**](LedgerStateSelector.md)> |  | [optional]
**cursor** | Option<**String**> | This cursor allows forward pagination, by providing the cursor from the previous request. | [optional]
**limit_per_page** | Option<**i32**> | The page size requested. | [optional]
**address** | **String** | Bech32m-encoded human readable version of the address. | 
**aggregation_level** | Option<[**models::ResourceAggregationLevel**](ResourceAggregationLevel.md)> |  | [optional]
**opt_ins** | Option<[**models::StateEntityFungiblesPageRequestOptIns**](StateEntityFungiblesPageRequestOptIns.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


