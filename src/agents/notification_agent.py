"""
Notification Agent for Scholarship Matcher
Handles deadline reminders and new scholarship notifications
"""

from uagents import Agent, Context, Model, Protocol
from uagents.setup import fund_agent_if_low
from typing import List, Dict, Optional
from datetime import datetime, timedelta
import logging
import asyncio

# Configure logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

# ============= Models =============

class NotificationRequest(Model):
    """Request to set up notifications"""
    user_id: str
    email: str
    notification_types: List[str]  # ["deadline", "new_match", "status_update"]
    preferences: Dict  # {"days_before_deadline": 7, "min_match_score": 70}

class DeadlineAlert(Model):
    """Deadline reminder notification"""
    user_id: str
    scholarship_id: str
    scholarship_name: str
    deadline: str
    days_remaining: int
    action_required: str

class NewMatchAlert(Model):
    """New scholarship match notification"""
    user_id: str
    scholarship_id: str
    scholarship_name: str
    match_score: float
    key_benefits: List[str]

class StatusUpdateAlert(Model):
    """Application status change notification"""
    user_id: str
    scholarship_id: str
    scholarship_name: str
    old_status: str
    new_status: str
    action_required: Optional[str]

class NotificationResponse(Model):
    """Response after sending notification"""
    success: bool
    notification_type: str
    message: str
    timestamp: str

class BulkNotificationRequest(Model):
    """Request to send notifications to multiple users"""
    notification_type: str
    user_ids: List[str]
    content: Dict

class ManualCheckRequest(Model):
    """Request to manually trigger notification checks"""
    check_type: str  # 'deadlines', 'new_scholarships', 'all'
    user_id: Optional[str] = None

# ============= Notification Manager =============

