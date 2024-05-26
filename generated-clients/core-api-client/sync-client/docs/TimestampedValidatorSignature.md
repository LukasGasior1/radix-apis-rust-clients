# TimestampedValidatorSignature

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**signature** | [**models::EcdsaSecp256k1Signature**](EcdsaSecp256k1Signature.md) |  | 
**timestamp_ms** | **u64** | An integer between `0` and `10^14`, marking the unix timestamp in ms. | 
**validator_address** | **String** | The Bech32m-encoded human readable version of the component address | 
**validator_key** | [**models::EcdsaSecp256k1PublicKey**](EcdsaSecp256k1PublicKey.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


