# POE2 Overlord Frontend

A modern, modular React 19 frontend for the POE2 Overlord overlay application built with Tauri 2, featuring a comprehensive routing system, component library, and real-time game monitoring capabilities.

## Architecture

The frontend has been completely refactored with modern React patterns, TanStack Router for type-safe navigation, and a clear separation of concerns with comprehensive state management:

### Directory Structure

```
src/
├── routes/              # TanStack Router-based routing system
│   ├── __root.tsx      # Root layout with navigation and providers
│   ├── index.tsx       # Dashboard/home page
│   ├── activity.tsx    # Activity monitoring and log display
│   ├── time-tracking.tsx # Time tracking sessions and statistics
│   └── settings.tsx    # Application configuration and settings
├── components/          # Reusable UI components
│   ├── button.tsx      # Button component with variants (primary, secondary, ghost)
│   ├── form/           # Form components with validation
│   │   ├── alert-message.tsx # Alert and error message display
│   │   ├── checkbox-input.tsx # Checkbox input component
│   │   ├── form-field.tsx # Form field wrapper with labels
│   │   ├── select-input.tsx # Select dropdown component
│   │   └── text-input.tsx # Text input component
│   ├── log-monitor/    # Log monitoring components
│   │   ├── activity-log.tsx # Main activity log display
│   │   ├── log-monitor.tsx # Log monitoring container
│   │   ├── monitoring-status.tsx # Monitoring status indicator
│   │   ├── recent-log-lines.tsx # Recent log entries display
│   │   └── scene-event-item.tsx # Individual scene change events
│   ├── page-header.tsx # Page header with navigation
│   ├── settings-form.tsx # Settings configuration form
│   ├── status-bar/     # Status and monitoring components
│   │   ├── status-bar.tsx # Main status bar
│   │   └── status-indicator.tsx # Status indicators
│   ├── time-tracking/  # Time tracking components
│   │   ├── active-sessions.tsx # Currently active sessions
│   │   ├── location-stats.tsx # Location statistics display
│   │   ├── session-history.tsx # Session history table
│   │   ├── stat-card.tsx # Statistics display cards
│   │   └── time-display.tsx # Time formatting utilities
│   ├── loading-spinner.tsx # Loading state component
│   ├── tooltip.tsx     # Tooltip component
│   ├── window-title.tsx # Window title management
│   └── index.ts        # Component exports for clean imports
├── hooks/              # Custom React hooks
│   ├── useGameProcess.ts # Game process management and monitoring
│   ├── useTimeTracking.ts # Time tracking state management
│   ├── useZoneMonitoring.ts # Zone monitoring and event handling
│   └── index.ts        # Hook exports
├── types/              # TypeScript type definitions
│   └── index.ts        # All application types and interfaces
├── utils/              # Utility functions
│   ├── constants.ts    # Application constants and configuration
│   ├── tailwind.ts     # Tailwind CSS configuration and utilities
│   ├── tauri.ts        # Tauri API utilities and command wrappers
│   └── index.ts        # Utility exports
├── globals.css         # Global styles and Tailwind CSS imports
├── main.tsx            # Application entry point with TanStack Router
└── routeTree.gen.ts    # Generated route tree for type safety
```

### Key Features

- **Modern Routing**: TanStack Router for type-safe, performant navigation with automatic code splitting
- **Modular Components**: Each UI element is a separate, reusable component with clear interfaces and proper TypeScript typing
- **Custom Hooks**: Business logic is separated into custom hooks for reusability, testing, and state management
- **Type Safety**: Full TypeScript support with proper interfaces, type definitions, and compile-time validation
- **Real-time Updates**: Live updates from backend services using Tauri's event system
- **Utility Functions**: Centralized utilities for common operations, Tauri integration, and data formatting
- **Constants**: Application configuration centralized in constants file with type safety
- **React 19**: Uses latest React features and modern patterns including hooks, concurrent features, and improved performance
- **Tailwind CSS 4**: Latest version with improved performance and utility-first CSS framework

### Component Design

All components follow these principles:

- **Single Responsibility**: Each component has one clear purpose and well-defined props
- **Props Interface**: Well-defined TypeScript interfaces for props with proper validation
- **Reusability**: Components are designed to be reused across the application
- **Accessibility**: Proper ARIA labels, keyboard navigation support, and semantic HTML
- **Styling**: Consistent use of Tailwind CSS 4 classes with utility-first approach
- **Error Boundaries**: Proper error handling and fallback states

