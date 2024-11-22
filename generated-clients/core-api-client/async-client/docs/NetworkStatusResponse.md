# NetworkStatusResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**current_epoch_round** | Option<[**models::EpochRound**](EpochRound.md)> | The current epoch and round of the ledger. It is not present until genesis has been run.  | [optional]
**current_protocol_version** | **String** | A descriptor for the current protocol version that the node is running.  | 
**current_state_identifier** | [**models::CommittedStateIdentifier**](CommittedStateIdentifier.md) | The current state identifier at the top of the node's copy of the ledger (i.e. as of the latest committed transaction).  | 
**genesis_epoch_round** | Option<[**models::EpochRound**](EpochRound.md)> | The epoch details for the genesis epoch and round. The genesis epoch will be the last Olympia epoch + 1, and have a \"fake\" round-number 1 (because there is no round for the genesis transaction). In the Gateway, this can be used for the epoch and round number before the first RoundUpdate transaction. It is not present until genesis has been run.  | [optional]
**post_genesis_epoch_round** | Option<[**models::EpochRound**](EpochRound.md)> | The post-genesis epoch and round.  | [optional]
**post_genesis_state_identifier** | Option<[**models::CommittedStateIdentifier**](CommittedStateIdentifier.md)> | The ledger state after the genesis transactions have been executed. It is not present until genesis has been run.  | [optional]
**pre_genesis_state_identifier** | [**models::CommittedStateIdentifier**](CommittedStateIdentifier.md) | The ledger state identifier of a fresh ledger before any genesis transactions.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


