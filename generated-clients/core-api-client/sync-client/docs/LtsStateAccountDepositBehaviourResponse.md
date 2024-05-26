# LtsStateAccountDepositBehaviourResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**default_deposit_rule** | [**models::DefaultDepositRule**](DefaultDepositRule.md) |  | 
**is_badge_authorized_depositor** | Option<**bool**> | Whether the input `badge` belongs to the account's set of authorized depositors. This field will only be present if any badge was passed in the request.  | [optional]
**ledger_header_summary** | [**models::LedgerHeaderSummary**](LedgerHeaderSummary.md) | The excerpt from the ledger header committed at the `state_version`. | 
**resource_specific_behaviours** | Option<[**std::collections::HashMap<String, models::ResourceSpecificDepositBehaviour>**](ResourceSpecificDepositBehaviour.md)> | A map from one of the input `resource_addresses` to its specific deposit behavior configured for this account. This field will only be present if an array of specific resource addresses was passed in the request (even if empty).  | [optional]
**state_version** | **u64** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


