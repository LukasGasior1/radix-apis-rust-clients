# StreamTransactionsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**at_ledger_state** | Option<[**models::LedgerStateSelector**](LedgerStateSelector.md)> |  | [optional]
**from_ledger_state** | Option<[**models::LedgerStateSelector**](LedgerStateSelector.md)> |  | [optional]
**cursor** | Option<**String**> | This cursor allows forward pagination, by providing the cursor from the previous request. | [optional]
**limit_per_page** | Option<**i32**> | The page size requested. | [optional]
**accounts_with_manifest_owner_method_calls** | Option<**Vec<String>**> | Allows specifying an array of account addresses. If specified, the response will contain only transactions that, for all specified accounts, contain manifest method calls to that account which require the owner role. See the [account docs](https://docs.radixdlt.com/docs/account) for more information. | [optional]
**accounts_without_manifest_owner_method_calls** | Option<**Vec<String>**> | Allows specifying an array of account addresses. If specified, the response will contain only transactions that, for all specified accounts, do NOT contain manifest method calls to that account which require owner role. See the [account docs](https://docs.radixdlt.com/docs/account) for more information. | [optional]
**affected_global_entities_filter** | Option<**Vec<String>**> | Allows specifying an array of global addresses. If specified, the response will contain transactions that affected all of the given global entities. A global entity is marked as \"affected\" by a transaction if any of its state (or its descendents' state) was modified as a result of the transaction. For performance reasons consensus manager and transaction tracker are excluded from that filter. | [optional]
**event_global_emitters_filter** | Option<**Vec<String>**> | Allows specifying an array of global addresses. If specified, the response will contain transactions in which all entities emitted events. If an event was published by an internal entity, it is going to be indexed as it is a global ancestor. For performance reasons events published by consensus manager and native XRD resource are excluded from that filter. | [optional]
**events_filter** | Option<[**Vec<models::StreamTransactionsRequestEventFilterItem>**](StreamTransactionsRequestEventFilterItem.md)> | Filters the transaction stream to transactions which emitted at least one event matching each filter (each filter can be satisfied by a different event). Currently *only* deposit and withdrawal events emitted by an internal vault entity are tracked. For the purpose of filtering, the emitter address is replaced by the global ancestor of the emitter, for example, the top-level account / component which contains the vault which emitted the event. | [optional]
**kind_filter** | Option<**String**> | Limit returned transactions by their kind. Defaults to `user`. | [optional]
**manifest_accounts_deposited_into_filter** | Option<**Vec<String>**> | Similar to `manifest_accounts_withdrawn_from_filter`, but will return only transactions with a manifest containing deposits to the given accounts. | [optional]
**manifest_accounts_withdrawn_from_filter** | Option<**Vec<String>**> | Allows specifying an array of account addresses. If specified, the response will contain only transactions with a manifest containing withdrawals from the given accounts. | [optional]
**manifest_badges_presented_filter** | Option<**Vec<String>**> | Allows specifying array of badge resource addresses. If specified, the response will contain only transactions where the given badges were presented. | [optional]
**manifest_class_filter** | Option<[**models::StreamTransactionsRequestAllOfManifestClassFilter**](StreamTransactionsRequest_allOf_manifest_class_filter.md)> |  | [optional]
**manifest_resources_filter** | Option<**Vec<String>**> | Allows specifying array of resource addresses. If specified, the response will contain only transactions containing the given resources in the manifest (regardless of their usage). | [optional]
**opt_ins** | Option<[**models::TransactionDetailsOptIns**](TransactionDetailsOptIns.md)> |  | [optional]
**order** | Option<**String**> | Configures the order of returned result set. Defaults to `desc`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


