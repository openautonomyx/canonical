pub mod contract;
pub mod r#box;
pub mod instruction;

use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use uuid::Uuid;

pub const PROTOCOL_VERSION: &str = "canonical.edge.v0.1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EntryKind {
    Identity,
    Execution,
    Evidence,
    Outcome,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub id: String,
    pub protocol: String,
    pub seq: u64,
    pub kind: EntryKind,
    pub ts: DateTime<Utc>,
    pub body: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskInput {
    pub identity: Value,
    pub execution: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplaySummary {
    pub entries: usize,
    pub identities: usize,
    pub executions: usize,
    pub evidence: usize,
    pub outcomes: usize,
    pub valid: bool,
}

pub fn validate_entry(entry: &Entry) -> Result<()> {
    if entry.protocol != PROTOCOL_VERSION {
        return Err(anyhow!("unsupported protocol: {}", entry.protocol));
    }
    if entry.id.trim().is_empty() {
        return Err(anyhow!("entry id is required"));
    }
    if !entry.body.is_object() {
        return Err(anyhow!("entry body must be an object"));
    }
    Ok(())
}

pub fn read_workspace(path: impl AsRef<Path>) -> Result<Vec<Entry>> {
    let path = path.as_ref();
    if !path.exists() {
        return Ok(Vec::new());
    }

    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    let mut entries = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let entry: Entry = serde_json::from_str(&line)?;
        validate_entry(&entry)?;
        entries.push(entry);
    }

    Ok(entries)
}

pub fn append_entry(path: impl AsRef<Path>, kind: EntryKind, body: Value) -> Result<Entry> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let seq = read_workspace(path)?.len() as u64;
    let entry = Entry {
        id: Uuid::new_v4().to_string(),
        protocol: PROTOCOL_VERSION.to_string(),
        seq,
        kind,
        ts: Utc::now(),
        body,
    };
    validate_entry(&entry)?;

    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    writeln!(file, "{}", serde_json::to_string(&entry)?)?;
    Ok(entry)
}

pub fn run(task_path: impl AsRef<Path>, workspace_path: impl AsRef<Path>) -> Result<ReplaySummary> {
    let raw = fs::read_to_string(task_path)?;
    let task: TaskInput = serde_json::from_str(&raw)?;
    let workspace_path = workspace_path.as_ref();

    append_entry(workspace_path, EntryKind::Identity, task.identity)?;
    append_entry(workspace_path, EntryKind::Execution, task.execution)?;
    append_entry(
        workspace_path,
        EntryKind::Evidence,
        serde_json::json!({
            "type": "execution_recorded",
            "summary": "Canonical execution was recorded in the workspace."
        }),
    )?;
    append_entry(
        workspace_path,
        EntryKind::Outcome,
        serde_json::json!({
            "status": "recorded",
            "summary": "Canonical edge loop completed."
        }),
    )?;

    replay(workspace_path)
}

pub fn replay(workspace_path: impl AsRef<Path>) -> Result<ReplaySummary> {
    let entries = read_workspace(workspace_path)?;
    let mut summary = ReplaySummary {
        entries: entries.len(),
        identities: 0,
        executions: 0,
        evidence: 0,
        outcomes: 0,
        valid: true,
    };

    for (expected, entry) in entries.iter().enumerate() {
        if entry.seq != expected as u64 {
            summary.valid = false;
        }
        match entry.kind {
            EntryKind::Identity => summary.identities += 1,
            EntryKind::Execution => summary.executions += 1,
            EntryKind::Evidence => summary.evidence += 1,
            EntryKind::Outcome => summary.outcomes += 1,
        }
    }

    Ok(summary)
}

pub fn validate(workspace_path: impl AsRef<Path>) -> Result<()> {
    let summary = replay(workspace_path)?;
    if !summary.valid {
        return Err(anyhow!("workspace sequence is invalid"));
    }
    Ok(())
}

pub fn report(workspace_path: impl AsRef<Path>, out_path: impl AsRef<Path>) -> Result<()> {
    let summary = replay(workspace_path)?;
    let out_path = out_path.as_ref();
    if let Some(parent) = out_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let content = format!(
        "# Canonical Edge Report\n\n- Entries: {}\n- Identity entries: {}\n- Execution entries: {}\n- Evidence entries: {}\n- Outcome entries: {}\n- Valid: {}\n",
        summary.entries,
        summary.identities,
        summary.executions,
        summary.evidence,
        summary.outcomes,
        summary.valid
    );
    fs::write(out_path, content)?;
    Ok(())
}
