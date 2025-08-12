use ic_cdk_macros::{init, query, update};
use candid::Principal;

mod types;
mod storage;
mod matching;

use types::*;

// ============= INITIALIZATION =============

#[init]
fn init() {
    ic_cdk::println!("Scholarship Matcher Canister Initialized");
    init_sample_scholarships();
}

fn init_sample_scholarships() {
    let scholarships = vec![
        // ========== INDONESIA SCHOLARSHIPS ==========
        Scholarship {
            id: "lpdp-2025".to_string(),
            name: "LPDP Scholarship 2025".to_string(),
            provider: "Lembaga Pengelola Dana Pendidikan".to_string(),
            country: "Indonesia".to_string(),
            scholarship_type: ScholarshipType::FullScholarship,
            degree_levels: vec![DegreeLevel::Master, DegreeLevel::PhD],
            fields_of_study: vec!["Any".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.0,
                required_degree: Some(DegreeLevel::Bachelor),
                age_limit: Some(35),
                nationalities: vec!["Indonesia".to_string()],
                required_skills: vec!["English".to_string()],
                work_experience_years: None,
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Full,
                living_allowance: Some(2000),
                travel_allowance: true,
                health_insurance: true,
                duration_months: 24,
            },
            deadline: 1743465600, // 2025-03-31
            application_url: "https://lpdp.kemenkeu.go.id".to_string(),
            description: "Full scholarship for Indonesian students to pursue higher education abroad".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "beasiswa-unggulan-2025".to_string(),
            name: "Beasiswa Unggulan Kemendikbud 2025".to_string(),
            provider: "Kementerian Pendidikan, Kebudayaan, Riset dan Teknologi".to_string(),
            country: "Indonesia".to_string(),
            scholarship_type: ScholarshipType::FullScholarship,
            degree_levels: vec![DegreeLevel::Bachelor, DegreeLevel::Master, DegreeLevel::PhD],
            fields_of_study: vec!["Science".to_string(), "Technology".to_string(), "Engineering".to_string(), "Mathematics".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.25,
                required_degree: None,
                age_limit: Some(23),
                nationalities: vec!["Indonesia".to_string()],
                required_skills: vec!["English".to_string(), "Research".to_string()],
                work_experience_years: None,
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Full,
                living_allowance: Some(1500),
                travel_allowance: false,
                health_insurance: true,
                duration_months: 48,
            },
            deadline: 1740787200, // 2025-02-28
            application_url: "https://beasiswaunggulan.kemdikbud.go.id".to_string(),
            description: "Scholarship for outstanding Indonesian students in STEM fields".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "djarum-plus-2025".to_string(),
            name: "Djarum Beasiswa Plus 2025".to_string(),
            provider: "Djarum Foundation".to_string(),
            country: "Indonesia".to_string(),
            scholarship_type: ScholarshipType::PartialScholarship,
            degree_levels: vec![DegreeLevel::Bachelor],
            fields_of_study: vec!["Any".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.0,
                required_degree: None,
                age_limit: Some(21),
                nationalities: vec!["Indonesia".to_string()],
                required_skills: vec!["Leadership".to_string(), "Communication".to_string()],
                work_experience_years: None,
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Partial(50),
                living_allowance: Some(750),
                travel_allowance: false,
                health_insurance: false,
                duration_months: 12,
            },
            deadline: 1738281600, // 2025-01-31
            application_url: "https://djarumbeasiswaplus.org".to_string(),
            description: "Scholarship for active and achieving undergraduate students".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "bca-finance-2025".to_string(),
            name: "BCA Finance Scholarship 2025".to_string(),
            provider: "BCA Finance".to_string(),
            country: "Indonesia".to_string(),
            scholarship_type: ScholarshipType::FullScholarship,
            degree_levels: vec![DegreeLevel::Bachelor],
            fields_of_study: vec!["Finance".to_string(), "Economics".to_string(), "Business".to_string(), "Accounting".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.5,
                required_degree: None,
                age_limit: Some(22),
                nationalities: vec!["Indonesia".to_string()],
                required_skills: vec!["Mathematics".to_string(), "English".to_string()],
                work_experience_years: None,
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Full,
                living_allowance: Some(1000),
                travel_allowance: false,
                health_insurance: true,
                duration_months: 48,
            },
            deadline: 1735689600, // 2025-01-01
            application_url: "https://bcafinance.co.id/scholarship".to_string(),
            description: "Full scholarship for finance and business students".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "tanoto-foundation-2025".to_string(),
            name: "Tanoto Foundation Scholarship 2025".to_string(),
            provider: "Tanoto Foundation".to_string(),
            country: "Indonesia".to_string(),
            scholarship_type: ScholarshipType::PartialScholarship,
            degree_levels: vec![DegreeLevel::Bachelor],
            fields_of_study: vec!["Education".to_string(), "Medicine".to_string(), "Public Health".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.0,
                required_degree: None,
                age_limit: Some(20),
                nationalities: vec!["Indonesia".to_string()],
                required_skills: vec!["Community Service".to_string(), "Leadership".to_string()],
                work_experience_years: None,
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Partial(75),
                living_allowance: Some(500),
                travel_allowance: false,
                health_insurance: true,
                duration_months: 48,
            },
            deadline: 1740787200, // 2025-02-28
            application_url: "https://tanotofoundation.org".to_string(),
            description: "Supporting students in education and healthcare fields".to_string(),
            created_at: ic_cdk::api::time(),
        },

        // ========== INTERNATIONAL SCHOLARSHIPS ==========
        Scholarship {
            id: "fulbright-2025".to_string(),
            name: "Fulbright Scholarship 2025".to_string(),
            provider: "U.S. Department of State".to_string(),
            country: "United States".to_string(),
            scholarship_type: ScholarshipType::FullScholarship,
            degree_levels: vec![DegreeLevel::Master, DegreeLevel::PhD],
            fields_of_study: vec!["Any".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.5,
                required_degree: Some(DegreeLevel::Bachelor),
                age_limit: None,
                nationalities: vec![], // All nationalities
                required_skills: vec!["English".to_string(), "Leadership".to_string()],
                work_experience_years: Some(2),
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Full,
                living_allowance: Some(2500),
                travel_allowance: true,
                health_insurance: true,
                duration_months: 24,
            },
            deadline: 1736899200, // 2025-01-15
            application_url: "https://fulbrightscholars.org".to_string(),
            description: "Prestigious scholarship for international students to study in the US".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "chevening-2025".to_string(),
            name: "Chevening Scholarship 2025".to_string(),
            provider: "UK Government".to_string(),
            country: "United Kingdom".to_string(),
            scholarship_type: ScholarshipType::FullScholarship,
            degree_levels: vec![DegreeLevel::Master],
            fields_of_study: vec!["Any".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.0,
                required_degree: Some(DegreeLevel::Bachelor),
                age_limit: None,
                nationalities: vec![], // All developing countries
                required_skills: vec!["English".to_string(), "Leadership".to_string()],
                work_experience_years: Some(2),
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Full,
                living_allowance: Some(1800),
                travel_allowance: true,
                health_insurance: true,
                duration_months: 12,
            },
            deadline: 1730419200, // 2024-11-01
            application_url: "https://chevening.org".to_string(),
            description: "UK government scholarship for future leaders".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "daad-2025".to_string(),
            name: "DAAD Scholarship 2025".to_string(),
            provider: "German Academic Exchange Service".to_string(),
            country: "Germany".to_string(),
            scholarship_type: ScholarshipType::FullScholarship,
            degree_levels: vec![DegreeLevel::Master, DegreeLevel::PhD],
            fields_of_study: vec!["Engineering".to_string(), "Science".to_string(), "Technology".to_string(), "Social Sciences".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.0,
                required_degree: Some(DegreeLevel::Bachelor),
                age_limit: Some(32),
                nationalities: vec![], // All developing countries
                required_skills: vec!["German".to_string(), "English".to_string()],
                work_experience_years: Some(1),
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Full,
                living_allowance: Some(1200),
                travel_allowance: true,
                health_insurance: true,
                duration_months: 24,
            },
            deadline: 1738281600, // 2025-01-31
            application_url: "https://daad.de".to_string(),
            description: "German scholarship for international students in STEM and social sciences".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "erasmus-mundus-2025".to_string(),
            name: "Erasmus Mundus Joint Masters 2025".to_string(),
            provider: "European Commission".to_string(),
            country: "Europe".to_string(),
            scholarship_type: ScholarshipType::FullScholarship,
            degree_levels: vec![DegreeLevel::Master],
            fields_of_study: vec!["Any".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.0,
                required_degree: Some(DegreeLevel::Bachelor),
                age_limit: None,
                nationalities: vec![], // All nationalities
                required_skills: vec!["English".to_string()],
                work_experience_years: None,
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Full,
                living_allowance: Some(1400),
                travel_allowance: true,
                health_insurance: true,
                duration_months: 24,
            },
            deadline: 1735689600, // 2025-01-01
            application_url: "https://erasmus-plus.ec.europa.eu".to_string(),
            description: "European joint masters programs with mobility".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "australia-awards-2025".to_string(),
            name: "Australia Awards Scholarship 2025".to_string(),
            provider: "Australian Government".to_string(),
            country: "Australia".to_string(),
            scholarship_type: ScholarshipType::FullScholarship,
            degree_levels: vec![DegreeLevel::Master, DegreeLevel::PhD],
            fields_of_study: vec!["Any".to_string()],
            eligibility: Eligibility {
                min_gpa: 2.9,
                required_degree: Some(DegreeLevel::Bachelor),
                age_limit: Some(42),
                nationalities: vec!["Indonesia".to_string(), "Vietnam".to_string(), "Philippines".to_string()], // ASEAN focus
                required_skills: vec!["English".to_string()],
                work_experience_years: Some(2),
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Full,
                living_allowance: Some(2200),
                travel_allowance: true,
                health_insurance: true,
                duration_months: 24,
            },
            deadline: 1740787200, // 2025-02-28
            application_url: "https://australiaawards.gov.au".to_string(),
            description: "Australian government scholarship for developing country students".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "mext-japan-2025".to_string(),
            name: "MEXT Scholarship Japan 2025".to_string(),
            provider: "Japanese Government".to_string(),
            country: "Japan".to_string(),
            scholarship_type: ScholarshipType::FullScholarship,
            degree_levels: vec![DegreeLevel::Bachelor, DegreeLevel::Master, DegreeLevel::PhD],
            fields_of_study: vec!["Any".to_string()],
            eligibility: Eligibility {
                min_gpa: 2.8,
                required_degree: None,
                age_limit: Some(35),
                nationalities: vec![], // All nationalities
                required_skills: vec!["Japanese".to_string(), "English".to_string()],
                work_experience_years: None,
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Full,
                living_allowance: Some(1200),
                travel_allowance: true,
                health_insurance: true,
                duration_months: 48,
            },
            deadline: 1735689600, // 2025-01-01
            application_url: "https://mext.go.jp".to_string(),
            description: "Japanese government scholarship for international students".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "kgsp-korea-2025".to_string(),
            name: "Korean Government Scholarship 2025".to_string(),
            provider: "Korean Government".to_string(),
            country: "South Korea".to_string(),
            scholarship_type: ScholarshipType::FullScholarship,
            degree_levels: vec![DegreeLevel::Bachelor, DegreeLevel::Master, DegreeLevel::PhD],
            fields_of_study: vec!["Any".to_string()],
            eligibility: Eligibility {
                min_gpa: 2.5,
                required_degree: None,
                age_limit: Some(25),
                nationalities: vec![], // All nationalities
                required_skills: vec!["Korean".to_string(), "English".to_string()],
                work_experience_years: None,
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Full,
                living_allowance: Some(800),
                travel_allowance: true,
                health_insurance: true,
                duration_months: 48,
            },
            deadline: 1740787200, // 2025-02-28
            application_url: "https://studyinkorea.go.kr".to_string(),
            description: "Korean government scholarship program for international students".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "csc-china-2025".to_string(),
            name: "Chinese Government Scholarship 2025".to_string(),
            provider: "China Scholarship Council".to_string(),
            country: "China".to_string(),
            scholarship_type: ScholarshipType::FullScholarship,
            degree_levels: vec![DegreeLevel::Bachelor, DegreeLevel::Master, DegreeLevel::PhD],
            fields_of_study: vec!["Any".to_string()],
            eligibility: Eligibility {
                min_gpa: 2.5,
                required_degree: None,
                age_limit: Some(35),
                nationalities: vec![], // All nationalities
                required_skills: vec!["Chinese".to_string(), "English".to_string()],
                work_experience_years: None,
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Full,
                living_allowance: Some(600),
                travel_allowance: true,
                health_insurance: true,
                duration_months: 48,
            },
            deadline: 1738281600, // 2025-01-31
            application_url: "https://csc.edu.cn".to_string(),
            description: "Chinese government scholarship for international students".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "swiss-excellence-2025".to_string(),
            name: "Swiss Government Excellence Scholarship 2025".to_string(),
            provider: "Swiss Government".to_string(),
            country: "Switzerland".to_string(),
            scholarship_type: ScholarshipType::FullScholarship,
            degree_levels: vec![DegreeLevel::Master, DegreeLevel::PhD],
            fields_of_study: vec!["Any".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.5,
                required_degree: Some(DegreeLevel::Bachelor),
                age_limit: Some(35),
                nationalities: vec![], // All nationalities
                required_skills: vec!["English".to_string(), "German".to_string(), "French".to_string()],
                work_experience_years: None,
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Full,
                living_allowance: Some(1800),
                travel_allowance: true,
                health_insurance: true,
                duration_months: 24,
            },
            deadline: 1735689600, // 2025-01-01
            application_url: "https://sbfi.admin.ch".to_string(),
            description: "Swiss government scholarship for excellent international students".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "vanier-canada-2025".to_string(),
            name: "Vanier Canada Graduate Scholarship 2025".to_string(),
            provider: "Government of Canada".to_string(),
            country: "Canada".to_string(),
            scholarship_type: ScholarshipType::FullScholarship,
            degree_levels: vec![DegreeLevel::PhD],
            fields_of_study: vec!["Health".to_string(), "Natural Sciences".to_string(), "Engineering".to_string(), "Social Sciences".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.7,
                required_degree: Some(DegreeLevel::Master),
                age_limit: None,
                nationalities: vec![], // All nationalities
                required_skills: vec!["English".to_string(), "French".to_string(), "Research".to_string()],
                work_experience_years: None,
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Full,
                living_allowance: Some(4000),
                travel_allowance: false,
                health_insurance: true,
                duration_months: 36,
            },
            deadline: 1730419200, // 2024-11-01
            application_url: "https://vanier.gc.ca".to_string(),
            description: "Canada's premier doctoral scholarship program".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "rhodes-oxford-2025".to_string(),
            name: "Rhodes Scholarship Oxford 2025".to_string(),
            provider: "Rhodes Trust".to_string(),
            country: "United Kingdom".to_string(),
            scholarship_type: ScholarshipType::FullScholarship,
            degree_levels: vec![DegreeLevel::Master, DegreeLevel::PhD],
            fields_of_study: vec!["Any".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.7,
                required_degree: Some(DegreeLevel::Bachelor),
                age_limit: Some(24),
                nationalities: vec![], // Selected countries
                required_skills: vec!["English".to_string(), "Leadership".to_string(), "Academic Excellence".to_string()],
                work_experience_years: None,
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Full,
                living_allowance: Some(2000),
                travel_allowance: true,
                health_insurance: true,
                duration_months: 24,
            },
            deadline: 1727740800, // 2024-10-01
            application_url: "https://rhodeshouse.ox.ac.uk".to_string(),
            description: "The world's oldest international scholarship program".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "gates-cambridge-2025".to_string(),
            name: "Gates Cambridge Scholarship 2025".to_string(),
            provider: "Gates Cambridge Trust".to_string(),
            country: "United Kingdom".to_string(),
            scholarship_type: ScholarshipType::FullScholarship,
            degree_levels: vec![DegreeLevel::Master, DegreeLevel::PhD],
            fields_of_study: vec!["Any".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.7,
                required_degree: Some(DegreeLevel::Bachelor),
                age_limit: None,
                nationalities: vec![], // Non-UK students
                required_skills: vec!["English".to_string(), "Leadership".to_string(), "Academic Excellence".to_string()],
                work_experience_years: None,
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Full,
                living_allowance: Some(2100),
                travel_allowance: true,
                health_insurance: true,
                duration_months: 24,
            },
            deadline: 1735689600, // 2025-01-01
            application_url: "https://gatescambridge.org".to_string(),
            description: "Prestigious scholarship for outstanding students at Cambridge University".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "clarendon-oxford-2025".to_string(),
            name: "Clarendon Scholarship Oxford 2025".to_string(),
            provider: "University of Oxford".to_string(),
            country: "United Kingdom".to_string(),
            scholarship_type: ScholarshipType::FullScholarship,
            degree_levels: vec![DegreeLevel::Master, DegreeLevel::PhD],
            fields_of_study: vec!["Any".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.5,
                required_degree: Some(DegreeLevel::Bachelor),
                age_limit: None,
                nationalities: vec![], // All nationalities
                required_skills: vec!["English".to_string(), "Academic Excellence".to_string()],
                work_experience_years: None,
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Full,
                living_allowance: Some(1800),
                travel_allowance: false,
                health_insurance: true,
                duration_months: 24,
            },
            deadline: 1735689600, // 2025-01-01
            application_url: "https://clarendon.ox.ac.uk".to_string(),
            description: "Oxford University's flagship graduate scholarship scheme".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "eiffel-france-2025".to_string(),
            name: "Eiffel Excellence Scholarship 2025".to_string(),
            provider: "French Government".to_string(),
            country: "France".to_string(),
            scholarship_type: ScholarshipType::FullScholarship,
            degree_levels: vec![DegreeLevel::Master, DegreeLevel::PhD],
            fields_of_study: vec!["Engineering".to_string(), "Science".to_string(), "Economics".to_string(), "Law".to_string(), "Political Science".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.0,
                required_degree: Some(DegreeLevel::Bachelor),
                age_limit: Some(30),
                nationalities: vec![], // All developing countries
                required_skills: vec!["French".to_string(), "English".to_string()],
                work_experience_years: None,
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Full,
                living_allowance: Some(1400),
                travel_allowance: true,
                health_insurance: true,
                duration_months: 24,
            },
            deadline: 1735689600, // 2025-01-01
            application_url: "https://campusfrance.org".to_string(),
            description: "French government scholarship for international excellence".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "orange-tulip-2025".to_string(),
            name: "Orange Tulip Scholarship 2025".to_string(),
            provider: "Dutch Universities".to_string(),
            country: "Netherlands".to_string(),
            scholarship_type: ScholarshipType::PartialScholarship,
            degree_levels: vec![DegreeLevel::Bachelor, DegreeLevel::Master],
            fields_of_study: vec!["Any".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.0,
                required_degree: None,
                age_limit: None,
                nationalities: vec!["Indonesia".to_string(), "India".to_string(), "China".to_string(), "Brazil".to_string()],
                required_skills: vec!["English".to_string(), "Dutch".to_string()],
                work_experience_years: None,
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Partial(50),
                living_allowance: Some(800),
                travel_allowance: false,
                health_insurance: false,
                duration_months: 24,
            },
            deadline: 1740787200, // 2025-02-28
            application_url: "https://orangetulipscholarship.nl".to_string(),
            description: "Scholarship for students from selected countries to study in Netherlands".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "swedish-institute-2025".to_string(),
            name: "Swedish Institute Scholarship 2025".to_string(),
            provider: "Swedish Institute".to_string(),
            country: "Sweden".to_string(),
            scholarship_type: ScholarshipType::FullScholarship,
            degree_levels: vec![DegreeLevel::Master],
            fields_of_study: vec!["Any".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.0,
                required_degree: Some(DegreeLevel::Bachelor),
                age_limit: None,
                nationalities: vec![], // Selected countries
                required_skills: vec!["English".to_string(), "Swedish".to_string()],
                work_experience_years: Some(3),
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Full,
                living_allowance: Some(1000),
                travel_allowance: true,
                health_insurance: true,
                duration_months: 24,
            },
            deadline: 1738281600, // 2025-01-31
            application_url: "https://si.se".to_string(),
            description: "Swedish government scholarship for international students".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "turkiye-burslari-2025".to_string(),
            name: "Türkiye Burslari Scholarship 2025".to_string(),
            provider: "Turkish Government".to_string(),
            country: "Turkey".to_string(),
            scholarship_type: ScholarshipType::FullScholarship,
            degree_levels: vec![DegreeLevel::Bachelor, DegreeLevel::Master, DegreeLevel::PhD],
            fields_of_study: vec!["Any".to_string()],
            eligibility: Eligibility {
                min_gpa: 2.5,
                required_degree: None,
                age_limit: Some(30),
                nationalities: vec![], // All nationalities except Turkey
                required_skills: vec!["Turkish".to_string(), "English".to_string()],
                work_experience_years: None,
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Full,
                living_allowance: Some(700),
                travel_allowance: true,
                health_insurance: true,
                duration_months: 48,
            },
            deadline: 1740787200, // 2025-02-28
            application_url: "https://turkiyeburslari.gov.tr".to_string(),
            description: "Turkish government scholarship for international students".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "hubert-humphrey-2025".to_string(),
            name: "Hubert Humphrey Fellowship 2025".to_string(),
            provider: "U.S. Department of State".to_string(),
            country: "United States".to_string(),
            scholarship_type: ScholarshipType::FullScholarship,
            degree_levels: vec![DegreeLevel::Master],
            fields_of_study: vec!["Public Administration".to_string(), "Law".to_string(), "Education".to_string(), "Journalism".to_string(), "Public Health".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.0,
                required_degree: Some(DegreeLevel::Bachelor),
                age_limit: None,
                nationalities: vec![], // Selected countries
                required_skills: vec!["English".to_string(), "Leadership".to_string()],
                work_experience_years: Some(5),
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Full,
                living_allowance: Some(2000),
                travel_allowance: true,
                health_insurance: true,
                duration_months: 10,
            },
            deadline: 1727740800, // 2024-10-01
            application_url: "https://humphreyfellowship.org".to_string(),
            description: "Non-degree fellowship for experienced professionals".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "commonwealth-2025".to_string(),
            name: "Commonwealth Scholarship 2025".to_string(),
            provider: "Commonwealth Scholarship Commission".to_string(),
            country: "United Kingdom".to_string(),
            scholarship_type: ScholarshipType::FullScholarship,
            degree_levels: vec![DegreeLevel::Master, DegreeLevel::PhD],
            fields_of_study: vec!["Any".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.0,
                required_degree: Some(DegreeLevel::Bachelor),
                age_limit: None,
                nationalities: vec!["Indonesia".to_string(), "India".to_string(), "Malaysia".to_string(), "Pakistan".to_string()], // Commonwealth countries
                required_skills: vec!["English".to_string()],
                work_experience_years: None,
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Full,
                living_allowance: Some(1600),
                travel_allowance: true,
                health_insurance: true,
                duration_months: 24,
            },
            deadline: 1735689600, // 2025-01-01
            application_url: "https://cscuk.fcdo.gov.uk".to_string(),
            description: "UK scholarship for students from Commonwealth countries".to_string(),
            created_at: ic_cdk::api::time(),
        },
    ];

    for scholarship in scholarships {
        let _ = storage::insert_scholarship(scholarship);
    }
}

