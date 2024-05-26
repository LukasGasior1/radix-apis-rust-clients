# LtsStateAccountDepositBehaviourRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_address** | **String** | The Bech32m-encoded human readable version of the account's address. | 
**badge** | Option<[**models::PresentedBadge**](PresentedBadge.md)> | The depositor badge to check against the account's set of authorized depositors. | [optional]
**network** | **String** | The logical name of the network | 
**resource_addresses** | Option<**Vec<String>**> | The resource addresses to check the deposit behaviours of. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


