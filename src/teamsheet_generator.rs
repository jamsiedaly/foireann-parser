use anyhow::{Result, Context};
use crate::reader;
use crate::models::{Gender, Member};
use std::collections::BTreeMap;

pub fn generate_teamsheets(input_file: &str, male_output_file: &str, female_output_file: &str) -> Result<()> {
    let members = reader::read_members_from_file(input_file)?;

    let unique_members = remove_duplicate_members(members);

    let mut mens_writer = csv::WriterBuilder::new()
        .has_headers(true)
        .from_path(male_output_file)
        .with_context(|| "Failed to create writer")?;

    let mut womens_writer = csv::WriterBuilder::new()
        .has_headers(true)
        .from_path(female_output_file)
        .with_context(|| "Failed to create writer")?;

    for member in unique_members.values() {
        match &member.gender {
            Gender::MALE => mens_writer.serialize(member),
            Gender::FEMALE => womens_writer.serialize(member)
        }?;
    };

    Ok(())
}

fn remove_duplicate_members(members: Vec<Member>) -> BTreeMap<String, Member> {
    let mut unique_members = BTreeMap::new();
    for member in members {
        let name = format!("{} {}", member.first_name_english, member.last_name_english).to_ascii_lowercase();
        if !unique_members.contains_key(&name) || member.identifier.is_some() {
            unique_members.insert(name, member);
        }
    }
    unique_members
}