// ============= USER MANAGEMENT =============

#[update]
fn register_user(
    name: String,
    email: String,
    education: Education,
    skills: Vec<String>,
    preferences: Preferences,
) -> String {
    let caller = ic_cdk::caller();
    
    // Check if user already exists
    if storage::get_user(&caller).is_some() {
        return "User already registered".to_string();
    }

    let user = UserProfile {
        id: caller,
        name,
        email,
        education,
        skills,
        preferences,
        created_at: ic_cdk::api::time(),
        updated_at: ic_cdk::api::time(),
    };

    match storage::insert_user(user) {
        Ok(_) => "User registered successfully".to_string(),
        Err(_) => "Registration failed".to_string(),
    }
}

#[query]
fn get_my_profile() -> String {
    let caller = ic_cdk::caller();
    
    match storage::get_user(&caller) {
        Some(user) => {
            // Return a simple JSON-like string that won't cause Candid issues
            format!("{{\"name\":\"{}\",\"email\":\"{}\",\"major\":\"{}\",\"university\":\"{}\"}}",
                user.name, user.email, user.education.major, user.education.university)
        },
        None => "{\"error\":\"User not found\"}".to_string(),
    }
}

// ============= SCHOLARSHIP MANAGEMENT =============

#[query]
fn count_scholarships() -> u32 {
    let scholarships = storage::get_all_scholarships();
    scholarships.len() as u32
}

