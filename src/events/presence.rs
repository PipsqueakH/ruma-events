//! Types for the *m.presence* event.

use events::EventType;

/// Informs the client of a user's presence state change.
#[derive(Debug, Deserialize, Serialize)]
pub struct PresenceEvent {
    pub content: PresenceEventContent,
    pub event_id: String,
    #[serde(rename="type")]
    pub event_type: EventType,
}

/// The payload of a `PresenceEvent`.
#[derive(Debug, Deserialize, Serialize)]
pub struct PresenceEventContent {
    /// The current avatar URL for this user.
    pub avatar_url: Option<String>,
    /// The current display name for this user.
    pub displayname: Option<String>,
    /// The last time since this used performed some action, in milliseconds.
    pub last_active_ago: Option<u64>,
    /// The presence state for this user.
    pub presence: PresenceState,
}

/// A description of a user's connectivity and availability for chat.
#[derive(Debug, Deserialize, Serialize)]
pub enum PresenceState {
    /// Connected to the service and available for chat.
    FreeForChat,
    /// Connected to the service but not visible to other users.
    Hidden,
    /// Disconnected from the service.
    Offline,
    /// Connected to the service.
    Online,
    /// Connected to the service but not available for chat.
    Unavailable,
}