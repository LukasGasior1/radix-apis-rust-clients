# LedgerStateSelector

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**epoch** | Option<**u64**> | If provided, the ledger state lower than or equal to the given epoch at round 0 is returned. | [optional]
**round** | Option<**u64**> | If provided must be accompanied with `epoch`, the ledger state lower than or equal to the given epoch and round is returned. | [optional]
**state_version** | Option<**u64**> | If provided, the latest ledger state lower than or equal to the given state version is returned. | [optional]
**timestamp** | Option<**String**> | If provided, the latest ledger state lower than or equal to the given round timestamp is returned. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


