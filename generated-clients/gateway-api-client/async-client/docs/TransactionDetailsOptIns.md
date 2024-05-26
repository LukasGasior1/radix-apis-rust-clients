# TransactionDetailsOptIns

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**affected_global_entities** | Option<**bool**> | if set to `true`, all affected global entities by given transaction are returned. | [optional][default to false]
**balance_changes** | Option<**bool**> | if set to `true`, returns the fungible and non-fungible balance changes.  **Warning!** This opt-in might be missing for recently committed transactions, in that case a `null` value will be returned. Retry the request until non-null value is returned.  | [optional][default to false]
**manifest_instructions** | Option<**bool**> | if set to `true`, manifest instructions for user transactions are returned. | [optional][default to false]
**raw_hex** | Option<**bool**> | if set to `true`, raw transaction hex is returned. | [optional][default to false]
**receipt_costing_parameters** | Option<**bool**> | if set to `true`, costing parameters inside receipt object is returned. | [optional][default to false]
**receipt_events** | Option<**bool**> | if set to `true`, events inside receipt object is returned. | [optional][default to false]
**receipt_fee_destination** | Option<**bool**> | if set to `true`, fee destination inside receipt object is returned. | [optional][default to false]
**receipt_fee_source** | Option<**bool**> | if set to `true`, fee source inside receipt object is returned. | [optional][default to false]
**receipt_fee_summary** | Option<**bool**> | if set to `true`, fee summary inside receipt object is returned. | [optional][default to false]
**receipt_output** | Option<**bool**> | (true by default) if set to `true`, transaction receipt output is returned. | [optional][default to true]
**receipt_state_changes** | Option<**bool**> | if set to `true`, state changes inside receipt object are returned. | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


