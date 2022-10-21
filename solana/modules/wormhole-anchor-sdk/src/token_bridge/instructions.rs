use anchor_lang::{prelude::*, solana_program};
use anchor_spl;

#[derive(AnchorDeserialize, AnchorSerialize)]
pub enum Instruction {
    Initialize,
    AttestToken,
    CompleteNative,
    CompleteWrapped,
    TransferWrapped,
    TransferNative,
    RegisterChain,
    CreateWrapped,
    UpgradeContract,
    CompleteNativeWithPayload,
    CompleteWrappedWithPayload,
    TransferWrappedWithPayload,
    TransferNativeWithPayload,
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct TransferNativeWithPayloadData {
    pub batch_id: u32,
    pub amount: u64,
    pub recipient_address: [u8; 32],
    pub recipient_chain: u16,
    pub payload: Vec<u8>,
    pub cpi_program_id: Option<Pubkey>,
}

#[derive(Accounts)]
pub struct TransferNativeWithPayload<'info> {
    pub payer: AccountInfo<'info>,
    pub config: AccountInfo<'info>,
    pub from: AccountInfo<'info>,
    pub mint: AccountInfo<'info>,
    pub custody: AccountInfo<'info>,
    pub authority_signer: AccountInfo<'info>,
    pub custody_signer: AccountInfo<'info>,
    pub wormhole_config: AccountInfo<'info>,
    pub wormhole_message: AccountInfo<'info>,
    pub wormhole_emitter: AccountInfo<'info>,
    pub wormhole_sequence: AccountInfo<'info>,
    pub wormhole_fee_collector: AccountInfo<'info>,
    pub clock: AccountInfo<'info>,
    pub sender: AccountInfo<'info>,
    pub rent: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    pub wormhole_program: AccountInfo<'info>,
}

