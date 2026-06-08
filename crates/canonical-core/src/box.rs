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
    pub runs: Vec<MiniRun>,
    #[serde(default)]
    pub evidence: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniRun {
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
            runs: Vec::new(),
            evidence: Vec::new(),
        }
    }

    pub fn push_run(&mut self, instruction: StructuredInstruction) -> &MiniRun {
        let run = MiniRun {
            id: Uuid::new_v4().to_string(),
            seq: self.runs.len() as u64,
            instruction,
            state: Value::Object(Default::default()),
            output: Value::Object(Default::default()),
            evidence: Vec::new(),
        };
        self.runs.push(run);
        self.runs.last().expect("run was just pushed")
    }
}
