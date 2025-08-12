"""
Scholarship Matching Agent using Fetch.ai uAgents
This agent handles intelligent matching between students and scholarships
"""

from uagents import Agent, Context, Model, Protocol
from uagents.setup import fund_agent_if_low
from typing import List, Dict, Optional
import json
import logging
from datetime import datetime

# Configure logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

# ============= Models =============

class UserProfile(Model):
    """User profile data model"""
    user_id: str
    name: str
    email: str
    gpa: float
    degree_level: str
    major: str
    university: str
    country: str
    skills: List[str]
    preferred_countries: List[str]
    preferred_fields: List[str]
    graduation_year: int

class Scholarship(Model):
    """Scholarship data model"""
    scholarship_id: str
    name: str
    provider: str
    country: str
    degree_levels: List[str]
    fields_of_study: List[str]
    min_gpa: float
    required_skills: List[str]
    deadline: str
    benefits: Dict

class MatchRequest(Model):
    """Request model for matching"""
    user_profile: UserProfile
    scholarships: List[Scholarship]
    top_n: Optional[int] = 5

class MatchResult(Model):
    """Individual match result"""
    scholarship_id: str
    user_id: str
    compatibility_score: float
    matched_criteria: List[str]
    missing_criteria: List[str]
    recommendation_reason: str

class MatchResponse(Model):
    """Response model with match results"""
    user_id: str
    matches: List[MatchResult]
    processing_time: float
    timestamp: str

# ============= Matching Engine =============

class ScholarshipMatcher:
    """Core matching logic"""
    
    @staticmethod
    def calculate_compatibility(user: UserProfile, scholarship: Scholarship) -> MatchResult:
        """Calculate compatibility score between user and scholarship"""
        
        score = 0.0
        matched_criteria = []
        missing_criteria = []
        
        # Academic Match (30%)
        academic_score = 0.0
        if user.gpa >= scholarship.min_gpa:
            academic_score = 100.0
            matched_criteria.append(f"GPA {user.gpa} >= {scholarship.min_gpa}")
        else:
            missing_criteria.append(f"GPA {user.gpa} < {scholarship.min_gpa}")
            # Partial score based on how close
            academic_score = (user.gpa / scholarship.min_gpa) * 100.0
        
        if user.degree_level in scholarship.degree_levels:
            matched_criteria.append(f"Degree level: {user.degree_level}")
        else:
            missing_criteria.append(f"Degree level {user.degree_level} not eligible")
            academic_score *= 0.5
        
        score += academic_score * 0.3
        
        # Field Match (25%)
        field_score = 0.0
        user_field_lower = user.major.lower()
        matching_fields = [
            field for field in scholarship.fields_of_study
            if field.lower() in user_field_lower or user_field_lower in field.lower()
        ]
        
        if matching_fields:
            field_score = 100.0
            matched_criteria.append(f"Field match: {', '.join(matching_fields)}")
        elif "All Fields" in scholarship.fields_of_study:
            field_score = 100.0
            matched_criteria.append("All fields eligible")
        else:
            field_score = 0.0
            missing_criteria.append(f"Field {user.major} not in eligible fields")
        
        score += field_score * 0.25
        
        # Location Match (20%)
        location_score = 0.0
        if scholarship.country in user.preferred_countries:
            location_score = 100.0
            matched_criteria.append(f"Preferred country: {scholarship.country}")
        elif not user.preferred_countries:  # No preference
            location_score = 75.0
        else:
            location_score = 50.0  # Not preferred but not excluded
        
        score += location_score * 0.2
        
        # Skills Match (15%)
        skills_score = 0.0
        if scholarship.required_skills:
            matched_skills = [
                skill for skill in scholarship.required_skills
                if skill in user.skills
            ]
            
            if matched_skills:
                skills_score = (len(matched_skills) / len(scholarship.required_skills)) * 100.0
                matched_criteria.append(f"Skills: {', '.join(matched_skills)}")
            
            missing_skills = [
                skill for skill in scholarship.required_skills
                if skill not in user.skills
            ]
            if missing_skills:
                missing_criteria.append(f"Missing skills: {', '.join(missing_skills)}")
        else:
            skills_score = 100.0  # No skills required
        
        score += skills_score * 0.15
        
        # Deadline Factor (10%)
        # Check if deadline is approaching (mock check)
        deadline_score = 100.0  # Assume all deadlines are okay for hackathon
        score += deadline_score * 0.1
        
        # Generate recommendation reason
        reason = ScholarshipMatcher._generate_recommendation_reason(
            score, matched_criteria, missing_criteria
        )
        
        return MatchResult(
            scholarship_id=scholarship.scholarship_id,
            user_id=user.user_id,
            compatibility_score=round(score, 2),
            matched_criteria=matched_criteria,
            missing_criteria=missing_criteria,
            recommendation_reason=reason
        )
    
    @staticmethod
    def _generate_recommendation_reason(score: float, matched: List[str], missing: List[str]) -> str:
        """Generate human-readable recommendation reason"""
        
        if score >= 90:
            return "Excellent match! You meet all major requirements. Apply immediately!"
        elif score >= 75:
            return "Strong match! You meet most requirements. Good chance of success."
        elif score >= 60:
            return "Good match. Consider strengthening your application in missing areas."
        elif score >= 40:
            return "Moderate match. You meet some requirements but missing key criteria."
        else:
            return "Low match. Consider other opportunities or improve your profile."
    
    @staticmethod
    def get_top_matches(user: UserProfile, scholarships: List[Scholarship], top_n: int = 5) -> List[MatchResult]:
        """Get top N scholarship matches for a user"""
        
        matches = []
        for scholarship in scholarships:
            match_result = ScholarshipMatcher.calculate_compatibility(user, scholarship)
            matches.append(match_result)
        
        # Sort by score (highest first)
        matches.sort(key=lambda x: x.compatibility_score, reverse=True)
        
        return matches[:top_n]