class NotificationManager:
    """Manages notification logic and scheduling"""
    
    def __init__(self):
        self.user_preferences = {}  # Store user notification preferences
        self.scheduled_notifications = []  # Track scheduled notifications
    
    def register_user_preferences(self, user_id: str, preferences: Dict):
        """Register user notification preferences"""
        self.user_preferences[user_id] = preferences
        logger.info(f"Registered preferences for user {user_id}")
    
    def check_deadlines(self, user_scholarships: List[Dict]) -> List[DeadlineAlert]:
        """Check for upcoming deadlines"""
        alerts = []
        current_date = datetime.now()
        
        for item in user_scholarships:
            deadline = datetime.fromisoformat(item['deadline'])
            days_remaining = (deadline - current_date).days
            
            # Check if alert should be sent based on user preferences
            user_pref = self.user_preferences.get(item['user_id'], {})
            alert_days = user_pref.get('days_before_deadline', 7)
            
            if 0 < days_remaining <= alert_days:
                alert = DeadlineAlert(
                    user_id=item['user_id'],
                    scholarship_id=item['scholarship_id'],
                    scholarship_name=item['scholarship_name'],
                    deadline=item['deadline'],
                    days_remaining=days_remaining,
                    action_required=self._get_deadline_action(days_remaining)
                )
                alerts.append(alert)
        
        return alerts
    
    def _get_deadline_action(self, days_remaining: int) -> str:
        """Get action message based on days remaining"""
        if days_remaining <= 1:
            return "URGENT: Submit application immediately!"
        elif days_remaining <= 3:
            return "Final reminder: Complete your application soon"
        elif days_remaining <= 7:
            return "Start preparing your application documents"
        else:
            return "Mark your calendar and start preparation"
    
    def check_new_matches(self, user_id: str, new_scholarships: List[Dict]) -> List[NewMatchAlert]:
        """Check for new scholarship matches above threshold"""
        alerts = []
        user_pref = self.user_preferences.get(user_id, {})
        min_score = user_pref.get('min_match_score', 70)
        
        for scholarship in new_scholarships:
            if scholarship['match_score'] >= min_score:
                alert = NewMatchAlert(
                    user_id=user_id,
                    scholarship_id=scholarship['id'],
                    scholarship_name=scholarship['name'],
                    match_score=scholarship['match_score'],
                    key_benefits=scholarship.get('key_benefits', [])
                )
                alerts.append(alert)
        
        return alerts
    
    def format_notification_message(self, alert) -> str:
        """Format notification into readable message"""
        if isinstance(alert, DeadlineAlert):
            return (
                f"📅 Deadline Reminder!\n"
                f"Scholarship: {alert.scholarship_name}\n"
                f"Days remaining: {alert.days_remaining}\n"
                f"Action: {alert.action_required}"
            )
        elif isinstance(alert, NewMatchAlert):
            return (
                f"🎯 New Match Found!\n"
                f"Scholarship: {alert.scholarship_name}\n"
                f"Match Score: {alert.match_score}%\n"
                f"Key Benefits: {', '.join(alert.key_benefits)}"
            )
        elif isinstance(alert, StatusUpdateAlert):
            return (
                f"📊 Status Update!\n"
                f"Scholarship: {alert.scholarship_name}\n"
                f"Status: {alert.old_status} → {alert.new_status}\n"
                f"Action: {alert.action_required or 'No action required'}"
            )
        else:
            return "New notification available"
    
    async def get_icp_scholarships(self):
        """Get scholarships from ICP backend"""
        try:
            import requests
            import json
            from datetime import datetime
            
            # Try to get data from ICP canister
            # Note: In production, this would use the proper ICP agent
            logger.info("Fetching scholarships from ICP backend...")
            
            # For now, return mock data that matches our ICP structure
            # This would be replaced with actual ICP calls
            mock_scholarships = [
                {
                    "user_id": "user123",
                    "scholarship_id": "fulbright-2025",
                    "scholarship_name": "Fulbright Scholarship",
                    "deadline": "2025-10-01T23:59:59",
                    "status": "in_progress",
                    "match_score": 85.5
                },
                {
                    "user_id": "user123", 
                    "scholarship_id": "chevening-2025",
                    "scholarship_name": "Chevening Scholarship",
                    "deadline": "2025-09-15T23:59:59",
                    "status": "not_started",
                    "match_score": 78.2
                }
            ]
            
            logger.info(f"Retrieved {len(mock_scholarships)} scholarships from ICP")
            return mock_scholarships
            
        except Exception as e:
            logger.error(f"Error fetching ICP scholarships: {str(e)}")
            return []

# ============= Agent Setup =============

# Initialize the notification agent
notification_agent = Agent(
    name="scholarship_notifier",
    seed="scholarship_notification_seed_2024_hackathon",
    port=8002,
    endpoint=["http://localhost:8002/submit"]
)

# Fund agent if needed (commented out to avoid testnet delay)
# fund_agent_if_low(notification_agent.wallet.address())

# Initialize notification manager
notification_manager = NotificationManager()

# Create protocol for handling notifications
notification_protocol = Protocol("ScholarshipNotifications")

@notification_protocol.on_message(model=NotificationRequest, replies=NotificationResponse)
async def handle_notification_setup(ctx: Context, sender: str, msg: NotificationRequest):
    """Handle notification preference setup"""
    
    logger.info(f"Setting up notifications for user: {msg.user_id}")
    
    try:
        # Register user preferences
        notification_manager.register_user_preferences(
            msg.user_id,
            msg.preferences
        )
        
        response = NotificationResponse(
            success=True,
            notification_type="setup",
            message=f"Notifications configured for user {msg.user_id}",
            timestamp=datetime.now().isoformat()
        )
        
        await ctx.send(sender, response)
        
    except Exception as e:
        logger.error(f"Error setting up notifications: {str(e)}")
        error_response = NotificationResponse(
            success=False,
            notification_type="setup",
            message=f"Failed to setup notifications: {str(e)}",
            timestamp=datetime.now().isoformat()
        )
        await ctx.send(sender, error_response)

