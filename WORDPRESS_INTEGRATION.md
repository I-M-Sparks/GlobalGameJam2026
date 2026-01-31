# WordPress Integration Guide

The game now supports cross-browser lobbies via WordPress backend. Here's how to use it:

## How It Works

The game is served from `http://localhost:8000`, but it can fetch and sync lobbies from a WordPress instance running elsewhere. You specify the WordPress URL via a query parameter.

## Usage

### 1. With WordPress Running Locally (Default)

If WordPress is running on the same machine at `http://localhost`, just open:

```
http://localhost:8000
```

The game will automatically use `http://localhost` as the WordPress backend.

### 2. With WordPress at a Different Address

If WordPress is running at a different IP/URL (e.g., on a different machine or different port), pass it as a query parameter:

```
http://localhost:8000?wp=http://192.168.1.100
http://localhost:8000?wp=http://example.com
http://localhost:8000?wp=http://wordpress.local:3000
```

Replace `192.168.1.100`, `example.com`, or `wordpress.local:3000` with your actual WordPress URL.

## What Happens

1. **Create Lobby**: When you create a lobby in the game, it sends a POST request to:
   ```
   {WORDPRESS_URL}/wp-json/themathar/v1/lobbies
   ```

2. **Fetch Lobbies**: The game polls every 1 second to fetch available lobbies from:
   ```
   {WORDPRESS_URL}/wp-json/themathar/v1/lobbies
   ```

3. **Cross-Browser Sync**: Since both browsers are fetching from the same WordPress backend, they'll see the same lobbies.

## Requirements

- WordPress must have the Themathar plugin activated
- WordPress REST API must be accessible
- Browser console logs show API calls for debugging

## Debug Logging

Open browser dev tools (F12) and look at the Console tab to see debug messages:

- 📡 POLL: Polling status
- 📡 Fetching lobbies from: [URL]
- ✅ Lobby creation sent to backend at [URL]
- 🎮 LOBBY CREATED: Shows local lobby creation
- 📡 CLIENT: Shows what's in local cache

## Troubleshooting

If you see "GET http://... 404 (File not found)":

1. Check that WordPress is running
2. Verify the WordPress URL is correct (with ?wp=... parameter if needed)
3. Verify the Themathar plugin is activated in WordPress
4. Check browser console for the exact URL being called

## Example Scenarios

### Scenario 1: Local Development
- WordPress: `http://localhost`
- Game: `http://localhost:8000`
- URL: `http://localhost:8000`

### Scenario 2: Separate Machines
- WordPress on Server: `http://192.168.1.50`
- Game on Computer: `http://localhost:8000?wp=http://192.168.1.50`

### Scenario 3: Docker/WSL
- WordPress in WSL: `http://172.30.44.51`
- Game on Windows: `http://localhost:8000?wp=http://172.30.44.51`
