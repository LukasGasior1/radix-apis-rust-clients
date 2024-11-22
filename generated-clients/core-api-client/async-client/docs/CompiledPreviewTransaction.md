# CompiledPreviewTransaction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**preview_transaction_hex** | **String** | A hex-encoded, compiled `RawPreviewTransaction`.  As of Cuttlefish, only `PreviewTransactionV2` is supported.  A `PreviewTransactionV2` can be created with a v2 transaction builder: * If using Rust, it can be created with a `TransactionV2Builder` using `build_preview_transaction()`   and then converted to hex with `preview_transaction.to_raw().unwrap().to_hex()` * If using the toolkit, you can create this using the v2 transaction builder.  Some subtleties: * Partial transactions can't be previewed. Instead, they must be wrapped inside a   transaction wrapper, so that the engine knows how to yield to them appropriately. * Currently the builder assumes that the signed partial transactions have real signatures.   This isn't strictly required, and we may create a builder in future which allows providing   public keys when building partial transactions for use in preview. * If you don't have signatures to hand, you can simply not sign the partial transactions,   and then use the `assume_all_signature_proofs` preview flag, although be advised that   this may result in the fee estimate being slightly lower during preview. * We may create more ergonomic builders for PreviewTransactions which allow use of   public keys to denote the signers of subintents. Let us know if this is important   for your use case.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


