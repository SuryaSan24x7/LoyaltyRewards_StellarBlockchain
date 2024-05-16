#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{mock_env, Address, MockStorage};

    #[test]
    fn test_issue_points() {
        // Mock environment
        let mut env = mock_env();
        let issuer = Address::new("issuer_address");
        let recipient = Address::new("recipient_address");
        let token_address = Address::new("token_address");
        let amount = 100;

        // Initialize contract storage with recipient's loyalty points balance
        let mut storage = MockStorage::new();
        storage.set(&recipient, &LoyaltyPoints { balance: 0 });

        // Call issue_points function
        LoyaltyPointsContract::issue_points(env.clone(), issuer.clone(), recipient.clone(), token_address.clone(), amount);

        // Check if loyalty points were issued correctly
        let loyalty_points: LoyaltyPoints = storage.get(&recipient).unwrap();
        assert_eq!(loyalty_points.balance, amount);
    }

    #[test]
    fn test_get_balance() {
        // Mock environment
        let mut env = mock_env();
        let account = Address::new("account_address");
        let balance = 200;

        // Initialize contract storage with account's loyalty points balance
        let mut storage = MockStorage::new();
        storage.set(&account, &LoyaltyPoints { balance });

        // Call get_balance function
        let retrieved_balance = LoyaltyPointsContract::get_balance(env.clone(), account.clone());

        // Check if retrieved balance matches the expected balance
        assert_eq!(retrieved_balance, balance);
    }
}
