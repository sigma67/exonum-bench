// Copyright 2019 Benedikt Putz
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate exonum_derive;

use exonum::{
    crypto::{gen_keypair, hash, PublicKey, SecretKey},
    messages::{to_hex_string},
};
use std::env;
use rand::Rng;
use std::time::{SystemTime};
use std::thread;
use std::time;
use std::slice::Chunks;

mod proto;
mod schema;
mod transactions;
mod api;

use transactions::{ TxTimestamp };
use schema::Timestamp;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Insufficient arguments. Please specify (1) number of tx per time unit, (2) total duration of test, (3) time unit (i.e. 1 sec).");
        return;
    }
    //benchmark parameters
    let count = args[1].parse::<usize>().unwrap();
    let seconds = args[2].parse::<usize>().unwrap();
    let wait = args[3].parse::<u64>().unwrap();

    //create transactions
    let mut txs : Vec<String> = vec![String::new(); count*seconds];
    let mut rng = rand::thread_rng();
    let keypair = gen_keypair();

    for i in 1..(count*seconds) {
        let x: u64 = rng.gen();
        if i == 1 {println!("{}", &x.to_string())}
        let tx = self::create_transaction(&keypair.0, &keypair.1, x.to_string());
        //api::post(&tx);
        txs[i] = tx;
    }

    //create chunks
    let posts = txs.chunks(count);
    let mut items : Vec<Vec<String>> = Vec::with_capacity(seconds);
    for p in posts {
        items.push(p.to_vec());
    }

    //post transactions
    let start = SystemTime::now();
    for p in items {
        let pre = SystemTime::now();
        api::post_async(p);
        let passed = SystemTime::now().duration_since(pre).unwrap();
        println!("{} for posting", passed.as_millis());
        if passed.as_millis() < u128::from(wait) {
            thread::sleep(time::Duration::from_millis(wait) - passed);
        }
    }

    let end= SystemTime::now();
    println!("Sent {} transactions in {}ms",
             (count*seconds).to_string(),
             end.duration_since(start).expect("").as_millis().to_string());
}

fn create_transaction(public: &PublicKey, secret: &SecretKey, payload: String) -> String {
    let content = Timestamp::new(&hash(payload.as_bytes()), "metadata");
    let tx = TxTimestamp::sign(&public, content, &secret);
    let data = to_hex_string(&tx);
    data
}
