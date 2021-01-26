use hdk3::prelude::*;

#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    // grant unrestricted access to whoami_open so random agents can call it
    let mut functions: GrantedFunctions = HashSet::new();
    functions.insert((zome_info()?.zome_name, "whoami_open".into()));
    create_cap_grant(CapGrantEntry {
        tag: "".into(),
        // empty access converts to unrestricted
        access: ().into(),
        functions,
    })?;

    Ok(InitCallbackResult::Pass)
}

#[hdk_extern]
fn set_access(_: ()) -> ExternResult<()> {
    // grant unrestricted access to whoami after local agent calls set_access
    let mut functions: GrantedFunctions = HashSet::new();
    functions.insert((zome_info()?.zome_name, "whoami".into()));
    create_cap_grant(CapGrantEntry {
        tag: "".into(),
        // empty access converts to unrestricted
        access: ().into(),
        functions,
    })?;

    Ok(())
}

// returns the current agent info
#[hdk_extern]
fn whoami(_: ()) -> ExternResult<AgentInfo> {
    Ok(agent_info()?)
}

#[hdk_extern]
fn whoami_open(_: ()) -> ExternResult<AgentInfo> {
    Ok(agent_info()?)
}

// returns the agent info reported by the given pub key
// in theory the output is the same as the input
// it's just that the output comes _from the opinion of the remote agent_
#[hdk_extern]
fn whoarethey(agent_pubkey: AgentPubKey) -> ExternResult<AgentInfo> {
    Ok(call_remote(
        agent_pubkey,
        zome_info()?.zome_name,
        "whoami".to_string().into(),
        None,
        &(),
    )?)
}

// returns the agent info reported by the given pub key
// in theory the output is the same as the input
// it's just that the output comes _from the opinion of the remote agent_
#[hdk_extern]
fn whoarethey_local(cell_id: CellId) -> ExternResult<AgentInfo> {
    call(
        Some(cell_id),
        zome_info()?.zome_name,
        "whoami".to_string().into(),
        None,
        &(),
    )
}

// Same as whoarethey_local, but uses the "whoami_open", which *should* be opened in init
#[hdk_extern]
fn whoarethey_local_open(cell_id: CellId) -> ExternResult<AgentInfo> {
    call(
        Some(cell_id),
        zome_info()?.zome_name,
        "whoami_open".to_string().into(),
        None,
        &(),
    )
}

/// Call the create entry zome from this zome.
/// The cell id must point to a cell which includes
/// the "create_entry" zome.
#[hdk_extern]
fn call_create_entry(cell_id: CellId) -> ExternResult<HeaderHash> {
    Ok(call(
        Some(cell_id),
        "create_entry".to_string().into(),
        "create_entry".to_string().into(),
        None,
        &(),
    )?)
}
