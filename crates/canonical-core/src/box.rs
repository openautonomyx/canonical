use crate::instruction::StructuredInstruction;
use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

pub const BOX_VERSION: &str = "box.v0.1";
pub const BOX_PACK_VERSION: &str = "pack.box.v0.1";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoxHeader {
    pub id: String,
    pub version: String,
    pub created_at: DateTime<Utc>,
    #[serde(default)]
    pub integrity: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoxManifest {
    pub identity: Value,
    #[serde(default)]
    pub direction: Value,
    #[serde(default)]
    pub contract: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BoxPayload {
    #[serde(default)]
    pub boxes: Vec<MicroBox>,
    #[serde(default)]
    pub evidence: Vec<Value>,
    #[serde(default)]
    pub artifacts: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredBox {
    pub header: BoxHeader,
    pub manifest: BoxManifest,
    pub payload: BoxPayload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoxPack {
    pub format: String,
    pub pack_at: DateTime<Utc>,
    pub box_id: String,
    pub data: StructuredBox,
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
            header: BoxHeader {
                id: Uuid::new_v4().to_string(),
                version: BOX_VERSION.to_string(),
                created_at: Utc::now(),
                integrity: Value::Object(Default::default()),
            },
            manifest: BoxManifest {
                identity,
                direction,
                contract: Value::Object(Default::default()),
            },
            payload: BoxPayload::default(),
        }
    }

    pub fn push_box(&mut self, instruction: StructuredInstruction) -> &MicroBox {
        let micro_box = MicroBox {
            id: Uuid::new_v4().to_string(),
            seq: self.payload.boxes.len() as u64,
            instruction,
            state: Value::Object(Default::default()),
            output: Value::Object(Default::default()),
            evidence: Vec::new(),
        };
        self.payload.boxes.push(micro_box);
        self.payload.boxes.last().expect("micro box was just pushed")
    }

    pub fn pack_box(self) -> BoxPack {
        BoxPack {
            format: BOX_PACK_VERSION.to_string(),
            pack_at: Utc::now(),
            box_id: self.header.id.clone(),
            data: self,
        }
    }
}

impl BoxPack {
    pub fn open_box(self) -> Result<StructuredBox> {
        if self.format != BOX_PACK_VERSION {
            return Err(anyhow!("unsupported box pack format: {}", self.format));
        }
        validate_box(&self.data)?;
        Ok(self.data)
    }
}

pub fn validate_box(value: &StructuredBox) -> Result<()> {
    if value.header.id.trim().is_empty() {
        return Err(anyhow!("box id is required"));
    }
    if value.header.version != BOX_VERSION {
        return Err(anyhow!("unsupported box version: {}", value.header.version));
    }
    if !value.manifest.identity.is_object() {
        return Err(anyhow!("box identity must be an object"));
    }
    for (expected, micro_box) in value.payload.boxes.iter().enumerate() {
        if micro_box.id.trim().is_empty() {
            return Err(anyhow!("micro box id is required"));
        }
        if micro_box.seq != expected as u64 {
            return Err(anyhow!("micro box sequence is invalid"));
        }
        if !micro_box.instruction.is_box_instruction() {
            return Err(anyhow!("micro box instruction must target instruct.box"));
        }
    }
    Ok(())
}
