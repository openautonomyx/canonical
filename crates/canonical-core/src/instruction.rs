use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BoxInstruction {
    Create,
    Open,
    Append,
    Read,
    Verify,
    Replay,
    Report,
    Seal,
    Ship,
    Retire,
}

impl BoxInstruction {
    pub fn as_str(&self) -> &'static str {
        match self {
            BoxInstruction::Create => "box.create",
            BoxInstruction::Open => "box.open",
            BoxInstruction::Append => "box.append",
            BoxInstruction::Read => "box.read",
            BoxInstruction::Verify => "box.verify",
            BoxInstruction::Replay => "box.replay",
            BoxInstruction::Report => "box.report",
            BoxInstruction::Seal => "box.seal",
            BoxInstruction::Ship => "box.ship",
            BoxInstruction::Retire => "box.retire",
        }
    }
}

pub const BOX_INSTRUCTION_SET: &[BoxInstruction] = &[
    BoxInstruction::Create,
    BoxInstruction::Open,
    BoxInstruction::Append,
    BoxInstruction::Read,
    BoxInstruction::Verify,
    BoxInstruction::Replay,
    BoxInstruction::Report,
    BoxInstruction::Seal,
    BoxInstruction::Ship,
    BoxInstruction::Retire,
];
