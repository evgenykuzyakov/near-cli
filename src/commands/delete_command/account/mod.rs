use dialoguer::Input;

pub mod operation_mode;
mod sender;

/// удаление аккаунта
#[derive(Debug, Default, clap::Clap)]
pub struct CliDeleteAccountAction {
    beneficiary_id: Option<near_primitives::types::AccountId>,
    #[clap(subcommand)]
    sign_option: Option<
        crate::commands::construct_transaction_command::sign_transaction::CliSignTransaction,
    >,
}

#[derive(Debug)]
pub struct DeleteAccountAction {
    pub beneficiary_id: near_primitives::types::AccountId,
    pub sign_option:
        crate::commands::construct_transaction_command::sign_transaction::SignTransaction,
}

impl From<CliDeleteAccountAction> for DeleteAccountAction {
    fn from(item: CliDeleteAccountAction) -> Self {
        let beneficiary_id: near_primitives::types::AccountId = match item.beneficiary_id {
            Some(cli_account_id) => cli_account_id,
            None => DeleteAccountAction::input_beneficiary_id(),
        };
        let sign_option = match item.sign_option {
            Some(cli_sign_transaction) => cli_sign_transaction.into(),
            None => crate::commands::construct_transaction_command::sign_transaction::SignTransaction::choose_sign_option(),
        };
        Self {
            beneficiary_id,
            sign_option,
        }
    }
}

impl DeleteAccountAction {
    pub fn input_beneficiary_id() -> near_primitives::types::AccountId {
        println!();
        Input::new()
            .with_prompt("Enter the beneficiary ID to delete this account ID")
            .interact_text()
            .unwrap()
    }

    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
        network_connection_config: Option<crate::common::ConnectionConfig>,
    ) -> crate::CliResult {
        let beneficiary_id: near_primitives::types::AccountId = self.beneficiary_id.clone();
        let action = near_primitives::transaction::Action::DeleteAccount(
            near_primitives::transaction::DeleteAccountAction { beneficiary_id },
        );
        let mut actions = prepopulated_unsigned_transaction.actions.clone();
        actions.push(action);
        let unsigned_transaction = near_primitives::transaction::Transaction {
            actions,
            ..prepopulated_unsigned_transaction
        };
        match self
            .sign_option
            .process(unsigned_transaction, network_connection_config.clone())
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
                        match transaction_info.transaction.actions[0] {
                            near_primitives::views::ActionView::DeleteAccount {
                                beneficiary_id: _,
                            } => {
                                println!(
                                    "\nAccount <{}> has been successfully deletted.",
                                    transaction_info.transaction.signer_id,
                                );
                            }
                            _ => unreachable!("Error"),
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
