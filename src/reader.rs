use anyhow::Context;
use csv::Trim;

use crate::models::Member;

pub fn read_members_from_file(file: &str) -> anyhow::Result<Vec<Member>> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .trim(Trim::All)
        .flexible(true)
        .comment(Some(b'#'))
        .from_path(file)
        .with_context(|| format!("Failed to read file {}", file))?;

    let members: Vec<Member> = reader.deserialize::<Member>().filter_map(|result| result.ok()).collect();
    Ok(members)
}
