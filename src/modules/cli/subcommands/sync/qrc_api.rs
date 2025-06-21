use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Serialize, Deserialize, Debug)]
pub struct QrcMessage {
    pub jsonrpc: String,
    pub method: String,
    pub params: Map<String, Value>,
    pub id: u16,
}

impl QrcMessage {
    pub fn new(method: String, params: Map<String, Value>) -> Self {
        let jsonrpc = String::from("2.0");
        let mut rng = rand::rng();
        let id: u16 = rng.random();
        QrcMessage {
            jsonrpc,
            method,
            params,
            id,
        }
    }

    ///Login to the QSYS server. Provide username and password if needed
    pub fn login(username: Option<String>, password: Option<String>) -> anyhow::Result<QrcMessage> {
        let mut parameters = Map::new();
        if let Some(username) = username {
            parameters.insert("User".to_string(), serde_json::Value::String(username));
        }
        if let Some(password) = password {
            parameters.insert("Password".to_string(), serde_json::Value::String(password));
        }
        Ok(QrcMessage::new(String::from("Logon"), parameters))
    }

    ///Send a no op to the server. MUST be sent every 60 seconds to keep the session alive
    pub fn noop() -> anyhow::Result<QrcMessage> {
        Ok(QrcMessage::new(String::from("NoOp"), Map::new()))
    }

    ///Get component methods for a given component
    pub fn get_component_controls(componet_name: &str) -> anyhow::Result<QrcMessage> {
        let mut parameters = Map::new();

        parameters.insert("Name".into(), componet_name.into());
        Ok(QrcMessage::new(
            String::from("Component.GetControls"),
            parameters,
        ))
    }

    pub async fn update_code(component_name: &str, code: String) -> anyhow::Result<QrcMessage> {
        let mut parameters = Map::new();
        let mut controls = Map::new();

        // Setup the controls
        controls.insert("Name".into(), "code".into());
        controls.insert("Value".into(), code.into());

        // Setup the parameters with the controls
        parameters.insert("Name".into(), component_name.into());
        parameters.insert("Controls".into(), vec![controls].into());

        Ok(QrcMessage::new(String::from("Component.Set"), parameters))
    }

    ///Format a JsonRpcCodec into a json string for sending
    pub fn format(self) -> anyhow::Result<String, serde_json::Error> {
        let mut converted_json = serde_json::to_string_pretty(&self)?;
        // Terminate the message with a null character
        converted_json.push_str("\0");

        return Ok(converted_json);
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonRpcServerReply {
    pub jsonrpc: String,
    pub result: bool,
    pub id: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonRpcServerAnnouncement {
    pub jsonrpc: String,
    pub method: String,
    pub params: Map<String, Value>,
}

pub enum ServerReply {
    Ack(JsonRpcServerReply),
    Announce(JsonRpcServerAnnouncement),
}

impl From<JsonRpcServerReply> for ServerReply {
    fn from(reply: JsonRpcServerReply) -> Self {
        ServerReply::Ack(reply)
    }
}

impl From<JsonRpcServerAnnouncement> for ServerReply {
    fn from(broadcast: JsonRpcServerAnnouncement) -> Self {
        ServerReply::Announce(broadcast)
    }
}

impl ServerReply {
    pub fn new(json_val: Value) -> anyhow::Result<Self, serde_json::Error> {
        if let Some(_) = json_val.get("result") {
            let reply: JsonRpcServerReply = serde_json::from_value(json_val)?;
            return Ok(Self::Ack(reply));
        } else if let Some(_) = json_val.get("method") {
            let broadcast: JsonRpcServerAnnouncement = serde_json::from_value(json_val)?;
            return Ok(Self::Announce(broadcast));
        } else {
            panic!("Unknown JSON structure"); // Handle unknown structures as needed
        }
    }
}

pub fn handle_server_reply(reply: ServerReply) {
    match reply {
        ServerReply::Ack(ref r) => {
            // Handle the JsonRpcServerReply variant
            println!("\n\nReceived Acknowledgement: {:?}\n\n", r);
        }
        ServerReply::Announce(ref b) => {
            // Handle the JsonRpcServerBroadcast variant
            println!("\n\nReceived Announcement: {:?}\n\n", b);
        }
    }
}
