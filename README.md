# POE2 Overlord

A comprehensive desktop application for tracking and analyzing Path of Exile 2 gameplay data, built with Tauri, React, and TypeScript.

## Overview

POE2 Overlord is a cross-platform desktop application that provides real-time monitoring and analysis of Path of Exile 2 gameplay. It features character tracking, zone analysis, walkthrough guides, and comprehensive data visualization.

## Architecture

### Frontend
- **React 18** with TypeScript for type safety
- **TanStack Query** for data fetching and caching
- **Tailwind CSS** for styling
- **Tauri** for desktop integration

### Backend
- **Rust** for high-performance backend services
- **Tauri** for desktop application framework
- **SQLite** for local data storage

## Key Features

### Character Management
- Real-time character data tracking
- Character creation, editing, and deletion
- Active character management
- Character statistics and analytics

### Zone Analysis
- Zone visit tracking and statistics
- Zone filtering and sorting
- Performance analytics
- Death tracking and analysis

### Walkthrough System
- Interactive walkthrough guides
- Progress tracking
- Zone-specific guidance
- Achievement tracking

### Real-time Monitoring
- Game process monitoring
- Server status tracking
- Event-driven updates
- Automatic data synchronization

## Getting Started

### Prerequisites
- Node.js 18+ and Yarn
- Rust 1.70+
- Tauri CLI

### Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd poe2-overlord
```

2. Install dependencies:
```bash
yarn install
```

3. Build and run:
```bash
yarn tauri dev
```

## Project Structure

```
packages/
├── frontend/          # React frontend application
│   ├── src/
│   │   ├── components/    # React components
│   │   ├── hooks/         # Custom React hooks
│   │   ├── routes/        # Application routes
│   │   ├── types/         # TypeScript type definitions
│   │   └── utils/         # Utility functions
│   └── package.json
├── backend/           # Rust backend application
│   ├── src/
│   │   ├── domain/        # Domain logic
│   │   ├── infrastructure/ # Infrastructure layer
│   │   └── application/   # Application services
│   └── Cargo.toml
└── README.md
```

## Hooks Architecture

The frontend uses a modular hook architecture for better maintainability and reusability. See the [Hooks Documentation](./packages/frontend/src/hooks/README.md) for detailed information.

### Key Hook Categories

- **Data Hooks**: Character data, zone data, walkthrough data
- **Event Hooks**: Tauri event listeners, real-time updates
- **Filtering Hooks**: Data filtering and sorting
- **Utility Hooks**: Error handling, caching, configuration

### Migration Guide

If you're updating from an older version, see the [Migration Guide](./packages/frontend/src/hooks/MIGRATION_GUIDE.md) for step-by-step instructions.

## Development

### Frontend Development
```bash
cd packages/frontend
yarn dev
```

### Backend Development
```bash
cd packages/backend
cargo run
```

### Full Application
```bash
yarn tauri dev
```

## Building

### Development Build
```bash
yarn tauri build --debug
```

### Production Build
```bash
yarn tauri build
```

## Testing

### Frontend Tests
```bash
cd packages/frontend
yarn test
```

### Backend Tests
```bash
cd packages/backend
cargo test
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Submit a pull request

### Code Style

- **Frontend**: ESLint + Prettier
- **Backend**: rustfmt
- **TypeScript**: Strict mode enabled
- **React**: Functional components with hooks

## Documentation

- [Hooks Documentation](./packages/frontend/src/hooks/README.md)
- [Migration Guide](./packages/frontend/src/hooks/MIGRATION_GUIDE.md)
- [API Documentation](./packages/backend/README.md)

## Performance

The application is optimized for performance with:
- React Query for efficient data caching
- Automatic cache invalidation on events
- Optimized re-renders with proper dependencies
- Dead code elimination
- TypeScript for compile-time optimizations

## Error Handling

The application uses standardized error handling patterns:
- Consistent error types across all hooks
- User-friendly error messages
- Automatic error recovery for recoverable errors
- React error boundary integration

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Support

For questions, issues, or contributions:
- Create an issue on GitHub
- Check the documentation
- Review the migration guide
- Test with the provided examples
