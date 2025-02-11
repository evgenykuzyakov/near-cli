use dialoguer::{theme::ColorfulTheme, Select};
use strum::{EnumDiscriminants, EnumIter, EnumMessage, IntoEnumIterator};

mod add_access_key_mode;
mod call_function_type;
mod create_account_type;
mod delete_access_key_type;
mod delete_account_type;
mod stake_near_tokens_type;
mod transfer_near_tokens_type;

#[derive(Debug, clap::Clap)]
pub enum CliNextAction {
    /// Choose next action
    AddAction(CliSelectAction),
    /// Go to transaction signing
    Skip(CliSkipAction),
}

#[derive(Debug, clap::Clap)]
pub enum CliSkipNextAction {
    /// Go to transaction signing
    Skip(CliSkipAction),
}

#[derive(Debug, EnumDiscriminants)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
pub enum NextAction {
    #[strum_discriminants(strum(message = "Select a new action"))]
    AddAction(SelectAction),
    #[strum_discriminants(strum(message = "Skip adding a new action"))]
    Skip(SkipAction),
}

impl From<CliNextAction> for NextAction {
    fn from(item: CliNextAction) -> Self {
        match item {
            CliNextAction::AddAction(cli_select_action) => {
                let select_action: SelectAction = SelectAction::from(cli_select_action);
                Self::AddAction(select_action)
            }
            CliNextAction::Skip(cli_skip_action) => {
                let skip_action: SkipAction = SkipAction::from(cli_skip_action);
                Self::Skip(skip_action)
            }
        }
    }
}

impl From<CliSkipNextAction> for NextAction {
    fn from(item: CliSkipNextAction) -> Self {
        match item {
            CliSkipNextAction::Skip(cli_skip_action) => {
                let skip_action: SkipAction = SkipAction::from(cli_skip_action);
                Self::Skip(skip_action)
            }
        }
    }
}

impl From<CliSkipNextAction> for CliNextAction {
    fn from(item: CliSkipNextAction) -> Self {
        match item {
            CliSkipNextAction::Skip(cli_skip_action) => Self::Skip(cli_skip_action),
        }
    }
}

impl NextAction {
    pub fn input_next_action() -> Self {
        println!();
        let variants = NextActionDiscriminants::iter().collect::<Vec<_>>();
        let next_action = variants
            .iter()
            .map(|p| p.get_message().unwrap().to_owned())
            .collect::<Vec<_>>();
        let select_next_action = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select an action that you want to add to the action:")
            .items(&next_action)
            .default(0)
            .interact()
            .unwrap();
        let cli_next_action = match variants[select_next_action] {
            NextActionDiscriminants::AddAction => CliNextAction::AddAction(Default::default()),
            NextActionDiscriminants::Skip => CliNextAction::Skip(Default::default()),
        };
        Self::from(cli_next_action)
    }

    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
        network_connection_config: Option<crate::common::ConnectionConfig>,
    ) -> crate::CliResult {
        match self {
            NextAction::AddAction(select_action) => {
                select_action
                    .process(prepopulated_unsigned_transaction, network_connection_config)
                    .await
            }
            NextAction::Skip(skip_action) => {
                skip_action
                    .process(prepopulated_unsigned_transaction, network_connection_config)
                    .await
            }
        }
    }
}

/// инструмент для добавления команды в транзакцию
#[derive(Debug, Default, clap::Clap)]
pub struct CliSelectAction {
    #[clap(subcommand)]
    transaction_subcommand: Option<CliActionSubcommand>,
}

#[derive(Debug)]
pub struct SelectAction {
    transaction_subcommand: ActionSubcommand,
}

impl From<CliSelectAction> for SelectAction {
    fn from(item: CliSelectAction) -> Self {
        let transaction_subcommand: ActionSubcommand = match item.transaction_subcommand {
            Some(cli_transaction_subcommand) => ActionSubcommand::from(cli_transaction_subcommand),
            None => ActionSubcommand::choose_action_command(),
        };
        Self {
            transaction_subcommand,
        }
    }
}

impl SelectAction {
    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
        network_connection_config: Option<crate::common::ConnectionConfig>,
    ) -> crate::CliResult {
        self.transaction_subcommand
            .process(prepopulated_unsigned_transaction, network_connection_config)
            .await
    }
}

#[derive(Debug, clap::Clap)]
pub enum CliActionSubcommand {
    /// Предоставьте данные для перевода Near
    TransferNEARTokens(self::transfer_near_tokens_type::CliTransferNEARTokensAction),
    /// Предоставьте данные для call function
    CallFunction(self::call_function_type::CliCallFunctionAction),
    /// Предоставьте данные для ставки
    StakeNEARTokens(self::stake_near_tokens_type::CliStakeNEARTokensAction),
    /// Предоставьте данные для создания аккаунта
    CreateAccount(self::create_account_type::CliCreateAccountAction),
    /// Предоставьте данные для удаления аккаунта
    DeleteAccount(self::delete_account_type::CliDeleteAccountAction),
    /// Предоставьте данные для добавления ключа доступа пользователю
    AddAccessKey(self::add_access_key_mode::CliAddAccessKeyMode),
    /// Предоставьте данные для удаления ключа доступа у пользователя
    DeleteAccessKey(self::delete_access_key_type::CliDeleteAccessKeyAction),
}

