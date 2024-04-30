/// Represents a remote node
pub trait Peer: Send{}

/// Handles the communication between nodes in the network
/// Can be TCP, UDP, WebSocket, ...
pub trait Transport{}
