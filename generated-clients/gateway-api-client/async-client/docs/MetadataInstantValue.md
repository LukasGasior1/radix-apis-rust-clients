# MetadataInstantValue

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**unix_timestamp_seconds** | **String** | A decimal string-encoded 64-bit signed integer, marking the unix timestamp in seconds.  Note: this field accurately represents the full range of possible on-ledger values (i.e. `-2^63 <= seconds < 2^63`).  | 
**value** | **String** | The RFC 3339 / ISO 8601 string representation of the timestamp. Will always use \"Z\" (denoting UTC) and omits milliseconds. E.g.: `2023-01-26T18:30:09Z`.  Note: This field will return clamped value if the actual on-ledger `unix_timestamp_seconds` value is outside the basic range supported by the RFC 3339 / ISO 8601 standard, which starts at year 1583 (i.e. the beginning of the Gregorian calendar) and ends at year 9999 (inclusive).  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


