use std::{
    collections::VecDeque,
    task::{Context, Poll},
};

use libp2p_core::transport::PortUse;
use libp2p_identity::PeerId;
use libp2p_swarm::{
    ConnectionId, FromSwarm, NetworkBehaviour, THandlerInEvent, THandlerOutEvent, ToSwarm,
};

use crate::server::handler::Handler;

#[derive(Debug)]
pub struct Event {}

#[derive(Default)]
pub struct Behaviour {
    /// Queue of actions to return when polled.
    queued_events: VecDeque<ToSwarm<Event, THandlerInEvent<Self>>>,
}

impl Behaviour {
    pub fn new() -> Self {
        Self::default()
    }
}

impl NetworkBehaviour for Behaviour {
    type ConnectionHandler = Handler;
    type ToSwarm = Event;

    fn handle_established_inbound_connection(
        &mut self,
        _connection_id: ConnectionId,
        _peer: PeerId,
        _local_addr: &libp2p_core::Multiaddr,
        _remote_addr: &libp2p_core::Multiaddr,
    ) -> Result<libp2p_swarm::THandler<Self>, libp2p_swarm::ConnectionDenied> {
        Ok(Handler::default())
    }

    fn handle_established_outbound_connection(
        &mut self,
        _connection_id: ConnectionId,
        _peer: PeerId,
        _addr: &libp2p_core::Multiaddr,
        _role_override: libp2p_core::Endpoint,
        _port_use: PortUse,
    ) -> Result<libp2p_swarm::THandler<Self>, libp2p_swarm::ConnectionDenied> {
        Ok(Handler::default())
    }

    fn on_swarm_event(&mut self, _event: FromSwarm) {}

    fn on_connection_handler_event(
        &mut self,
        _event_source: PeerId,
        _connection_id: ConnectionId,
        super::handler::Event {}: THandlerOutEvent<Self>,
    ) {
        self.queued_events
            .push_back(ToSwarm::GenerateEvent(Event {}))
    }

    #[tracing::instrument(level = "trace", name = "NetworkBehaviour::poll", skip(self))]
    fn poll(&mut self, _: &mut Context<'_>) -> Poll<ToSwarm<Self::ToSwarm, THandlerInEvent<Self>>> {
        if let Some(event) = self.queued_events.pop_front() {
            return Poll::Ready(event);
        }

        Poll::Pending
    }
}
