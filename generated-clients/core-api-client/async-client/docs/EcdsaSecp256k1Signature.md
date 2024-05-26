# EcdsaSecp256k1Signature

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key_type** | [**models::PublicKeyType**](PublicKeyType.md) |  | 
**signature_hex** | **String** | A hex-encoded recoverable ECDSA Secp256k1 signature (65 bytes). The first byte is the recovery id, the remaining 64 bytes are the compact signature, ie `CONCAT(R, s)` where `R` and `s` are each 32-bytes in padded big-endian format. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


