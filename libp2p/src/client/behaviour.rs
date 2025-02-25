use std::{
    collections::{HashSet, VecDeque},
    sync::atomic::{AtomicUsize, Ordering},
    task::{Context, Poll},
};

use libp2p_core::{Multiaddr, transport::PortUse};
use libp2p_identity::PeerId;
use libp2p_swarm::{
    ConnectionClosed, ConnectionId, FromSwarm, NetworkBehaviour, NotifyHandler, THandlerInEvent,
    THandlerOutEvent, ToSwarm, derive_prelude::ConnectionEstablished,
};

use crate::client::handler::Handler;

static NEXT_RUN_ID: AtomicUsize = AtomicUsize::new(1);

#[derive(Debug)]
pub struct Event {
    #[allow(unused)]
    pub id: usize,
    #[allow(unused)]
    pub result: Result<(), String>,
}

#[derive(Default)]
pub struct Behaviour {
    /// Queue of actions to return when polled.
    queued_events: VecDeque<ToSwarm<Event, THandlerInEvent<Self>>>,
    /// Set of connected peers.
    connected: HashSet<PeerId>,
}

impl Behaviour {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn perf(
        &mut self,
        server: PeerId,
        upload_bytes: u64,
        download_bytes: u64,
    ) -> Result<(), NotConnected> {
        if !self.connected.contains(&server) {
            return Err(NotConnected {});
        }

        self.queued_events.push_back(ToSwarm::NotifyHandler {
            peer_id: server,
            handler: NotifyHandler::Any,
            event: crate::client::handler::Command {
                id: NEXT_RUN_ID.fetch_add(1, Ordering::SeqCst),
                upload_bytes,
                download_bytes,
            },
        });

        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub struct NotConnected();

impl std::fmt::Display for NotConnected {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "not connected to peer")
    }
}

impl NetworkBehaviour for Behaviour {
    type ConnectionHandler = Handler;
    type ToSwarm = Event;

    fn handle_established_outbound_connection(
        &mut self,
        _connection_id: ConnectionId,
        _peer: PeerId,
        _addr: &Multiaddr,
        _role_override: libp2p_core::Endpoint,
        _port_use: PortUse,
    ) -> Result<libp2p_swarm::THandler<Self>, libp2p_swarm::ConnectionDenied> {
        Ok(Handler::default())
    }

    fn handle_established_inbound_connection(
        &mut self,
        _connection_id: ConnectionId,
        _peer: PeerId,
        _local_addr: &Multiaddr,
        _remote_addr: &Multiaddr,
    ) -> Result<libp2p_swarm::THandler<Self>, libp2p_swarm::ConnectionDenied> {
        Ok(Handler::default())
    }

    fn on_swarm_event(&mut self, event: FromSwarm) {
        match event {
            FromSwarm::ConnectionEstablished(ConnectionEstablished { peer_id, .. }) => {
                self.connected.insert(peer_id);
            }
            FromSwarm::ConnectionClosed(ConnectionClosed {
                peer_id,
                connection_id: _,
                endpoint: _,
                remaining_established,
                ..
            }) => {
                if remaining_established == 0 {
                    assert!(self.connected.remove(&peer_id));
                }
            }
            _ => {}
        }
    }

    fn on_connection_handler_event(
        &mut self,
        _event_source: PeerId,
        _connection_id: ConnectionId,
        super::handler::Event { id, result }: THandlerOutEvent<Self>,
    ) {
        self.queued_events
            .push_back(ToSwarm::GenerateEvent(Event { id, result }));
    }

    #[tracing::instrument(level = "trace", name = "NetworkBehaviour::poll", skip(self))]
    fn poll(&mut self, _: &mut Context<'_>) -> Poll<ToSwarm<Self::ToSwarm, THandlerInEvent<Self>>> {
        if let Some(event) = self.queued_events.pop_front() {
            return Poll::Ready(event);
        }

        Poll::Pending
    }
}
