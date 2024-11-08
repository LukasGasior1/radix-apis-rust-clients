# EncryptedTransactionMessage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**curve_decryptor_sets** | [**Vec<models::EncryptedMessageCurveDecryptorSet>**](EncryptedMessageCurveDecryptorSet.md) |  | 
**encrypted_hex** | **String** | The hex-encoded (128-bit) AES-GCM encrypted bytes of an SBOR-encoded `PlaintextTransactionMessage`. The bytes are serialized as the concatenation `Nonce/IV (12 bytes) || Cipher (variable length) || Tag/MAC (16 bytes)`:  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


