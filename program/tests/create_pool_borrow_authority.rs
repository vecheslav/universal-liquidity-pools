#![cfg(feature = "test-bpf")]

mod utils;

use solana_program_test::*;
use universal_liquidity_pools::state::AccountType;
use utils::*;

async fn setup() -> (ProgramTestContext, TestPoolMarket, TestPool) {
    let mut context = program_test().start_with_context().await;

    let test_pool_market = TestPoolMarket::new();
    test_pool_market.init(&mut context).await.unwrap();

    let test_pool = TestPool::new(&test_pool_market);
    test_pool
        .create(&mut context, &test_pool_market)
        .await
        .unwrap();

    (context, test_pool_market, test_pool)
}

#[tokio::test]
async fn success() {
    let (mut context, test_pool_market, test_pool) = setup().await;

    let test_pool_borrow_authority = TestPoolBorrowAuthority::new(&test_pool);
    test_pool_borrow_authority
        .create(&mut context, &test_pool_market, &test_pool, SHARE_ALLOWED)
        .await
        .unwrap();

    let pool_borrow_authority = test_pool_borrow_authority.get_data(&mut context).await;

    assert_eq!(
        pool_borrow_authority.account_type,
        AccountType::PoolBorrowAuthority
    );
}
