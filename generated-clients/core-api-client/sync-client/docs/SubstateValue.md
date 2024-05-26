# SubstateValue

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**substate_data** | Option<[**models::Substate**](Substate.md)> | The typed substate value. Only returned if enabled in SubstateFormatOptions on your request (default true). | [optional]
**substate_data_hash** | Option<**String**> | The hex-encoded Blake2b-256 hash of the substate data bytes. Only returned if enabled in SubstateFormatOptions on your request (default false). | [optional]
**substate_hex** | Option<**String**> | The hex-encoded, SBOR-encoded substate data bytes. Only returned if enabled in SubstateFormatOptions on your request (default false). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