#[derive(Debug, EnumDiscriminants)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
pub enum ActionSubcommand {
    #[strum_discriminants(strum(message = "Transfer NEAR Tokens"))]
    TransferNEARTokens(self::transfer_near_tokens_type::TransferNEARTokensAction),
    #[strum_discriminants(strum(message = "Call a Function"))]
    CallFunction(self::call_function_type::CallFunctionAction),
    #[strum_discriminants(strum(message = "Stake NEAR Tokens"))]
    StakeNEARTokens(self::stake_near_tokens_type::StakeNEARTokensAction),
    #[strum_discriminants(strum(message = "Create an Account"))]
    CreateAccount(self::create_account_type::CreateAccountAction),
    #[strum_discriminants(strum(message = "Delete an Account"))]
    DeleteAccount(self::delete_account_type::DeleteAccountAction),
    #[strum_discriminants(strum(message = "Add an Access Key"))]
    AddAccessKey(self::add_access_key_mode::AddAccessKeyMode),
    #[strum_discriminants(strum(message = "Detete an Access Key"))]
    DeleteAccessKey(self::delete_access_key_type::DeleteAccessKeyAction),
}

impl From<CliActionSubcommand> for ActionSubcommand {
    fn from(item: CliActionSubcommand) -> Self {
        match item {
            CliActionSubcommand::TransferNEARTokens(cli_transfer_near_token) => {
                Self::TransferNEARTokens(cli_transfer_near_token.into())
            }
            CliActionSubcommand::CreateAccount(cli_create_account) => {
                Self::CreateAccount(cli_create_account.into())
            }
            CliActionSubcommand::DeleteAccount(cli_delete_account) => {
                Self::DeleteAccount(cli_delete_account.into())
            }
            CliActionSubcommand::AddAccessKey(cli_add_access_key) => {
                Self::AddAccessKey(cli_add_access_key.into())
            }
            CliActionSubcommand::DeleteAccessKey(cli_delete_access_key) => {
                Self::DeleteAccessKey(cli_delete_access_key.into())
            }
            CliActionSubcommand::StakeNEARTokens(cli_stake_near_token) => {
                Self::StakeNEARTokens(cli_stake_near_token.into())
            }
            CliActionSubcommand::CallFunction(cli_call_function) => {
                Self::CallFunction(cli_call_function.into())
            }
        }
    }
}

impl ActionSubcommand {
    pub fn choose_action_command() -> ActionSubcommand {
        println!();
        let variants = ActionSubcommandDiscriminants::iter().collect::<Vec<_>>();
        let action_subcommands = variants
            .iter()
            .map(|p| p.get_message().unwrap().to_owned())
            .collect::<Vec<_>>();
        let select_action_subcommand = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select an action that you want to add to the action:")
            .items(&action_subcommands)
            .default(0)
            .interact()
            .unwrap();
        let cli_action_subcomand = match variants[select_action_subcommand] {
            ActionSubcommandDiscriminants::TransferNEARTokens => {
                CliActionSubcommand::TransferNEARTokens(Default::default())
            }
            ActionSubcommandDiscriminants::CallFunction => {
                CliActionSubcommand::CallFunction(Default::default())
            }
            ActionSubcommandDiscriminants::StakeNEARTokens => {
                CliActionSubcommand::StakeNEARTokens(Default::default())
            }
            ActionSubcommandDiscriminants::CreateAccount => {
                CliActionSubcommand::CreateAccount(Default::default())
            }
            ActionSubcommandDiscriminants::DeleteAccount => {
                CliActionSubcommand::DeleteAccount(Default::default())
            }
            ActionSubcommandDiscriminants::AddAccessKey => {
                CliActionSubcommand::AddAccessKey(Default::default())
            }
            ActionSubcommandDiscriminants::DeleteAccessKey => {
                CliActionSubcommand::DeleteAccessKey(Default::default())
            }
        };
        Self::from(cli_action_subcomand)
    }

    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
        network_connection_config: Option<crate::common::ConnectionConfig>,
    ) -> crate::CliResult {
        match self {
            ActionSubcommand::TransferNEARTokens(args_transfer) => {
                args_transfer
                    .process(prepopulated_unsigned_transaction, network_connection_config)
                    .await
            }
            ActionSubcommand::CallFunction(args_function) => {
                args_function
                    .process(prepopulated_unsigned_transaction, network_connection_config)
                    .await
            }
            ActionSubcommand::StakeNEARTokens(args_stake) => {
                args_stake
                    .process(prepopulated_unsigned_transaction, network_connection_config)
                    .await
            }
            ActionSubcommand::CreateAccount(args_create_account) => {
                args_create_account
                    .process(prepopulated_unsigned_transaction, network_connection_config)
                    .await
            }
            ActionSubcommand::DeleteAccount(args_delete_account) => {
                args_delete_account
                    .process(prepopulated_unsigned_transaction, network_connection_config)
                    .await
            }
            ActionSubcommand::AddAccessKey(args_add_access_key) => {
                args_add_access_key
                    .process(prepopulated_unsigned_transaction, network_connection_config)
                    .await
            }
            ActionSubcommand::DeleteAccessKey(args_delete_access_key) => {
                args_delete_access_key
                    .process(prepopulated_unsigned_transaction, network_connection_config)
                    .await
            }
        }
    }
}