#[query]
fn get_all_scholarships() -> String {
    let scholarships = storage::get_all_scholarships();
    if scholarships.is_empty() {
        "No scholarships found".to_string()
    } else {
        let mut result = format!("Found {} scholarships:\n", scholarships.len());
        for scholarship in scholarships {
            result.push_str(&format!(
                "- {} ({}) - Provider: {} - Country: {}\n",
                scholarship.name, scholarship.id, scholarship.provider, scholarship.country
            ));
        }
        result
    }
}

#[query]
fn get_scholarship_details(id: String) -> String {
    match storage::get_scholarship(&id) {
        Some(scholarship) => {
            format!("Scholarship: {} by {} - Monthly Allowance: ${} - Deadline: {}", 
                scholarship.name, scholarship.provider, 
                scholarship.benefits.living_allowance.unwrap_or(0), 
                scholarship.deadline)
        },
        None => "Scholarship not found".to_string(),
    }
}

// ============= SIMPLE FUNCTIONS FOR TESTING =============

#[query]
fn test_connection() -> String {
    "Connection successful!".to_string()
}

#[update]
fn manual_init_scholarships() -> String {
    init_sample_scholarships();
    let count = storage::get_all_scholarships().len();
    format!("Manually initialized {} scholarships", count)
}

#[query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

