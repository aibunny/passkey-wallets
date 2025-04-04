use ethers::prelude::*;

abigen!(
    AccountFactory,
    r#"[
        {
          "type": "constructor",
          "inputs": [
            { "name": "_entryPoint", "type": "address", "internalType": "contract IEntryPoint" }
          ],
          "stateMutability": "nonpayable"
        },
        {
          "type": "function",
          "name": "accountImplementation",
          "inputs": [],
          "outputs": [
            { "name": "", "type": "address", "internalType": "contract Account" }
          ],
          "stateMutability": "view"
        },
        {
          "type": "function",
          "name": "createAccount",
          "inputs": [
            { "name": "initialKeySlot", "type": "uint8", "internalType": "uint8" },
            { "name": "initialKey", "type": "bytes32[2]", "internalType": "bytes32[2]" },
            { "name": "salt", "type": "uint256", "internalType": "uint256" }
          ],
          "outputs": [
            { "name": "account", "type": "address", "internalType": "contract Account" }
          ],
          "stateMutability": "payable"
        },
        {
          "type": "function",
          "name": "entryPoint",
          "inputs": [],
          "outputs": [
            { "name": "", "type": "address", "internalType": "contract IEntryPoint" }
          ],
          "stateMutability": "view"
        },
        {
          "type": "function",
          "name": "getAddress",
          "inputs": [
            { "name": "initialKeySlot", "type": "uint8", "internalType": "uint8" },
            { "name": "initialKey", "type": "bytes32[2]", "internalType": "bytes32[2]" },
            { "name": "salt", "type": "uint256", "internalType": "uint256" }
          ],
          "outputs": [
            { "name": "predictedAddress", "type": "address", "internalType": "address" }
          ],
          "stateMutability": "view"
        },
        {
          "type": "event",
          "name": "AccountCreated",
          "inputs": [
            { "name": "account", "type": "address", "indexed": true, "internalType": "address" },
            { "name": "initialKeySlot", "type": "uint8", "indexed": false, "internalType": "uint8" },
            { "name": "initialKey", "type": "bytes32[2]", "indexed": false, "internalType": "bytes32[2]" },
            { "name": "salt", "type": "uint256", "indexed": false, "internalType": "uint256" }
          ],
          "anonymous": false
        },
        {
          "type": "error",
          "name": "Create2EmptyBytecode",
          "inputs": []
        },
        {
          "type": "error",
          "name": "FailedDeployment",
          "inputs": []
        },
        {
          "type": "error",
          "name": "InsufficientBalance",
          "inputs": [
            { "name": "balance", "type": "uint256", "internalType": "uint256" },
            { "name": "needed", "type": "uint256", "internalType": "uint256" }
          ]
        }
    ]"#;
);


pub use AccountFactory;