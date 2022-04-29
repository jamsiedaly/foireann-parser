use serde::{Deserialize, Serialize, Serializer};
use std::fmt::Debug;
use serde::ser::{SerializeStruct};

#[derive(Debug, Deserialize)]
pub struct Member {
    pub identifier: Option<String>,
    #[serde(rename( deserialize = "Forename"))]
    pub first_name_english: String,
    #[serde(rename( deserialize = "Surname"))]
    pub last_name_english: String,
    #[serde(rename( deserialize = "Irish Forename"))]
    pub first_name_irish: Option<String>,
    #[serde(rename( deserialize = "Irish Surname"))]
    pub last_name_irish: Option<String>,
    #[serde(rename = "gender")]
    pub gender: Gender
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Gender {
    MALE,
    FEMALE
}

impl serde::Serialize for Member {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut state = serializer.serialize_struct("Member", 6)?;
        match &self.identifier {
            Some(id) => state.serialize_field("identifier", id),
            None => state.serialize_field("identifier", "0000000")
        }?;
        state.serialize_field("first_name_english", &first_letter_to_upper_case(&self.first_name_english))?;
        state.serialize_field("last_name_english", &first_letter_to_upper_case(&self.last_name_english))?;
        match &self.first_name_irish {
            Some(name) => state.serialize_field("first_name_irish", name),
            None => state.serialize_field("first_name_irish", &first_letter_to_upper_case(&self.first_name_english))
        }?;
        match &self.last_name_irish {
            Some(name) => state.serialize_field("last_name_irish", name),
            None => state.serialize_field("last_name_irish", &first_letter_to_upper_case(&self.last_name_english))
        }?;
        state.serialize_field("gender", &self.gender)?;
        state.end()
    }
}

fn first_letter_to_upper_case(s1: &str) -> String {
    let mut c = s1.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}