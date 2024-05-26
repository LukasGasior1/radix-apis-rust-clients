# NetworkConfigurationResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**address_types** | [**Vec<models::AddressType>**](AddressType.md) |  | 
**network** | **String** | The logical name of the network | 
**network_hrp_suffix** | **String** | The network suffix used for Bech32m HRPs used for addressing. | 
**network_id** | **u32** | The logical id of the network | 
**usd_price_in_xrd** | **String** | The current value of the protocol-based USD/XRD multiplier (i.e. an amount of XRDs to be paid for 1 USD). A decimal is formed of some signed integer `m` of attos (`10^(-18)`) units, where `-2^(192 - 1) <= m < 2^(192 - 1)`.  | 
**version** | [**models::NetworkConfigurationResponseVersion**](NetworkConfigurationResponse_version.md) |  | 
**well_known_addresses** | [**models::NetworkConfigurationResponseWellKnownAddresses**](NetworkConfigurationResponse_well_known_addresses.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


