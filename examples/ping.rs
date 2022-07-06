// Copyright 2018 Parity Technologies (UK) Ltd.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

//! Ping example
//!
//! See ../src/tutorial.rs for a step-by-step guide building the example below.
//!
//! In the first terminal window, run:
//!
//! ```sh
//! cargo run --example ping
//! ```
//!
//! It will print the PeerId and the listening addresses, e.g. `Listening on
//! "/ip4/0.0.0.0/tcp/24915"`
//!
//! In the second terminal window, start a new instance of the example with:
//!
//! ```sh
//! cargo run --example ping -- /ip4/127.0.0.1/tcp/24915
//! ```
//!
//! The two nodes establish a connection, negotiate the ping protocol
//! and begin pinging each other.

use futures::prelude::*;
use libp2p::swarm::{Swarm, SwarmEvent};
use libp2p::{identity, ping, Multiaddr, PeerId};
use std::error::Error;
use std::env;
use std::process;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("Local peer id: {:?}", local_peer_id);

    let transport = libp2p::development_transport(local_key).await?;

    // Create a ping network behaviour.
    //
    // For illustrative purposes, the ping protocol is configured to
    // keep the connection alive, so a continuous sequence of pings
    // can be observed.
    let behaviour = ping::Behaviour::new(ping::Config::new().with_keep_alive(true));

    let mut swarm = Swarm::new(transport, behaviour, local_peer_id);
    
    let port: u16; // default port
    let remote: Multiaddr;

    match args.len() {
        // no arguments
        1 => {
            println!("Try passing some arguments!");
            process::exit(1);
        },
        // 1 argument
        2 => {
            let num = &args[1];
            port = match num.parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("error: first argument not an integer");
                    process::exit(2);
                },
            };
            let mut address: String = "/ip4/0.0.0.0/tcp/".to_owned();
            address.push_str(&port.to_string());
            swarm.listen_on(address.parse()?)?;

        },
        // 2 arguments
        3 => {
            let num = &args[1];
            port = match num.parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("error: first argument not an integer");
                    process::exit(3);
                },
            };
            let mut address: String = "/ip4/0.0.0.0/tcp/".to_owned();
            address.push_str(&port.to_string());
            swarm.listen_on(address.parse()?)?;

            let addr = &args[2];
            remote = addr.parse()?;
            swarm.dial(remote)?;
            println!("Dialed {}", addr);
        },
        // more than 2 arguments
        _ => {
            println!("Too many arguments!");
            process::exit(4);
        },
    }

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {:?}", address),
            SwarmEvent::Behaviour(event) => println!("{:?}", event),
            _ => {}
        }
    }
}
