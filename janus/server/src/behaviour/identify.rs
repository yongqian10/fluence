/*
 *   MIT License
 *
 *   Copyright (c) 2020 Fluence Labs Limited
 *
 *   Permission is hereby granted, free of charge, to any person obtaining a copy
 *   of this software and associated documentation files (the "Software"), to deal
 *   in the Software without restriction, including without limitation the rights
 *   to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 *   copies of the Software, and to permit persons to whom the Software is
 *   furnished to do so, subject to the following conditions:
 *
 *   The above copyright notice and this permission notice shall be included in all
 *   copies or substantial portions of the Software.
 *
 *   THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 *   IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 *   FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 *   AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 *   LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 *   OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 *   SOFTWARE.
 */

use super::ServerBehaviour;
use itertools::Itertools;
use libp2p::identify::IdentifyEvent;
use libp2p::identity::PublicKey;
use libp2p::swarm::NetworkBehaviourEventProcess;
use parity_multiaddr::{Multiaddr, Protocol};
use std::net::IpAddr;

/// Network address information is exchanged via Identify protocol.
/// That information is passed to relay, so nodes know each other's addresses
impl NetworkBehaviourEventProcess<IdentifyEvent> for ServerBehaviour {
    fn inject_event(&mut self, event: IdentifyEvent) {
        match event {
            IdentifyEvent::Received { peer_id, info, .. } => {
                log::debug!(
                    "Identify received from {}: protocols: {:?} version: {} listen addrs {:?}",
                    peer_id.to_base58(),
                    info.protocols,
                    info.protocol_version,
                    info.listen_addrs
                );
                let supports_kademlia =
                    info.protocols.iter().any(|p| p.contains("/ipfs/kad/1.0.0"));
                match info.public_key {
                    PublicKey::Ed25519(public_key) if supports_kademlia => {
                        let addresses = filter_addresses(info.listen_addrs);
                        self.router.add_kad_node(peer_id, addresses, public_key);
                    }
                    _ if supports_kademlia => {
                        log::error!(
                            "Unable to add node {} to kademlia, public key {:?} is not supported. \
                            Only ed25519 is supported. Will fallback to Direct routing.",
                            peer_id.to_base58(),
                            info.public_key
                        );
                    }
                    _ => {}
                }
            }

            // TODO: handle error?
            IdentifyEvent::Error { error, peer_id } => {
                log::error!("Identify error on {}: {}", peer_id.to_base58(), error);
            }

            // We don't care about Sent identification info
            IdentifyEvent::Sent { .. } => {}
        }
    }
}

fn filter_addresses(addresses: Vec<Multiaddr>) -> Vec<Multiaddr> {
    // Deduplicate addresses
    let addresses: Vec<_> = addresses.into_iter().unique().collect();

    // Check if there's at least single global IP address
    let exists_global = addresses.iter().any(is_global_maddr);

    if !exists_global {
        log::warn!("No globally-reachable IP addresses found. Are we running on localhost?");
        // If there are no global addresses, we are most likely running locally
        // So take loopback address, and go with it.
        addresses.into_iter().filter(is_local_maddr).collect()
    } else {
        // Keep only global addresses
        addresses.into_iter().filter(is_global_maddr).collect()
    }
}

fn is_global(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(addr) => {
            !addr.is_private()
                && !addr.is_loopback()
                && !addr.is_link_local()
                && !addr.is_broadcast()
                && !addr.is_documentation()
                && !addr.is_unspecified()
        }
        IpAddr::V6(addr) => !addr.is_loopback() && !addr.is_unspecified(),
    }
}

fn is_global_maddr(maddr: &Multiaddr) -> bool {
    maddr.iter().any(|p| match p {
        Protocol::Ip4(addr) => is_global(addr.into()),
        _ => false,
    })
}

fn is_local_maddr(maddr: &Multiaddr) -> bool {
    maddr.iter().any(|p| match p {
        Protocol::Ip4(addr) if addr.is_loopback() => true,
        _ => false,
    })
}
