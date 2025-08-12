"""
Scholarship Notification Agent using Fetch.ai uAgents
This agent handles sending notifications about scholarship matches and deadlines
"""

from uagents import Agent, Context, Model, Protocol
from uagents.setup import fund_agent_if_low
from typing import List, Dict, Optional
import json
import logging
from datetime import datetime, timedelta
import asyncio

# Configure logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

# ============= Models =============

class NotificationRequest(Model):
    """Notification request data model"""
    user_id: str
    message: str
    notification_type: str  # 'match', 'deadline', 'reminder'
    scholarship_id: Optional[str] = None
    urgency: str = 'normal'  # 'low', 'normal', 'high'

class UserPreferences(Model):
    """User notification preferences"""
    user_id: str
    email: str
    email_enabled: bool = True
    push_enabled: bool = True
    deadline_reminders: bool = True
    match_notifications: bool = True

class NotificationResponse(Model):
    """Response to notification request"""
    success: bool
    message: str
    notification_id: Optional[str] = None

class DeadlineAlert(Model):
    """Deadline alert model"""
    scholarship_id: str
    title: str
    deadline: str
    days_remaining: int
    user_ids: List[str]

# ============= Notification Service =============

class NotificationService:
    """Service for handling various types of notifications"""
    
    def __init__(self):
        self.user_preferences = {}
        self.notification_queue = []
    
    def add_user_preferences(self, preferences: UserPreferences):
        """Add or update user notification preferences"""
        self.user_preferences[preferences.user_id] = preferences
    
    async def send_email_notification(self, user_id: str, subject: str, body: str):
        """Send email notification (mock implementation)"""
        if user_id not in self.user_preferences:
            return False
        
        prefs = self.user_preferences[user_id]
        if not prefs.email_enabled:
            return False
        
        # In a real implementation, this would integrate with an email service
        logger.info(f"📧 EMAIL to {prefs.email}: {subject}")
        logger.info(f"   Body: {body}")
        return True
    
    async def send_push_notification(self, user_id: str, title: str, message: str):
        """Send push notification (mock implementation)"""
        if user_id not in self.user_preferences:
            return False
        
        prefs = self.user_preferences[user_id]
        if not prefs.push_enabled:
            return False
        
        # In a real implementation, this would integrate with a push service
        logger.info(f"📱 PUSH to {user_id}: {title}")
        logger.info(f"   Message: {message}")
        return True
    
    async def process_scholarship_match(self, user_id: str, scholarship_title: str, match_score: float):
        """Process scholarship match notification"""
        if user_id not in self.user_preferences:
            return False
        
        prefs = self.user_preferences[user_id]
        if not prefs.match_notifications:
            return False
        
        subject = f"New Scholarship Match Found! ({match_score}% compatibility)"
        body = f"""
        Great news! We found a scholarship that matches your profile:
        
        Scholarship: {scholarship_title}
        Compatibility Score: {match_score}%
        
        Log in to your scholarship portal to view details and apply.
        """
        
        # Send both email and push notification
        email_sent = await self.send_email_notification(user_id, subject, body.strip())
        push_sent = await self.send_push_notification(user_id, "New Match!", f"{scholarship_title} - {match_score}% match")
        
        return email_sent or push_sent
    
    async def process_deadline_reminder(self, alert: DeadlineAlert):
        """Process deadline reminder for multiple users"""
        success_count = 0
        
        for user_id in alert.user_ids:
            if user_id not in self.user_preferences:
                continue
                
            prefs = self.user_preferences[user_id]
            if not prefs.deadline_reminders:
                continue
            
            urgency = "🚨 URGENT" if alert.days_remaining <= 3 else "⏰ REMINDER"
            subject = f"{urgency}: {alert.title} deadline in {alert.days_remaining} days"
            body = f"""
            Deadline Alert for: {alert.title}
            
            Application Deadline: {alert.deadline}
            Days Remaining: {alert.days_remaining}
            
            Don't miss out on this opportunity! Complete your application today.
            """
            
            email_sent = await self.send_email_notification(user_id, subject, body.strip())
            push_sent = await self.send_push_notification(user_id, urgency, f"{alert.title} - {alert.days_remaining} days left")
            
            if email_sent or push_sent:
                success_count += 1
        
        return success_count

# ============= Agent Setup =============

# Initialize the notification agent
notification_agent = Agent(
    name="scholarship_notifications",
    seed="scholarship_notification_seed_phrase_2024_hackathon",
    port=8002,
    endpoint=["http://localhost:8002/submit"]
)

# Fund agent if needed (for Fetch.ai network)
fund_agent_if_low(notification_agent.wallet.address())

# Initialize notification service
notification_service = NotificationService()

# Create protocol for handling notification requests
notification_protocol = Protocol("ScholarshipNotifications")

