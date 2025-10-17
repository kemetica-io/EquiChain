use activitypub::server::ActivityPub;
use serde_json::json;

pub fn create_tip_activity(user: &str, amount: u64, zk_proof: &str) -> serde_json::Value {
    json!({
        "@context": ["https://www.w3.org/ns/activitystreams"],
        "type": "TipActivity",
        "actor": user,
        "object": {
            "type": "Note",
            "content": "Tipped via EquiChain!",
            "rht": { "amount": amount, "proof": zk_proof } // ZKP embed
        }
    })
}

// Export to Android JNI or Docker
