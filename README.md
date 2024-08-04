# Rustgram

A Telegram bot built with Rust and Tauri. This project integrates a Telegram bot with a desktop application, providing a seamless interface to control the bot's functionalities.

## Features

- **Real-time Updates:** Get real-time updates from Telegram.
- **Customizable Keyboards:** Easily create and manage custom keyboards for user interactions.
- **Efficient State Management:** Robust state management using Rust's concurrency features.
- **User-Friendly Interface:** Control your bot through a desktop application built with Tauri.

## Installation

Follow these steps to set up and run the project:

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Yarn](https://yarnpkg.com/)

### Clone the Repository

```sh
git clone https://github.com/awfulfacekid/rustgram.git
```

### Navigate to the Project Directory

```sh
cd rustgram
```

### Install Dependencies

```sh
yarn
```

### Running the Application
To run the application in development mode:


```sh
yarn tauri dev
```

To build the application for production:

```sh
yarn tauri build
```


### Usage
Once the application is running, you can interact with your Telegram bot through the provided interface. Configure the bot token, set up greeting messages, and manage button responses easily from the desktop application.
