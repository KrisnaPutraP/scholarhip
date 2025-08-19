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
            deadline: 1775001600, // 2026-03-31
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
            deadline: 1772323200, // 2026-02-28
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
            fields_of_study: vec!["Business Administration".to_string(), "Economics".to_string(), "Management".to_string(), "Marketing".to_string(), "Entrepreneurship".to_string()],
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
            deadline: 1769817600, // 2026-01-31
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
            deadline: 1767225600, // 2026-01-01
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
            deadline: 1772323200, // 2026-02-28
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
            fields_of_study: vec!["International Relations".to_string(), "Public Policy".to_string(), "American Studies".to_string(), "Education".to_string(), "Social Sciences".to_string()],
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
            deadline: 1768435200, // 2026-01-15
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
            fields_of_study: vec!["Public Administration".to_string(), "International Development".to_string(), "Law".to_string(), "Political Science".to_string(), "Business".to_string()],
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
            deadline: 1761955200, // 2025-11-01
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
            deadline: 1769817600, // 2026-01-31
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
            deadline: 1767225600, // 2026-01-01
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
            deadline: 1772323200, // 2026-02-28
            application_url: "https://australiaawards.gov.au".to_string(),
            description: "Australian government scholarship for developing country students".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "microsoft-scholarship-2025".to_string(),
            name: "Microsoft Diversity in Technology Scholarship 2025".to_string(),
            provider: "Microsoft Corporation".to_string(),
            country: "United States".to_string(),
            scholarship_type: ScholarshipType::PartialScholarship,
            degree_levels: vec![DegreeLevel::Bachelor, DegreeLevel::Master],
            fields_of_study: vec!["Computer Science".to_string(), "Information Technology".to_string(), "Software Engineering".to_string(), "Cybersecurity".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.0,
                required_degree: Some(DegreeLevel::HighSchool),
                age_limit: None,
                nationalities: vec![], // All nationalities
                required_skills: vec!["Programming".to_string(), "English".to_string(), "Leadership".to_string()],
                work_experience_years: None,
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Partial(80),
                living_allowance: Some(1500),
                travel_allowance: false,
                health_insurance: false,
                duration_months: 36,
            },
            deadline: 1769817600, // 2026-01-31
            application_url: "https://careers.microsoft.com/scholarships".to_string(),
            description: "Microsoft scholarship promoting diversity in technology fields".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "google-dev-scholarship-2025".to_string(),
            name: "Google Developer Scholarship 2025".to_string(),
            provider: "Google LLC".to_string(),
            country: "United States".to_string(),
            scholarship_type: ScholarshipType::PartialScholarship,
            degree_levels: vec![DegreeLevel::Bachelor, DegreeLevel::Master],
            fields_of_study: vec!["Computer Science".to_string(), "Software Engineering".to_string(), "Data Science".to_string(), "Artificial Intelligence".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.2,
                required_degree: Some(DegreeLevel::HighSchool),
                age_limit: None,
                nationalities: vec![], // All nationalities
                required_skills: vec!["Programming".to_string(), "English".to_string(), "Problem Solving".to_string()],
                work_experience_years: None,
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Partial(70),
                living_allowance: Some(1000),
                travel_allowance: false,
                health_insurance: true,
                duration_months: 36,
            },
            deadline: 1767225600, // 2026-01-01
            application_url: "https://developers.google.com/scholarships".to_string(),
            description: "Google scholarship for aspiring developers and tech professionals".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "nsf-research-grant-2025".to_string(),
            name: "NSF Graduate Research Fellowship 2025".to_string(),
            provider: "National Science Foundation".to_string(),
            country: "United States".to_string(),
            scholarship_type: ScholarshipType::ResearchGrant,
            degree_levels: vec![DegreeLevel::Master, DegreeLevel::PhD],
            fields_of_study: vec!["Physics".to_string(), "Chemistry".to_string(), "Biology".to_string(), "Engineering".to_string(), "Mathematics".to_string(), "Computer Science".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.5,
                required_degree: Some(DegreeLevel::Bachelor),
                age_limit: None,
                nationalities: vec!["United States".to_string()], // US citizens/residents only
                required_skills: vec!["Research".to_string(), "Scientific Writing".to_string(), "English".to_string()],
                work_experience_years: None,
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Full,
                living_allowance: Some(3000),
                travel_allowance: true,
                health_insurance: true,
                duration_months: 36,
            },
            deadline: 1761955200, // 2025-11-01
            application_url: "https://nsf.gov/grfp".to_string(),
            description: "Prestigious research fellowship for graduate students in STEM fields".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "rotary-exchange-2025".to_string(),
            name: "Rotary Youth Exchange Program 2025".to_string(),
            provider: "Rotary International".to_string(),
            country: "Various".to_string(),
            scholarship_type: ScholarshipType::ExchangeProgram,
            degree_levels: vec![DegreeLevel::Bachelor],
            fields_of_study: vec!["International Relations".to_string(), "Cultural Studies".to_string(), "Languages".to_string(), "Business".to_string()],
            eligibility: Eligibility {
                min_gpa: 2.8,
                required_degree: Some(DegreeLevel::HighSchool),
                age_limit: Some(26),
                nationalities: vec![], // All nationalities
                required_skills: vec!["English".to_string(), "Cultural Adaptability".to_string(), "Communication".to_string()],
                work_experience_years: None,
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Partial(50),
                living_allowance: Some(500),
                travel_allowance: true,
                health_insurance: true,
                duration_months: 12,
            },
            deadline: 1769817600, // 2026-01-31
            application_url: "https://rotary.org/youth-exchange".to_string(),
            description: "Cultural exchange program for young leaders and students".to_string(),
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
            deadline: 1761955200, // 2025-11-01
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
            deadline: 1759276800, // 2025-10-01
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
            deadline: 1767225600, // 2026-01-01
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
            deadline: 1767225600, // 2026-01-01
            application_url: "https://clarendon.ox.ac.uk".to_string(),
            description: "Oxford University's flagship graduate scholarship scheme".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "gates-cambridge-2025".to_string(),
            name: "Gates Cambridge Scholarship 2025".to_string(),
            provider: "Gates Cambridge Trust".to_string(),
            country: "United Kingdom".to_string(),
            scholarship_type: ScholarshipType::FullScholarship,
            degree_levels: vec![DegreeLevel::Master, DegreeLevel::PhD],
            fields_of_study: vec!["Public Health".to_string(), "Social Sciences".to_string(), "Education".to_string(), "Medicine".to_string(), "Development Studies".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.7,
                required_degree: Some(DegreeLevel::Bachelor),
                age_limit: None,
                nationalities: vec![], // All nationalities except UK
                required_skills: vec!["English".to_string(), "Leadership".to_string(), "Social Impact".to_string()],
                work_experience_years: Some(2),
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Full,
                living_allowance: Some(2000),
                travel_allowance: true,
                health_insurance: true,
                duration_months: 36,
            },
            deadline: 1764633600, // 2025-12-02
            application_url: "https://gatescambridge.org".to_string(),
            description: "Prestigious scholarship for future leaders committed to improving lives globally".to_string(),
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
            deadline: 1772323200, // 2026-02-28
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
            deadline: 1769817600, // 2026-01-31
            application_url: "https://si.se".to_string(),
            description: "Swedish government scholarship for international students".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "ieee-scholarship-2025".to_string(),
            name: "IEEE Women in Engineering Scholarship 2025".to_string(),
            provider: "Institute of Electrical and Electronics Engineers".to_string(),
            country: "United States".to_string(),
            scholarship_type: ScholarshipType::PartialScholarship,
            degree_levels: vec![DegreeLevel::Bachelor, DegreeLevel::Master],
            fields_of_study: vec!["Electrical Engineering".to_string(), "Computer Engineering".to_string(), "Electronics".to_string(), "Telecommunications".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.2,
                required_degree: Some(DegreeLevel::HighSchool),
                age_limit: None,
                nationalities: vec![], // All nationalities
                required_skills: vec!["Engineering".to_string(), "Mathematics".to_string(), "English".to_string()],
                work_experience_years: None,
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Partial(60),
                living_allowance: Some(800),
                travel_allowance: false,
                health_insurance: false,
                duration_months: 24,
            },
            deadline: 1769817600, // 2026-01-31
            application_url: "https://ieee.org/membership/women/scholarships".to_string(),
            description: "Professional engineering scholarship for women in electrical and computer engineering".to_string(),
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
            deadline: 1759276800, // 2025-10-01
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
            deadline: 1767225600, // 2026-01-01
            application_url: "https://cscuk.fcdo.gov.uk".to_string(),
            description: "UK scholarship for students from Commonwealth countries".to_string(),
            created_at: ic_cdk::api::time(),
        },

        // ========== ADDITIONAL 25 SCHOLARSHIPS ==========
        Scholarship {
            id: "daad-germany-2025".to_string(),
            name: "DAAD Scholarship Germany 2025".to_string(),
            provider: "German Academic Exchange Service".to_string(),
            country: "Germany".to_string(),
            scholarship_type: ScholarshipType::FullScholarship,
            degree_levels: vec![DegreeLevel::Master, DegreeLevel::PhD],
            fields_of_study: vec!["Engineering".to_string(), "Natural Sciences".to_string(), "Social Sciences".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.0,
                required_degree: Some(DegreeLevel::Bachelor),
                age_limit: Some(32),
                nationalities: vec![],
                required_skills: vec!["German".to_string(), "English".to_string()],
                work_experience_years: Some(2),
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Full,
                living_allowance: Some(1200),
                travel_allowance: true,
                health_insurance: true,
                duration_months: 24,
            },
            deadline: 1775001600, // 2026-03-31
            application_url: "https://daad.de".to_string(),
            description: "German government scholarship for international graduate students".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "orange-tulip-netherlands-2025".to_string(),
            name: "Orange Tulip Scholarship 2025".to_string(),
            provider: "Nuffic Netherlands".to_string(),
            country: "Netherlands".to_string(),
            scholarship_type: ScholarshipType::PartialScholarship,
            degree_levels: vec![DegreeLevel::Bachelor, DegreeLevel::Master],
            fields_of_study: vec!["Any".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.0,
                required_degree: None,
                age_limit: None,
                nationalities: vec!["Indonesia".to_string(), "India".to_string(), "Brazil".to_string(), "Mexico".to_string()],
                required_skills: vec!["English".to_string()],
                work_experience_years: None,
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Partial(50),
                living_allowance: Some(800),
                travel_allowance: false,
                health_insurance: false,
                duration_months: 24,
            },
            deadline: 1772323200, // 2026-02-28
            application_url: "https://orangetulipscholarship.nl".to_string(),
            description: "Dutch scholarship program for students from selected countries".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "belgian-development-2025".to_string(),
            name: "Belgian Development Cooperation Scholarship 2025".to_string(),
            provider: "Belgian Government".to_string(),
            country: "Belgium".to_string(),
            scholarship_type: ScholarshipType::FullScholarship,
            degree_levels: vec![DegreeLevel::Master],
            fields_of_study: vec!["Development Studies".to_string(), "Public Health".to_string(), "Environment".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.0,
                required_degree: Some(DegreeLevel::Bachelor),
                age_limit: Some(40),
                nationalities: vec!["Indonesia".to_string(), "Vietnam".to_string(), "Morocco".to_string()],
                required_skills: vec!["French".to_string(), "English".to_string()],
                work_experience_years: Some(2),
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Full,
                living_allowance: Some(1100),
                travel_allowance: true,
                health_insurance: true,
                duration_months: 12,
            },
            deadline: 1775001600, // 2026-03-31
            application_url: "https://ares-ac.be".to_string(),
            description: "Belgian scholarship for development cooperation".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "christian-scholars-foundation-2025".to_string(),
            name: "Christian Scholars Foundation Fellowship 2025".to_string(),
            provider: "Christian Scholars Foundation".to_string(),
            country: "United States".to_string(),
            scholarship_type: ScholarshipType::PartialScholarship,
            degree_levels: vec![DegreeLevel::Master, DegreeLevel::PhD],
            fields_of_study: vec!["Theology".to_string(), "Philosophy".to_string(), "Religious Studies".to_string(), "Ethics".to_string(), "History".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.3,
                required_degree: Some(DegreeLevel::Bachelor),
                age_limit: None,
                nationalities: vec![], // All nationalities
                required_skills: vec!["English".to_string(), "Research".to_string(), "Critical Thinking".to_string()],
                work_experience_years: None,
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Partial(75),
                living_allowance: Some(1200),
                travel_allowance: false,
                health_insurance: false,
                duration_months: 24,
            },
            deadline: 1767225600, // 2026-01-01
            application_url: "https://christianscholars.org".to_string(),
            description: "Faith-based scholarship for Christian scholars in humanities and theology".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "aga-khan-foundation-2025".to_string(),
            name: "Aga Khan Foundation Scholarship 2025".to_string(),
            provider: "Aga Khan Foundation".to_string(),
            country: "Multiple Countries".to_string(),
            scholarship_type: ScholarshipType::PartialScholarship,
            degree_levels: vec![DegreeLevel::Master, DegreeLevel::PhD],
            fields_of_study: vec!["Any".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.5,
                required_degree: Some(DegreeLevel::Bachelor),
                age_limit: None,
                nationalities: vec!["Bangladesh".to_string(), "India".to_string(), "Pakistan".to_string(), "Afghanistan".to_string()],
                required_skills: vec!["English".to_string()],
                work_experience_years: Some(2),
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Partial(70),
                living_allowance: Some(1000),
                travel_allowance: false,
                health_insurance: false,
                duration_months: 24,
            },
            deadline: 1775001600, // 2026-03-31
            application_url: "https://akf.org.uk".to_string(),
            description: "Foundation scholarship for outstanding students from developing countries".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "joint-japan-world-bank-2025".to_string(),
            name: "Joint Japan World Bank Graduate Scholarship 2025".to_string(),
            provider: "World Bank".to_string(),
            country: "Multiple Countries".to_string(),
            scholarship_type: ScholarshipType::FullScholarship,
            degree_levels: vec![DegreeLevel::Master],
            fields_of_study: vec!["Development Studies".to_string(), "Economics".to_string(), "Public Policy".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.0,
                required_degree: Some(DegreeLevel::Bachelor),
                age_limit: Some(45),
                nationalities: vec!["Indonesia".to_string(), "Vietnam".to_string(), "Philippines".to_string()],
                required_skills: vec!["English".to_string()],
                work_experience_years: Some(3),
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Full,
                living_allowance: Some(2000),
                travel_allowance: true,
                health_insurance: true,
                duration_months: 24,
            },
            deadline: 1775001600, // 2026-03-31
            application_url: "https://worldbank.org".to_string(),
            description: "World Bank scholarship for development professionals".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "rotary-peace-fellowship-2025".to_string(),
            name: "Rotary Peace Fellowship 2025".to_string(),
            provider: "Rotary Foundation".to_string(),
            country: "Multiple Countries".to_string(),
            scholarship_type: ScholarshipType::FullScholarship,
            degree_levels: vec![DegreeLevel::Master],
            fields_of_study: vec!["Peace Studies".to_string(), "Conflict Resolution".to_string(), "International Relations".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.0,
                required_degree: Some(DegreeLevel::Bachelor),
                age_limit: None,
                nationalities: vec![],
                required_skills: vec!["English".to_string(), "Leadership".to_string()],
                work_experience_years: Some(3),
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Full,
                living_allowance: Some(1500),
                travel_allowance: true,
                health_insurance: true,
                duration_months: 15,
            },
            deadline: 1769040000, // 2026-01-22
            application_url: "https://rotary.org".to_string(),
            description: "Fellowship for future leaders in peace and conflict resolution".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "mastercard-foundation-2025".to_string(),
            name: "Mastercard Foundation Scholars Program 2025".to_string(),
            provider: "Mastercard Foundation".to_string(),
            country: "Multiple Countries".to_string(),
            scholarship_type: ScholarshipType::FullScholarship,
            degree_levels: vec![DegreeLevel::Bachelor, DegreeLevel::Master],
            fields_of_study: vec!["Any".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.0,
                required_degree: None,
                age_limit: Some(35),
                nationalities: vec!["Ghana".to_string(), "Kenya".to_string(), "Rwanda".to_string(), "Uganda".to_string()],
                required_skills: vec!["English".to_string(), "Leadership".to_string()],
                work_experience_years: None,
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Full,
                living_allowance: Some(1800),
                travel_allowance: true,
                health_insurance: true,
                duration_months: 48,
            },
            deadline: 1772323200, // 2026-02-28
            application_url: "https://mastercardfdn.org".to_string(),
            description: "Comprehensive scholarship program for African students".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "ford-foundation-2025".to_string(),
            name: "Ford Foundation Fellowship 2025".to_string(),
            provider: "Ford Foundation".to_string(),
            country: "United States".to_string(),
            scholarship_type: ScholarshipType::FullScholarship,
            degree_levels: vec![DegreeLevel::PhD],
            fields_of_study: vec!["Social Sciences".to_string(), "Humanities".to_string(), "STEM".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.5,
                required_degree: Some(DegreeLevel::Master),
                age_limit: None,
                nationalities: vec!["United States".to_string()],
                required_skills: vec!["English".to_string(), "Research".to_string()],
                work_experience_years: None,
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Full,
                living_allowance: Some(2700),
                travel_allowance: false,
                health_insurance: true,
                duration_months: 36,
            },
            deadline: 1767830400, // 2026-01-08
            application_url: "https://fordfoundation.org".to_string(),
            description: "Fellowship for underrepresented minorities in academia".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "google-lime-2025".to_string(),
            name: "Google Lime Scholarship 2025".to_string(),
            provider: "Google Inc.".to_string(),
            country: "Multiple Countries".to_string(),
            scholarship_type: ScholarshipType::PartialScholarship,
            degree_levels: vec![DegreeLevel::Bachelor, DegreeLevel::Master],
            fields_of_study: vec!["Computer Science".to_string(), "Engineering".to_string(), "Technology".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.5,
                required_degree: None,
                age_limit: None,
                nationalities: vec![],
                required_skills: vec!["Programming".to_string(), "English".to_string()],
                work_experience_years: None,
            },
            benefits: Benefits {
                tuition_coverage: Coverage::None,
                living_allowance: Some(10000),
                travel_allowance: false,
                health_insurance: false,
                duration_months: 12,
            },
            deadline: 1767225600, // 2026-01-01
            application_url: "https://google.com/scholarships".to_string(),
            description: "Scholarship for students with disabilities in computer science".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "microsoft-scholarship-2025".to_string(),
            name: "Microsoft Scholarship Program 2025".to_string(),
            provider: "Microsoft Corporation".to_string(),
            country: "Multiple Countries".to_string(),
            scholarship_type: ScholarshipType::PartialScholarship,
            degree_levels: vec![DegreeLevel::Bachelor, DegreeLevel::Master],
            fields_of_study: vec!["Computer Science".to_string(), "Engineering".to_string(), "Mathematics".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.0,
                required_degree: None,
                age_limit: None,
                nationalities: vec![],
                required_skills: vec!["Programming".to_string(), "English".to_string()],
                work_experience_years: None,
            },
            benefits: Benefits {
                tuition_coverage: Coverage::None,
                living_allowance: Some(12000),
                travel_allowance: false,
                health_insurance: false,
                duration_months: 12,
            },
            deadline: 1775001600, // 2026-03-31
            application_url: "https://microsoft.com/scholarships".to_string(),
            description: "Microsoft scholarship for underrepresented groups in technology".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "adobe-research-2025".to_string(),
            name: "Adobe Research Scholarship 2025".to_string(),
            provider: "Adobe Inc.".to_string(),
            country: "Multiple Countries".to_string(),
            scholarship_type: ScholarshipType::PartialScholarship,
            degree_levels: vec![DegreeLevel::Master, DegreeLevel::PhD],
            fields_of_study: vec!["Computer Science".to_string(), "Design".to_string(), "Digital Media".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.5,
                required_degree: Some(DegreeLevel::Bachelor),
                age_limit: None,
                nationalities: vec![],
                required_skills: vec!["Research".to_string(), "Programming".to_string(), "English".to_string()],
                work_experience_years: None,
            },
            benefits: Benefits {
                tuition_coverage: Coverage::None,
                living_allowance: Some(10000),
                travel_allowance: true,
                health_insurance: false,
                duration_months: 12,
            },
            deadline: 1772323200, // 2026-02-28
            application_url: "https://research.adobe.com".to_string(),
            description: "Research scholarship for students in computer science and design".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "aws-machine-learning-2025".to_string(),
            name: "AWS Machine Learning Scholarship 2025".to_string(),
            provider: "Amazon Web Services".to_string(),
            country: "Multiple Countries".to_string(),
            scholarship_type: ScholarshipType::PartialScholarship,
            degree_levels: vec![DegreeLevel::Bachelor, DegreeLevel::Master],
            fields_of_study: vec!["Machine Learning".to_string(), "Artificial Intelligence".to_string(), "Data Science".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.0,
                required_degree: None,
                age_limit: None,
                nationalities: vec![],
                required_skills: vec!["Programming".to_string(), "Mathematics".to_string(), "English".to_string()],
                work_experience_years: None,
            },
            benefits: Benefits {
                tuition_coverage: Coverage::None,
                living_allowance: Some(4000),
                travel_allowance: false,
                health_insurance: false,
                duration_months: 12,
            },
            deadline: 1769817600, // 2026-01-31
            application_url: "https://aws.amazon.com/scholarships".to_string(),
            description: "Scholarship and training program for machine learning students".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "nvidia-graduate-fellowship-2025".to_string(),
            name: "NVIDIA Graduate Fellowship 2025".to_string(),
            provider: "NVIDIA Corporation".to_string(),
            country: "Multiple Countries".to_string(),
            scholarship_type: ScholarshipType::FullScholarship,
            degree_levels: vec![DegreeLevel::PhD],
            fields_of_study: vec!["Computer Science".to_string(), "Engineering".to_string(), "AI".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.7,
                required_degree: Some(DegreeLevel::Master),
                age_limit: None,
                nationalities: vec![],
                required_skills: vec!["Research".to_string(), "GPU Computing".to_string(), "English".to_string()],
                work_experience_years: None,
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Full,
                living_allowance: Some(50000),
                travel_allowance: true,
                health_insurance: true,
                duration_months: 12,
            },
            deadline: 1767225600, // 2026-01-01
            application_url: "https://nvidia.com/fellowships".to_string(),
            description: "Fellowship for outstanding PhD students in AI and computing".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "facebook-fellowship-2025".to_string(),
            name: "Meta PhD Fellowship 2025".to_string(),
            provider: "Meta Platforms Inc.".to_string(),
            country: "Multiple Countries".to_string(),
            scholarship_type: ScholarshipType::FullScholarship,
            degree_levels: vec![DegreeLevel::PhD],
            fields_of_study: vec!["Computer Science".to_string(), "AI".to_string(), "Robotics".to_string(), "VR/AR".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.5,
                required_degree: Some(DegreeLevel::Bachelor),
                age_limit: None,
                nationalities: vec![],
                required_skills: vec!["Research".to_string(), "Programming".to_string(), "English".to_string()],
                work_experience_years: None,
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Full,
                living_allowance: Some(42000),
                travel_allowance: true,
                health_insurance: false,
                duration_months: 24,
            },
            deadline: 1769817600, // 2026-01-31
            application_url: "https://research.facebook.com".to_string(),
            description: "Fellowship for PhD students working on cutting-edge research".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "apple-scholars-2025".to_string(),
            name: "Apple Scholars in AI/ML PhD Fellowship 2025".to_string(),
            provider: "Apple Inc.".to_string(),
            country: "Multiple Countries".to_string(),
            scholarship_type: ScholarshipType::FullScholarship,
            degree_levels: vec![DegreeLevel::PhD],
            fields_of_study: vec!["Machine Learning".to_string(), "AI".to_string(), "Computer Vision".to_string(), "NLP".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.7,
                required_degree: Some(DegreeLevel::Master),
                age_limit: None,
                nationalities: vec![],
                required_skills: vec!["Machine Learning".to_string(), "Research".to_string(), "English".to_string()],
                work_experience_years: None,
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Full,
                living_allowance: Some(60000),
                travel_allowance: true,
                health_insurance: true,
                duration_months: 24,
            },
            deadline: 1769040000, // 2026-01-22
            application_url: "https://machinelearning.apple.com".to_string(),
            description: "Fellowship for exceptional PhD students in AI and machine learning".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "spacex-internship-2025".to_string(),
            name: "SpaceX Scholarship and Internship 2025".to_string(),
            provider: "SpaceX".to_string(),
            country: "United States".to_string(),
            scholarship_type: ScholarshipType::PartialScholarship,
            degree_levels: vec![DegreeLevel::Bachelor, DegreeLevel::Master],
            fields_of_study: vec!["Aerospace Engineering".to_string(), "Mechanical Engineering".to_string(), "Electrical Engineering".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.5,
                required_degree: None,
                age_limit: None,
                nationalities: vec!["United States".to_string()],
                required_skills: vec!["Engineering".to_string(), "Problem Solving".to_string(), "English".to_string()],
                work_experience_years: None,
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Partial(50),
                living_allowance: Some(8000),
                travel_allowance: false,
                health_insurance: false,
                duration_months: 12,
            },
            deadline: 1775001600, // 2026-03-31
            application_url: "https://spacex.com/careers".to_string(),
            description: "Scholarship and internship program for future space engineers".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "tesla-scholarship-2025".to_string(),
            name: "Tesla STEM Scholarship 2025".to_string(),
            provider: "Tesla Inc.".to_string(),
            country: "Multiple Countries".to_string(),
            scholarship_type: ScholarshipType::PartialScholarship,
            degree_levels: vec![DegreeLevel::Bachelor, DegreeLevel::Master],
            fields_of_study: vec!["Engineering".to_string(), "Computer Science".to_string(), "Physics".to_string(), "Mathematics".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.5,
                required_degree: None,
                age_limit: None,
                nationalities: vec![],
                required_skills: vec!["STEM".to_string(), "Innovation".to_string(), "English".to_string()],
                work_experience_years: None,
            },
            benefits: Benefits {
                tuition_coverage: Coverage::None,
                living_allowance: Some(15000),
                travel_allowance: false,
                health_insurance: false,
                duration_months: 12,
            },
            deadline: 1772323200, // 2026-02-28
            application_url: "https://tesla.com/scholarships".to_string(),
            description: "Scholarship for students passionate about sustainable technology".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "who-tdr-2025".to_string(),
            name: "WHO TDR Postgraduate Training Scheme 2025".to_string(),
            provider: "World Health Organization".to_string(),
            country: "Multiple Countries".to_string(),
            scholarship_type: ScholarshipType::FullScholarship,
            degree_levels: vec![DegreeLevel::Master, DegreeLevel::PhD],
            fields_of_study: vec!["Public Health".to_string(), "Medicine".to_string(), "Epidemiology".to_string(), "Tropical Medicine".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.0,
                required_degree: Some(DegreeLevel::Bachelor),
                age_limit: Some(40),
                nationalities: vec!["Indonesia".to_string(), "Philippines".to_string(), "India".to_string()],
                required_skills: vec!["English".to_string(), "Research".to_string()],
                work_experience_years: Some(2),
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Full,
                living_allowance: Some(1800),
                travel_allowance: true,
                health_insurance: true,
                duration_months: 24,
            },
            deadline: 1775001600, // 2026-03-31
            application_url: "https://who.int/tdr".to_string(),
            description: "WHO training scheme for health research in developing countries".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "un-women-scholarship-2025".to_string(),
            name: "UN Women Scholarship 2025".to_string(),
            provider: "United Nations Women".to_string(),
            country: "Multiple Countries".to_string(),
            scholarship_type: ScholarshipType::FullScholarship,
            degree_levels: vec![DegreeLevel::Master, DegreeLevel::PhD],
            fields_of_study: vec!["Gender Studies".to_string(), "Human Rights".to_string(), "International Relations".to_string(), "Social Work".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.3,
                required_degree: Some(DegreeLevel::Bachelor),
                age_limit: None,
                nationalities: vec![], // All nationalities
                required_skills: vec!["English".to_string(), "Gender Advocacy".to_string(), "Research".to_string()],
                work_experience_years: Some(2),
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Full,
                living_allowance: Some(1800),
                travel_allowance: true,
                health_insurance: true,
                duration_months: 24,
            },
            deadline: 1772323200, // 2026-02-28
            application_url: "https://unwomen.org/scholarships".to_string(),
            description: "Scholarship promoting women's leadership and gender equality".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "unicef-innovation-2025".to_string(),
            name: "UNICEF Innovation Fellowship 2025".to_string(),
            provider: "United Nations Children's Fund".to_string(),
            country: "Multiple Countries".to_string(),
            scholarship_type: ScholarshipType::ResearchGrant,
            degree_levels: vec![DegreeLevel::Master, DegreeLevel::PhD],
            fields_of_study: vec!["Child Development".to_string(), "Education Technology".to_string(), "Public Health".to_string(), "Social Innovation".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.4,
                required_degree: Some(DegreeLevel::Bachelor),
                age_limit: Some(35),
                nationalities: vec![], // All nationalities
                required_skills: vec!["English".to_string(), "Innovation".to_string(), "Project Management".to_string()],
                work_experience_years: Some(1),
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Partial(80),
                living_allowance: Some(2000),
                travel_allowance: true,
                health_insurance: true,
                duration_months: 18,
            },
            deadline: 1769817600, // 2026-01-31
            application_url: "https://unicef.org/innovation".to_string(),
            description: "Fellowship for innovative solutions to children's challenges".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "mit-presidential-fellowship-2025".to_string(),
            name: "MIT Presidential Fellowship 2025".to_string(),
            provider: "Massachusetts Institute of Technology".to_string(),
            country: "United States".to_string(),
            scholarship_type: ScholarshipType::FullScholarship,
            degree_levels: vec![DegreeLevel::PhD],
            fields_of_study: vec!["Engineering".to_string(), "Computer Science".to_string(), "Physics".to_string(), "Mathematics".to_string(), "Artificial Intelligence".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.8,
                required_degree: Some(DegreeLevel::Master),
                age_limit: None,
                nationalities: vec![], // All nationalities
                required_skills: vec!["Research".to_string(), "English".to_string(), "Academic Excellence".to_string()],
                work_experience_years: None,
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Full,
                living_allowance: Some(4000),
                travel_allowance: true,
                health_insurance: true,
                duration_months: 48,
            },
            deadline: 1764633600, // 2025-12-02
            application_url: "https://mit.edu/fellowships".to_string(),
            description: "Prestigious PhD fellowship for exceptional students in STEM".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "stanford-knight-hennessy-2025".to_string(),
            name: "Stanford Knight-Hennessy Scholars 2025".to_string(),
            provider: "Stanford University".to_string(),
            country: "United States".to_string(),
            scholarship_type: ScholarshipType::FullScholarship,
            degree_levels: vec![DegreeLevel::Master, DegreeLevel::PhD],
            fields_of_study: vec!["Business".to_string(), "Engineering".to_string(), "Medicine".to_string(), "Law".to_string(), "Education".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.7,
                required_degree: Some(DegreeLevel::Bachelor),
                age_limit: None,
                nationalities: vec![], // All nationalities
                required_skills: vec!["Leadership".to_string(), "English".to_string(), "Innovation".to_string()],
                work_experience_years: None,
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Full,
                living_allowance: Some(3500),
                travel_allowance: true,
                health_insurance: true,
                duration_months: 36,
            },
            deadline: 1759276800, // 2025-10-01
            application_url: "https://knight-hennessy.stanford.edu".to_string(),
            description: "Stanford's flagship fellowship for future global leaders".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "harvard-nieman-fellowship-2025".to_string(),
            name: "Harvard Nieman Fellowship 2025".to_string(),
            provider: "Harvard University".to_string(),
            country: "United States".to_string(),
            scholarship_type: ScholarshipType::ExchangeProgram,
            degree_levels: vec![DegreeLevel::Master],
            fields_of_study: vec!["Journalism".to_string(), "Media Studies".to_string(), "Communications".to_string(), "Digital Media".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.2,
                required_degree: Some(DegreeLevel::Bachelor),
                age_limit: None,
                nationalities: vec![], // All nationalities
                required_skills: vec!["Journalism".to_string(), "English".to_string(), "Writing".to_string()],
                work_experience_years: Some(5),
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Full,
                living_allowance: Some(2800),
                travel_allowance: true,
                health_insurance: true,
                duration_months: 10,
            },
            deadline: 1767225600, // 2026-01-01
            application_url: "https://nieman.harvard.edu".to_string(),
            description: "Fellowship for accomplished journalists to study at Harvard".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "oxford-weidenfeld-hoffmann-2025".to_string(),
            name: "Oxford Weidenfeld-Hoffmann Scholarship 2025".to_string(),
            provider: "University of Oxford".to_string(),
            country: "United Kingdom".to_string(),
            scholarship_type: ScholarshipType::FullScholarship,
            degree_levels: vec![DegreeLevel::Master],
            fields_of_study: vec!["Leadership Studies".to_string(), "Public Policy".to_string(), "International Development".to_string(), "Politics".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.6,
                required_degree: Some(DegreeLevel::Bachelor),
                age_limit: Some(29),
                nationalities: vec![], // Developing countries
                required_skills: vec!["English".to_string(), "Leadership".to_string(), "Public Service".to_string()],
                work_experience_years: Some(2),
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Full,
                living_allowance: Some(2200),
                travel_allowance: true,
                health_insurance: true,
                duration_months: 12,
            },
            deadline: 1769817600, // 2026-01-31
            application_url: "https://weidenfeld-hoffmann.ox.ac.uk".to_string(),
            description: "Oxford scholarship for future leaders from developing countries".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "cambridge-trust-scholarship-2025".to_string(),
            name: "Cambridge Trust International Scholarship 2025".to_string(),
            provider: "University of Cambridge".to_string(),
            country: "United Kingdom".to_string(),
            scholarship_type: ScholarshipType::PartialScholarship,
            degree_levels: vec![DegreeLevel::Master, DegreeLevel::PhD],
            fields_of_study: vec!["Natural Sciences".to_string(), "Engineering".to_string(), "Mathematics".to_string(), "Medicine".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.5,
                required_degree: Some(DegreeLevel::Bachelor),
                age_limit: None,
                nationalities: vec![], // All nationalities
                required_skills: vec!["English".to_string(), "Research".to_string(), "Academic Excellence".to_string()],
                work_experience_years: None,
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Partial(75),
                living_allowance: Some(1500),
                travel_allowance: false,
                health_insurance: true,
                duration_months: 24,
            },
            deadline: 1767225600, // 2026-01-01
            application_url: "https://cambridgetrust.org".to_string(),
            description: "Cambridge scholarship for outstanding international students".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "princeton-fellowship-2025".to_string(),
            name: "Princeton Graduate Fellowship 2025".to_string(),
            provider: "Princeton University".to_string(),
            country: "United States".to_string(),
            scholarship_type: ScholarshipType::FullScholarship,
            degree_levels: vec![DegreeLevel::PhD],
            fields_of_study: vec!["Economics".to_string(), "Public Policy".to_string(), "Political Science".to_string(), "International Affairs".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.7,
                required_degree: Some(DegreeLevel::Master),
                age_limit: None,
                nationalities: vec![], // All nationalities
                required_skills: vec!["Research".to_string(), "English".to_string(), "Analytical Thinking".to_string()],
                work_experience_years: None,
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Full,
                living_allowance: Some(3800),
                travel_allowance: true,
                health_insurance: true,
                duration_months: 60,
            },
            deadline: 1764633600, // 2025-12-02
            application_url: "https://princeton.edu/gradschool".to_string(),
            description: "Princeton's premier PhD fellowship for social sciences".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "yale-world-fellows-2025".to_string(),
            name: "Yale World Fellows 2025".to_string(),
            provider: "Yale University".to_string(),
            country: "United States".to_string(),
            scholarship_type: ScholarshipType::ExchangeProgram,
            degree_levels: vec![DegreeLevel::Master],
            fields_of_study: vec!["Global Affairs".to_string(), "Leadership".to_string(), "Public Administration".to_string(), "Social Entrepreneurship".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.4,
                required_degree: Some(DegreeLevel::Bachelor),
                age_limit: Some(39),
                nationalities: vec![], // All nationalities
                required_skills: vec!["English".to_string(), "Leadership".to_string(), "Global Perspective".to_string()],
                work_experience_years: Some(5),
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Full,
                living_allowance: Some(3000),
                travel_allowance: true,
                health_insurance: true,
                duration_months: 6,
            },
            deadline: 1767830400, // 2026-01-08
            application_url: "https://worldfellows.yale.edu".to_string(),
            description: "Yale's flagship leadership program for global professionals".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "caltech-phd-fellowship-2025".to_string(),
            name: "Caltech PhD Fellowship 2025".to_string(),
            provider: "California Institute of Technology".to_string(),
            country: "United States".to_string(),
            scholarship_type: ScholarshipType::FullScholarship,
            degree_levels: vec![DegreeLevel::PhD],
            fields_of_study: vec!["Physics".to_string(), "Chemistry".to_string(), "Astronomy".to_string(), "Engineering".to_string(), "Applied Mathematics".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.8,
                required_degree: Some(DegreeLevel::Bachelor),
                age_limit: None,
                nationalities: vec![], // All nationalities
                required_skills: vec!["Research".to_string(), "Mathematics".to_string(), "English".to_string()],
                work_experience_years: None,
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Full,
                living_allowance: Some(3600),
                travel_allowance: true,
                health_insurance: true,
                duration_months: 60,
            },
            deadline: 1764633600, // 2025-12-02
            application_url: "https://caltech.edu/gradstudies".to_string(),
            description: "Caltech's premier PhD fellowship in physical sciences".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "eth-zurich-excellence-2025".to_string(),
            name: "ETH Zurich Excellence Scholarship 2025".to_string(),
            provider: "ETH Zurich".to_string(),
            country: "Switzerland".to_string(),
            scholarship_type: ScholarshipType::PartialScholarship,
            degree_levels: vec![DegreeLevel::Master],
            fields_of_study: vec!["Engineering".to_string(), "Computer Science".to_string(), "Architecture".to_string(), "Natural Sciences".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.7,
                required_degree: Some(DegreeLevel::Bachelor),
                age_limit: None,
                nationalities: vec![], // All nationalities
                required_skills: vec!["English".to_string(), "German".to_string(), "Technical Excellence".to_string()],
                work_experience_years: None,
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Partial(80),
                living_allowance: Some(1800),
                travel_allowance: false,
                health_insurance: true,
                duration_months: 24,
            },
            deadline: 1764633600, // 2025-12-02
            application_url: "https://ethz.ch/excellence-scholarship".to_string(),
            description: "ETH Zurich scholarship for outstanding engineering students".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "kaist-scholarship-2025".to_string(),
            name: "KAIST Global IT Technology Scholarship 2025".to_string(),
            provider: "Korea Advanced Institute of Science and Technology".to_string(),
            country: "South Korea".to_string(),
            scholarship_type: ScholarshipType::FullScholarship,
            degree_levels: vec![DegreeLevel::Master, DegreeLevel::PhD],
            fields_of_study: vec!["Information Technology".to_string(), "Computer Science".to_string(), "Artificial Intelligence".to_string(), "Robotics".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.3,
                required_degree: Some(DegreeLevel::Bachelor),
                age_limit: Some(30),
                nationalities: vec![], // All nationalities
                required_skills: vec!["Programming".to_string(), "English".to_string(), "Korean".to_string()],
                work_experience_years: None,
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Full,
                living_allowance: Some(1000),
                travel_allowance: true,
                health_insurance: true,
                duration_months: 24,
            },
            deadline: 1772323200, // 2026-02-28
            application_url: "https://kaist.ac.kr/admission".to_string(),
            description: "KAIST scholarship for international students in technology".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "tsinghua-scholarship-2025".to_string(),
            name: "Tsinghua University Scholarship 2025".to_string(),
            provider: "Tsinghua University".to_string(),
            country: "China".to_string(),
            scholarship_type: ScholarshipType::PartialScholarship,
            degree_levels: vec![DegreeLevel::Master, DegreeLevel::PhD],
            fields_of_study: vec!["Engineering".to_string(), "Business Administration".to_string(), "Public Policy".to_string(), "Architecture".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.2,
                required_degree: Some(DegreeLevel::Bachelor),
                age_limit: Some(35),
                nationalities: vec![], // All nationalities
                required_skills: vec!["Chinese".to_string(), "English".to_string(), "Academic Excellence".to_string()],
                work_experience_years: None,
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Partial(70),
                living_allowance: Some(800),
                travel_allowance: false,
                health_insurance: true,
                duration_months: 24,
            },
            deadline: 1772323200, // 2026-02-28
            application_url: "https://tsinghua.edu.cn/admission".to_string(),
            description: "Tsinghua scholarship for international graduate students".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "university-of-tokyo-scholarship-2025".to_string(),
            name: "University of Tokyo MEXT Scholarship 2025".to_string(),
            provider: "University of Tokyo".to_string(),
            country: "Japan".to_string(),
            scholarship_type: ScholarshipType::FullScholarship,
            degree_levels: vec![DegreeLevel::Master, DegreeLevel::PhD],
            fields_of_study: vec!["Engineering".to_string(), "Natural Sciences".to_string(), "Medicine".to_string(), "Economics".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.4,
                required_degree: Some(DegreeLevel::Bachelor),
                age_limit: Some(35),
                nationalities: vec![], // All nationalities
                required_skills: vec!["Japanese".to_string(), "English".to_string(), "Research".to_string()],
                work_experience_years: None,
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Full,
                living_allowance: Some(1400),
                travel_allowance: true,
                health_insurance: true,
                duration_months: 24,
            },
            deadline: 1767225600, // 2026-01-01
            application_url: "https://u-tokyo.ac.jp/admission".to_string(),
            description: "University of Tokyo scholarship for international graduate students".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "australian-national-university-2025".to_string(),
            name: "ANU Chancellor's International Scholarship 2025".to_string(),
            provider: "Australian National University".to_string(),
            country: "Australia".to_string(),
            scholarship_type: ScholarshipType::PartialScholarship,
            degree_levels: vec![DegreeLevel::Master, DegreeLevel::PhD],
            fields_of_study: vec!["Political Science".to_string(), "International Relations".to_string(), "Public Policy".to_string(), "Economics".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.5,
                required_degree: Some(DegreeLevel::Bachelor),
                age_limit: None,
                nationalities: vec![], // All nationalities
                required_skills: vec!["English".to_string(), "Research".to_string(), "Critical Thinking".to_string()],
                work_experience_years: None,
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Partial(50),
                living_allowance: Some(1200),
                travel_allowance: false,
                health_insurance: true,
                duration_months: 24,
            },
            deadline: 1772323200, // 2026-02-28
            application_url: "https://anu.edu.au/scholarships".to_string(),
            description: "ANU scholarship for outstanding international students".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "mcgill-university-scholarship-2025".to_string(),
            name: "McGill University Graduate Excellence Award 2025".to_string(),
            provider: "McGill University".to_string(),
            country: "Canada".to_string(),
            scholarship_type: ScholarshipType::PartialScholarship,
            degree_levels: vec![DegreeLevel::Master, DegreeLevel::PhD],
            fields_of_study: vec!["Medicine".to_string(), "Engineering".to_string(), "Natural Sciences".to_string(), "Management".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.6,
                required_degree: Some(DegreeLevel::Bachelor),
                age_limit: None,
                nationalities: vec![], // All nationalities
                required_skills: vec!["English".to_string(), "French".to_string(), "Research".to_string()],
                work_experience_years: None,
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Partial(60),
                living_allowance: Some(1800),
                travel_allowance: false,
                health_insurance: true,
                duration_months: 24,
            },
            deadline: 1769817600, // 2026-01-31
            application_url: "https://mcgill.ca/gradapplicants/funding".to_string(),
            description: "McGill scholarship for exceptional graduate students".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "sorbonne-university-scholarship-2025".to_string(),
            name: "Sorbonne University International Scholarship 2025".to_string(),
            provider: "Sorbonne University".to_string(),
            country: "France".to_string(),
            scholarship_type: ScholarshipType::PartialScholarship,
            degree_levels: vec![DegreeLevel::Master, DegreeLevel::PhD],
            fields_of_study: vec!["Literature".to_string(), "Philosophy".to_string(), "History".to_string(), "Arts".to_string(), "Sciences".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.3,
                required_degree: Some(DegreeLevel::Bachelor),
                age_limit: Some(30),
                nationalities: vec![], // All nationalities
                required_skills: vec!["French".to_string(), "English".to_string(), "Academic Writing".to_string()],
                work_experience_years: None,
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Partial(70),
                living_allowance: Some(1200),
                travel_allowance: false,
                health_insurance: true,
                duration_months: 24,
            },
            deadline: 1775001600, // 2026-03-31
            application_url: "https://sorbonne-universite.fr/admission".to_string(),
            description: "Sorbonne scholarship for international students in humanities and sciences".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "technical-university-munich-2025".to_string(),
            name: "TUM Global Incentive Scholarship 2025".to_string(),
            provider: "Technical University of Munich".to_string(),
            country: "Germany".to_string(),
            scholarship_type: ScholarshipType::PartialScholarship,
            degree_levels: vec![DegreeLevel::Master],
            fields_of_study: vec!["Engineering".to_string(), "Technology Management".to_string(), "Computer Science".to_string(), "Natural Sciences".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.5,
                required_degree: Some(DegreeLevel::Bachelor),
                age_limit: None,
                nationalities: vec![], // All nationalities
                required_skills: vec!["German".to_string(), "English".to_string(), "Technical Skills".to_string()],
                work_experience_years: None,
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Partial(60),
                living_allowance: Some(600),
                travel_allowance: false,
                health_insurance: true,
                duration_months: 24,
            },
            deadline: 1769817600, // 2026-01-31
            application_url: "https://tum.de/scholarships".to_string(),
            description: "TUM scholarship for international students in engineering and technology".to_string(),
            created_at: ic_cdk::api::time(),
        },
        Scholarship {
            id: "rockefeller-foundation-fellowship-2025".to_string(),
            name: "Rockefeller Foundation Food System Vision Prize 2025".to_string(),
            provider: "Rockefeller Foundation".to_string(),
            country: "Multiple Countries".to_string(),
            scholarship_type: ScholarshipType::ResearchGrant,
            degree_levels: vec![DegreeLevel::Master, DegreeLevel::PhD],
            fields_of_study: vec!["Agricultural Sciences".to_string(), "Food Technology".to_string(), "Nutrition".to_string(), "Environmental Science".to_string()],
            eligibility: Eligibility {
                min_gpa: 3.4,
                required_degree: Some(DegreeLevel::Bachelor),
                age_limit: None,
                nationalities: vec![], // All nationalities
                required_skills: vec!["English".to_string(), "Research".to_string(), "Innovation".to_string()],
                work_experience_years: Some(2),
            },
            benefits: Benefits {
                tuition_coverage: Coverage::Full,
                living_allowance: Some(2500),
                travel_allowance: true,
                health_insurance: true,
                duration_months: 18,
            },
            deadline: 1775001600, // 2026-03-31
            application_url: "https://rockefellerfoundation.org/fellowships".to_string(),
            description: "Research grant for innovative food system solutions".to_string(),
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
    
    // Debug logging
    ic_cdk::println!("🔍 Register attempt from Principal: {}", caller.to_text());
    ic_cdk::println!("📝 Registration data - Name: {}, Email: {}", name, email);
    
    // Check if user already exists
    if storage::get_user(&caller).is_some() {
        ic_cdk::println!("❌ User {} already exists", caller.to_text());
        return format!("User {} already registered", caller.to_text());
    }

    let user = UserProfile {
        id: caller,
        name: name.clone(),
        email,
        education,
        skills,
        preferences,
        created_at: ic_cdk::api::time(),
        updated_at: ic_cdk::api::time(),
    };

    match storage::insert_user(user) {
        Ok(_) => {
            ic_cdk::println!("✅ User {} registered successfully with name: {}", caller.to_text(), name);
            format!("User {} registered successfully with name: {}!", caller.to_text(), name)
        },
        Err(_) => {
            ic_cdk::println!("❌ Registration failed for {}", caller.to_text());
            "Registration failed".to_string()
        },
    }
}

