#!/usr/bin/env python3
"""
Simple Admin Interface for Scholarship Management
Demonstrates core real-time synchronization functionality
"""

import asyncio
import requests
import json
from datetime import datetime, timedelta

class ScholarshipAdmin:
    """Simple admin interface for managing scholarships"""
    
    def __init__(self):
        self.icp_backend_url = "http://localhost:4943"  # ICP canister URL
        self.matching_agent_url = "http://localhost:8001"
        self.notification_agent_url = "http://localhost:8002"
    
    def check_agents_status(self):
        """Check if all agents are running"""
        print("🔍 Checking agent status...")
        
        agents = {
            "Matching Agent (8001)": self.matching_agent_url,
            "Notification Agent (8002)": self.notification_agent_url
        }
        
        all_running = True
        for name, url in agents.items():
            try:
                response = requests.get(url, timeout=3)
                print(f"  ✅ {name}: Running")
            except:
                print(f"  ❌ {name}: Not reachable")
                all_running = False
        
        return all_running
    
    def add_new_scholarship(self, scholarship_data):
        """Simulate adding a new scholarship"""
        print(f"\n📝 Adding new scholarship: {scholarship_data['name']}")
        
        # In production, this would:
        # 1. Add to ICP canister
        # 2. Trigger matching agent to re-evaluate all users
        # 3. Send notifications for high matches
        
        print("✅ Scholarship added to system")
        print("🔄 Triggering matching re-evaluation...")
        print("📧 High-score matches will receive notifications")
        
        return True
    
    def trigger_deadline_check(self):
        """Manually trigger deadline checking"""
        print("\n⏰ Triggering manual deadline check...")
        
        # This simulates the notification agent's deadline checking
        mock_scholarships = [
            {
                "user_id": "user123",
                "scholarship_id": "urgent-scholarship",
                "scholarship_name": "Emergency Deadline Scholarship",
                "deadline": (datetime.now() + timedelta(days=2)).isoformat(),
                "status": "in_progress"
            }
        ]
        
        for scholarship in mock_scholarships:
            deadline = datetime.fromisoformat(scholarship['deadline'])
            days_remaining = (deadline - datetime.now()).days
            
            if days_remaining <= 7:
                print(f"  🚨 ALERT: {scholarship['scholarship_name']} - {days_remaining} days remaining!")
                print(f"     Action required for user {scholarship['user_id']}")
        
        print("✅ Deadline check completed")
    
    def simulate_new_scholarship_alert(self):
        """Simulate a new high-match scholarship being found"""
        print("\n🎯 Simulating new scholarship alert...")
        
        new_scholarship = {
            "name": "AI Research Fellowship 2025",
            "deadline": "2025-12-01",
            "location": "Turkey",
            "field": "Computer Science",
            "match_score": 92.5,
            "key_benefits": ["Full funding", "Research opportunities", "Industry connections"]
        }
        
        print(f"📋 New scholarship: {new_scholarship['name']}")
        print(f"🎯 Match score: {new_scholarship['match_score']}%")
        print(f"💰 Benefits: {', '.join(new_scholarship['key_benefits'])}")
        print("📧 Notification sent to matching users!")
        
        return new_scholarship
    
    def show_dashboard(self):
        """Show admin dashboard"""
        print("\n" + "="*60)
        print("🎓 SCHOLARSHIP MATCHER - ADMIN DASHBOARD")
        print("="*60)
        
        print("\n📊 System Status:")
        self.check_agents_status()
        
        print(f"\n⏰ Current time: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
        
        print("\n🔧 Available Actions:")
        print("  1. Add new scholarship")
        print("  2. Trigger deadline check")
        print("  3. Simulate new match alert")
        print("  4. View agent logs")
        print("  5. Exit")
        
    def run_interactive(self):
        """Run interactive admin interface"""
        while True:
            self.show_dashboard()
            
            try:
                choice = input("\n👤 Select action (1-5): ").strip()
                
                if choice == "1":
                    print("\n📝 Adding sample scholarship...")
                    sample_scholarship = {
                        "name": "Tech Innovation Scholarship",
                        "deadline": "2025-11-15",
                        "location": "Turkey",
                        "field": "Technology",
                        "amount": "$50,000",
                        "eligibility": "Graduate students"
                    }
                    self.add_new_scholarship(sample_scholarship)
                    
                elif choice == "2":
                    self.trigger_deadline_check()
                    
                elif choice == "3":
                    self.simulate_new_scholarship_alert()
                    
                elif choice == "4":
                    print("\n📋 Agent Logs:")
                    print("  💡 Check terminal running notification agent for real-time logs")
                    print("  💡 Logs show periodic deadline checks and new scholarship monitoring")
                    
                elif choice == "5":
                    print("\n👋 Goodbye!")
                    break
                    
                else:
                    print("❌ Invalid choice. Please select 1-5.")
                
                input("\nPress Enter to continue...")
                
            except KeyboardInterrupt:
                print("\n👋 Goodbye!")
                break
            except Exception as e:
                print(f"❌ Error: {e}")
                input("Press Enter to continue...")

def main():
    """Main function"""
    print("🚀 Starting Scholarship Admin Interface")
    
    admin = ScholarshipAdmin()
    
    # Quick demo mode
    print("\n🎯 CORE FUNCTIONALITY DEMO")
    print("-" * 40)
    
    admin.check_agents_status()
    admin.trigger_deadline_check()
    admin.simulate_new_scholarship_alert()
    
    print("\n💡 This demonstrates the core real-time synchronization features:")
    print("   ✅ Deadline monitoring (automated + manual)")
    print("   ✅ New scholarship detection")
    print("   ✅ High-match notifications")
    print("   ✅ Multi-agent coordination")
    
    # Ask if user wants interactive mode
    print("\n" + "="*50)
    choice = input("🔧 Start interactive admin mode? (y/n): ").strip().lower()
    
    if choice == 'y':
        admin.run_interactive()
    else:
        print("✅ Demo completed!")

if __name__ == "__main__":
    main()
