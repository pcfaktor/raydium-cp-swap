pub mod raydium_cp_swap_fuzz_instructions {
    use crate::accounts_snapshots::*;
    use trident_client::fuzzing::*;
    #[derive(Arbitrary, DisplayIx, FuzzTestExecutor, FuzzDeserialize)]
    pub enum FuzzInstruction {
        CreateAmmConfig(CreateAmmConfig),
        UpdateAmmConfig(UpdateAmmConfig),
        UpdatePoolStatus(UpdatePoolStatus),
        CollectProtocolFee(CollectProtocolFee),
        CollectFundFee(CollectFundFee),
        Initialize(Initialize),
        Deposit(Deposit),
        Withdraw(Withdraw),
        SwapBaseInput(SwapBaseInput),
        SwapBaseOutput(SwapBaseOutput),
    }
    #[derive(Arbitrary, Debug)]
    pub struct CreateAmmConfig {
        pub accounts: CreateAmmConfigAccounts,
        pub data: CreateAmmConfigData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct CreateAmmConfigAccounts {
        pub owner: AccountId,
        pub amm_config: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct CreateAmmConfigData {
        pub index: u16,
        pub trade_fee_rate: u64,
        pub protocol_fee_rate: u64,
        pub fund_fee_rate: u64,
        pub create_pool_fee: u64,
    }
    #[derive(Arbitrary, Debug)]
    pub struct UpdateAmmConfig {
        pub accounts: UpdateAmmConfigAccounts,
        pub data: UpdateAmmConfigData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct UpdateAmmConfigAccounts {
        pub owner: AccountId,
        pub amm_config: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct UpdateAmmConfigData {
        pub param: u8,
        pub value: u64,
    }
    #[derive(Arbitrary, Debug)]
    pub struct UpdatePoolStatus {
        pub accounts: UpdatePoolStatusAccounts,
        pub data: UpdatePoolStatusData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct UpdatePoolStatusAccounts {
        pub authority: AccountId,
        pub pool_state: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct UpdatePoolStatusData {
        pub status: u8,
    }
    #[derive(Arbitrary, Debug)]
    pub struct CollectProtocolFee {
        pub accounts: CollectProtocolFeeAccounts,
        pub data: CollectProtocolFeeData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct CollectProtocolFeeAccounts {
        pub owner: AccountId,
        pub authority: AccountId,
        pub pool_state: AccountId,
        pub amm_config: AccountId,
        pub token_0_vault: AccountId,
        pub token_1_vault: AccountId,
        pub vault_0_mint: AccountId,
        pub vault_1_mint: AccountId,
        pub recipient_token_0_account: AccountId,
        pub recipient_token_1_account: AccountId,
        pub token_program: AccountId,
        pub token_program_2022: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct CollectProtocolFeeData {
        pub amount_0_requested: u64,
        pub amount_1_requested: u64,
    }
    #[derive(Arbitrary, Debug)]
    pub struct CollectFundFee {
        pub accounts: CollectFundFeeAccounts,
        pub data: CollectFundFeeData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct CollectFundFeeAccounts {
        pub owner: AccountId,
        pub authority: AccountId,
        pub pool_state: AccountId,
        pub amm_config: AccountId,
        pub token_0_vault: AccountId,
        pub token_1_vault: AccountId,
        pub vault_0_mint: AccountId,
        pub vault_1_mint: AccountId,
        pub recipient_token_0_account: AccountId,
        pub recipient_token_1_account: AccountId,
        pub token_program: AccountId,
        pub token_program_2022: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct CollectFundFeeData {
        pub amount_0_requested: u64,
        pub amount_1_requested: u64,
    }
    #[derive(Arbitrary, Debug)]
    pub struct Initialize {
        pub accounts: InitializeAccounts,
        pub data: InitializeData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitializeAccounts {
        pub creator: AccountId,
        pub amm_config: AccountId,
        pub authority: AccountId,
        pub pool_state: AccountId,
        pub token_0_mint: AccountId,
        pub token_1_mint: AccountId,
        pub lp_mint: AccountId,
        pub creator_token_0: AccountId,
        pub creator_token_1: AccountId,
        pub creator_lp_token: AccountId,
        pub token_0_vault: AccountId,
        pub token_1_vault: AccountId,
        pub create_pool_fee: AccountId,
        pub observation_state: AccountId,
        pub token_program: AccountId,
        pub token_0_program: AccountId,
        pub token_1_program: AccountId,
        pub associated_token_program: AccountId,
        pub system_program: AccountId,
        pub rent: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitializeData {
        pub init_amount_0: u64,
        pub init_amount_1: u64,
        pub open_time: u64,
    }
    #[derive(Arbitrary, Debug)]
    pub struct Deposit {
        pub accounts: DepositAccounts,
        pub data: DepositData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct DepositAccounts {
        pub owner: AccountId,
        pub authority: AccountId,
        pub pool_state: AccountId,
        pub owner_lp_token: AccountId,
        pub token_0_account: AccountId,
        pub token_1_account: AccountId,
        pub token_0_vault: AccountId,
        pub token_1_vault: AccountId,
        pub token_program: AccountId,
        pub token_program_2022: AccountId,
        pub vault_0_mint: AccountId,
        pub vault_1_mint: AccountId,
        pub lp_mint: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct DepositData {
        pub lp_token_amount: u64,
        pub maximum_token_0_amount: u64,
        pub maximum_token_1_amount: u64,
    }
    #[derive(Arbitrary, Debug)]
    pub struct Withdraw {
        pub accounts: WithdrawAccounts,
        pub data: WithdrawData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct WithdrawAccounts {
        pub owner: AccountId,
        pub authority: AccountId,
        pub pool_state: AccountId,
        pub owner_lp_token: AccountId,
        pub token_0_account: AccountId,
        pub token_1_account: AccountId,
        pub token_0_vault: AccountId,
        pub token_1_vault: AccountId,
        pub token_program: AccountId,
        pub token_program_2022: AccountId,
        pub vault_0_mint: AccountId,
        pub vault_1_mint: AccountId,
        pub lp_mint: AccountId,
        pub memo_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct WithdrawData {
        pub lp_token_amount: u64,
        pub minimum_token_0_amount: u64,
        pub minimum_token_1_amount: u64,
    }
    #[derive(Arbitrary, Debug)]
    pub struct SwapBaseInput {
        pub accounts: SwapBaseInputAccounts,
        pub data: SwapBaseInputData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct SwapBaseInputAccounts {
        pub payer: AccountId,
        pub authority: AccountId,
        pub amm_config: AccountId,
        pub pool_state: AccountId,
        pub input_token_account: AccountId,
        pub output_token_account: AccountId,
        pub input_vault: AccountId,
        pub output_vault: AccountId,
        pub input_token_program: AccountId,
        pub output_token_program: AccountId,
        pub input_token_mint: AccountId,
        pub output_token_mint: AccountId,
        pub observation_state: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct SwapBaseInputData {
        pub amount_in: u64,
        pub minimum_amount_out: u64,
    }
    #[derive(Arbitrary, Debug)]
    pub struct SwapBaseOutput {
        pub accounts: SwapBaseOutputAccounts,
        pub data: SwapBaseOutputData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct SwapBaseOutputAccounts {
        pub payer: AccountId,
        pub authority: AccountId,
        pub amm_config: AccountId,
        pub pool_state: AccountId,
        pub input_token_account: AccountId,
        pub output_token_account: AccountId,
        pub input_vault: AccountId,
        pub output_vault: AccountId,
        pub input_token_program: AccountId,
        pub output_token_program: AccountId,
        pub input_token_mint: AccountId,
        pub output_token_mint: AccountId,
        pub observation_state: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct SwapBaseOutputData {
        pub max_amount_in: u64,
        pub amount_out: u64,
    }
    impl<'info> IxOps<'info> for CreateAmmConfig {
        type IxData = raydium_cp_swap::instruction::CreateAmmConfig;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = CreateAmmConfigSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = raydium_cp_swap::instruction::CreateAmmConfig {
                index: todo!(),
                trade_fee_rate: todo!(),
                protocol_fee_rate: todo!(),
                fund_fee_rate: todo!(),
                create_pool_fee: todo!(),
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = raydium_cp_swap::accounts::CreateAmmConfig {
                owner: todo!(),
                amm_config: todo!(),
                system_program: todo!(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for UpdateAmmConfig {
        type IxData = raydium_cp_swap::instruction::UpdateAmmConfig;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = UpdateAmmConfigSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = raydium_cp_swap::instruction::UpdateAmmConfig {
                param: todo!(),
                value: todo!(),
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = raydium_cp_swap::accounts::UpdateAmmConfig {
                owner: todo!(),
                amm_config: todo!(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for UpdatePoolStatus {
        type IxData = raydium_cp_swap::instruction::UpdatePoolStatus;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = UpdatePoolStatusSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = raydium_cp_swap::instruction::UpdatePoolStatus { status: todo!() };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = raydium_cp_swap::accounts::UpdatePoolStatus {
                authority: todo!(),
                pool_state: todo!(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for CollectProtocolFee {
        type IxData = raydium_cp_swap::instruction::CollectProtocolFee;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = CollectProtocolFeeSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = raydium_cp_swap::instruction::CollectProtocolFee {
                amount_0_requested: todo!(),
                amount_1_requested: todo!(),
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = raydium_cp_swap::accounts::CollectProtocolFee {
                owner: todo!(),
                authority: todo!(),
                pool_state: todo!(),
                amm_config: todo!(),
                token_0_vault: todo!(),
                token_1_vault: todo!(),
                vault_0_mint: todo!(),
                vault_1_mint: todo!(),
                recipient_token_0_account: todo!(),
                recipient_token_1_account: todo!(),
                token_program: todo!(),
                token_program_2022: todo!(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for CollectFundFee {
        type IxData = raydium_cp_swap::instruction::CollectFundFee;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = CollectFundFeeSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = raydium_cp_swap::instruction::CollectFundFee {
                amount_0_requested: todo!(),
                amount_1_requested: todo!(),
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = raydium_cp_swap::accounts::CollectFundFee {
                owner: todo!(),
                authority: todo!(),
                pool_state: todo!(),
                amm_config: todo!(),
                token_0_vault: todo!(),
                token_1_vault: todo!(),
                vault_0_mint: todo!(),
                vault_1_mint: todo!(),
                recipient_token_0_account: todo!(),
                recipient_token_1_account: todo!(),
                token_program: todo!(),
                token_program_2022: todo!(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for Initialize {
        type IxData = raydium_cp_swap::instruction::Initialize;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = InitializeSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = raydium_cp_swap::instruction::Initialize {
                init_amount_0: todo!(),
                init_amount_1: todo!(),
                open_time: todo!(),
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = raydium_cp_swap::accounts::Initialize {
                creator: todo!(),
                amm_config: todo!(),
                authority: todo!(),
                pool_state: todo!(),
                token_0_mint: todo!(),
                token_1_mint: todo!(),
                lp_mint: todo!(),
                creator_token_0: todo!(),
                creator_token_1: todo!(),
                creator_lp_token: todo!(),
                token_0_vault: todo!(),
                token_1_vault: todo!(),
                create_pool_fee: todo!(),
                observation_state: todo!(),
                token_program: todo!(),
                token_0_program: todo!(),
                token_1_program: todo!(),
                associated_token_program: todo!(),
                system_program: todo!(),
                rent: todo!(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for Deposit {
        type IxData = raydium_cp_swap::instruction::Deposit;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = DepositSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = raydium_cp_swap::instruction::Deposit {
                lp_token_amount: todo!(),
                maximum_token_0_amount: todo!(),
                maximum_token_1_amount: todo!(),
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = raydium_cp_swap::accounts::Deposit {
                owner: todo!(),
                authority: todo!(),
                pool_state: todo!(),
                owner_lp_token: todo!(),
                token_0_account: todo!(),
                token_1_account: todo!(),
                token_0_vault: todo!(),
                token_1_vault: todo!(),
                token_program: todo!(),
                token_program_2022: todo!(),
                vault_0_mint: todo!(),
                vault_1_mint: todo!(),
                lp_mint: todo!(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for Withdraw {
        type IxData = raydium_cp_swap::instruction::Withdraw;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = WithdrawSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = raydium_cp_swap::instruction::Withdraw {
                lp_token_amount: todo!(),
                minimum_token_0_amount: todo!(),
                minimum_token_1_amount: todo!(),
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = raydium_cp_swap::accounts::Withdraw {
                owner: todo!(),
                authority: todo!(),
                pool_state: todo!(),
                owner_lp_token: todo!(),
                token_0_account: todo!(),
                token_1_account: todo!(),
                token_0_vault: todo!(),
                token_1_vault: todo!(),
                token_program: todo!(),
                token_program_2022: todo!(),
                vault_0_mint: todo!(),
                vault_1_mint: todo!(),
                lp_mint: todo!(),
                memo_program: todo!(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for SwapBaseInput {
        type IxData = raydium_cp_swap::instruction::SwapBaseInput;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = SwapBaseInputSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = raydium_cp_swap::instruction::SwapBaseInput {
                amount_in: todo!(),
                minimum_amount_out: todo!(),
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = raydium_cp_swap::accounts::Swap {
                payer: todo!(),
                authority: todo!(),
                amm_config: todo!(),
                pool_state: todo!(),
                input_token_account: todo!(),
                output_token_account: todo!(),
                input_vault: todo!(),
                output_vault: todo!(),
                input_token_program: todo!(),
                output_token_program: todo!(),
                input_token_mint: todo!(),
                output_token_mint: todo!(),
                observation_state: todo!(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for SwapBaseOutput {
        type IxData = raydium_cp_swap::instruction::SwapBaseOutput;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = SwapBaseOutputSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = raydium_cp_swap::instruction::SwapBaseOutput {
                max_amount_in: todo!(),
                amount_out: todo!(),
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = raydium_cp_swap::accounts::Swap {
                payer: todo!(),
                authority: todo!(),
                amm_config: todo!(),
                pool_state: todo!(),
                input_token_account: todo!(),
                output_token_account: todo!(),
                input_vault: todo!(),
                output_vault: todo!(),
                input_token_program: todo!(),
                output_token_program: todo!(),
                input_token_mint: todo!(),
                output_token_mint: todo!(),
                observation_state: todo!(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    #[doc = r" Use AccountsStorage<T> where T can be one of:"]
    #[doc = r" Keypair, PdaStore, TokenStore, MintStore, ProgramStore"]
    #[derive(Default)]
    pub struct FuzzAccounts {
        amm_config: AccountsStorage<todo!()>,
        associated_token_program: AccountsStorage<todo!()>,
        authority: AccountsStorage<todo!()>,
        create_pool_fee: AccountsStorage<todo!()>,
        creator: AccountsStorage<todo!()>,
        creator_lp_token: AccountsStorage<todo!()>,
        creator_token_0: AccountsStorage<todo!()>,
        creator_token_1: AccountsStorage<todo!()>,
        input_token_account: AccountsStorage<todo!()>,
        input_token_mint: AccountsStorage<todo!()>,
        input_token_program: AccountsStorage<todo!()>,
        input_vault: AccountsStorage<todo!()>,
        lp_mint: AccountsStorage<todo!()>,
        memo_program: AccountsStorage<todo!()>,
        observation_state: AccountsStorage<todo!()>,
        output_token_account: AccountsStorage<todo!()>,
        output_token_mint: AccountsStorage<todo!()>,
        output_token_program: AccountsStorage<todo!()>,
        output_vault: AccountsStorage<todo!()>,
        owner: AccountsStorage<todo!()>,
        owner_lp_token: AccountsStorage<todo!()>,
        payer: AccountsStorage<todo!()>,
        pool_state: AccountsStorage<todo!()>,
        recipient_token_0_account: AccountsStorage<todo!()>,
        recipient_token_1_account: AccountsStorage<todo!()>,
        rent: AccountsStorage<todo!()>,
        system_program: AccountsStorage<todo!()>,
        token_0_account: AccountsStorage<todo!()>,
        token_0_mint: AccountsStorage<todo!()>,
        token_0_program: AccountsStorage<todo!()>,
        token_0_vault: AccountsStorage<todo!()>,
        token_1_account: AccountsStorage<todo!()>,
        token_1_mint: AccountsStorage<todo!()>,
        token_1_program: AccountsStorage<todo!()>,
        token_1_vault: AccountsStorage<todo!()>,
        token_program: AccountsStorage<todo!()>,
        token_program_2022: AccountsStorage<todo!()>,
        vault_0_mint: AccountsStorage<todo!()>,
        vault_1_mint: AccountsStorage<todo!()>,
    }
}