/// инструмент, показывающий окончание набора команд в одной транзакции
#[derive(Debug, Default, clap::Clap)]
pub struct CliSkipAction {
    #[clap(subcommand)]
    sign_option: Option<super::sign_transaction::CliSignTransaction>,
}

#[derive(Debug)]
pub struct SkipAction {
    pub sign_option: super::sign_transaction::SignTransaction,
}

impl From<CliSkipAction> for SkipAction {
    fn from(item: CliSkipAction) -> Self {
        let sign_option: super::sign_transaction::SignTransaction = match item.sign_option {
            Some(cli_sign_transaction) => cli_sign_transaction.into(),
            None => super::sign_transaction::SignTransaction::choose_sign_option(),
        };
        Self { sign_option }
    }
}

impl SkipAction {
    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
        network_connection_config: Option<crate::common::ConnectionConfig>,
    ) -> crate::CliResult {
        match self
            .sign_option
            .process(
                prepopulated_unsigned_transaction,
                network_connection_config.clone(),
            )
            .await?
        {
            Some(transaction_info) => {
                match transaction_info.status {
                    near_primitives::views::FinalExecutionStatus::NotStarted
                    | near_primitives::views::FinalExecutionStatus::Started => unreachable!(),
                    near_primitives::views::FinalExecutionStatus::Failure(tx_execution_error) => {
                        crate::common::print_transaction_error(tx_execution_error).await
                    }
                    near_primitives::views::FinalExecutionStatus::SuccessValue(_) => {
                        for action in transaction_info.transaction.actions {
                            match action {
                                near_primitives::views::ActionView::CreateAccount => {
                                    println!(
                                        "\nNew account <{}> has been successfully created.",
                                        transaction_info.transaction.receiver_id,
                                    );
                                }
                                near_primitives::views::ActionView::DeployContract { code: _ } => {
                                    println!("\n Contract code has been successfully deployed.",);
                                }
                                near_primitives::views::ActionView::FunctionCall {
                                    method_name,
                                    args: _,
                                    gas: _,
                                    deposit: _,
                                } => {
                                    println!(
                                        "\nThe \"{}\" call to <{}> on behalf of <{}> succeeded.",
                                        method_name,
                                        transaction_info.transaction.receiver_id,
                                        transaction_info.transaction.signer_id,
                                    );
                                }
                                near_primitives::views::ActionView::Transfer { deposit } => {
                                    println!(
                                        "\n<{}> has transferred {} to <{}> successfully.",
                                        transaction_info.transaction.signer_id,
                                        crate::common::NearBalance::from_yoctonear(deposit),
                                        transaction_info.transaction.receiver_id,
                                    );
                                }
                                near_primitives::views::ActionView::Stake {
                                    stake,
                                    public_key: _,
                                } => {
                                    println!(
                                        "\nValidator <{}> has successfully staked {}.",
                                        transaction_info.transaction.signer_id,
                                        crate::common::NearBalance::from_yoctonear(stake),
                                    );
                                }
                                near_primitives::views::ActionView::AddKey {
                                    public_key,
                                    access_key: _,
                                } => {
                                    println!(
                                        "Added access key = {} to {}.",
                                        public_key, transaction_info.transaction.receiver_id,
                                    );
                                }
                                near_primitives::views::ActionView::DeleteKey { public_key } => {
                                    println!(
                                        "\nAccess key <{}> for account <{}> has been successfully deletted.",
                                        public_key,
                                        transaction_info.transaction.signer_id,
                                    );
                                }
                                near_primitives::views::ActionView::DeleteAccount {
                                    beneficiary_id: _,
                                } => {
                                    println!(
                                        "\nAccount <{}> has been successfully deletted.",
                                        transaction_info.transaction.signer_id,
                                    );
                                }
                            }
                        }
                    }
                }
                let transaction_explorer: url::Url = match network_connection_config {
                    Some(connection_config) => connection_config.transaction_explorer(),
                    None => unreachable!("Error"),
                };
                println!("\nTransaction Id {id}.\n\nTo see the transaction in the transaction explorer, please open this url in your browser:
                    \n{path}{id}\n", id=transaction_info.transaction_outcome.id, path=transaction_explorer);
            }
            None => {}
        };
        Ok(())
    }
}