#[query]
fn get_my_profile() -> String {
    let caller = ic_cdk::caller();
    
    // Debug logging
    ic_cdk::println!("🔍 Profile request from Principal: {}", caller.to_text());
    
    match storage::get_user(&caller) {
        Some(user) => {
            ic_cdk::println!("✅ Found profile for {}: {}", caller.to_text(), user.name);
            // Return a simple JSON-like string that won't cause Candid issues
            format!("{{\"principal\":\"{}\",\"name\":\"{}\",\"email\":\"{}\",\"major\":\"{}\",\"university\":\"{}\"}}",
                caller.to_text(), user.name, user.email, user.education.major, user.education.university)
        },
        None => {
            ic_cdk::println!("❌ No profile found for Principal: {}", caller.to_text());
            format!("{{\"error\":\"User not found\",\"principal\":\"{}\"}}", caller.to_text())
        },
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
        // Return JSON-like structured data for easier parsing
        let mut result = format!("{{\"count\":{},\"scholarships\":[", scholarships.len());
        for (i, scholarship) in scholarships.iter().enumerate() {
            if i > 0 {
                result.push(',');
            }
            result.push_str(&format!(
                "{{\"id\":\"{}\",\"name\":\"{}\",\"provider\":\"{}\",\"country\":\"{}\",\"type\":\"{:?}\",\"fields\":[{}],\"degree_levels\":[{}],\"deadline\":{},\"description\":\"{}\"}}",
                scholarship.id,
                scholarship.name,
                scholarship.provider,
                scholarship.country,
                scholarship.scholarship_type,
                scholarship.fields_of_study.iter().map(|f| format!("\"{}\"", f)).collect::<Vec<_>>().join(","),
                scholarship.degree_levels.iter().map(|d| format!("\"{:?}\"", d)).collect::<Vec<_>>().join(","),
                scholarship.deadline,
                scholarship.description
            ));
        }
        result.push_str("]}");
        result
    }
}

#[query]
fn get_scholarships_data() -> Vec<Scholarship> {
    storage::get_all_scholarships()
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
        // Return matches without "Top Recommendations:" prefix
        top_matches.join(", ")
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
        // Return matches without "Your matches:" prefix
        good_matches.join(", ")
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
