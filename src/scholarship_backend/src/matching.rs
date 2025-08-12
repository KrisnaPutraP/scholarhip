use crate::types::*;
use crate::storage;
use candid::Principal;

pub struct MatchingEngine;

impl MatchingEngine {
    /// Calculate compatibility score between a user and a scholarship
    pub fn calculate_compatibility(
        user: &UserProfile,
        scholarship: &Scholarship,
    ) -> MatchResult {
        let mut score = 0.0;
        let mut matched_criteria = Vec::new();
        let mut missing_criteria = Vec::new();

        // Academic Match (30%)
        let academic_score = Self::calculate_academic_match(user, scholarship, &mut matched_criteria, &mut missing_criteria);
        score += academic_score * 0.3;

        // Field Match (25%)
        let field_score = Self::calculate_field_match(user, scholarship, &mut matched_criteria, &mut missing_criteria);
        score += field_score * 0.25;

        // Location Match (20%)
        let location_score = Self::calculate_location_match(user, scholarship, &mut matched_criteria, &mut missing_criteria);
        score += location_score * 0.2;

        // Eligibility Match (15%)
        let eligibility_score = Self::calculate_eligibility_match(user, scholarship, &mut matched_criteria, &mut missing_criteria);
        score += eligibility_score * 0.15;

        // Skills Match (10%)
        let skills_score = Self::calculate_skills_match(user, scholarship, &mut matched_criteria, &mut missing_criteria);
        score += skills_score * 0.1;

        MatchResult {
            scholarship_id: scholarship.id.clone(),
            user_id: user.id,
            compatibility_score: score,
            matched_criteria,
            missing_criteria,
            matched_at: ic_cdk::api::time(),
        }
    }

    fn calculate_academic_match(
        user: &UserProfile,
        scholarship: &Scholarship,
        matched: &mut Vec<String>,
        missing: &mut Vec<String>,
    ) -> f32 {
        let mut score = 0.0;
        let mut total_weight = 0.0;

        // GPA Match (weight: 60%)
        if user.education.gpa >= scholarship.eligibility.min_gpa {
            score += 0.6;
            matched.push(format!("GPA: {} >= {}", user.education.gpa, scholarship.eligibility.min_gpa));
        } else {
            missing.push(format!("GPA: {} < required {}", user.education.gpa, scholarship.eligibility.min_gpa));
        }
        total_weight += 0.6;

        // Degree Level Match (weight: 40%)
        if scholarship.degree_levels.contains(&user.education.degree_level) {
            score += 0.4;
            matched.push(format!("Degree Level: {:?}", user.education.degree_level));
        } else {
            missing.push(format!("Degree Level: {:?} not eligible", user.education.degree_level));
        }
        total_weight += 0.4;

        score / total_weight
    }

    fn calculate_field_match(
        user: &UserProfile,
        scholarship: &Scholarship,
        matched: &mut Vec<String>,
        missing: &mut Vec<String>,
    ) -> f32 {
        // Check if user's major matches scholarship fields
        let user_field = user.education.major.to_lowercase();
        let matching_fields: Vec<String> = scholarship
            .fields_of_study
            .iter()
            .filter(|field| {
                field.to_lowercase().contains(&user_field) || 
                user_field.contains(&field.to_lowercase())
            })
            .cloned()
            .collect();

        if !matching_fields.is_empty() {
            matched.push(format!("Field of Study: {}", matching_fields.join(", ")));
            1.0
        } else if scholarship.fields_of_study.is_empty() {
            // No specific field requirement
            matched.push("Field of Study: All fields eligible".to_string());
            1.0
        } else {
            missing.push(format!("Field of Study: {} not in eligible fields", user.education.major));
            0.0
        }
    }

    fn calculate_location_match(
        user: &UserProfile,
        scholarship: &Scholarship,
        matched: &mut Vec<String>,
        _missing: &mut Vec<String>,
    ) -> f32 {
        // Check if scholarship country matches user preferences
        let user_countries = &user.preferences.preferred_countries;
        let scholarship_countries = vec![scholarship.country.clone()];
        let matching_countries: Vec<String> = user_countries
            .iter()
            .filter(|country| scholarship_countries.contains(country))
            .cloned()
            .collect();
            
        if user_countries.iter().any(|country| scholarship_countries.contains(country)) {
            matched.push(format!("Preferred Country: {}", matching_countries.join(", ")));
            1.0
        } else if user.preferences.preferred_countries.is_empty() {
            // User has no preference
            0.75
        } else {
            // Partial score for non-preferred but not excluded
            0.5
        }
    }