// ============= USER MANAGEMENT =============

#[update]
fn update_user_profile(
    name: Option<String>,
    email: Option<String>, 
    education: Option<Education>,
    skills: Option<Vec<String>>,
    preferences: Option<Preferences>,
) -> String {
    let caller = ic_cdk::caller();
    
    match storage::get_user(&caller) {
        Some(mut user) => {
            if let Some(n) = name { user.name = n; }
            if let Some(e) = email { user.email = e; }
            if let Some(ed) = education { user.education = ed; }
            if let Some(s) = skills { user.skills = s; }
            if let Some(p) = preferences { user.preferences = p; }
            user.updated_at = ic_cdk::api::time();
            
            match storage::insert_user(user) {
                Ok(_) => "Profile updated successfully".to_string(),
                Err(_) => "Failed to update profile".to_string(),
            }
        },
        None => "User not found".to_string(),
    }
}

// ============= SCHOLARSHIP MANAGEMENT =============

#[update]
fn add_scholarship(scholarship: Scholarship) -> String {
    match storage::insert_scholarship(scholarship.clone()) {
        Ok(_) => format!("Scholarship '{}' added successfully", scholarship.name),
        Err(_) => "Failed to add scholarship".to_string(),
    }
}

#[query]
fn get_scholarship_by_id(id: String) -> String {
    match storage::get_scholarship(&id) {
        Some(scholarship) => format!(
            "{{\"id\":\"{}\",\"name\":\"{}\",\"provider\":\"{}\",\"country\":\"{}\"}}",
            scholarship.id, scholarship.name, scholarship.provider, scholarship.country
        ),
        None => "Scholarship not found".to_string(),
    }
}

