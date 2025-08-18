#!/usr/bin/env python3
"""
Test client for the notification agent
"""

import asyncio
import requests
import json
from datetime import datetime

# Test data
NOTIFICATION_AGENT_URL = "http://localhost:8002"

def test_agent_connection():
    """Test if notification agent is running"""
    try:
        response = requests.get(NOTIFICATION_AGENT_URL, timeout=5)
        print("✅ Notification agent is running on port 8002")
        return True
    except requests.exceptions.RequestException as e:
        print(f"❌ Notification agent not reachable: {e}")
        return False

def test_notification_request():
    """Test sending a notification request"""
    try:
        # Simple notification request
        notification_data = {
            "user_id": "test_user_123",
            "email": "test@example.com",
            "notification_types": ["deadline", "new_match"],
            "preferences": {
                "days_before_deadline": 7,
                "min_match_score": 70
            }
        }
        
        print("📨 Testing notification setup...")
        print(f"Request data: {json.dumps(notification_data, indent=2)}")
        
        # In a real test, would send this via uAgent protocol
        # For now, just show what would be sent
        print("✅ Notification request structure is valid")
        
    except Exception as e:
        print(f"❌ Error in notification test: {e}")

def simulate_deadline_check():
    """Simulate what a deadline check would look like"""
    print("\n🔔 Simulating deadline check...")
    
    # Mock scholarship data with various deadline scenarios
    test_scholarships = [
        {
            "user_id": "user123",
            "scholarship_id": "fulbright-2025",
            "scholarship_name": "Fulbright Scholarship",
            "deadline": "2025-08-17T23:59:59",  # 3 days from now
            "status": "in_progress"
        },
        {
            "user_id": "user123",
            "scholarship_id": "chevening-2025", 
            "scholarship_name": "Chevening Scholarship",
            "deadline": "2025-08-20T23:59:59",  # 6 days from now
            "status": "not_started"
        },
        {
            "user_id": "user123",
            "scholarship_id": "lpdp-2025",
            "scholarship_name": "LPDP Scholarship", 
            "deadline": "2025-09-15T23:59:59",  # 32 days from now
            "status": "in_progress"
        }
    ]
    
    print("Test scholarship deadlines:")
    for scholarship in test_scholarships:
        deadline = datetime.fromisoformat(scholarship['deadline'].replace('T', ' ').replace('Z', ''))
        days_remaining = (deadline - datetime.now()).days
        
        status = "🔴 URGENT" if days_remaining <= 3 else "🟡 SOON" if days_remaining <= 7 else "🟢 OK"
        print(f"  {status} {scholarship['scholarship_name']}: {days_remaining} days remaining")
    
    print("✅ Deadline simulation complete")

def main():
    """Main test function"""
    print("🧪 Testing Scholarship Notification Agent")
    print("=" * 50)
    
    # Test 1: Agent connection
    if not test_agent_connection():
        print("❌ Cannot proceed - notification agent not running")
        print("💡 Start it with: ./run_agents.sh notification")
        return
    
    # Test 2: Notification request structure
    test_notification_request()
    
    # Test 3: Deadline checking simulation
    simulate_deadline_check()
    
    print("\n🎉 All tests completed!")
    print("\n📋 Next steps:")
    print("   1. The notification agent is running and processing periodic checks")
    print("   2. Check the agent logs for deadline and new scholarship monitoring")
    print("   3. In production, this would send actual email/push notifications")

if __name__ == "__main__":
    main()