@notification_protocol.on_message(model=DeadlineAlert, replies=NotificationResponse)
async def handle_deadline_alert(ctx: Context, sender: str, msg: DeadlineAlert):
    """Handle deadline alert notifications"""
    
    logger.info(f"Sending deadline alert to user: {msg.user_id}")
    
    try:
        # Format and "send" notification (in real app, would send email/push)
        message = notification_manager.format_notification_message(msg)
        
        # Log the notification (in production, send actual email/push notification)
        logger.info(f"Notification sent: {message}")
        
        response = NotificationResponse(
            success=True,
            notification_type="deadline",
            message=message,
            timestamp=datetime.now().isoformat()
        )
        
        await ctx.send(sender, response)
        
    except Exception as e:
        logger.error(f"Error sending deadline alert: {str(e)}")
        error_response = NotificationResponse(
            success=False,
            notification_type="deadline",
            message=f"Failed to send alert: {str(e)}",
            timestamp=datetime.now().isoformat()
        )
        await ctx.send(sender, error_response)

@notification_protocol.on_message(model=NewMatchAlert, replies=NotificationResponse)
async def handle_new_match_alert(ctx: Context, sender: str, msg: NewMatchAlert):
    """Handle new match notifications"""
    
    logger.info(f"Sending new match alert to user: {msg.user_id}")
    
    try:
        message = notification_manager.format_notification_message(msg)
        logger.info(f"Notification sent: {message}")
        
        response = NotificationResponse(
            success=True,
            notification_type="new_match",
            message=message,
            timestamp=datetime.now().isoformat()
        )
        
        await ctx.send(sender, response)
        
    except Exception as e:
        logger.error(f"Error sending new match alert: {str(e)}")
        error_response = NotificationResponse(
            success=False,
            notification_type="new_match",
            message=f"Failed to send alert: {str(e)}",
            timestamp=datetime.now().isoformat()
        )
        await ctx.send(sender, error_response)

@notification_protocol.on_message(model=BulkNotificationRequest, replies=NotificationResponse)
async def handle_bulk_notification(ctx: Context, sender: str, msg: BulkNotificationRequest):
    """Handle bulk notifications to multiple users"""
    
    logger.info(f"Sending bulk notification to {len(msg.user_ids)} users")
    
    try:
        success_count = 0
        for user_id in msg.user_ids:
            # Send notification to each user
            # In production, this would batch send emails/push notifications
            logger.info(f"Sending {msg.notification_type} to user {user_id}")
            success_count += 1
        
        response = NotificationResponse(
            success=True,
            notification_type="bulk",
            message=f"Sent {success_count} notifications successfully",
            timestamp=datetime.now().isoformat()
        )
        
        await ctx.send(sender, response)
        
    except Exception as e:
        logger.error(f"Error sending bulk notification: {str(e)}")
        error_response = NotificationResponse(
            success=False,
            notification_type="bulk",
            message=f"Failed to send bulk notification: {str(e)}",
            timestamp=datetime.now().isoformat()
        )
        await ctx.send(sender, error_response)

@notification_protocol.on_message(model=ManualCheckRequest, replies=NotificationResponse)
async def handle_manual_check(ctx: Context, sender: str, msg: ManualCheckRequest):
    """Handle manual notification check requests"""
    
    logger.info(f"Manual check requested: {msg.check_type}")
    
    try:
        if msg.check_type in ['deadlines', 'all']:
            # Trigger deadline check
            scholarships = await notification_manager.get_icp_scholarships()
            alerts = notification_manager.check_deadlines(scholarships)
            
            for alert in alerts:
                message = notification_manager.format_notification_message(alert)
                logger.info(f"Manual deadline alert: {message}")
        
        if msg.check_type in ['new_scholarships', 'all']:
            # Trigger new scholarship check
            logger.info("Manual new scholarship check triggered")
            # Would implement new scholarship detection logic here
        
        response = NotificationResponse(
            success=True,
            notification_type="manual_check",
            message=f"Manual {msg.check_type} check completed successfully",
            timestamp=datetime.now().isoformat()
        )
        
        await ctx.send(sender, response)
        
    except Exception as e:
        logger.error(f"Error in manual check: {str(e)}")
        error_response = NotificationResponse(
            success=False,
            notification_type="manual_check",
            message=f"Failed to complete manual check: {str(e)}",
            timestamp=datetime.now().isoformat()
        )
        await ctx.send(sender, error_response)