#[query]
fn search_scholarships(
    field: Option<String>,
    degree_level: Option<DegreeLevel>, 
    country: Option<String>
) -> String {
    let scholarships = storage::get_all_scholarships();
    let mut results = Vec::new();
    
    for scholarship in scholarships {
        let mut matches = true;
        
        if let Some(ref f) = field {
            if !scholarship.fields_of_study.iter().any(|field_of_study| 
                field_of_study.to_lowercase().contains(&f.to_lowercase())
            ) {
                matches = false;
            }
        }
        
        if let Some(ref dl) = degree_level {
            if !scholarship.degree_levels.contains(dl) {
                matches = false;
            }
        }
        
        if let Some(ref c) = country {
            if !scholarship.country.to_lowercase().contains(&c.to_lowercase()) {
                matches = false;
            }
        }
        
        if matches {
            results.push(format!("{} - {}", scholarship.name, scholarship.provider));
        }
    }
    
    if results.is_empty() {
        "No scholarships found matching criteria".to_string()
    } else {
        format!("Found {} scholarships: {}", results.len(), results.join(", "))
    }
}

// ============= MATCHING & RECOMMENDATIONS =============

#[query]
fn get_my_recommendations(limit: Option<u32>) -> String {
    let caller = ic_cdk::caller();
    
    let user = match storage::get_user(&caller) {
        Some(user) => user,
        None => return "User not found".to_string(),
    };
    
    let scholarships = storage::get_all_scholarships();
    let mut matches = Vec::new();
    
    for scholarship in scholarships {
        let match_result = matching::MatchingEngine::calculate_compatibility(&user, &scholarship);
        matches.push((match_result.compatibility_score, scholarship.name.clone()));
    }
    
    matches.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    
    let take_count = limit.unwrap_or(5) as usize;
    let top_matches: Vec<String> = matches
        .into_iter()
        .take(take_count)
        .map(|(score, name)| format!("{}: {:.1}%", name, score * 100.0))
        .collect();
    
    if top_matches.is_empty() {
        "No recommendations found".to_string()
    } else {
        format!("Top Recommendations: {}", top_matches.join(", "))
    }
}

