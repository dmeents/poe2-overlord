# POE2 Overlord Frontend

A modern, modular React 19 frontend for the POE2 Overlord overlay application built with Tauri 2, featuring a comprehensive routing system and component library.

## Architecture

The frontend has been completely refactored with modern React patterns, TanStack Router for navigation, and a clear separation of concerns:

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
│   ├── usePoe2Process.ts # POE2 process management and monitoring
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

- **Modern Routing**: TanStack Router for type-safe, performant navigation with code splitting
- **Modular Components**: Each UI element is a separate, reusable component with clear interfaces
- **Custom Hooks**: Business logic is separated into custom hooks for reusability and testing
- **Type Safety**: Full TypeScript support with proper interfaces and type definitions
- **Utility Functions**: Centralized utilities for common operations and Tauri integration
- **Constants**: Application configuration centralized in constants file
- **React 19**: Uses latest React features and modern patterns including hooks and concurrent features

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

- **Custom Hooks**: Local state managed through custom hooks with proper cleanup
- **Tauri Integration**: Backend communication through custom utilities and command wrappers
- **Event Handling**: Proper cleanup of event listeners and async operations
- **Process Monitoring**: Real-time updates of POE2 process status
- **Route-based State**: State management tied to specific routes for better organization

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

- **Log Monitor**: Main log monitoring container
- **Activity Log**: Activity log display with filtering
- **Monitoring Status**: Real-time monitoring status
- **Recent Log Lines**: Recent log entries with formatting
- **Scene Event Item**: Individual scene change event display

### Time Tracking Components

- **Active Sessions**: Currently active time tracking sessions
- **Location Stats**: Statistics for different game locations
- **Session History**: Historical session data
- **Stat Card**: Statistics display cards
- **Time Display**: Time formatting utilities

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

- **React 19**: Latest React with modern features and improved performance
- **TypeScript 5.8**: Full type safety and modern TypeScript features
- **TanStack Router**: Type-safe routing with code splitting and performance optimizations
- **Tailwind CSS 4**: Latest version with improved performance and features
- **Heroicons**: Modern icon library with consistent design
- **Tauri API 2.8**: Desktop application framework integration
- **clsx + tailwind-merge**: Class name utilities for conditional styling

## Development Scripts

- `yarn dev` - Start Vite dev server with hot reloading
- `yarn build` - Build the frontend for production
- `yarn lint` - Run ESLint for code quality
- `yarn preview` - Preview the production build
- `yarn format` - Format code with Prettier
- `yarn clean` - Clean build artifacts
- `yarn tauri:dev` - Start Tauri development mode
- `yarn tauri:build` - Build Tauri application

## Integration with Backend

The frontend communicates with the Rust backend through:

- **Tauri Commands**: Direct function calls to Rust backend through custom utilities
- **Event System**: Real-time updates from backend services via subscriptions
- **Process Monitoring**: Live status updates of POE2 process
- **Window Management**: Control overlay positioning and behavior
- **File Monitoring**: Real-time log file monitoring and updates

## Performance Considerations

- **React 19**: Improved rendering performance and concurrent features
- **Code Splitting**: TanStack Router provides automatic route-based code splitting
- **Tailwind CSS 4**: Improved CSS generation and optimization
- **Bundle Optimization**: Vite provides efficient bundling and tree shaking
- **Lazy Loading**: Components are loaded on-demand for optimal performance
- **Type Safety**: TanStack Router provides compile-time route validation

## Routing System

The application uses TanStack Router for type-safe navigation:

- **Route-based Code Splitting**: Automatic code splitting for better performance
- **Type-safe Navigation**: Compile-time route validation and parameter typing
- **Nested Routes**: Support for complex routing hierarchies
- **Route Guards**: Protection for authenticated or protected routes
- **Search Params**: Type-safe handling of URL search parameters

## Error Handling

- **Error Boundaries**: Proper error handling at component and route levels
- **Form Validation**: Client-side validation with user-friendly error messages
- **Network Errors**: Graceful handling of Tauri command failures
- **Loading States**: Proper loading states for async operations
- **Fallback UI**: Fallback components for error scenarios

## Accessibility

- **ARIA Labels**: Proper accessibility attributes for screen readers
- **Keyboard Navigation**: Full keyboard navigation support
- **Color Contrast**: High contrast design for better visibility
- **Semantic HTML**: Proper HTML semantics for better accessibility
- **Focus Management**: Proper focus handling and management
