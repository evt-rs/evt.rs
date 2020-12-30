use serde::{Deserialize, Serialize};

// Deposit command message
// Send to the account service to effect a deposit
#[derive(Serialize, Deserialize)]
struct Deposit {
    account_id: String,
    amount: usize,
    time: String,
}

// Deposited event message
// Event is written by the handler when a deposit is successfully processed
#[derive(Serialize, Deserialize)]
struct Deposited {
    account_id: String,
    amount: usize,
    time: String,
    processed_time: String,
}

// Withdraw command message
// Send to the account service to effect a withdrawal
#[derive(Serialize, Deserialize)]
struct Withdraw {
    account_id: String,
    amount: usize,
    time: String,
}

// Withdrawn event message
// Event is written by the handler when a withdrawal is successfully processed
#[derive(Serialize, Deserialize)]
struct Withdrawn {
    account_id: String,
    amount: usize,
    time: String,
    processed_time: String,
}

// WithdrawalRejected event message
// Event is written by the handler when a withdrawal cannot be successfully
// processed, as when there are insufficient funds
#[derive(Serialize, Deserialize)]
struct WithdrawalRejected {
    account_id: String,
    amount: usize,
    time: String,
    processed_time: String,
}

// Account entity
// The account service's model object
struct Account {
    id: String,
    balance: usize,
}

impl Account {
    fn deposit(&mut self, amount: usize) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: usize) {
        self.balance -= amount;
    }

    fn has_sufficient_funds(&self, amount: usize) -> bool {
        self.balance >= amount
    }
}

// // Account entity projection
// // Applies account events to an account entity
// impl Projection<Deposited> for Account {
//     fn apply(&mut self, deposited: Message<Deposited>) {
//         let amount = deposited.amount;
//         self.deposit(amount);
//         self.id = deposited.account_id;
//     }
// }
//
// impl Projection<Withdrawn> for Account {
//     fn apply(&mut self, withdrawn: Message<Withdrawn>) {
//         let amount = withdrawn.amount;
//         self.withdraw(amount);
//         self.id = withdrawn.account_id;
//     }
// }
//
// // Account command handler with withdrawal implementation
// // Business logic for processing a withdrawal
// #[derive(evt::Handler)]
// struct Handler {
//     store: &MessageStore,
//     category: String,
// }
//
// impl Clock for Handler {}
//
// impl EntityStore<Account> for Handler {
//     fn get_store(&self) -> &MessageStore {
//         this.store
//     }
// }
//
//
// impl WriteMessage for Handler {
//     fn get_store(&self) -> &MessageStore {
//         this.store
//     }
// }
//
// impl Handler {
//     #[handler(Deposit)]
//     fn handle_deposit(&self, deposit: Message<Deposit>) {
//         let account_id = deposit.account_id;
//
//         let time = self.iso8601();
//
//         let mut deposited = Message<Deposited>.follow(&deposit);
//         deposited.processed_time = time;
//
//         let stream_name = stream_name!(&self.category, id = &account_id);
//
//         self.write(deposited, stream_name);
//     }
//
//     #[handler(Withdraw)]
//     fn handle_withdraw(&self, withdraw: Message<Withdraw>) {
//         let account_id = withdraw.account_id;
//
//         let account = self.fetch<Account>(&account_id);
//
//         let time = self.iso8601();
//
//         let stream_name = stream_name!(&self.category, id = &account_id);
//
//         if !account.has_sufficient_funds(withdraw.amount) {
//             let mut withdrawal_rejected = Message<WithdrawalRejected>.follow(&withdraw);
//             withdrawal_rejected.time = time;
//
//             self.write(withdrawal_rejected, stream_name);
//             return;
//         }
//
//         let mut withdrawn = Message<Withdrawn>.follow(&withdraw);
//         withdrawn.time = time;
//
//         self.write(withdrawn, stream_name);
//     }
// }
//
//
// fn start(settings: Settings) {
//     let store = MessageStore.build(settings);
//
//     let handler = Handler { category: "account", store };
//
//     let consumer = Consumer { handler };
//
//     // this has some ???
//     consumer.start(stream_name!("account", type = "command"));
// }
//
fn main() {
    // component_host.start("account-service", |h| {
    //   h.register(start)
    // })
}
