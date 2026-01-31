#!/bin/bash
# Test script for Themathar Game API

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
BASE_URL="${1:-http://localhost}"
PLAYER_COUNT="${2:-3}"

echo -e "${BLUE}=== Themathar Game API Test ===${NC}"
echo "Base URL: $BASE_URL"
echo "Creating $PLAYER_COUNT players"
echo ""

# Function to make API calls
call_api() {
    local endpoint=$1
    local method=${2:-GET}
    local data=$3
    
    if [ -z "$data" ]; then
        curl -s -X $method \
            -H "Content-Type: application/json" \
            "$BASE_URL/wp-json/themathar/v1$endpoint"
    else
        curl -s -X $method \
            -H "Content-Type: application/json" \
            -d "$data" \
            "$BASE_URL/wp-json/themathar/v1$endpoint"
    fi
}

# Store player credentials
declare -a PLAYERS

# Test 1: Create Players
echo -e "${YELLOW}Test 1: Creating players...${NC}"
for i in $(seq 1 $PLAYER_COUNT); do
    PLAYER_NAME="TestPlayer$i"
    echo -n "Creating $PLAYER_NAME... "
    
    RESPONSE=$(call_api "/player/create" "POST" "{\"player_name\": \"$PLAYER_NAME\"}")
    
    if echo "$RESPONSE" | grep -q "\"success\":true"; then
        PLAYER_ID=$(echo $RESPONSE | grep -oP '"player_id":"?\K[^"]*')
        PLAYER_TOKEN=$(echo $RESPONSE | grep -oP '"player_token":"?\K[^"]*')
        echo -e "${GREEN}OK${NC}"
        echo "  ID: $PLAYER_ID"
        echo "  Token: ${PLAYER_TOKEN:0:20}..."
        
        PLAYERS+=("$PLAYER_NAME|$PLAYER_ID|$PLAYER_TOKEN")
    else
        echo -e "${RED}FAILED${NC}"
        echo "Response: $RESPONSE"
        exit 1
    fi
done

echo ""

# Test 2: Get Initial Game State
echo -e "${YELLOW}Test 2: Get game state...${NC}"
RESPONSE=$(call_api "/game/state" "GET")
echo "Initial state:"
echo "$RESPONSE" | jq '.' 2>/dev/null || echo "$RESPONSE"
echo ""

# Test 3: Join Queue
echo -e "${YELLOW}Test 3: Players joining queue...${NC}"
for PLAYER in "${PLAYERS[@]}"; do
    IFS='|' read -r NAME ID TOKEN <<< "$PLAYER"
    echo -n "$NAME joining queue... "
    
    RESPONSE=$(call_api "/queue/join" "POST" "{\"player_id\": \"$ID\", \"player_token\": \"$TOKEN\", \"player_name\": \"$NAME\"}")
    
    if echo "$RESPONSE" | grep -q "\"success\":true"; then
        POSITION=$(echo $RESPONSE | grep -oP '"queue_position":\K[^,}]*')
        IS_ACTIVE=$(echo $RESPONSE | grep -oP '"is_active":\K[^,}]*')
        echo -e "${GREEN}OK${NC} (Position: $POSITION, Active: $IS_ACTIVE)"
    else
        echo -e "${RED}FAILED${NC}"
        echo "Response: $RESPONSE"
    fi
done

echo ""

# Test 4: Check Game State After Joins
echo -e "${YELLOW}Test 4: Game state after joins...${NC}"
RESPONSE=$(call_api "/game/state" "GET")
echo "$RESPONSE" | jq '.data.game_state' 2>/dev/null || echo "$RESPONSE"
echo ""

# Test 5: First Player Ends Turn
echo -e "${YELLOW}Test 5: Active player ends turn...${NC}"
FIRST_PLAYER="${PLAYERS[0]}"
IFS='|' read -r NAME ID TOKEN <<< "$FIRST_PLAYER"
echo "Player $NAME ending turn..."

RESPONSE=$(call_api "/turn/end" "POST" "{\"player_id\": \"$ID\", \"player_token\": \"$TOKEN\", \"is_active_player\": true}")

if echo "$RESPONSE" | grep -q "\"success\":true"; then
    echo -e "${GREEN}Turn ended successfully${NC}"
    echo "$RESPONSE" | jq '.data' 2>/dev/null || echo "$RESPONSE"
else
    echo -e "${RED}Failed to end turn${NC}"
    echo "Response: $RESPONSE"
fi

echo ""

# Test 6: Check Final Game State
echo -e "${YELLOW}Test 6: Final game state...${NC}"
RESPONSE=$(call_api "/game/state" "GET")
echo "$RESPONSE" | jq '.data.game_state' 2>/dev/null || echo "$RESPONSE"
echo ""

# Test 7: Test next player taking turn after timeout
echo -e "${YELLOW}Test 7: Next player taking turn (simulating 60+ second wait)...${NC}"
SECOND_PLAYER="${PLAYERS[1]}"
IFS='|' read -r NAME ID TOKEN <<< "$SECOND_PLAYER"
echo "Player $NAME attempting to take turn (note: may fail if < 60 seconds elapsed)..."

RESPONSE=$(call_api "/turn/end" "POST" "{\"player_id\": \"$ID\", \"player_token\": \"$TOKEN\", \"is_active_player\": false}")

if echo "$RESPONSE" | grep -q "\"success\":true"; then
    echo -e "${GREEN}Successfully took turn${NC}"
    echo "$RESPONSE" | jq '.data' 2>/dev/null || echo "$RESPONSE"
elif echo "$RESPONSE" | grep -q "\"time_remaining\""; then
    TIME_REMAINING=$(echo $RESPONSE | grep -oP '"time_remaining":\K[^,}]*')
    echo -e "${YELLOW}Cannot take turn yet - $TIME_REMAINING seconds remaining${NC}"
else
    echo -e "${YELLOW}Not ready to take turn${NC}"
    echo "Response: $RESPONSE"
fi

echo ""
echo -e "${GREEN}=== Test Complete ===${NC}"