### Styling

- **Tailwind CSS 4**: Latest version with improved performance and utility-first CSS framework
- **Custom Theme**: POE2-inspired dark theme with custom color palette
- **Responsive Design**: Components adapt to different screen sizes and orientations
- **Consistent Spacing**: Uses Tailwind's spacing scale for consistency across components
- **Dark Mode**: Optimized dark theme for gaming environments
- **Sharp Design**: Fantasy-adjacent design with sharp edges rather than soft, rounded material design

### State Management

- **Custom Hooks**: Local state managed through custom hooks with proper cleanup and lifecycle management
- **Tauri Integration**: Backend communication through custom utilities and command wrappers with error handling
- **Event Handling**: Proper cleanup of event listeners and async operations with React 19 patterns
- **Game Process Monitoring**: Real-time updates of Path of Exile 2 game process status with live status indicators
- **Time Tracking State**: Comprehensive time tracking state management with session handling and statistics
- **Zone Monitoring**: Real-time zone and scene change monitoring with event subscriptions
- **Route-based State**: State management tied to specific routes for better organization and performance
- **Error Boundaries**: Proper error handling and fallback states throughout the application

## Available Components

### Core Components

- **Button**: Versatile button component with multiple variants and sizes
- **Loading Spinner**: Loading state component for async operations
- **Tooltip**: Contextual help and information display
- **Window Title**: Window title management for overlay behavior

### Form Components

- **Form Field**: Wrapper component for form inputs with labels and validation
- **Text Input**: Text input with validation and error states
- **Select Input**: Dropdown selection component
- **Checkbox Input**: Checkbox with proper accessibility
- **Alert Message**: Error and success message display

### Layout Components

- **Page Header**: Main application header with navigation
- **Status Bar**: Status indicators and monitoring information
- **Status Indicator**: Visual status indicators for various states

### Log Monitoring Components

- **Log Monitor**: Main log monitoring container with real-time updates
- **Activity Log**: Activity log display with filtering and search capabilities
- **Monitoring Status**: Real-time monitoring status with connection indicators
- **Recent Log Lines**: Recent log entries with formatting and timestamp display
- **Scene Event Item**: Individual scene change event display with type-specific styling

### Time Tracking Components

- **Active Sessions**: Currently active time tracking sessions with real-time updates
- **Location Stats**: Statistics for different game locations with aggregated data
- **Session History**: Historical session data with filtering and search
- **Stat Card**: Statistics display cards with formatted time and visit counts
- **Time Display**: Time formatting utilities with human-readable formats

## Development

### Adding New Routes

1. Create the route component in `src/routes/`
2. Define the route in the router configuration
3. Add navigation links in the appropriate components
4. Ensure proper TypeScript typing for route parameters

### Adding New Components

1. Create the component in `src/components/` with proper TypeScript interfaces
2. Export from `src/components/index.ts`
3. Use in the appropriate routes or other components
4. Follow the established component patterns and styling guidelines

### Adding New Hooks

1. Create the hook in `src/hooks/` with proper cleanup
2. Export from `src/hooks/index.ts`
3. Use in components as needed
4. Ensure proper cleanup of resources and event listeners

### Styling Guidelines

- Use Tailwind CSS 4 utility classes for styling
- Maintain consistent spacing using Tailwind's scale
- Follow the sharp, fantasy-adjacent design aesthetic
- Keep components responsive and accessible
- Use consistent color schemes from the POE2 theme

### TypeScript Best Practices

- Define interfaces for all component props
- Use proper typing for Tauri commands and responses
- Leverage TypeScript's type inference where possible
- Export types from `src/types/index.ts` for consistency
- Use proper typing for route parameters and navigation

## Dependencies

### Core Framework

- **React 19**: Latest React with modern features, improved performance, and concurrent features
- **TypeScript 5.8**: Full type safety and modern TypeScript features with strict configuration
- **TanStack Router**: Type-safe routing with automatic code splitting and performance optimizations
- **Tauri API 2.8**: Desktop application framework integration with command and event support

### Styling & UI

