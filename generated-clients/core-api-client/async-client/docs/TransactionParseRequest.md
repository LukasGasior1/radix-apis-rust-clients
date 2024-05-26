# TransactionParseRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**network** | **String** | The logical name of the network | 
**parse_mode** | Option<**String**> | The type of transaction payload that should be assumed. If omitted, \"Any\" is used - where the payload is attempted to be parsed as each of the following in turn: Notarized, Signed, Unsigned, Ledger.  | [optional]
**payload_hex** | **String** | A hex-encoded payload of a full transaction or a partial transaction - either a notarized transaction, a signed transaction intent an unsigned transaction intent, or a ledger payload.  | 
**response_mode** | Option<**String**> | The amount of information to return in the response. \"Basic\" includes the type, validity information, and any relevant identifiers. \"Full\" also includes the fully parsed information. If omitted, \"Full\" is used.  | [optional]
**transaction_format_options** | Option<[**models::TransactionFormatOptions**](TransactionFormatOptions.md)> |  | [optional]
**validation_mode** | Option<**String**> | The type of validation that should be performed, if the payload correctly decompiles as a Notarized Transaction. This is only relevant for Notarized payloads. If omitted, \"Static\" is used.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


