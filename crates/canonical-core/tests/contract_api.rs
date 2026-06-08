use canonical_core::contract::{
    validate_agent_contract, validate_delivery_receipt, validate_platform_contract, AgentContract,
    DeliveryReceipt, DeliveryState, PlatformContract, INSTRUCTION_ROOT, PLATFORM_CONTRACT_VERSION,
};
use serde_json::json;

#[test]
fn platform_contract_defaults_to_openapi_boundary() {
    let platform = PlatformContract::new("fabric.contract.api");

    assert_eq!(platform.version, PLATFORM_CONTRACT_VERSION);
    assert_eq!(platform.instruction_root, INSTRUCTION_ROOT);
    assert!(platform.evidence_required);
    validate_platform_contract(&platform).expect("platform contract should validate");
}

#[test]
fn agent_contract_binds_to_platform_contract() {
    let platform = PlatformContract::new("fabric.contract.api");
    let agent = AgentContract {
        agent_id: "agent.contract.validator".to_string(),
        platform_contract_id: platform.id.clone(),
        platform_contract_version: platform.version.clone(),
        accepted_instruction_root: platform.instruction_root.clone(),
        delivery_boundary: "validate contract-bound delivery".to_string(),
        required_skills: vec!["contract.validate".to_string()],
        tool_permissions: vec!["box.read".to_string(), "box.verify".to_string()],
        expected_output: json!({ "type": "validation_report" }),
        evidence_obligations: vec!["validation_result".to_string()],
    };

    validate_agent_contract(&platform, &agent).expect("agent contract should bind");
}

#[test]
fn delivery_receipt_requires_contract_agent_and_evidence() {
    let platform = PlatformContract::new("fabric.contract.api");
    let receipt = DeliveryReceipt {
        contract_id: platform.id.clone(),
        agent_id: "agent.contract.validator".to_string(),
        state: DeliveryState::Delivered,
        summary: "contract validated".to_string(),
        evidence: vec![json!({ "type": "test", "status": "passed" })],
    };

    validate_delivery_receipt(&platform, &receipt).expect("delivery receipt should validate");
}

#[test]
fn delivery_receipt_without_evidence_is_rejected() {
    let platform = PlatformContract::new("fabric.contract.api");
    let receipt = DeliveryReceipt {
        contract_id: platform.id.clone(),
        agent_id: "agent.contract.validator".to_string(),
        state: DeliveryState::Delivered,
        summary: "contract claimed without proof".to_string(),
        evidence: vec![],
    };

    assert!(validate_delivery_receipt(&platform, &receipt).is_err());
}
