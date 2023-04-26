use web3::types::{Address, H160};
use web3::transports::Http;
use web3::{Web3, Transport};

fn main() {
    // 连接以太坊节点
    let transport = Http::new("https://infura.io").unwrap();
    let web3 = Web3::new(transport);

    // 创建账号
    let new_account = web3.personal().new_account("hey_mr_bridge").unwrap();

    // 输出新账号地址
    let address: H160 = new_account.parse().unwrap();
    println!("New account address: {:?}", Address::from(address));
}
