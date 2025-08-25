# POE2 Master Frontend

A modern, modular React frontend for the POE2 Master overlay application built with Tauri.

## Architecture

The frontend has been refactored to follow modern React patterns with a clear separation of concerns:

### Directory Structure

```
src/
├── components/          # Reusable UI components
│   ├── Button.tsx      # Button component with variants
│   ├── Footer.tsx      # Application footer
│   ├── InfoPanel.tsx   # Information display panel
│   ├── ProcessStatus.tsx # POE2 process status display
│   ├── QuickActions.tsx # Action buttons grid
│   ├── StatusDot.tsx   # Status indicator component
│   ├── TitleBar.tsx    # Application title bar
│   ├── WindowControls.tsx # Window management controls
│   └── index.ts        # Component exports
├── hooks/              # Custom React hooks
│   ├── usePoe2Process.ts # POE2 process management
│   ├── useWindowControls.ts # Window control state
│   └── index.ts        # Hook exports
├── types/              # TypeScript type definitions
│   └── index.ts        # All application types
├── utils/              # Utility functions
│   ├── cn.ts           # Class name merging utility
│   ├── constants.ts    # Application constants
│   ├── tauri.ts        # Tauri API utilities
│   └── index.ts        # Utility exports
├── App.tsx             # Main application component
├── App.css             # Application-specific styles
├── index.css           # Global styles and Tailwind imports
└── main.tsx            # Application entry point
```

### Key Features

- **Modular Components**: Each UI element is a separate, reusable component
- **Custom Hooks**: Business logic is separated into custom hooks
- **Type Safety**: Full TypeScript support with proper interfaces
- **Utility Functions**: Centralized utilities for common operations
- **Constants**: Application configuration centralized in constants file
- **Modern React**: Uses React 19 features and modern patterns

### Component Design

All components follow these principles:

- **Single Responsibility**: Each component has one clear purpose
- **Props Interface**: Well-defined TypeScript interfaces for props
- **Reusability**: Components are designed to be reused across the application
- **Accessibility**: Proper ARIA labels and keyboard navigation support

### Styling

- **Tailwind CSS**: Utility-first CSS framework
- **Custom Theme**: POE2-inspired dark theme with custom color palette
- **Responsive Design**: Components adapt to different screen sizes
- **Consistent Spacing**: Uses Tailwind's spacing scale for consistency

### State Management

- **Custom Hooks**: Local state managed through custom hooks
- **Tauri Integration**: Backend communication through custom utilities
- **Event Handling**: Proper cleanup of event listeners

## Development

### Adding New Components

1. Create the component in `src/components/`
2. Define the props interface in `src/types/`
3. Export from `src/components/index.ts`
4. Use in the main App component

### Adding New Hooks

1. Create the hook in `src/hooks/`
2. Export from `src/hooks/index.ts`
3. Use in components as needed

### Styling Guidelines

- Use Tailwind utility classes for styling
- Follow the established color palette in `tailwind.config.js`
- Maintain consistent spacing using Tailwind's scale
- Use the `cn()` utility for conditional class names

## Dependencies

- **React 19**: Latest React with modern features
- **TypeScript**: Full type safety
- **Tailwind CSS**: Utility-first styling
- **Lucide React**: Icon library
- **Tauri API**: Desktop application framework
- **clsx + tailwind-merge**: Class name utilities
