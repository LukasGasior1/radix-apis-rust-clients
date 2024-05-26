# GenesisLedgerTransaction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**payload_hex** | Option<**String**> | The hex-encoded full ledger transaction payload. Only returned if enabled in TransactionFormatOptions on your request. | [optional]
**r#type** | [**models::LedgerTransactionType**](LedgerTransactionType.md) |  | 
**is_flash** | **bool** | The first genesis \"transaction\" flashes state into the database to prepare for the bootstrap transaction. Such a transaction does not have an associated `system_transaction`  | 
**system_transaction** | Option<[**models::SystemTransaction**](SystemTransaction.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


