# TransactionPreviewRequestFlags

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**assume_all_signature_proofs** | **bool** |  | 
**disable_auth_checks** | Option<**bool**> | Can be used to skip performing auth checks during runtime.  This could be used to e.g.: * Preview protocol update style transactions * Mint resources for previewing trades with resources you don't own. If doing this, be warned:   * Only resources which were potentially mintable/burnable at creation time   will be mintable/burnable, due to feature flags on the resource.   * Please see the below warning about unexpected results if using this approach.  Please be warned - this mode of operation is quite a departure from normal operation: * Calculated fees will likely be lower than a standard execution * This mode can subtly break invariants some dApp code might rely on, or result in unexpected behaviour, so the resulting execution result might not be valid for your needs. For example, if I used this flag to mint pool units to preview a redemption (or some dApp interaction which behind the scenes redeemed them), they'd redeem for less than they're currently worth, because the blueprint code relies on the total supply of the pool units to calculate their redemption worth, and you've just inflated the total supply through the mint operation.  | [optional]
**skip_epoch_check** | **bool** |  | 
**use_free_credit** | **bool** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


