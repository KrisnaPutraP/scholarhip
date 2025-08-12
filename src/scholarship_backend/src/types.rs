use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;

// User Related Types
#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct UserProfile {
    pub id: Principal,
    pub name: String,
    pub email: String,
    pub education: Education,
    pub skills: Vec<String>,
    pub preferences: Preferences,
    pub created_at: u64,
    pub updated_at: u64,
}

// Simplified version for API responses
#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct UserProfileResponse {
    pub name: String,
    pub email: String,
    pub education: Education,
    pub skills: Vec<String>,
    pub preferences: Preferences,
    pub created_at: u64,
    pub updated_at: u64,
}

// Minimal version for testing
#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct MinimalUserProfile {
    pub name: String,
    pub email: String,
    pub major: String,
    pub university: String,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct Education {
    pub degree_level: DegreeLevel,
    pub major: String,
    pub university: String,
    pub gpa: f32,
    pub graduation_year: u32,
    pub country: String,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug, PartialEq)]
pub enum DegreeLevel {
    HighSchool,
    Bachelor,
    Master,
    PhD,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct Preferences {
    pub preferred_countries: Vec<String>,
    pub preferred_fields: Vec<String>,
    pub scholarship_type: ScholarshipType,
    pub min_amount: Option<u32>,
}

// Scholarship Related Types
#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct Scholarship {
    pub id: String,
    pub name: String,
    pub provider: String,
    pub country: String,
    pub scholarship_type: ScholarshipType,
    pub degree_levels: Vec<DegreeLevel>,
    pub fields_of_study: Vec<String>,
    pub eligibility: Eligibility,
    pub benefits: Benefits,
    pub deadline: u64,
    pub application_url: String,
    pub description: String,
    pub created_at: u64,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug, PartialEq)]
pub enum ScholarshipType {
    FullScholarship,
    PartialScholarship,
    ResearchGrant,
    ExchangeProgram,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct Eligibility {
    pub min_gpa: f32,
    pub required_degree: Option<DegreeLevel>,
    pub age_limit: Option<u8>,
    pub nationalities: Vec<String>, // Empty means all nationalities
    pub required_skills: Vec<String>,
    pub work_experience_years: Option<u8>,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct Benefits {
    pub tuition_coverage: Coverage,
    pub living_allowance: Option<u32>, // Monthly amount in USD
    pub travel_allowance: bool,
    pub health_insurance: bool,
    pub duration_months: u32,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum Coverage {
    Full,
    Partial(u8), // Percentage
    None,
}

// Matching Related Types
#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct MatchResult {
    pub scholarship_id: String,
    pub user_id: Principal,
    pub compatibility_score: f32,
    pub matched_criteria: Vec<String>,
    pub missing_criteria: Vec<String>,
    pub matched_at: u64,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct ApplicationStatus {
    pub id: String,
    pub user_id: Principal,
    pub scholarship_id: String,
    pub status: Status,
    pub notes: String,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum Status {
    Bookmarked,
    InProgress,
    Submitted,
    Accepted,
    Rejected,
}

// API Response Types
#[derive(CandidType, Deserialize, Serialize)]
pub enum ApiResponse<T> {
    Ok {
        success: bool,
        message: String,
        data: Option<T>,
    },
    Err {
        success: bool,
        message: String,
    },
}

// Error Types
#[derive(CandidType, Deserialize, Serialize, Debug)]
pub enum ScholarshipError {
    NotFound,
    AlreadyExists,
    InvalidInput(String),
    Unauthorized,
    StorageError,
}