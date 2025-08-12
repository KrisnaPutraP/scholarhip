"""
Bridge connector between ICP canister and Fetch.ai uAgents
Handles communication between blockchain and AI agents
"""

import asyncio
import json
import logging
from typing import Dict, List, Optional
from datetime import datetime
import aiohttp
from uagents import Agent, Context, Model
from ic import Principal, Client, Identity, Canister

# Configure logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

# ============= Configuration =============

class Config:
    """Configuration for ICP and Agent connections"""
    
    # ICP Settings
    ICP_URL = "http://localhost:8080"  # Local replica
    # ICP_URL = "https://ic0.app"  # Mainnet
    CANISTER_ID = "your-canister-id-here"  # Replace with actual canister ID
    
    # Agent Settings
    MATCHING_AGENT_URL = "http://localhost:8001/submit"
    NOTIFICATION_AGENT_URL = "http://localhost:8002/submit"
    
    # Bridge Settings
    BRIDGE_PORT = 8003
    POLL_INTERVAL = 60  # seconds

# ============= Models =============

class ICPRequest(Model):
    """Request from ICP canister"""
    request_type: str  # "match", "notify", "update"
    user_id: str
    data: Dict

class ICPResponse(Model):
    """Response to ICP canister"""
    success: bool
    request_type: str
    data: Dict
    timestamp: str

# ============= ICP Connector =============

class ICPConnector:
    """Handles communication with ICP canister"""
    
    def __init__(self, canister_id: str, network_url: str):
        self.canister_id = canister_id
        self.network_url = network_url
        self.client = None
        self.canister = None
        
    async def initialize(self):
        """Initialize connection to ICP canister"""
        try:
            # In production, use proper identity management
            # For hackathon, using anonymous identity
            self.client = Client(url=self.network_url)
            logger.info(f"Connected to ICP at {self.network_url}")
            return True
        except Exception as e:
            logger.error(f"Failed to connect to ICP: {str(e)}")
            return False
    
    async def fetch_pending_requests(self) -> List[Dict]:
        """Fetch pending AI processing requests from canister"""
        try:
            # In production, this would call the actual canister method
            # For demo, returning mock data
            return [
                {
                    "request_type": "match",
                    "user_id": "user123",
                    "data": {
                        "profile": {
                            "gpa": 3.5,
                            "major": "Computer Science",
                            "skills": ["Python", "AI", "Blockchain"]
                        }
                    }
                }
            ]
        except Exception as e:
            logger.error(f"Error fetching requests: {str(e)}")
            return []
    
    async def send_response(self, response: ICPResponse) -> bool:
        """Send processed response back to canister"""
        try:
            # In production, this would call the canister update method
            logger.info(f"Sending response to canister: {response.request_type}")
            return True
        except Exception as e:
            logger.error(f"Error sending response: {str(e)}")
            return False

# ============= Agent Connector =============

class AgentConnector:
    """Handles communication with Fetch.ai agents"""
    
    def __init__(self):
        self.session = None
        
    async def initialize(self):
        """Initialize HTTP session for agent communication"""
        self.session = aiohttp.ClientSession()
        logger.info("Agent connector initialized")
    
    async def send_to_matching_agent(self, user_data: Dict) -> Dict:
        """Send matching request to matching agent"""
        try:
            async with self.session.post(
                Config.MATCHING_AGENT_URL,
                json=user_data,
                headers={"Content-Type": "application/json"}
            ) as response:
                if response.status == 200:
                    return await response.json()
                else:
                    logger.error(f"Matching agent returned status {response.status}")
                    return {"error": "Failed to get matches"}
        except Exception as e:
            logger.error(f"Error communicating with matching agent: {str(e)}")
            return {"error": str(e)}
    
    async def send_to_notification_agent(self, notification_data: Dict) -> Dict:
        """Send notification request to notification agent"""
        try:
            async with self.session.post(
                Config.NOTIFICATION_AGENT_URL,
                json=notification_data,
                headers={"Content-Type": "application/json"}
            ) as response:
                if response.status == 200:
                    return await response.json()
                else:
                    logger.error(f"Notification agent returned status {response.status}")
                    return {"error": "Failed to send notification"}
        except Exception as e:
            logger.error(f"Error communicating with notification agent: {str(e)}")
            return {"error": str(e)}
    
    async def cleanup(self):
        """Clean up resources"""
        if self.session:
            await self.session.close()

# ============= Bridge Orchestrator =============

