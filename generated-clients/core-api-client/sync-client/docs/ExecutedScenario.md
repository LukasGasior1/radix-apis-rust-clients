# ExecutedScenario

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**addresses** | **std::collections::HashMap<String, String>** | Well-named addresses touched/created by the Scenario, keyed by their name.  | 
**committed_transactions** | [**Vec<models::ExecutedScenarioTransaction>**](ExecutedScenarioTransaction.md) | Transactions successfully committed by the Scenario. | 
**logical_name** | **String** |  | 
**sequence_number** | **u32** | An index of the Scenario (reflecting its execution order). | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


