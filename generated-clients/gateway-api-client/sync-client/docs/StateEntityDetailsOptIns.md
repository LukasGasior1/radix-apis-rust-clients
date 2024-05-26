# StateEntityDetailsOptIns

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ancestor_identities** | Option<**bool**> | if set to `true`, ancestor addresses - `parent_address`, `owner_address` and `global_address` for entities are returned. | [optional][default to false]
**component_royalty_config** | Option<**bool**> | if set to `true`, `royalty_config` for component entities is returned. | [optional][default to false]
**component_royalty_vault_balance** | Option<**bool**> | if set to `true`, `royalty_vault_balance` for component entities is returned. | [optional][default to false]
**explicit_metadata** | Option<**Vec<String>**> | allows specifying explicitly metadata properties which should be returned in response. | [optional]
**non_fungible_include_nfids** | Option<**bool**> | if set to `true`, first page of non fungible ids are returned for each non fungible resource, with `next_cursor` which can be later used at `/state/entity/page/non-fungible-vault/ids` endpoint. | [optional][default to false]
**package_royalty_vault_balance** | Option<**bool**> | if set to `true`, `royalty_vault_balance` for package entities is returned. | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


