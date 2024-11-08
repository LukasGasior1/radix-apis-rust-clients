# TransactionHeaderV2

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**notary_is_signatory** | **bool** | Specifies whether the notary public key should be included in the transaction signers list | 
**notary_public_key** | [**models::PublicKey**](PublicKey.md) |  | 
**tip_basis_points** | **u64** | An integer between `0` and `2^32 - 1 = 4294967295`, giving the validator tip as a basis points amount. That is, a value of `1` corresponds to an additional tip on 0.01% of the base fee.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


