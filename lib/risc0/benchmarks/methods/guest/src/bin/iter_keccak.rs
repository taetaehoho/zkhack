// Copyright 2023 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![no_main]

use risc0_zkvm::{guest::env, sha::Digest};
use sha3::{Digest as _, Keccak256};

risc0_zkvm::entry!(main);

pub fn main() {
    let (num_iter, data): (u32, Vec<u8>) = env::read();

    let mut hash = keccak(&data);
    for _ in 1..num_iter {
        hash = keccak(hash);
    }

    let digest = Digest::try_from(hash).unwrap();
    env::commit(&digest)
}

#[inline]
pub fn keccak(data: impl AsRef<[u8]>) -> [u8; 32] {
    Keccak256::digest(data).into()
}
