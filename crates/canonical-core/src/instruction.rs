use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const INSTRUCT_BOX: &str = "instruct.box";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InstructionVerb {
    Create,
    Open,
    Append,
    Read,
    Verify,
    Replay,
    Report,
    Seal,
    Ship,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InstructionScope {
    Edge,
    Local,
    Workspace,
    Registry,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredInstruction {
    pub root: String,
    pub verb: InstructionVerb,
    pub scope: InstructionScope,
    #[serde(default)]
    pub constraints: Value,
    #[serde(default)]
    pub payload: Value,
}

impl StructuredInstruction {
    pub fn instruct_box(verb: InstructionVerb, scope: InstructionScope, payload: Value) -> Self {
        Self {
            root: INSTRUCT_BOX.to_string(),
            verb,
            scope,
            constraints: Value::Object(Default::default()),
            payload,
        }
    }

    pub fn is_box_instruction(&self) -> bool {
        self.root == INSTRUCT_BOX
    }
}