#[query]
fn get_my_matches() -> String {
    let caller = ic_cdk::caller();
    
    let user = match storage::get_user(&caller) {
        Some(user) => user,
        None => return "User not found".to_string(),
    };
    
    let scholarships = storage::get_all_scholarships();
    let mut good_matches = Vec::new();
    
    for scholarship in scholarships {
        let match_result = matching::MatchingEngine::calculate_compatibility(&user, &scholarship);
        if match_result.compatibility_score > 0.5 { // 50% threshold
            good_matches.push(format!(
                "{}: {:.1}% match",
                scholarship.name,
                match_result.compatibility_score * 100.0
            ));
        }
    }
    
    if good_matches.is_empty() {
        "No good matches found (>50%)".to_string()
    } else {
        format!("Your matches: {}", good_matches.join(", "))
    }
}

// ============= APPLICATION TRACKING =============

#[update]
fn create_application(scholarship_id: String, notes: String) -> String {
    let caller = ic_cdk::caller();
    
    // Check if scholarship exists
    if storage::get_scholarship(&scholarship_id).is_none() {
        return "Scholarship not found".to_string();
    }
    
    // Check if user exists
    if storage::get_user(&caller).is_none() {
        return "User not found".to_string();
    }
    
    let application = ApplicationStatus {
        id: format!("{}_{}", scholarship_id, ic_cdk::api::time()),
        user_id: caller,
        scholarship_id: scholarship_id.clone(),
        status: Status::InProgress,
        notes,
        created_at: ic_cdk::api::time(),
        updated_at: ic_cdk::api::time(),
    };
    
    match storage::insert_application(application) {
        Ok(_) => format!("Application created for scholarship: {}", scholarship_id),
        Err(_) => "Failed to create application".to_string(),
    }
}