class BridgeOrchestrator:
    """Main orchestrator for ICP-Agent bridge"""
    
    def __init__(self):
        self.icp_connector = ICPConnector(Config.CANISTER_ID, Config.ICP_URL)
        self.agent_connector = AgentConnector()
        self.running = False
        
    async def initialize(self):
        """Initialize all connectors"""
        logger.info("Initializing bridge orchestrator...")
        
        # Initialize ICP connection
        icp_success = await self.icp_connector.initialize()
        if not icp_success:
            logger.error("Failed to initialize ICP connector")
            return False
        
        # Initialize agent connector
        await self.agent_connector.initialize()
        
        logger.info("Bridge orchestrator initialized successfully")
        return True
    
    async def process_request(self, request: Dict) -> ICPResponse:
        """Process a single request from ICP"""
        request_type = request.get("request_type")
        user_id = request.get("user_id")
        data = request.get("data", {})
        
        logger.info(f"Processing {request_type} request for user {user_id}")
        
        try:
            if request_type == "match":
                # Send to matching agent
                result = await self.agent_connector.send_to_matching_agent(data)
                
                return ICPResponse(
                    success=True,
                    request_type="match",
                    data=result,
                    timestamp=datetime.now().isoformat()
                )
                
            elif request_type == "notify":
                # Send to notification agent
                result = await self.agent_connector.send_to_notification_agent(data)
                
                return ICPResponse(
                    success=True,
                    request_type="notify",
                    data=result,
                    timestamp=datetime.now().isoformat()
                )
                
            else:
                logger.warning(f"Unknown request type: {request_type}")
                return ICPResponse(
                    success=False,
                    request_type=request_type,
                    data={"error": "Unknown request type"},
                    timestamp=datetime.now().isoformat()
                )
                
        except Exception as e:
            logger.error(f"Error processing request: {str(e)}")
            return ICPResponse(
                success=False,
                request_type=request_type,
                data={"error": str(e)},
                timestamp=datetime.now().isoformat()
            )
    
    async def run_bridge_loop(self):
        """Main bridge loop - polls ICP for requests and processes them"""
        self.running = True
        logger.info("Starting bridge loop...")
        
        while self.running:
            try:
                # Fetch pending requests from ICP
                requests = await self.icp_connector.fetch_pending_requests()
                
                if requests:
                    logger.info(f"Found {len(requests)} pending requests")
                    
                    # Process each request
                    for request in requests:
                        response = await self.process_request(request)
                        
                        # Send response back to ICP
                        success = await self.icp_connector.send_response(response)
                        
                        if success:
                            logger.info(f"Successfully processed request for user {request.get('user_id')}")
                        else:
                            logger.error(f"Failed to send response for user {request.get('user_id')}")
                
                # Wait before next poll
                await asyncio.sleep(Config.POLL_INTERVAL)
                
            except Exception as e:
                logger.error(f"Error in bridge loop: {str(e)}")
                await asyncio.sleep(10)  # Wait before retrying
    
    async def stop(self):
        """Stop the bridge"""
        logger.info("Stopping bridge...")
        self.running = False
        await self.agent_connector.cleanup()

# ============= Bridge Agent =============

# Create bridge agent for monitoring and management
bridge_agent = Agent(
    name="icp_bridge",
    seed="icp_bridge_seed_2024_hackathon",
    port=Config.BRIDGE_PORT,
    endpoint=[f"http://localhost:{Config.BRIDGE_PORT}/submit"]
)

# Bridge orchestrator instance
orchestrator = None

@bridge_agent.on_event("startup")
async def startup(ctx: Context):
    """Initialize bridge on agent startup"""
    global orchestrator
    
    logger.info("Starting ICP-Agent bridge...")
    
    orchestrator = BridgeOrchestrator()
    success = await orchestrator.initialize()
    
    if success:
        logger.info("Bridge initialized successfully")
    else:
        logger.error("Failed to initialize bridge")

@bridge_agent.on_interval(period=60.0)  # Run every 60 seconds
async def run_bridge_loop(ctx: Context):
    """Periodic bridge operations"""
    global orchestrator
    
    if orchestrator:
        try:
            # Fetch pending requests from ICP
            requests = await orchestrator.icp_connector.fetch_pending_requests()
            
            if requests:
                logger.info(f"Found {len(requests)} pending requests")
                
                # Process each request
                for request in requests:
                    response = await orchestrator.process_request(request)
                    
                    # Send response back to ICP
                    success = await orchestrator.icp_connector.send_response(response)
                    
                    if success:
                        logger.info(f"Successfully processed request for user {request.get('user_id')}")
                    else:
                        logger.error(f"Failed to send response for user {request.get('user_id')}")
            
        except Exception as e:
            logger.error(f"Error in bridge loop: {e}")
    else:
        logger.warning("Bridge orchestrator not initialized")

@bridge_agent.on_event("shutdown")
async def shutdown(ctx: Context):
    """Cleanup on agent shutdown"""
    global orchestrator
    
    if orchestrator:
        await orchestrator.stop()
    
    logger.info("Bridge stopped")

# ============= Direct HTTP API =============

async def handle_direct_request(request_data: Dict) -> Dict:
    """Handle direct HTTP requests for testing"""
    global orchestrator
    
    if not orchestrator:
        return {"error": "Bridge not initialized"}
    
    response = await orchestrator.process_request(request_data)
    
    return {
        "success": response.success,
        "data": response.data,
        "timestamp": response.timestamp
    }

# ============= Testing Functions =============

async def test_bridge():
    """Test bridge functionality"""
    
    # Initialize bridge
    orchestrator = BridgeOrchestrator()
    await orchestrator.initialize()
    
    # Test match request
    test_request = {
        "request_type": "match",
        "user_id": "test_user_123",
        "data": {
            "profile": {
                "gpa": 3.7,
                "major": "Computer Science",
                "university": "MIT",
                "skills": ["Python", "Machine Learning", "Blockchain"],
                "preferred_countries": ["USA", "UK"]
            },
            "scholarships": [
                {
                    "id": "test_scholarship_1",
                    "name": "Test Scholarship",
                    "min_gpa": 3.5,
                    "fields": ["Computer Science"],
                    "country": "USA"
                }
            ]
        }
    }
    
    print("\n=== Testing Bridge ===")
    response = await orchestrator.process_request(test_request)
    print(f"Response: {response}")
    
    # Cleanup
    await orchestrator.stop()

# ============= Main Entry Point =============

if __name__ == "__main__":
    import sys
    
    if len(sys.argv) > 1 and sys.argv[1] == "test":
        # Run test
        asyncio.run(test_bridge())
    else:
        # Run bridge agent
        print(f"Starting ICP-Agent Bridge on port {Config.BRIDGE_PORT}...")
        print(f"Agent address: {bridge_agent.address}")
        print(f"Polling ICP canister every {Config.POLL_INTERVAL} seconds")
        
        bridge_agent.run()