use dialoguer::Input;

/// данные об отправителе транзакции
#[derive(Debug, Default, clap::Clap)]
pub struct CliSender {
    pub sender_account_id: Option<String>,
    #[clap(subcommand)]
    public_key_mode: Option<super::public_key_mode::CliPublicKeyMode>,
}

#[derive(Debug)]
pub struct Sender {
    pub sender_account_id: String,
    pub public_key_mode: super::public_key_mode::PublicKeyMode,
}

impl From<CliSender> for Sender {
    fn from(item: CliSender) -> Self {
        let sender_account_id: String = match item.sender_account_id {
            Some(cli_sender_account_id) => cli_sender_account_id,
            None => Sender::input_sender_account_id(),
        };
        let public_key_mode = match item.public_key_mode {
            Some(cli_public_key_mode) => {
                super::public_key_mode::PublicKeyMode::from(cli_public_key_mode)
            }
            None => super::public_key_mode::PublicKeyMode::choose_public_key_mode(),
        };
        Self {
            sender_account_id,
            public_key_mode,
        }
    }
}

impl Sender {
    pub fn input_sender_account_id() -> String {
        println!();
        Input::new()
            .with_prompt("What is the account ID of the sender?")
            .interact_text()
            .unwrap()
    }

    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
        network_connection_config: Option<crate::common::ConnectionConfig>,
    ) -> crate::CliResult {
        let unsigned_transaction = near_primitives::transaction::Transaction {
            signer_id: self.sender_account_id.clone(),
            receiver_id: self.sender_account_id.clone(),
            ..prepopulated_unsigned_transaction
        };
        self.public_key_mode
            .process(unsigned_transaction, network_connection_config)
            .await
    }
}
