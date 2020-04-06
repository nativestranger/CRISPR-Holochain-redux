use hdk::{
    error::ZomeApiResult,
    holochain_core_types::{
        entry::Entry,
    },
    holochain_persistence_api::cas::content::{
        Address
    },
    prelude::*,
};
use holochain_anchors::anchor;
use crate::note::{
    NOTES_ANCHOR_TYPE,
    NOTES_ANCHOR_TEXT,
    NOTE_ID_LINK_TYPE,
    NOTE_ID_ENTRY_NAME,
    NOTE_ENTRY_LINK_TYPE,
    NOTE_ENTRY_NAME,
    NoteId,
    NoteEntry,
    Note,
};

fn notes_anchor() -> ZomeApiResult<Address> {
    anchor(NOTES_ANCHOR_TYPE.to_string(), NOTES_ANCHOR_TEXT.to_string())
}

pub fn create_note(note_entry: NoteEntry) -> ZomeApiResult<Note> {
    let note_anchor = notes_anchor()?;
    let entry = Entry::App(NOTE_ENTRY_NAME.into(), note_entry.clone().into());
    let entry_address = hdk::commit_entry(&entry)?;
    let note_id = NoteId::new(entry_address.clone())?;
    let note_id_entry = Entry::App(NOTE_ID_ENTRY_NAME.into(), note_id.clone().into());
    let note_id_address = hdk::commit_entry(&note_id_entry)?;
    hdk::link_entries(&note_id_address, &entry_address, NOTE_ENTRY_LINK_TYPE, "")?;
    hdk::link_entries(&note_anchor, &note_id_address, NOTE_ID_LINK_TYPE, "")?;
    Note::new(note_id, note_entry)
}

pub fn get_note(id: Address) -> ZomeApiResult<Note> {
    let note_id: NoteId = hdk::utils::get_as_type(id.clone())?;
    if let Some(link) = hdk::get_links(&id, LinkMatch::Exactly(NOTE_ENTRY_LINK_TYPE), LinkMatch::Any)?.links().get(0) {
        let note_entry: NoteEntry = hdk::utils::get_as_type(link.address.clone())?;
        Note::new(note_id, note_entry)
    }
    else {
        Err(hdk::error::ZomeApiError::Internal("No Note at this address".to_string()))
    }
}

pub fn update_note(id: Address, note_input: NoteEntry) -> ZomeApiResult<Note> {
    let note_id: NoteId = hdk::utils::get_as_type(id.clone())?;
    if let Some(link) = hdk::get_links(&id, LinkMatch::Exactly(NOTE_ENTRY_LINK_TYPE), LinkMatch::Any)?.links().get(0) {
        let entry_address = link.address.clone();
        let updated_entry_address = hdk::update_entry(Entry::App(NOTE_ENTRY_NAME.into(), note_input.clone().into()), &entry_address.clone())?;
        hdk::remove_link(&id, &entry_address, NOTE_ENTRY_LINK_TYPE, "")?;
        hdk::link_entries(&id, &updated_entry_address, NOTE_ENTRY_LINK_TYPE, "")?;
        Note::new(note_id, note_input)
    }
    else {
        Err(hdk::error::ZomeApiError::Internal("No Note at this address".to_string()))
    }
}

pub fn remove_note(id: Address) -> ZomeApiResult<Address> {
    if let Some(link) = hdk::get_links(&id, LinkMatch::Exactly(NOTE_ENTRY_LINK_TYPE), LinkMatch::Any)?.links().get(0) {
        let entry_address = link.address.clone();
        hdk::remove_link(&id, &entry_address, NOTE_ENTRY_LINK_TYPE, "")?;
        hdk::remove_link(&notes_anchor()?, &id, NOTE_ID_LINK_TYPE, "")?;
        hdk::remove_entry(&entry_address)?;
        hdk::remove_entry(&id)
    }
    else {
        Err(hdk::error::ZomeApiError::Internal("No Note at this address".to_string()))
    }
}

pub fn list_notes() -> ZomeApiResult<Vec<Note>> {
    hdk::get_links(&notes_anchor()?, LinkMatch::Exactly(NOTE_ID_LINK_TYPE), LinkMatch::Any)?.addresses()
    .iter()
    .map(|address| get_note(address.clone()))
    .collect()
}
