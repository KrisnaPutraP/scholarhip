#!/bin/bash

# Script to run the Scholarship Matching Agent
# Usage: ./run_agents.sh [start|stop|status]

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check Python installation
check_python() {
    if ! command -v python3 &> /dev/null; then
        echo -e "${RED}Python 3 is not installed${NC}"
        exit 1
    fi
    echo -e "${GREEN}✓ Python 3 found${NC}"
}

# Install requirements
install_requirements() {
    echo -e "${YELLOW}Installing Python requirements...${NC}"
    
    # Check if virtual environment exists, create if not
    if [ ! -d "venv" ]; then
        echo -e "${YELLOW}Creating virtual environment...${NC}"
        python3 -m venv venv
    fi
    
    # Activate virtual environment and install requirements
    source venv/bin/activate
    pip install -r requirements.txt
    echo -e "${GREEN}✓ Requirements installed${NC}"
}

# Start matching agent
start_matching_agent() {
    echo -e "${YELLOW}Starting Matching Agent...${NC}"
    venv/bin/python matching_agent.py &
    echo $! > matching_agent.pid
    echo -e "${GREEN}✓ Matching Agent started (PID: $(cat matching_agent.pid))${NC}"
}

# Stop matching agent
stop_matching_agent() {
    echo -e "${YELLOW}Stopping Matching Agent...${NC}"
    
    if [ -f matching_agent.pid ]; then
        kill $(cat matching_agent.pid) 2>/dev/null
        rm matching_agent.pid
        echo -e "${GREEN}✓ Matching Agent stopped${NC}"
    else
        echo -e "${RED}✗ No matching agent PID file found${NC}"
    fi
}

# Check agent status
check_status() {
    echo -e "${YELLOW}Agent Status:${NC}"
    
    if [ -f matching_agent.pid ] && ps -p $(cat matching_agent.pid) > /dev/null 2>&1; then
        echo -e "${GREEN}✓ Matching Agent: Running (PID: $(cat matching_agent.pid))${NC}"
    else
        echo -e "${RED}✗ Matching Agent: Not running${NC}"
    fi
}

# Show usage
show_usage() {
    echo -e "${YELLOW}Usage: $0 [start|stop|status]${NC}"
    echo ""
    echo "Commands:"
    echo "  start   - Start the matching agent"
    echo "  stop    - Stop the matching agent"
    echo "  status  - Check agent status"
}

# Main script logic
case "$1" in
    "start")
        check_python
        install_requirements
        start_matching_agent
        check_status
        ;;
    "stop")
        stop_matching_agent
        ;;
    "status")
        check_status
        ;;
    "")
        show_usage
        ;;
    *)
        echo -e "${RED}Invalid option: $1${NC}"
        show_usage
        exit 1
        ;;
esac

echo -e "${GREEN}Done!${NC}"