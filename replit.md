# Project Overview

## Overview

This is a Telegram bot written in Rust using the teloxide library for playing a dice guessing game. The bot offers three different game modes and provides an interactive experience with keyboard buttons.

## System Architecture

### Technology Stack
- **Backend**: Rust
- **Framework**: teloxide (Telegram bot framework)
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
- Bot initialization
- Environment variable handling
- Dispatcher setup and execution

## Game Modes

The bot supports three distinct game modes:

1. **Even/Odd Game**: Users guess whether the dice result will be even or odd
2. **High/Low Game**: Users guess whether the result will be higher than 3.5 (4-6) or lower (1-3)
3. **Exact Number Game**: Users guess the exact number that will appear (1-6)

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

## User Preferences

- Language: Russian for bot messages and user interaction
- Communication style: Simple, everyday language
- Technology preference: Rust with teloxide library for Telegram bots

---

**Current Status**: Telegram dice bot fully implemented in Rust. All core functionality is complete including interactive keyboard buttons, game logic, and proper state management. Bot requires BOT_TOKEN environment variable to connect to Telegram API.