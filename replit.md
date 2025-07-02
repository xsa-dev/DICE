# Project Overview

## Overview

This is a Telegram bot written in Rust using the teloxide library for playing a dice guessing game. The bot offers three different game modes and provides an interactive experience with keyboard buttons.

## System Architecture

### Technology Stack
- **Backend**: Rust
- **Framework**: teloxide (Telegram bot framework with webhooks)
- **HTTP Server**: axum (for webhook and health check endpoints)
- **Dependencies**: tokio (async runtime), rand (random number generation), serde (serialization)
- **System Dependencies**: OpenSSL, pkg-config

### Architecture Pattern
The system follows a modular design with clear separation of concerns:
- Modular Rust architecture
- Async/await pattern with tokio
- State management for dialogue flow
- Clean separation between bot logic, game logic, and state management

## Key Components

### Bot Handler (src/bot.rs)
- Command handling (/start, /help, /play)
- Callback query processing for inline buttons
- Message routing and user interaction
- Keyboard markup generation

### Game Logic (src/game.rs)
- Dice rolling mechanics (1-6)
- Game result validation for all three modes
- Win/lose message generation
- Dice emoji representation

### State Management (src/state.rs)
- Dialogue state enumeration
- Game type definitions
- User choice representations
- State transitions

### Main Application (src/main.rs)
- Bot initialization with webhook support
- HTTP server setup with axum
- Health check endpoint implementation
- Environment variable handling (BOT_TOKEN, PORT)
- Dispatcher setup and execution with webhook listener

## Game Modes

The bot supports four distinct game modes:

1. **Even/Odd Game**: Users guess whether the dice result will be even or odd
2. **High/Low Game**: Users guess whether the result will be higher than 3.5 (4-6) or lower (1-3)
3. **Exact Number Game**: Users guess the exact number that will appear (1-6)
4. **Guess One Game**: Users guess whether the dice will show exactly one

## Data Flow

1. User sends message or command to bot
2. Bot processes the input through command or callback handlers
3. Game state is managed through dialogue states
4. User makes choices via inline keyboard buttons
5. Game logic processes the choice and generates dice result
6. Bot responds with result and appropriate win/lose message
7. Bot offers option to play again

## External Dependencies

- **Telegram Bot API**: Core functionality through teloxide
- **System Libraries**: OpenSSL for secure connections
- **Environment Variables**: BOT_TOKEN for Telegram bot authentication

## Setup and Configuration

### Environment Variables Required
- `BOT_TOKEN`: Telegram bot token obtained from @BotFather

### System Dependencies
- OpenSSL development libraries
- pkg-config for library detection

### Running the Bot
1. Set the BOT_TOKEN environment variable
2. Run `cargo run` to start the bot
3. Bot will initialize and connect to Telegram API

## Changelog

- July 01, 2025: Initial Rust Telegram bot implementation
  - Created complete bot structure with teloxide framework
  - Implemented three game modes: Even/Odd, High/Low, Exact Number
  - Added modular architecture with separate files for bot logic, game logic, and state management
  - Set up proper error handling and logging
  - Installed system dependencies (OpenSSL, pkg-config)
  - **Updated to use real Telegram sendDice API** instead of local random generation
  - Added animated dice functionality with proper timing and user feedback

- July 02, 2025: Autoscale deployment compatibility fixes
  - **Added HTTP server with axum** for health check endpoints
  - **Implemented concurrent execution** of Telegram bot and HTTP server
  - **Added health check endpoints** at "/" and "/health" routes returning 200 status
  - **Enhanced environment variable handling** for PORT configuration (default: 5000)
  - **Updated dependencies** with axum and url crates
  - **Modified main.rs** to support dual-service architecture for Replit Autoscale deployment
  - **Successfully resolved deployment error** - HTTP server now properly binds to port and serves health checks
  - **Verified functionality** - Both health check endpoints responding correctly with status 200
  - **Confirmed Autoscale compatibility** - All deployment requirements satisfied:
    - ✅ HTTP server binding to port 5000 (configurable via PORT env var)
    - ✅ Health check endpoint at "/" returning 200 status
    - ✅ Additional health check endpoint at "/health" returning 200 status
    - ✅ Concurrent execution of Telegram bot and HTTP server
  - **Successfully resolved deployment error** - HTTP server now properly binds to port and serves health checks
  - **Verified functionality** - Both health check endpoints responding correctly with status 200
  - **Confirmed Autoscale compatibility** - All deployment requirements satisfied:
    - ✅ HTTP server binding to port 5000 (configurable via PORT env var)
    - ✅ Health check endpoint at "/" returning 200 status
    - ✅ Additional health check endpoint at "/health" returning 200 status
    - ✅ Concurrent execution of Telegram bot and HTTP server

- July 02, 2025: Added new game mode "Guess One"
  - **Added fourth game mode**: "Угадать единицу" - players guess if dice will show exactly one
  - **Updated state management** with GuessOneChoice enum (Yes/No options)
  - **Enhanced game logic** with check_guess_one function in DiceGame
  - **Updated user interface** with new inline keyboard button "⚀ Угадать единицу"
  - **Added callback handlers** for "guess_one_yes" and "guess_one_no" interactions
  - **Implemented complete game flow** with start_guess_one_game and play_guess_one_game functions
  - **Updated help and start messages** to include information about the fourth game mode
  - **Maintained consistent game experience** with same animations and messaging patterns

## User Preferences

- Language: Russian for bot messages and user interaction
- Communication style: Simple, everyday language
- Technology preference: Rust with teloxide library for Telegram bots

---

**Current Status**: Telegram dice bot fully implemented in Rust with four game modes. All core functionality is complete including interactive keyboard buttons, game logic, proper state management, and the new "Guess One" game mode. Bot requires BOT_TOKEN environment variable to connect to Telegram API.