use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

pub const PLATFORM_CONTRACT_VERSION: &str = "platform.contract.v0.1";
pub const INSTRUCTION_ROOT: &str = "instruct.box";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DeliveryState {
    Delivered,
    FailedWithEvidence,
    RejectedWithReason,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlatformContract {
    pub id: String,
    pub version: String,
    pub scope: String,
    pub instruction_root: String,
    #[serde(default)]
    pub policy: Value,
    #[serde(default)]
    pub evidence_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentContract {
    pub agent_id: String,
    pub platform_contract_id: String,
    pub platform_contract_version: String,
    pub accepted_instruction_root: String,
    pub delivery_boundary: String,
    #[serde(default)]
    pub required_skills: Vec<String>,
    #[serde(default)]
    pub tool_permissions: Vec<String>,
    #[serde(default)]
    pub expected_output: Value,
    #[serde(default)]
    pub evidence_obligations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryReceipt {
    pub contract_id: String,
    pub agent_id: String,
    pub state: DeliveryState,
    pub summary: String,
    #[serde(default)]
    pub evidence: Vec<Value>,
}

impl PlatformContract {
    pub fn new(scope: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            version: PLATFORM_CONTRACT_VERSION.to_string(),
            scope: scope.into(),
            instruction_root: INSTRUCTION_ROOT.to_string(),
            policy: Value::Object(Default::default()),
            evidence_required: true,
        }
    }
}

pub fn validate_platform_contract(contract: &PlatformContract) -> Result<()> {
    if contract.id.trim().is_empty() {
        return Err(anyhow!("platform contract id is required"));
    }
    if contract.version != PLATFORM_CONTRACT_VERSION {
        return Err(anyhow!("unsupported platform contract version: {}", contract.version));
    }
    if contract.scope.trim().is_empty() {
        return Err(anyhow!("platform contract scope is required"));
    }
    if contract.instruction_root != INSTRUCTION_ROOT {
        return Err(anyhow!("platform contract must use instruct.box"));
    }
    Ok(())
}

pub fn validate_agent_contract(platform: &PlatformContract, agent: &AgentContract) -> Result<()> {
    validate_platform_contract(platform)?;
    if agent.agent_id.trim().is_empty() {
        return Err(anyhow!("agent id is required"));
    }
    if agent.platform_contract_id != platform.id {
        return Err(anyhow!("agent contract must bind to platform contract id"));
    }
    if agent.platform_contract_version != platform.version {
        return Err(anyhow!("agent contract must bind to platform contract version"));
    }
    if agent.accepted_instruction_root != platform.instruction_root {
        return Err(anyhow!("agent must accept platform instruction root"));
    }
    if agent.delivery_boundary.trim().is_empty() {
        return Err(anyhow!("agent delivery boundary is required"));
    }
    Ok(())
}

pub fn validate_delivery_receipt(platform: &PlatformContract, receipt: &DeliveryReceipt) -> Result<()> {
    validate_platform_contract(platform)?;
    if receipt.contract_id != platform.id {
        return Err(anyhow!("delivery receipt must bind to platform contract id"));
    }
    if receipt.agent_id.trim().is_empty() {
        return Err(anyhow!("delivery receipt agent id is required"));
    }
    if receipt.summary.trim().is_empty() {
        return Err(anyhow!("delivery receipt summary is required"));
    }
    if platform.evidence_required && receipt.evidence.is_empty() {
        return Err(anyhow!("delivery receipt requires evidence"));
    }
    match receipt.state {
        DeliveryState::Delivered | DeliveryState::FailedWithEvidence | DeliveryState::RejectedWithReason => Ok(()),
    }
}
