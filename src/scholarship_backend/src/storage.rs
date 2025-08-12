use candid::Principal;
use crate::types::*;
use std::cell::RefCell;
use std::collections::HashMap;

// Storage implementation using regular HashMap for now
// We'll convert to stable storage later when we resolve the key type issues
thread_local! {
    static USERS: RefCell<HashMap<Principal, UserProfile>> = RefCell::new(HashMap::new());
    static SCHOLARSHIPS: RefCell<HashMap<String, Scholarship>> = RefCell::new(HashMap::new());
    static MATCHES: RefCell<HashMap<String, MatchResult>> = RefCell::new(HashMap::new());
    static APPLICATIONS: RefCell<HashMap<String, ApplicationStatus>> = RefCell::new(HashMap::new());
}

// User functions
pub fn insert_user(user: UserProfile) -> Result<(), ScholarshipError> {
    USERS.with(|users| {
        users.borrow_mut().insert(user.id, user);
        Ok(())
    })
}

pub fn get_user(id: &Principal) -> Option<UserProfile> {
    USERS.with(|users| {
        users.borrow().get(id).cloned()
    })
}

pub fn update_user(user: UserProfile) -> Result<(), ScholarshipError> {
    USERS.with(|users| {
        users.borrow_mut().insert(user.id, user);
        Ok(())
    })
}

// Scholarship functions
pub fn insert_scholarship(scholarship: Scholarship) -> Result<(), ScholarshipError> {
    SCHOLARSHIPS.with(|scholarships| {
        scholarships.borrow_mut().insert(scholarship.id.clone(), scholarship);
        Ok(())
    })
}

pub fn get_scholarship(id: &str) -> Option<Scholarship> {
    SCHOLARSHIPS.with(|scholarships| {
        scholarships.borrow().get(id).cloned()
    })
}

pub fn get_all_scholarships() -> Vec<Scholarship> {
    SCHOLARSHIPS.with(|scholarships| {
        scholarships.borrow()
            .values()
            .cloned()
            .collect()
    })
}

// Match functions
pub fn insert_match(match_result: MatchResult) -> Result<(), ScholarshipError> {
    MATCHES.with(|matches| {
        let key = format!("{}_{}", match_result.user_id.to_text(), match_result.scholarship_id);
        matches.borrow_mut().insert(key, match_result);
        Ok(())
    })
}

pub fn get_user_matches(user_id: &Principal) -> Vec<MatchResult> {
    MATCHES.with(|matches| {
        matches.borrow()
            .values()
            .filter(|match_result| match_result.user_id == *user_id)
            .cloned()
            .collect()
    })
}

// Application functions
pub fn insert_application(application: ApplicationStatus) -> Result<(), ScholarshipError> {
    APPLICATIONS.with(|applications| {
        applications.borrow_mut().insert(application.id.clone(), application);
        Ok(())
    })
}

pub fn update_application_status(id: &str, status: Status) -> Result<(), ScholarshipError> {
    APPLICATIONS.with(|applications| {
        let mut apps = applications.borrow_mut();
        if let Some(app) = apps.get_mut(id) {
            app.status = status;
            app.updated_at = ic_cdk::api::time();
            Ok(())
        } else {
            Err(ScholarshipError::NotFound)
        }
    })
}

pub fn get_user_applications(user_id: &Principal) -> Vec<ApplicationStatus> {
    APPLICATIONS.with(|applications| {
        applications.borrow()
            .values()
            .filter(|app| app.user_id == *user_id)
            .cloned()
            .collect()
    })
}