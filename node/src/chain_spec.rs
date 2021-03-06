use iroha_client_no_std::crypto as iroha_crypto;
use sc_service::ChainType;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{crypto::AccountId32, sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::sp_std::convert::TryFrom;
use sp_runtime::traits::{IdentifyAccount, Verify};
use substrate_iroha_bridge_runtime::{
    AccountId, AuraConfig, BalancesConfig, DOTConfig, GenesisConfig, GrandpaConfig,
    IrohaBridgeConfig, KSMConfig, Signature, SudoConfig, SystemConfig, XORConfig, WASM_BINARY,
};

// Note this is the URL for the telemetry server
//const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate an authority key for Aura
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
    (get_from_seed::<AuraId>(s), get_from_seed::<GrandpaId>(s))
}

pub fn development_config() -> ChainSpec {
    ChainSpec::from_genesis(
        "Development",
        "dev",
        ChainType::Development,
        || {
            testnet_genesis(
                vec![authority_keys_from_seed("Alice")],
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
                    AccountId32::from([
                        52u8, 45, 84, 67, 137, 84, 47, 252, 35, 59, 237, 44, 144, 70, 71, 206, 243,
                        67, 8, 115, 247, 189, 204, 26, 181, 226, 232, 81, 123, 12, 81, 120,
                    ]),
                ],
                vec![iroha_crypto::PublicKey::try_from(vec![
                    52u8, 45, 84, 67, 137, 84, 47, 252, 35, 59, 237, 44, 144, 70, 71, 206, 243, 67,
                    8, 115, 247, 189, 204, 26, 181, 226, 232, 81, 123, 12, 81, 120,
                ])
                .unwrap()],
                true,
            )
        },
        vec![],
        None,
        None,
        None,
        None,
    )
}

pub fn local_testnet_config() -> ChainSpec {
    ChainSpec::from_genesis(
        "Local Testnet",
        "local_testnet",
        ChainType::Local,
        || {
            testnet_genesis(
                vec![
                    authority_keys_from_seed("Alice"),
                    authority_keys_from_seed("Bob"),
                ],
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie"),
                    get_account_id_from_seed::<sr25519::Public>("Dave"),
                    get_account_id_from_seed::<sr25519::Public>("Eve"),
                    get_account_id_from_seed::<sr25519::Public>("Ferdie"),
                    get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
                    AccountId32::from([
                        52u8, 45, 84, 67, 137, 84, 47, 252, 35, 59, 237, 44, 144, 70, 71, 206, 243,
                        67, 8, 115, 247, 189, 204, 26, 181, 226, 232, 81, 123, 12, 81, 120,
                    ]),
                ],
                vec![iroha_crypto::PublicKey::try_from(vec![
                    52u8, 45, 84, 67, 137, 84, 47, 252, 35, 59, 237, 44, 144, 70, 71, 206, 243, 67,
                    8, 115, 247, 189, 204, 26, 181, 226, 232, 81, 123, 12, 81, 120,
                ])
                .unwrap()],
                true,
            )
        },
        vec![],
        None,
        None,
        None,
        None,
    )
}

fn testnet_genesis(
    initial_authorities: Vec<(AuraId, GrandpaId)>,
    root_key: AccountId,
    endowed_accounts: Vec<AccountId>,
    iroha_peers: Vec<iroha_crypto::PublicKey>,
    _enable_println: bool,
) -> GenesisConfig {
    GenesisConfig {
        system: Some(SystemConfig {
            code: WASM_BINARY.to_vec(),
            changes_trie_config: Default::default(),
        }),
        // balances: Some(BalancesConfig {
        // 	balances: endowed_accounts.iter().cloned().map(|k|(k, 1 << 60)).collect(),
        // }),
        aura: Some(AuraConfig {
            authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
        }),
        grandpa: Some(GrandpaConfig {
            authorities: initial_authorities
                .iter()
                .map(|x| (x.1.clone(), 1))
                .collect(),
        }),
        sudo: Some(SudoConfig { key: root_key }),
        balances_Instance1: Some(XORConfig {
            balances: endowed_accounts
				.iter()
				.cloned()
				// .filter(|x| {
				// 	x != &AccountId32::from([
				// 		52u8, 45, 84, 67, 137, 84, 47, 252, 35, 59, 237, 44, 144, 70, 71, 206, 243,
				// 		67, 8, 115, 247, 189, 204, 26, 181, 226, 232, 81, 123, 12, 81, 120,
				// 	])
				// })
				// .map(|k| (k, 1 << 60))
				.map(|k| (k, 0))
				.collect(),
        }),
        balances_Instance2: Some(DOTConfig {
            balances: endowed_accounts.iter().cloned().map(|k| (k, 0)).collect(),
        }),
        balances_Instance3: Some(KSMConfig {
            balances: endowed_accounts.iter().cloned().map(|k| (k, 0)).collect(),
        }),
        balances: Some(BalancesConfig {
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, 1 << 60))
                .collect(),
        }),
        iroha_bridge: Some(IrohaBridgeConfig {
            authorities: endowed_accounts.clone(),
            iroha_peers,
        }),
    }
}
