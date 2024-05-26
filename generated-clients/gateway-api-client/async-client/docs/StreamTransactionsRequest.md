# StreamTransactionsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**at_ledger_state** | Option<[**models::LedgerStateSelector**](LedgerStateSelector.md)> |  | [optional]
**from_ledger_state** | Option<[**models::LedgerStateSelector**](LedgerStateSelector.md)> |  | [optional]
**cursor** | Option<**String**> | This cursor allows forward pagination, by providing the cursor from the previous request. | [optional]
**limit_per_page** | Option<**i32**> | The page size requested. | [optional]
**accounts_with_manifest_owner_method_calls** | Option<**Vec<String>**> |  | [optional]
**accounts_without_manifest_owner_method_calls** | Option<**Vec<String>**> |  | [optional]
**affected_global_entities_filter** | Option<**Vec<String>**> |  | [optional]
**events_filter** | Option<[**Vec<models::StreamTransactionsRequestEventFilterItem>**](StreamTransactionsRequestEventFilterItem.md)> |  | [optional]
**kind_filter** | Option<**String**> | Limit returned transactions by their kind. Defaults to `user`. | [optional]
**manifest_accounts_deposited_into_filter** | Option<**Vec<String>**> |  | [optional]
**manifest_accounts_withdrawn_from_filter** | Option<**Vec<String>**> |  | [optional]
**manifest_badges_presented_filter** | Option<**Vec<String>**> |  | [optional]
**manifest_class_filter** | Option<[**models::StreamTransactionsRequestAllOfManifestClassFilter**](StreamTransactionsRequest_allOf_manifest_class_filter.md)> |  | [optional]
**manifest_resources_filter** | Option<**Vec<String>**> |  | [optional]
**opt_ins** | Option<[**models::TransactionDetailsOptIns**](TransactionDetailsOptIns.md)> |  | [optional]
**order** | Option<**String**> | Configures the order of returned result set. Defaults to `desc`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


