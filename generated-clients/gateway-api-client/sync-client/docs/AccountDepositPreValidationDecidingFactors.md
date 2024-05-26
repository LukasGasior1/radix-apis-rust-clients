# AccountDepositPreValidationDecidingFactors

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**default_deposit_rule** | [**models::AccountDefaultDepositRule**](AccountDefaultDepositRule.md) |  | 
**is_badge_authorized_depositor** | Option<**bool**> | Whether the input badge belongs to the account's set of authorized depositors. This field will only be present if any badge was passed in the request. | [optional]
**resource_specific_details** | Option<[**Vec<models::AccountDepositPreValidationDecidingFactorsResourceSpecificDetailsItem>**](AccountDepositPreValidationDecidingFactorsResourceSpecificDetailsItem.md)> | Returns deciding factors for each resource. Contains only information about resources presented in the request, not all resource preference rules for queried account. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


