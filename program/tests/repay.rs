#![cfg(feature = "test-bpf")]

mod utils;

use solana_program_test::*;
use utils::*;

async fn setup() -> (
    ProgramTestContext,
    TestPoolMarket,
    TestPool,
    TestPoolBorrowAuthority,
    User,
) {
    let mut context = program_test().start_with_context().await;

    let test_pool_market = TestPoolMarket::new();
    test_pool_market.init(&mut context).await.unwrap();

    let test_pool = TestPool::new(&test_pool_market);
    test_pool
        .create(&mut context, &test_pool_market)
        .await
        .unwrap();

    let test_pool_borrow_authority = TestPoolBorrowAuthority::new(&test_pool);
    test_pool_borrow_authority
        .create(&mut context, &test_pool_market, &test_pool, SHARE_ALLOWED)
        .await
        .unwrap();

    let user = add_user(&mut context, &test_pool, 101).await.unwrap();

    test_pool
        .deposit(&mut context, &test_pool_market, &user, 100)
        .await
        .unwrap();

    (
        context,
        test_pool_market,
        test_pool,
        test_pool_borrow_authority,
        user,
    )
}

#[tokio::test]
async fn success() {
    let (mut context, test_pool_market, test_pool, test_pool_borrow_authority, user) =
        setup().await;
    let amount_allowed =
        get_amount_allowed(&mut context, &test_pool, &test_pool_borrow_authority).await;

    test_pool
        .borrow(
            &mut context,
            &test_pool_market,
            &test_pool_borrow_authority,
            &test_pool_borrow_authority.borrow_authority,
            &user.source,
            amount_allowed,
        )
        .await
        .unwrap();

    test_pool
        .repay(
            &mut context,
            &test_pool_market,
            &test_pool_borrow_authority,
            &user,
            amount_allowed,
            1,
        )
        .await
        .unwrap();

    assert_eq!(get_token_balance(&mut context, &user.source).await, 0);
    assert_eq!(
        test_pool.get_data(&mut context).await.total_amount_borrowed,
        0
    );
}
