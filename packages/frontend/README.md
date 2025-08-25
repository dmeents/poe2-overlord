# POE2 Overlord Frontend

A modern, modular React 19 frontend for the POE2 Overlord overlay application built with Tauri 2.

## Architecture

The frontend has been built with modern React patterns and a clear separation of concerns:

### Directory Structure

```
src/
├── components/          # Reusable UI components
│   ├── Button.tsx      # Button component with variants (primary, secondary, ghost)
│   ├── Footer.tsx      # Application footer with version and technology info
│   ├── InfoPanel.tsx   # Information display panel with customizable content
│   ├── ProcessStatus.tsx # POE2 process status display with refresh capability
│   ├── QuickActions.tsx # Action buttons grid for common operations
│   ├── StatusDot.tsx   # Status indicator component (red/green for offline/online)
│   ├── TitleBar.tsx    # Application title bar with window controls
│   ├── WindowControls.tsx # Window management controls (minimize, close)
│   └── index.ts        # Component exports for clean imports
├── hooks/              # Custom React hooks
│   ├── usePoe2Process.ts # POE2 process management and monitoring
│   ├── useWindowControls.ts # Window control state and operations
│   └── index.ts        # Hook exports
├── types/              # TypeScript type definitions
│   └── index.ts        # All application types and interfaces
├── utils/              # Utility functions
│   ├── cn.ts           # Class name merging utility (clsx + tailwind-merge)
│   ├── constants.ts    # Application constants and configuration
│   ├── tauri.ts        # Tauri API utilities and command wrappers
│   └── index.ts        # Utility exports
├── App.tsx             # Main application component with layout
├── index.css           # Global styles and Tailwind CSS imports
└── main.tsx            # Application entry point with React 19
```

### Key Features

- **Modular Components**: Each UI element is a separate, reusable component with clear interfaces
- **Custom Hooks**: Business logic is separated into custom hooks for reusability
- **Type Safety**: Full TypeScript support with proper interfaces and type definitions
- **Utility Functions**: Centralized utilities for common operations and Tauri integration
- **Constants**: Application configuration centralized in constants file
- **Modern React**: Uses React 19 features and modern patterns including hooks

### Component Design

All components follow these principles:

- **Single Responsibility**: Each component has one clear purpose and well-defined props
- **Props Interface**: Well-defined TypeScript interfaces for props with proper validation
- **Reusability**: Components are designed to be reused across the application
- **Accessibility**: Proper ARIA labels, keyboard navigation support, and semantic HTML
- **Styling**: Consistent use of Tailwind CSS classes with the `cn()` utility for conditional styling

### Styling

- **Tailwind CSS 4**: Latest version with utility-first CSS framework
- **Custom Theme**: POE2-inspired dark theme with custom color palette
- **Responsive Design**: Components adapt to different screen sizes and orientations
- **Consistent Spacing**: Uses Tailwind's spacing scale for consistency across components
- **Dark Mode**: Optimized dark theme for gaming environments

### State Management

- **Custom Hooks**: Local state managed through custom hooks with proper cleanup
- **Tauri Integration**: Backend communication through custom utilities and command wrappers
- **Event Handling**: Proper cleanup of event listeners and async operations
- **Process Monitoring**: Real-time updates of POE2 process status

## Available Components

### Core Components

- **Button**: Versatile button component with multiple variants and sizes
- **StatusDot**: Visual indicator for process status (red/green)
- **TitleBar**: Main application header with process status and window controls

### Layout Components

- **InfoPanel**: Flexible information display with customizable content
- **Footer**: Application footer with version and technology information
- **QuickActions**: Grid of action buttons for common operations

### Window Management

- **WindowControls**: Minimize and close window functionality
- **ProcessStatus**: Display and refresh POE2 process information

## Development

### Adding New Components

1. Create the component in `src/components/`
2. Define the props interface in `src/types/` if needed
3. Export from `src/components/index.ts`
4. Use in the main App component or other components

### Adding New Hooks

1. Create the hook in `src/hooks/`
2. Export from `src/hooks/index.ts`
3. Use in components as needed
4. Ensure proper cleanup of resources and event listeners

### Styling Guidelines

- Use Tailwind utility classes for styling
- Follow the established color palette in `tailwind.config.js`
- Maintain consistent spacing using Tailwind's scale
- Use the `cn()` utility for conditional class names
- Keep components responsive and accessible

### TypeScript Best Practices

- Define interfaces for all component props
- Use proper typing for Tauri commands and responses
- Leverage TypeScript's type inference where possible
- Export types from `src/types/index.ts` for consistency

## Dependencies

- **React 19**: Latest React with modern features and improved performance
- **TypeScript 5.8**: Full type safety and modern TypeScript features
- **Tailwind CSS 4**: Latest version with improved performance and features
- **Lucide React**: Modern icon library with consistent design
- **Tauri API 2.8**: Desktop application framework integration
- **clsx + tailwind-merge**: Class name utilities for conditional styling

## Development Scripts

- `yarn dev` - Start Vite dev server with hot reloading
- `yarn build` - Build the frontend for production
- `yarn lint` - Run ESLint for code quality
- `yarn preview` - Preview the production build
- `yarn format` - Format code with Prettier
- `yarn clean` - Clean build artifacts

## Integration with Backend

The frontend communicates with the Rust backend through:

- **Tauri Commands**: Direct function calls to Rust backend
- **Event System**: Real-time updates from backend services
- **Process Monitoring**: Live status updates of POE2 process
- **Window Management**: Control overlay positioning and behavior

## Performance Considerations

- **React 19**: Improved rendering performance and concurrent features
- **Code Splitting**: Components are properly split for optimal loading
- **Tailwind CSS 4**: Improved CSS generation and optimization
- **Bundle Optimization**: Vite provides efficient bundling and tree shaking
