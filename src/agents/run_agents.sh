#!/bin/bash

# Script to run all Fetch.ai agents for Scholarship Matcher
# Usage: ./run_agents.sh [all|matching|notification|bridge|stop]

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
    echo -e "${YELLOW}Starting Matching Agent on port 8001...${NC}"
    venv/bin/python matching_agent.py &
    echo $! > matching_agent.pid
    echo -e "${GREEN}✓ Matching Agent started (PID: $(cat matching_agent.pid))${NC}"
}

# Start notification agent
start_notification_agent() {
    echo -e "${YELLOW}Starting Notification Agent on port 8002...${NC}"
    venv/bin/python notification_agent.py &
    echo $! > notification_agent.pid
    echo -e "${GREEN}✓ Notification Agent started (PID: $(cat notification_agent.pid))${NC}"
}

# Start bridge agent
# Start ICP bridge
start_bridge_agent() {
    echo -e "${YELLOW}Starting ICP Bridge on port 8003...${NC}"
    venv/bin/python icp_connector.py &
    echo $! > bridge_agent.pid
    echo -e "${GREEN}✓ Bridge Agent started (PID: $(cat bridge_agent.pid))${NC}"
}

# Stop all agents
stop_all_agents() {
    echo -e "${YELLOW}Stopping all agents...${NC}"
    
    # Stop matching agent
    if [ -f matching_agent.pid ]; then
        kill $(cat matching_agent.pid) 2>/dev/null
        rm matching_agent.pid
        echo -e "${GREEN}✓ Matching Agent stopped${NC}"
    fi
    
    # Stop notification agent
    if [ -f notification_agent.pid ]; then
        kill $(cat notification_agent.pid) 2>/dev/null
        rm notification_agent.pid
        echo -e "${GREEN}✓ Notification Agent stopped${NC}"
    fi
    
    # Stop bridge agent
    if [ -f bridge_agent.pid ]; then
        kill $(cat bridge_agent.pid) 2>/dev/null
        rm bridge_agent.pid
        echo -e "${GREEN}✓ Bridge Agent stopped${NC}"
    fi
}

# Check agent status
check_status() {
    echo -e "${YELLOW}Agent Status:${NC}"
    
    # Check matching agent
    if [ -f matching_agent.pid ] && ps -p $(cat matching_agent.pid) > /dev/null 2>&1; then
        echo -e "${GREEN}✓ Matching Agent: Running (PID: $(cat matching_agent.pid))${NC}"
    else
        echo -e "${RED}✗ Matching Agent: Not running${NC}"
    fi
    
    # Check notification agent
    if [ -f notification_agent.pid ] && ps -p $(cat notification_agent.pid) > /dev/null 2>&1; then
        echo -e "${GREEN}✓ Notification Agent: Running (PID: $(cat notification_agent.pid))${NC}"
    else
        echo -e "${RED}✗ Notification Agent: Not running${NC}"
    fi
    
    # Check bridge agent
    if [ -f bridge_agent.pid ] && ps -p $(cat bridge_agent.pid) > /dev/null 2>&1; then
        echo -e "${GREEN}✓ Bridge Agent: Running (PID: $(cat bridge_agent.pid))${NC}"
    else
        echo -e "${RED}✗ Bridge Agent: Not running${NC}"
    fi
}

# Show logs
show_logs() {
    echo -e "${YELLOW}Showing agent logs (Ctrl+C to stop)...${NC}"
    tail -f *.log 2>/dev/null || echo -e "${RED}No log files found${NC}"
}

# Main menu
show_menu() {
    echo "========================================="
    echo "  Scholarship Matcher - Agent Manager"
    echo "========================================="
    echo "1. Start all agents"
    echo "2. Start matching agent only"
    echo "3. Start notification agent only"
    echo "4. Start bridge agent only"
    echo "5. Stop all agents"
    echo "6. Check status"
    echo "7. Show logs"
    echo "8. Install requirements"
    echo "9. Exit"
    echo "========================================="
}

# Main script logic
main() {
    # Check Python first
    check_python
    
    # Handle command line arguments
    case "$1" in
        all)
            install_requirements
            start_matching_agent
            sleep 2
            start_notification_agent
            sleep 2
            start_bridge_agent
            echo -e "${GREEN}All agents started successfully!${NC}"
            check_status
            ;;
        matching)
            start_matching_agent
            ;;
        notification)
            start_notification_agent
            ;;
        bridge)
            start_bridge_agent
            ;;
        stop)
            stop_all_agents
            ;;
        status)
            check_status
            ;;
        logs)
            show_logs
            ;;
        install)
            install_requirements
            ;;
        "")
            # Interactive mode
            while true; do
                show_menu
                read -p "Select option: " choice
                case $choice in
                    1)
                        install_requirements
                        start_matching_agent
                        sleep 2
                        start_notification_agent
                        sleep 2
                        start_bridge_agent
                        echo -e "${GREEN}All agents started!${NC}"
                        ;;
                    2)
                        start_matching_agent
                        ;;
                    3)
                        start_notification_agent
                        ;;
                    4)
                        start_bridge_agent
                        ;;
                    5)
                        stop_all_agents
                        ;;
                    6)
                        check_status
                        ;;
                    7)
                        show_logs
                        ;;
                    8)
                        install_requirements
                        ;;
                    9)
                        echo "Exiting..."
                        exit 0
                        ;;
                    *)
                        echo -e "${RED}Invalid option${NC}"
                        ;;
                esac
                echo ""
                read -p "Press Enter to continue..."
            done
            ;;
        *)
            echo "Usage: $0 [all|matching|notification|bridge|stop|status|logs|install]"
            echo ""
            echo "Options:"
            echo "  all          - Start all agents"
            echo "  matching     - Start matching agent only"
            echo "  notification - Start notification agent only"
            echo "  bridge       - Start bridge agent only"
            echo "  stop         - Stop all agents"
            echo "  status       - Check agent status"
            echo "  logs         - Show agent logs"
            echo "  install      - Install Python requirements"
            echo ""
            echo "Run without arguments for interactive mode"
            exit 1
            ;;
    esac
}

# Run main function
main "$@"