pub fn transfer_native_with_payload<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, TransferNativeWithPayload<'info>>,
    batch_id: u32,
    amount: u64,
    recipient_address: [u8; 32],
    recipient_chain: u16,
    payload: Vec<u8>,
    cpi_program_id: &Pubkey,
) -> Result<()> {
    let ix = solana_program::instruction::Instruction {
        program_id: ctx.program.key(),
        accounts: vec![
            AccountMeta::new(ctx.accounts.payer.key(), true),
            AccountMeta::new_readonly(ctx.accounts.config.key(), false),
            AccountMeta::new(ctx.accounts.from.key(), false),
            AccountMeta::new(ctx.accounts.mint.key(), false),
            AccountMeta::new(ctx.accounts.custody.key(), false),
            AccountMeta::new_readonly(ctx.accounts.authority_signer.key(), false),
            AccountMeta::new_readonly(ctx.accounts.custody_signer.key(), false),
            AccountMeta::new(ctx.accounts.wormhole_config.key(), false),
            AccountMeta::new(ctx.accounts.wormhole_message.key(), true),
            AccountMeta::new_readonly(ctx.accounts.wormhole_emitter.key(), false),
            AccountMeta::new(ctx.accounts.wormhole_sequence.key(), false),
            AccountMeta::new(ctx.accounts.wormhole_fee_collector.key(), false),
            AccountMeta::new_readonly(solana_program::sysvar::clock::id(), false),
            AccountMeta::new(ctx.accounts.sender.key(), true),
            AccountMeta::new_readonly(solana_program::sysvar::rent::id(), false),
            AccountMeta::new_readonly(ctx.accounts.system_program.key(), false),
            AccountMeta::new_readonly(ctx.accounts.wormhole_program.key(), false),
            AccountMeta::new_readonly(anchor_spl::token::spl_token::id(), false),
        ],
        data: (
            Instruction::TransferNativeWithPayload,
            TransferNativeWithPayloadData {
                batch_id,
                amount,
                recipient_address,
                recipient_chain,
                payload,
                cpi_program_id: Some(*cpi_program_id),
            },
        )
            .try_to_vec()?,
    };

    solana_program::program::invoke_signed(
        &ix,
        &ToAccountInfos::to_account_infos(&ctx),
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

pub fn approve_and_transfer_native_with_payload<'a, 'b, 'c, 'info>(
    approve_ctx: CpiContext<'a, 'b, 'c, 'info, anchor_spl::token::Approve<'info>>,
    transfer_ctx: CpiContext<'a, 'b, 'c, 'info, TransferNativeWithPayload<'info>>,
    batch_id: u32,
    amount: u64,
    recipient_address: [u8; 32],
    recipient_chain: u16,
    payload: Vec<u8>,
    cpi_program_id: &Pubkey,
) -> Result<()> {
    anchor_spl::token::approve(approve_ctx, amount)?;

    transfer_native_with_payload(
        transfer_ctx,
        batch_id,
        amount,
        recipient_address,
        recipient_chain,
        payload,
        cpi_program_id,
    )
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct TransferWrappedWithPayloadData {
    pub batch_id: u32,
    pub amount: u64,
    pub recipient_address: [u8; 32],
    pub recipient_chain: u16,
    pub payload: Vec<u8>,
    pub cpi_program_id: Option<Pubkey>,
}

#[derive(Accounts)]
pub struct TransferWrappedWithPayload<'info> {
    pub payer: AccountInfo<'info>,
    pub config: AccountInfo<'info>,
    pub from: AccountInfo<'info>,
    pub from_owner: AccountInfo<'info>,
    pub wrapped_mint: AccountInfo<'info>,
    pub wrapped_metadata: AccountInfo<'info>,
    pub authority_signer: AccountInfo<'info>,
    pub wormhole_config: AccountInfo<'info>,
    pub wormhole_message: AccountInfo<'info>,
    pub wormhole_emitter: AccountInfo<'info>,
    pub wormhole_sequence: AccountInfo<'info>,
    pub wormhole_fee_collector: AccountInfo<'info>,
    pub clock: AccountInfo<'info>,
    pub sender: AccountInfo<'info>,
    pub rent: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    pub wormhole_program: AccountInfo<'info>,
}

pub fn transfer_wrapped_with_payload<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, TransferWrappedWithPayload<'info>>,
    batch_id: u32,
    amount: u64,
    recipient_address: [u8; 32],
    recipient_chain: u16,
    payload: Vec<u8>,
    cpi_program_id: &Pubkey,
) -> Result<()> {
    let ix = solana_program::instruction::Instruction {
        program_id: ctx.program.key(),
        accounts: vec![
            AccountMeta::new(ctx.accounts.payer.key(), true),
            AccountMeta::new_readonly(ctx.accounts.config.key(), false),
            AccountMeta::new(ctx.accounts.from.key(), false),
            AccountMeta::new_readonly(ctx.accounts.from_owner.key(), false),
            AccountMeta::new(ctx.accounts.wrapped_mint.key(), false),
            AccountMeta::new_readonly(ctx.accounts.wrapped_metadata.key(), false),
            AccountMeta::new_readonly(ctx.accounts.authority_signer.key(), false),
            AccountMeta::new(ctx.accounts.wormhole_config.key(), false),
            AccountMeta::new(ctx.accounts.wormhole_message.key(), true),
            AccountMeta::new_readonly(ctx.accounts.wormhole_emitter.key(), false),
            AccountMeta::new(ctx.accounts.wormhole_sequence.key(), false),
            AccountMeta::new(ctx.accounts.wormhole_fee_collector.key(), false),
            AccountMeta::new_readonly(solana_program::sysvar::clock::id(), false),
            AccountMeta::new(ctx.accounts.sender.key(), true),
            AccountMeta::new_readonly(solana_program::sysvar::rent::id(), false),
            AccountMeta::new_readonly(ctx.accounts.system_program.key(), false),
            AccountMeta::new_readonly(ctx.accounts.wormhole_program.key(), false),
            AccountMeta::new_readonly(anchor_spl::token::spl_token::id(), false),
        ],
        data: (
            Instruction::TransferWrappedWithPayload,
            TransferWrappedWithPayloadData {
                batch_id,
                amount,
                recipient_address,
                recipient_chain,
                payload,
                cpi_program_id: Some(*cpi_program_id),
            },
        )
            .try_to_vec()?,
    };

    solana_program::program::invoke_signed(
        &ix,
        &ToAccountInfos::to_account_infos(&ctx),
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

pub fn approve_and_transfer_wrapped_with_payload<'a, 'b, 'c, 'info>(
    approve_ctx: CpiContext<'a, 'b, 'c, 'info, anchor_spl::token::Approve<'info>>,
    transfer_ctx: CpiContext<'a, 'b, 'c, 'info, TransferWrappedWithPayload<'info>>,
    batch_id: u32,
    amount: u64,
    recipient_address: [u8; 32],
    recipient_chain: u16,
    payload: Vec<u8>,
    cpi_program_id: &Pubkey,
) -> Result<()> {
    anchor_spl::token::approve(approve_ctx, amount)?;

    transfer_wrapped_with_payload(
        transfer_ctx,
        batch_id,
        amount,
        recipient_address,
        recipient_chain,
        payload,
        cpi_program_id,
    )
}
