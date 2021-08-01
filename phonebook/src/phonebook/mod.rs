use crate::contact::Contact;
use std::collections::HashMap;

pub struct PhoneBook {
    contacts: HashMap<String, Contact>,
}

impl PhoneBook {
    pub fn new() -> PhoneBook {
        PhoneBook {
            contacts: HashMap::new(),
        }
    }

    pub fn insert(&mut self, contact: Contact) {
        self.contacts.insert(contact.id.clone(), contact);
    }

    pub fn get(&self, id: &str) -> Option<&Contact> {
        self.contacts.get(id)
    }
}