    fn calculate_eligibility_match(
        user: &UserProfile,
        scholarship: &Scholarship,
        matched: &mut Vec<String>,
        missing: &mut Vec<String>,
    ) -> f32 {
        let mut score = 0.0;
        let mut criteria_count = 0;

        // Nationality check
        if scholarship.eligibility.nationalities.is_empty() ||
           scholarship.eligibility.nationalities.iter().any(|n| n == "Indonesia" || n == "All") {
            score += 1.0;
            matched.push("Nationality: Eligible".to_string());
        } else {
            missing.push("Nationality: Not eligible".to_string());
        }
        criteria_count += 1;

        // Age limit check (if applicable)
        if let Some(age_limit) = scholarship.eligibility.age_limit {
            // Estimate age from graduation year
            let current_year = 2025;
            let estimated_age = current_year - user.education.graduation_year + 22;
            if estimated_age <= age_limit as u32 {
                score += 1.0;
                matched.push(format!("Age: {} <= {}", estimated_age, age_limit));
            } else {
                missing.push(format!("Age: {} > limit {}", estimated_age, age_limit));
            }
            criteria_count += 1;
        }

        // Work experience check (if applicable)
        if scholarship.eligibility.work_experience_years.is_some() {
            // For hackathon, assume user meets this if they have graduated
            let current_year = 2025;
            if current_year > user.education.graduation_year {
                score += 1.0;
                matched.push("Work Experience: Meets requirement".to_string());
            } else {
                missing.push("Work Experience: Insufficient".to_string());
            }
            criteria_count += 1;
        }

        if criteria_count > 0 {
            score / criteria_count as f32
        } else {
            1.0 // No additional eligibility criteria
        }
    }

    fn calculate_skills_match(
        user: &UserProfile,
        scholarship: &Scholarship,
        matched: &mut Vec<String>,
        missing: &mut Vec<String>,
    ) -> f32 {
        if scholarship.eligibility.required_skills.is_empty() {
            return 1.0;
        }

        let matched_skills: Vec<String> = scholarship
            .eligibility
            .required_skills
            .iter()
            .filter(|skill| user.skills.contains(skill))
            .cloned()
            .collect();

        let match_percentage = matched_skills.len() as f32 / scholarship.eligibility.required_skills.len() as f32;

        if !matched_skills.is_empty() {
            matched.push(format!("Skills: {}", matched_skills.join(", ")));
        }

        let missing_skills: Vec<String> = scholarship
            .eligibility
            .required_skills
            .iter()
            .filter(|skill| !user.skills.contains(skill))
            .cloned()
            .collect();

        if !missing_skills.is_empty() {
            missing.push(format!("Missing Skills: {}", missing_skills.join(", ")));
        }

        match_percentage
    }

    /// Get top N scholarship recommendations for a user
    pub fn get_recommendations(user_id: &Principal, top_n: usize) -> Vec<MatchResult> {
        let user = match storage::get_user(user_id) {
            Some(user) => user,
            None => return Vec::new(),
        };

        let scholarships = storage::get_all_scholarships();
        let mut matches: Vec<MatchResult> = scholarships
            .iter()
            .map(|scholarship| Self::calculate_compatibility(&user, scholarship))
            .filter(|m| m.compatibility_score > 30.0) // Minimum threshold
            .collect();

        // Sort by score (highest first)
        matches.sort_by(|a, b| b.compatibility_score.partial_cmp(&a.compatibility_score).unwrap());

        // Store matches for future reference
        for match_result in matches.iter().take(top_n) {
            let _ = storage::insert_match(match_result.clone());
        }

        matches.into_iter().take(top_n).collect()
    }

    /// Filter scholarships by specific criteria
    pub fn filter_scholarships(
        country: Option<String>,
        degree_level: Option<DegreeLevel>,
        field: Option<String>,
        _min_score: Option<f32>,
    ) -> Vec<Scholarship> {
        let all_scholarships = storage::get_all_scholarships();
        
        all_scholarships
            .into_iter()
            .filter(|s| {
                let mut pass = true;
                
                if let Some(ref c) = country {
                    pass = pass && s.country == *c;
                }
                
                if let Some(ref d) = degree_level {
                    pass = pass && s.degree_levels.contains(d);
                }
                
                if let Some(ref f) = field {
                    pass = pass && s.fields_of_study.iter().any(|sf| 
                        sf.to_lowercase().contains(&f.to_lowercase())
                    );
                }
                
                pass
            })
            .collect()
    }
}