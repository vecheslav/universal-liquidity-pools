use super::{get_account, TestPool, TestPoolMarket};
use solana_program::{program_pack::Pack, pubkey::Pubkey};
use solana_program_test::ProgramTestContext;
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
    transport,
};
use universal_liquidity_pools::{
    find_pool_borrow_authority_program_address, id, instruction, state::PoolBorrowAuthority,
};

pub const SHARE_ALLOWED: u16 = 5_000; // 50% of the total pool

#[derive(Debug)]
pub struct TestPoolBorrowAuthority {
    pub pool_borrow_authority_pubkey: Pubkey,
    pub borrow_authority: Keypair,
}

impl TestPoolBorrowAuthority {
    pub fn new(test_pool: &TestPool) -> Self {
        let borrow_authority = Keypair::new();
        let (pool_borrow_authority_pubkey, _) = find_pool_borrow_authority_program_address(
            &id(),
            &test_pool.pool_pubkey,
            &borrow_authority.pubkey(),
        );

        Self {
            pool_borrow_authority_pubkey,
            borrow_authority,
        }
    }

    pub async fn get_data(&self, context: &mut ProgramTestContext) -> PoolBorrowAuthority {
        let account = get_account(context, &self.pool_borrow_authority_pubkey).await;
        PoolBorrowAuthority::unpack_unchecked(&account.data).unwrap()
    }

    pub async fn create(
        &self,
        context: &mut ProgramTestContext,
        test_pool_market: &TestPoolMarket,
        test_pool: &TestPool,
        share_allowed: u16,
    ) -> transport::Result<()> {
        let tx = Transaction::new_signed_with_payer(
            &[instruction::create_pool_borrow_authority(
                &id(),
                &test_pool_market.pool_market.pubkey(),
                &test_pool.pool_pubkey,
                &self.borrow_authority.pubkey(),
                &test_pool_market.manager.pubkey(),
                share_allowed,
            )],
            Some(&context.payer.pubkey()),
            &[&context.payer, &test_pool_market.manager],
            context.last_blockhash,
        );

        context.banks_client.process_transaction(tx).await
    }

    pub async fn update(
        &self,
        context: &mut ProgramTestContext,
        test_pool_market: &TestPoolMarket,
        test_pool: &TestPool,
        share_allowed: u16,
    ) -> transport::Result<()> {
        let tx = Transaction::new_signed_with_payer(
            &[instruction::update_pool_borrow_authority(
                &id(),
                &test_pool.pool_pubkey,
                &self.borrow_authority.pubkey(),
                &test_pool_market.manager.pubkey(),
                share_allowed,
            )],
            Some(&context.payer.pubkey()),
            &[&context.payer, &test_pool_market.manager],
            context.last_blockhash,
        );

        context.banks_client.process_transaction(tx).await
    }

    pub async fn delete(
        &self,
        context: &mut ProgramTestContext,
        test_pool_market: &TestPoolMarket,
        test_pool: &TestPool,
    ) -> transport::Result<()> {
        let tx = Transaction::new_signed_with_payer(
            &[instruction::delete_pool_borrow_authority(
                &id(),
                &test_pool.pool_pubkey,
                &self.borrow_authority.pubkey(),
                &context.payer.pubkey(),
                &test_pool_market.manager.pubkey(),
            )],
            Some(&context.payer.pubkey()),
            &[&context.payer, &test_pool_market.manager],
            context.last_blockhash,
        );

        context.banks_client.process_transaction(tx).await
    }
}
