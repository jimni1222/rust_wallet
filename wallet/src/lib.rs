//
// Copyright 2018 rust-wallet developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
extern crate bitcoin;
extern crate crypto;
extern crate hex;
extern crate rand;
extern crate secp256k1;
// extern crate bitcoin_rpc_client;
extern crate bitcoin_bech32;
extern crate byteorder;
extern crate rocksdb;
#[macro_use]
extern crate serde_derive;
extern crate bip39;
extern crate electrumx_client;
extern crate log;
extern crate serde;
extern crate serde_json;
extern crate simple_logger;

pub mod account;
mod db;
pub mod default;
pub mod electrumx;
pub mod error;
pub mod interface;
pub mod keyfactory;
pub mod mnemonic;
pub mod walletlibrary;