- **Tailwind CSS 4**: Latest version with improved performance, features, and utility-first approach
- **Heroicons**: Modern icon library with consistent design and React components
- **clsx + tailwind-merge**: Class name utilities for conditional styling and class merging

### Development Tools

- **Vite 7**: Fast development server and build tool with HMR support
- **ESLint 9**: Code quality and consistency with modern configuration
- **Prettier**: Code formatting with consistent style
- **@tanstack/router-plugin**: Vite plugin for TanStack Router integration

## Development Scripts

- `pnpm dev` - Start Vite dev server with hot reloading
- `pnpm build` - Build the frontend for production
- `pnpm lint` - Run ESLint for code quality
- `pnpm preview` - Preview the production build
- `pnpm format` - Format code with Prettier
- `pnpm clean` - Clean build artifacts
- `pnpm tauri:dev` - Start Tauri development mode
- `pnpm tauri:build` - Build Tauri application

## Integration with Backend

The frontend communicates with the Rust backend through a comprehensive integration layer:

### Tauri Commands

- **Direct Function Calls**: Rust backend functions called through custom utilities with proper error handling
- **Type-safe Commands**: All commands are properly typed with TypeScript interfaces
- **Error Handling**: Comprehensive error handling with user-friendly error messages
- **Async Operations**: Proper handling of async operations with loading states

### Event System

- **Real-time Updates**: Live updates from backend services via Tauri event subscriptions
- **Event Types**: Support for log events, time tracking events, and server status events
- **Event Filtering**: Frontend can filter and process specific event types
- **Event Cleanup**: Proper cleanup of event listeners and subscriptions

### Specific Integrations

- **Game Process Monitoring**: Live status updates of Path of Exile 2 game process with real-time indicators
- **Time Tracking**: Real-time time tracking updates with session management
- **Log Monitoring**: Live log file monitoring with scene change detection
- **Configuration Management**: Dynamic configuration updates with validation
- **Server Status**: Real-time server status monitoring with ping information

## Performance Considerations

- **React 19**: Improved rendering performance, concurrent features, and automatic batching
- **Code Splitting**: TanStack Router provides automatic route-based code splitting for optimal bundle sizes
- **Tailwind CSS 4**: Improved CSS generation, optimization, and reduced bundle size
- **Bundle Optimization**: Vite provides efficient bundling, tree shaking, and modern ES modules
- **Lazy Loading**: Components and routes are loaded on-demand for optimal performance
- **Type Safety**: TanStack Router provides compile-time route validation and type safety
- **Event Optimization**: Efficient event handling with proper cleanup and minimal re-renders
- **State Management**: Optimized state management with proper memoization and dependency arrays

## Routing System

The application uses TanStack Router for type-safe navigation with comprehensive features:

- **Route-based Code Splitting**: Automatic code splitting for better performance and smaller bundle sizes
- **Type-safe Navigation**: Compile-time route validation, parameter typing, and search parameter validation
- **Nested Routes**: Support for complex routing hierarchies with proper layout management
- **Route Guards**: Protection for authenticated or protected routes with proper redirects
- **Search Params**: Type-safe handling of URL search parameters with validation
- **Route Preloading**: Automatic route preloading for improved user experience
- **Route Transitions**: Smooth transitions between routes with loading states
- **Generated Route Tree**: Auto-generated route tree for type safety and performance

## Error Handling

- **Error Boundaries**: Proper error handling at component and route levels with fallback UI
- **Form Validation**: Client-side validation with user-friendly error messages and real-time feedback
- **Network Errors**: Graceful handling of Tauri command failures with retry mechanisms
- **Loading States**: Proper loading states for async operations with skeleton screens
- **Fallback UI**: Fallback components for error scenarios with recovery options
- **Error Logging**: Comprehensive error logging for debugging and monitoring
- **User Feedback**: Clear error messages and success notifications for user actions

## Accessibility

- **ARIA Labels**: Proper accessibility attributes for screen readers and assistive technologies
- **Keyboard Navigation**: Full keyboard navigation support with proper tab order and shortcuts
- **Color Contrast**: High contrast design for better visibility and readability
- **Semantic HTML**: Proper HTML semantics for better accessibility and SEO
- **Focus Management**: Proper focus handling and management with visible focus indicators
- **Screen Reader Support**: Comprehensive screen reader support with proper announcements
- **Responsive Design**: Accessible design across different screen sizes and devices