# ============= Agent Setup =============

# Initialize the matching agent
matching_agent = Agent(
    name="scholarship_matcher",
    seed="scholarship_matching_seed_phrase_2024_hackathon",
    port=8001,
    endpoint=["http://localhost:8001/submit"]
)

# Fund agent if needed (for Fetch.ai network)
fund_agent_if_low(matching_agent.wallet.address())

# Create protocol for handling matching requests
matching_protocol = Protocol("ScholarshipMatching")

@matching_protocol.on_message(model=MatchRequest, replies=MatchResponse)
async def handle_match_request(ctx: Context, sender: str, msg: MatchRequest):
    """Handle incoming match requests"""
    
    start_time = datetime.now()
    
    logger.info(f"Received match request for user: {msg.user_profile.user_id}")
    logger.info(f"Processing {len(msg.scholarships)} scholarships")
    
    try:
        # Perform matching
        matches = ScholarshipMatcher.get_top_matches(
            msg.user_profile,
            msg.scholarships,
            msg.top_n or 5
        )
        
        # Calculate processing time
        processing_time = (datetime.now() - start_time).total_seconds()
        
        # Prepare response
        response = MatchResponse(
            user_id=msg.user_profile.user_id,
            matches=matches,
            processing_time=processing_time,
            timestamp=datetime.now().isoformat()
        )
        
        logger.info(f"Matching completed. Found {len(matches)} matches in {processing_time:.2f}s")
        
        # Send response back
        await ctx.send(sender, response)
        
    except Exception as e:
        logger.error(f"Error processing match request: {str(e)}")
        # Send error response
        error_response = MatchResponse(
            user_id=msg.user_profile.user_id,
            matches=[],
            processing_time=0.0,
            timestamp=datetime.now().isoformat()
        )
        await ctx.send(sender, error_response)

# Include protocol in agent
matching_agent.include(matching_protocol)

# ============= Standalone Testing Functions =============

def test_matching():
    """Test matching logic with sample data"""
    
    # Sample user
    test_user = UserProfile(
        user_id="test_user_001",
        name="John Doe",
        email="john@example.com",
        gpa=3.5,
        degree_level="Bachelor",
        major="Computer Science",
        university="MIT",
        country="Indonesia",
        skills=["Leadership", "Research", "English Proficiency"],
        preferred_countries=["United States", "United Kingdom"],
        preferred_fields=["Computer Science", "AI"],
        graduation_year=2024
    )
    
    # Sample scholarships
    test_scholarships = [
        Scholarship(
            scholarship_id="fulbright-2025",
            name="Fulbright Scholarship",
            provider="US Department of State",
            country="United States",
            degree_levels=["Bachelor", "Master"],
            fields_of_study=["All Fields"],
            min_gpa=3.0,
            required_skills=["English Proficiency", "Research Skills"],
            deadline="2025-10-01",
            benefits={"tuition": "full", "stipend": 1800}
        ),
        Scholarship(
            scholarship_id="chevening-2025",
            name="Chevening Scholarship",
            provider="UK Government",
            country="United Kingdom",
            degree_levels=["Master"],
            fields_of_study=["All Fields"],
            min_gpa=3.3,
            required_skills=["Leadership", "English Proficiency"],
            deadline="2025-11-01",
            benefits={"tuition": "full", "stipend": 1200}
        )
    ]
    
    # Get matches
    matches = ScholarshipMatcher.get_top_matches(test_user, test_scholarships)
    
    # Print results
    print("\n=== Matching Results ===")
    for match in matches:
        print(f"\nScholarship: {match.scholarship_id}")
        print(f"Score: {match.compatibility_score}%")
        print(f"Recommendation: {match.recommendation_reason}")
        print(f"Matched: {', '.join(match.matched_criteria)}")
        if match.missing_criteria:
            print(f"Missing: {', '.join(match.missing_criteria)}")

# ============= Main Entry Point =============

if __name__ == "__main__":
    print("Starting Scholarship Matching Agent...")
    print(f"Agent address: {matching_agent.address}")
    print("Listening for matching requests on port 8001")
    
    # Run test if needed
    # test_matching()
    
    # Run the agent
    matching_agent.run()