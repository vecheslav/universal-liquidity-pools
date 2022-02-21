#![cfg(feature = "test-bpf")]

mod utils;

use solana_program_test::*;
use universal_liquidity_pools::state::AccountType;
use utils::*;

async fn setup() -> (ProgramTestContext, TestPoolMarket) {
    let mut context = program_test().start_with_context().await;

    let test_pool_market = TestPoolMarket::new();
    test_pool_market.init(&mut context).await.unwrap();

    (context, test_pool_market)
}

#[tokio::test]
async fn success() {
    let (mut context, test_pool_market) = setup().await;

    let test_pool = TestPool::new(&test_pool_market);
    test_pool
        .create(&mut context, &test_pool_market)
        .await
        .unwrap();

    let pool = test_pool.get_data(&mut context).await;

    assert_eq!(pool.account_type, AccountType::Pool);
}
