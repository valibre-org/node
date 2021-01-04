use core::marker::PhantomData;
use sp_keyring::AccountKeyring;
use substrate_subxt::{extrinsic::PairSigner, sudo::SudoCallExt, ClientBuilder};
use valiu_node_commons::{Asset, Collateral};
use valiu_node_rpc::{AttestCall, ValiuRuntime};

#[tokio::test]
async fn attest() {
    let _ = env_logger::builder().is_test(true).try_init();

    let alice_pair = PairSigner::new(AccountKeyring::Alice.pair());
    let bob_pair = PairSigner::new(AccountKeyring::Alice.pair());

    let mut client = ClientBuilder::<ValiuRuntime>::new().build().await.unwrap();

    let _ = crate::add_bob_as_a_member(&mut client, &alice_pair).await;

    let encoded = client
        .encode(AttestCall {
            asset: Asset::Collateral(Collateral::Usdc),
            balance: 1,
            offer_rates: vec![],
            phantom: PhantomData,
        })
        .unwrap();

    let _ = client.sudo_and_watch(&bob_pair, &encoded).await;
}