# AccessControllerFieldStateValue

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**controlled_vault** | [**models::EntityReference**](EntityReference.md) |  | 
**has_primary_role_badge_withdraw_attempt** | **bool** | Whether the primary role badge withdraw is currently being attempted. | 
**has_recovery_role_badge_withdraw_attempt** | **bool** | Whether the recovery role badge withdraw is currently being attempted. | 
**is_primary_role_locked** | **bool** | Whether the primary role is currently locked. | 
**primary_role_recovery_attempt** | Option<[**models::PrimaryRoleRecoveryAttempt**](PrimaryRoleRecoveryAttempt.md)> | The current attempt to recover a primary role, if any. | [optional]
**recovery_badge_resource_address** | **String** | The Bech32m-encoded human readable version of the resource address | 
**recovery_role_recovery_attempt** | Option<[**models::RecoveryRoleRecoveryAttempt**](RecoveryRoleRecoveryAttempt.md)> | The current attempt to recover a recovery role, if any. | [optional]
**timed_recovery_delay_minutes** | Option<**u64**> | An integer between `0` and `2^32 - 1`, specifying the amount of time (in minutes) that it takes for timed recovery to be done. When not present, then timed recovery can not be performed through this access controller.  | [optional]
**xrd_fee_vault** | Option<[**models::EntityReference**](EntityReference.md)> | An optional helper vault storing some amount of XRD that can be used by any of the roles for locking fees.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


