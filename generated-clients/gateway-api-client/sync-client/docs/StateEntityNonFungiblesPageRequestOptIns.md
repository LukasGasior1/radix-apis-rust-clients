# StateEntityNonFungiblesPageRequestOptIns

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**explicit_metadata** | Option<**Vec<String>**> | allows specifying explicitly metadata properties which should be returned in response, limited to max 20 items. | [optional]
**non_fungible_include_nfids** | Option<**bool**> | if set to `true`, first page of non fungible ids are returned for each non fungible resource, with cursor which can be later used at `/state/entity/page/non-fungible-vault/ids` endpoint. | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