@notification_protocol.on_message(model=NotificationRequest, replies=NotificationResponse)
async def handle_notification_request(ctx: Context, sender: str, msg: NotificationRequest):
    """Handle incoming notification requests"""
    try:
        logger.info(f"Received notification request from {sender}")
        logger.info(f"Type: {msg.notification_type}, User: {msg.user_id}")
        
        success = False
        notification_id = f"notif_{datetime.now().strftime('%Y%m%d_%H%M%S')}_{msg.user_id}"
        
        if msg.notification_type == "match":
            # Extract scholarship info from message
            if msg.scholarship_id:
                success = await notification_service.process_scholarship_match(
                    msg.user_id, 
                    msg.scholarship_id, 
                    85.0  # Mock score, would be passed in real implementation
                )
        
        elif msg.notification_type == "deadline":
            # Create deadline alert
            alert = DeadlineAlert(
                scholarship_id=msg.scholarship_id or "unknown",
                title=msg.message.split(":")[0] if ":" in msg.message else msg.message,
                deadline="2024-12-31",  # Mock deadline
                days_remaining=7,  # Mock days
                user_ids=[msg.user_id]
            )
            success_count = await notification_service.process_deadline_reminder(alert)
            success = success_count > 0
        
        elif msg.notification_type == "reminder":
            # Send general reminder
            success = await notification_service.send_push_notification(
                msg.user_id, "Reminder", msg.message
            )
        
        response = NotificationResponse(
            success=success,
            message="Notification sent successfully" if success else "Failed to send notification",
            notification_id=notification_id if success else None
        )
        
        logger.info(f"Notification request processed: {response.success}")
        await ctx.send(sender, response)
        
    except Exception as e:
        logger.error(f"Error processing notification request: {e}")
        await ctx.send(sender, NotificationResponse(
            success=False,
            message=f"Error: {str(e)}"
        ))

@notification_protocol.on_message(model=UserPreferences)
async def handle_user_preferences(ctx: Context, sender: str, msg: UserPreferences):
    """Handle user preference updates"""
    try:
        notification_service.add_user_preferences(msg)
        logger.info(f"Updated preferences for user {msg.user_id}")
        
    except Exception as e:
        logger.error(f"Error updating user preferences: {e}")

# Register the protocol
notification_agent.include(notification_protocol)

# ============= Background Tasks =============

@notification_agent.on_interval(period=3600.0)  # Check every hour
async def check_deadlines(ctx: Context):
    """Periodically check for upcoming deadlines"""
    logger.info("Checking for upcoming scholarship deadlines...")
    
    # Mock deadline check - in real implementation, this would query the IC canister
    mock_deadlines = [
        {
            "scholarship_id": "sch_001",
            "title": "Tech Innovation Scholarship",
            "deadline": "2024-12-15",
            "interested_users": ["user_001", "user_002"]
        },
        {
            "scholarship_id": "sch_002", 
            "title": "Global Leadership Grant",
            "deadline": "2024-12-20",
            "interested_users": ["user_003"]
        }
    ]
    
    for deadline_info in mock_deadlines:
        # Calculate days remaining
        deadline_date = datetime.strptime(deadline_info["deadline"], "%Y-%m-%d")
        days_remaining = (deadline_date - datetime.now()).days
        
        if days_remaining <= 7 and days_remaining > 0:  # Alert within 7 days
            alert = DeadlineAlert(
                scholarship_id=deadline_info["scholarship_id"],
                title=deadline_info["title"],
                deadline=deadline_info["deadline"],
                days_remaining=days_remaining,
                user_ids=deadline_info["interested_users"]
            )
            
            await notification_service.process_deadline_reminder(alert)

# ============= Test Functions =============

async def test_notifications():
    """Test notification functionality"""
    
    # Add test user preferences
    test_prefs = UserPreferences(
        user_id="test_user_001",
        email="test@example.com",
        email_enabled=True,
        push_enabled=True,
        deadline_reminders=True,
        match_notifications=True
    )
    notification_service.add_user_preferences(test_prefs)
    
    # Test match notification
    await notification_service.process_scholarship_match(
        "test_user_001", 
        "AI Research Fellowship", 
        92.5
    )
    
    # Test deadline notification
    test_alert = DeadlineAlert(
        scholarship_id="test_sch_001",
        title="Machine Learning Scholarship",
        deadline="2024-12-25",
        days_remaining=5,
        user_ids=["test_user_001"]
    )
    await notification_service.process_deadline_reminder(test_alert)

# ============= Main Entry Point =============

if __name__ == "__main__":
    print("Starting Scholarship Notification Agent...")
    print(f"Agent address: {notification_agent.address}")
    print("Listening for notification requests on port 8002")
    
    # Run test if needed
    # asyncio.run(test_notifications())
    
    # Run the agent
    notification_agent.run()