#[update]
fn update_application_status(application_id: String, status: Status) -> String {
    match storage::update_application_status(&application_id, status) {
        Ok(_) => "Application status updated successfully".to_string(),
        Err(_) => "Failed to update application status".to_string(),
    }
}

#[query]
fn get_my_applications() -> String {
    let caller = ic_cdk::caller();
    let applications = storage::get_user_applications(&caller);
    
    if applications.is_empty() {
        "No applications found".to_string()
    } else {
        let app_list: Vec<String> = applications
            .into_iter()
            .map(|app| format!("{}: {:?}", app.scholarship_id, app.status))
            .collect();
        format!("Your applications: {}", app_list.join(", "))
    }
}

// ============= BOOKMARKS =============

#[update] 
fn bookmark_scholarship(scholarship_id: String) -> String {
    let caller = ic_cdk::caller();
    
    // Check if scholarship exists
    if storage::get_scholarship(&scholarship_id).is_none() {
        return "Scholarship not found".to_string();
    }
    
    // Create a bookmark application
    let application = ApplicationStatus {
        id: format!("bookmark_{}_{}", scholarship_id, ic_cdk::api::time()),
        user_id: caller,
        scholarship_id: scholarship_id.clone(),
        status: Status::Bookmarked,
        notes: "Bookmarked by user".to_string(),
        created_at: ic_cdk::api::time(),
        updated_at: ic_cdk::api::time(),
    };
    
    match storage::insert_application(application) {
        Ok(_) => format!("Scholarship '{}' bookmarked successfully", scholarship_id),
        Err(_) => "Failed to bookmark scholarship".to_string(),
    }
}

