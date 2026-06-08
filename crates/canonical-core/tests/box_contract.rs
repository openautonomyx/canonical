use canonical_core::r#box::{validate_box, StructuredBox, BOX_PACK_VERSION};
use canonical_core::instruction::{InstructionScope, InstructionVerb, StructuredInstruction, INSTRUCT_BOX};
use serde_json::json;

#[test]
fn structured_instruction_targets_instruct_box() {
    let instruction = StructuredInstruction::instruct_box(
        InstructionVerb::Create,
        InstructionScope::Edge,
        json!({ "purpose": "verify contract" }),
    );

    assert_eq!(instruction.root, INSTRUCT_BOX);
    assert!(instruction.is_box_instruction());
}

#[test]
fn box_accepts_microbox_and_validates_sequence() {
    let instruction = StructuredInstruction::instruct_box(
        InstructionVerb::Verify,
        InstructionScope::Local,
        json!({ "target": "box" }),
    );
    let mut bx = StructuredBox::new(
        json!({ "actor": "builder" }),
        json!({ "goal": "prove box contract" }),
    );

    bx.push_box(instruction);

    validate_box(&bx).expect("box should validate");
    assert_eq!(bx.payload.boxes.len(), 1);
    assert_eq!(bx.payload.boxes[0].seq, 0);
}

#[test]
fn box_pack_round_trip_preserves_box() {
    let bx = StructuredBox::new(
        json!({ "actor": "builder" }),
        json!({ "goal": "round trip" }),
    );
    let box_id = bx.header.id.clone();

    let pack = bx.pack_box();
    assert_eq!(pack.format, BOX_PACK_VERSION);
    assert_eq!(pack.box_id, box_id);

    let opened = pack.open_box().expect("box pack should open");
    assert_eq!(opened.header.id, box_id);
}
