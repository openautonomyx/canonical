use crate::instruction::StructuredInstruction;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredBox {
    pub id: String,
    pub version: String,
    pub created_at: DateTime<Utc>,
    pub identity: Value,
    #[serde(default)]
    pub direction: Value,
    #[serde(default)]
    pub boxes: Vec<MicroBox>,
    #[serde(default)]
    pub evidence: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MicroBox {
    pub id: String,
    pub seq: u64,
    pub instruction: StructuredInstruction,
    #[serde(default)]
    pub state: Value,
    #[serde(default)]
    pub output: Value,
    #[serde(default)]
    pub evidence: Vec<Value>,
}

impl StructuredBox {
    pub fn new(identity: Value, direction: Value) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            version: "box.v0.1".to_string(),
            created_at: Utc::now(),
            identity,
            direction,
            boxes: Vec::new(),
            evidence: Vec::new(),
        }
    }

    pub fn push_box(&mut self, instruction: StructuredInstruction) -> &MicroBox {
        let micro_box = MicroBox {
            id: Uuid::new_v4().to_string(),
            seq: self.boxes.len() as u64,
            instruction,
            state: Value::Object(Default::default()),
            output: Value::Object(Default::default()),
            evidence: Vec::new(),
        };
        self.boxes.push(micro_box);
        self.boxes.last().expect("micro box was just pushed")
    }
}