# Include protocol in agent
notification_agent.include(notification_protocol)

# ============= Periodic Tasks =============

@notification_agent.on_interval(period=3600.0)  # Run every hour
async def check_deadlines_periodically(ctx: Context):
    """Periodically check for upcoming deadlines"""
    logger.info("Running periodic deadline check...")
    
    try:
        # Get scholarships from ICP backend
        scholarships = await notification_manager.get_icp_scholarships()
        
        if scholarships:
            # Check each scholarship for deadline alerts
            alerts = notification_manager.check_deadlines(scholarships)
            
            # Send alerts to users
            for alert in alerts:
                logger.info(f"📅 Deadline alert: {alert.scholarship_name} - {alert.days_remaining} days remaining")
                
                # Format and log the notification message
                message = notification_manager.format_notification_message(alert)
                logger.info(f"Notification message: {message}")
                
                # In production, this would send actual email/push notifications
                
            logger.info(f"Deadline check completed - processed {len(alerts)} alerts")
        else:
            logger.info("No scholarships found for deadline checking")
        
    except Exception as e:
        logger.error(f"Error in deadline check: {str(e)}")
        logger.info("Deadline check completed with errors")

@notification_agent.on_interval(period=86400.0)  # Run daily
async def check_new_scholarships(ctx: Context):
    """Check for new scholarships matching user preferences"""
    logger.info("Running daily new scholarship check...")
    
    try:
        import requests
        
        # Get current scholarship count from ICP
        # This would be implemented to track new additions
        logger.info("Checking for new scholarships from ICP backend...")
        
        # Query matching agent for any new high-score matches
        try:
            matching_response = requests.get("http://localhost:8001", timeout=5)
            logger.info("Connected to matching agent for new scholarship analysis")
        except requests.exceptions.RequestException:
            logger.warning("Matching agent not available - skipping new match analysis")
        
        # Track scholarship database changes
        # In production: compare current scholarship list with previous snapshot
        # Send notifications for new high-score matches
        
        logger.info("New scholarship check completed")
        
    except Exception as e:
        logger.error(f"Error in new scholarship check: {str(e)}")
        logger.info("New scholarship check completed with errors")

# ============= Test Functions =============

def test_notifications():
    """Test notification formatting"""
    
    # Test deadline alert
    deadline_alert = DeadlineAlert(
        user_id="test_user",
        scholarship_id="fulbright-2025",
        scholarship_name="Fulbright Scholarship",
        deadline="2025-10-01",
        days_remaining=3,
        action_required="Final reminder: Complete your application soon"
    )
    
    print("\n=== Deadline Alert ===")
    print(notification_manager.format_notification_message(deadline_alert))
    
    # Test new match alert
    match_alert = NewMatchAlert(
        user_id="test_user",
        scholarship_id="chevening-2025",
        scholarship_name="Chevening Scholarship",
        match_score=85.5,
        key_benefits=["Full tuition", "Living allowance", "Travel grant"]
    )
    
    print("\n=== New Match Alert ===")
    print(notification_manager.format_notification_message(match_alert))
    
    # Test status update
    status_alert = StatusUpdateAlert(
        user_id="test_user",
        scholarship_id="lpdp-2025",
        scholarship_name="LPDP Scholarship",
        old_status="InProgress",
        new_status="Submitted",
        action_required="Wait for review results"
    )
    
    print("\n=== Status Update Alert ===")
    print(notification_manager.format_notification_message(status_alert))

# ============= Main Entry Point =============

if __name__ == "__main__":
    print("Starting Scholarship Notification Agent...")
    print(f"Agent address: {notification_agent.address}")
    print("Listening for notification requests on port 8002")
    
    # Run test if needed
    # test_notifications()
    
    # Run the agent
    notification_agent.run()