#[query]
fn get_my_bookmarks() -> String {
    let caller = ic_cdk::caller();
    let applications = storage::get_user_applications(&caller);
    
    let bookmarks: Vec<String> = applications
        .into_iter()
        .filter(|app| matches!(app.status, Status::Bookmarked))
        .map(|app| app.scholarship_id)
        .collect();
    
    if bookmarks.is_empty() {
        "No bookmarks found".to_string()
    } else {
        format!("Your bookmarks: {}", bookmarks.join(", "))
    }
}

#[query] 
fn calculate_match_score(scholarship_id: String) -> String {
    let caller = ic_cdk::caller();
    
    // Get user profile
    let user = match storage::get_user(&caller) {
        Some(user) => user,
        None => return "Error: User not found. Please register first.".to_string(),
    };
    
    // Get scholarship
    let scholarship = match storage::get_scholarship(&scholarship_id) {
        Some(scholarship) => scholarship,
        None => return "Error: Scholarship not found.".to_string(),
    };
    
    // Calculate match using the matching engine
    let match_result = matching::MatchingEngine::calculate_compatibility(&user, &scholarship);
    
    format!(
        "Match Score: {:.2}% for {} | Matched: {} | Missing: {}",
        match_result.compatibility_score * 100.0,
        scholarship.name,
        match_result.matched_criteria.join(", "),
        match_result.missing_criteria.join(", ")
    )
}
