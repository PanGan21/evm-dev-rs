use std::{collections::HashMap, vec};

use evm_dev_rs::evm;
use primitive_types::U256;
use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
struct EvmTest {
    name: String,
    hint: String,
    code: Code,
    expect: Expect,
    tx: Option<TxDataRaw>,
    block: Option<BlockDataRaw>,
    #[serde(default)]
    state: StateRaw,
}

#[derive(Debug, Deserialize)]
struct Code {
    #[serde(
        default = "default_string",
        deserialize_with = "deserialize_string_or_empty"
    )]
    asm: String,
    bin: String,
}

fn default_string() -> String {
    String::new() // Returns an empty string
}

fn deserialize_string_or_empty<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    Ok(opt.unwrap_or_else(default_string))
}

#[derive(Debug, Deserialize)]
struct TxDataRaw {
    to: Option<String>,
    from: Option<String>,
    origin: Option<String>,
    gasprice: Option<String>,
    value: Option<String>,
}

#[derive(Debug, Deserialize)]
struct BlockDataRaw {
    basefee: Option<String>,
    coinbase: Option<String>,
    timestamp: Option<String>,
    number: Option<String>,
    difficulty: Option<String>,
    gaslimit: Option<String>,
    chainid: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
struct StateRaw {
    #[serde(flatten)]
    entries: HashMap<String, AddressDataRaw>,
}

#[derive(Debug, Deserialize)]
struct AddressDataRaw {
    nonce: Option<String>,
    balance: Option<String>,
    #[serde(default)]
    code: Option<Code>,
}

#[derive(Debug, Deserialize)]
struct Expect {
    stack: Option<Vec<String>>,
    success: bool,
}

fn main() {
    let text = std::fs::read_to_string("./evm.json").unwrap();
    let data: Vec<EvmTest> = serde_json::from_str(&text).unwrap();

    let total = data.len();

    for (index, test) in data.iter().enumerate() {
        println!("Test {} of {}: {}", index + 1, total, test.name);

        let code = hex::decode(&test.code.bin).unwrap();

        let tx = match &test.tx {
            Some(tx) => {
                // [2..] is necessary to delete the initial 0x
                let to = hex::decode(format!(
                    "{:0>64}",
                    &tx.to.as_ref().unwrap_or(&String::from("aa"))[2..]
                ))
                .unwrap();

                let from = hex::decode(format!(
                    "{:0>64}",
                    &tx.from.as_ref().unwrap_or(&String::from("aa"))[2..]
                ))
                .unwrap();

                let origin = hex::decode(format!(
                    "{:0>64}",
                    &tx.origin.as_ref().unwrap_or(&String::from("aa"))[2..]
                ))
                .unwrap();

                let gasprice = hex::decode(format!(
                    "{:0>64}",
                    &tx.gasprice.as_ref().unwrap_or(&String::from("aa"))[2..]
                ))
                .unwrap();

                let value = hex::decode(format!(
                    "{:0>64}",
                    &tx.value.as_ref().unwrap_or(&String::from("aa"))[2..]
                ))
                .unwrap();

                vec![to, from, origin, gasprice, value]
            }
            None => vec![],
        };

        let block = match &test.block {
            Some(block) => {
                // [2..] is necessary to delete the initial 0x
                let basefee = hex::decode(format!(
                    "{:0>64}",
                    &block.basefee.as_ref().unwrap_or(&String::from("aa"))[2..]
                ))
                .unwrap();

                let coinbase = hex::decode(format!(
                    "{:0>64}",
                    &block.coinbase.as_ref().unwrap_or(&String::from("aa"))[2..]
                ))
                .unwrap();

                let timestamp = hex::decode(format!(
                    "{:0>64}",
                    &block.timestamp.as_ref().unwrap_or(&String::from("aa"))[2..]
                ))
                .unwrap();

                let number = hex::decode(format!(
                    "{:0>64}",
                    &block.number.as_ref().unwrap_or(&String::from("aa"))[2..]
                ))
                .unwrap();

                let difficulty = hex::decode(format!(
                    "{:0>64}",
                    &block.difficulty.as_ref().unwrap_or(&String::from("aa"))[2..]
                ))
                .unwrap();

                let gaslimit = hex::decode(format!(
                    "{:0>64}",
                    &block.gaslimit.as_ref().unwrap_or(&String::from("aa"))[2..]
                ))
                .unwrap();

                let chainid = hex::decode(format!(
                    "{:0>64}",
                    &block.chainid.as_ref().unwrap_or(&String::from("aa"))[2..]
                ))
                .unwrap();

                vec![
                    basefee, coinbase, timestamp, number, difficulty, gaslimit, chainid,
                ]
            }
            None => vec![],
        };

        let state = test
            .state
            .entries
            .is_empty()
            .then(HashMap::default)
            .unwrap_or_else(|| {
                test.state
                    .entries
                    .iter()
                    .map(|(address, data)| {
                        let address_bytes = hex::decode(format!("{:0>64}", &address[2..])).unwrap();

                        let nonce = data
                            .nonce
                            .as_deref()
                            .unwrap_or("0")
                            .parse::<usize>()
                            .unwrap();

                        let balance = hex::decode(format!(
                            "{:0>64}",
                            &data.balance.as_deref().unwrap_or("0xaa")[2..]
                        ))
                        .unwrap();

                        let code = hex::decode(data.code.as_ref().map_or("", |c| &c.bin)).unwrap();

                        (address_bytes, (nonce, balance, code))
                    })
                    .collect::<HashMap<_, _>>()
            });

        let result = evm(code, tx, block, state);

        let mut expected_stack: Vec<U256> = Vec::new();
        if let Some(ref stacks) = test.expect.stack {
            for value in stacks {
                expected_stack.push(U256::from_str_radix(value, 16).unwrap());
            }
        }

        let mut matching = result.stack.len() == expected_stack.len();
        if matching {
            for i in 0..result.stack.len() {
                if result.stack[i] != expected_stack[i] {
                    matching = false;
                    break;
                }
            }
        }

        matching = matching && result.success == test.expect.success;

        if !matching {
            println!("Instructions: \n{}\n", test.code.asm);

            println!("Expected success: {:?}", test.expect.success);
            println!("Expected stack: [");
            for v in expected_stack {
                println!("  {:#X},", v);
            }
            println!("]\n");

            println!("Actual success: {:?}", result.success);
            println!("Actual stack: [");
            for v in result.stack {
                println!("  {:#X},", v);
            }
            println!("]\n");

            println!("\nHint: {}\n", test.hint);
            println!("Progress: {}/{}\n\n", index, total);
            panic!("Test failed");
        }
        println!("PASS");
    }

    println!("EVM implemented successfully!")
}
