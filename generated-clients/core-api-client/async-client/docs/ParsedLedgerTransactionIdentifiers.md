# ParsedLedgerTransactionIdentifiers

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**intent_hash** | Option<**String**> | The hex-encoded intent hash for a user transaction, also known as the transaction id. This hash identifies the core content \"intent\" of the transaction. Each intent can only be committed once. This hash gets signed by any signatories on the transaction, to create the signed intent.  | [optional]
**intent_hash_bech32m** | Option<**String**> | The Bech32m-encoded human readable `IntentHash`. | [optional]
**ledger_hash** | **String** | The hex-encoded ledger payload transaction hash. This is a wrapper for both user transactions, and system transactions such as genesis and round changes.  | 
**ledger_hash_bech32m** | **String** | The Bech32m-encoded human readable `LedgerPayloadHash`. | 
**payload_hash** | Option<**String**> | The hex-encoded notarized transaction hash for a user transaction. This hash identifies the full submittable notarized transaction - ie the signed intent, plus the notary signature.  | [optional]
**payload_hash_bech32m** | Option<**String**> | The Bech32m-encoded human readable `NotarizedTransactionHash`. | [optional]
**signed_intent_hash** | Option<**String**> | The hex-encoded signed intent hash for a user transaction. This hash identifies the transaction intent, plus additional signatures. This hash is signed by the notary, to create the submittable NotarizedTransaction.  | [optional]
**signed_intent_hash_bech32m** | Option<**String**> | The Bech32m-encoded human readable `SignedIntentHash`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


