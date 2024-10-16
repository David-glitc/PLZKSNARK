use subxt_lightclient::{ChainConfig, LightClient};
/// Creates and configures a new Polkadot light node.
///
/// # Returns
///
/// - A light client configured to connect to the Polkadot network.
///
/// # Example
/// ```rust
/// use polkadot_light_zksnark::create_light_node;
/// let light_node = create_light_node();
/// ```
pub fn create_light_node() -> LightClient {

    let config = ChainConfig::default();
    let light_client = LightClient::new(config);
    light_client
}