# EncryptedMessageDecryptor

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**aes_wrapped_key_hex** | **String** | The hex-encoded wrapped key bytes from applying RFC 3394 (256-bit) AES-KeyWrap to the 128-bit message ephemeral public key, with the secret KEK provided by static Diffie-Helman between the decryptor public key, and the `dh_ephemeral_public_key` for that curve type. The bytes are serialized (according to RFC 3394) as the concatenation `IV (first 8 bytes) || Cipher (wrapped 128-bit key, encoded as two 64-bit blocks)`.  | 
**public_key_fingerprint_hex** | **String** | The last 8 bytes of the Blake2b-256 hash of the public key bytes, in their standard Radix byte-serialization. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


