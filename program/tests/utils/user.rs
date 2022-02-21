use super::{create_token_account, mint_tokens, TestPool};
use solana_program::pubkey::Pubkey;
use solana_program_test::ProgramTestContext;
use solana_sdk::{
    signature::{Keypair, Signer},
    transport,
};

#[derive(Debug)]
pub struct User {
    pub owner: Keypair,
    pub source: Pubkey,
    pub destination: Pubkey,
}

impl User {
    pub fn pubkey(&self) -> Pubkey {
        self.owner.pubkey()
    }
}

pub async fn add_user(
    context: &mut ProgramTestContext,
    test_pool: &TestPool,
    mint_amount: u64,
) -> transport::Result<User> {
    let user = Keypair::new();
    let source = Keypair::new();
    let destination = Keypair::new();

    create_token_account(
        context,
        &source,
        &test_pool.token_mint.pubkey(),
        &user.pubkey(),
    )
    .await?;

    create_token_account(
        context,
        &destination,
        &test_pool.pool_mint.pubkey(),
        &user.pubkey(),
    )
    .await?;

    mint_tokens(
        context,
        &test_pool.token_mint.pubkey(),
        &source.pubkey(),
        mint_amount,
    )
    .await?;

    Ok(User {
        owner: user,
        source: source.pubkey(),
        destination: destination.pubkey(),
    })
}
