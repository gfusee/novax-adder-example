use novax::adder::adder::AdderContract;
use novax::code::DeployData;
use novax::executor::NetworkExecutor;
use novax::{CodeMetadata, Wallet};
use num_bigint::BigUint;

const GATEWAY_URL: &str = "https://devnet-gateway.multiversx.com";

#[tokio::main]
async fn main() {
    let deploy_data = DeployData {
        code: "adder.wasm",
        metadata: CodeMetadata::UPGRADEABLE | CodeMetadata::READABLE
    };

    // We suppose a .pem wallet file, called "wallet.pem", is present in the root folder (where the Cargo.toml is located)
    // and the underlying wallet has enough EGLD to perform deployment and call
    let mut tx_executor = NetworkExecutor::new(
        GATEWAY_URL,
        &Wallet::from_pem_file("wallet.pem").unwrap()
    );

    println!("➡️ Deploying the adder contract...");

    // Deploy the adder contract
    let (new_address, _) = AdderContract::deploy(
        deploy_data,
        &mut tx_executor,
        600_000_000,
        &BigUint::from(10u8)
    )
        .await
        .expect("Deployment failed");

    println!("✅ Adder contract deployed at {}", new_address.to_bech32_string().unwrap());
    println!(r#"➡️ Calling the "add" endpoint on the previously deployed contract..."#);

    // Increase the adder contract's sum
    AdderContract::new(
        new_address.clone()
    )
        .call(tx_executor, 50_000_000)
        .add(&BigUint::from(15u8))
        .await
        .expect("Endpoint call failed");

    println!(r#"✅ Successfully called the "add" endpoint!"#);
    println!(r#"➡️ Querying the "getSum" view on the previously deployed contract..."#);

    // Query the stored sum value
    let current_sum = AdderContract::new(
        new_address.clone()
    )
        .query(GATEWAY_URL)
        .get_sum()
        .await
        .expect("Query failed");

    println!("✅ Done! Deployed contract address: {}, current sum: {}", new_address.to_bech32_string().unwrap(), current_sum)

}
