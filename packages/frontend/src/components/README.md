# General-Purpose Components

This directory contains general-purpose components that can be used throughout the application. These components are not specific to any particular feature but provide common functionality like loading states, tooltips, and UI elements.

## Components

### Button

A versatile button component with multiple variants and sizes.

```tsx
import { Button } from '@/components';

<Button variant='primary' size='md' onClick={handleClick}>
  Click me
</Button>;
```

**Variants**: `primary`, `secondary`, `outline`, `ghost`, `icon`
**Sizes**: `xs`, `sm`, `md`, `lg`

### LoadingSpinner

A loading spinner component with customizable message. Perfect for indicating loading states throughout the application.

```tsx
import { LoadingSpinner } from '@/components';

<LoadingSpinner message="Loading data..." />
<LoadingSpinner message="Processing..." className="my-4" />
```

**Props**:

- `message`: The text to display next to the spinner
- `className`: Additional CSS classes for styling

### Tooltip

A tooltip component that displays additional information on hover. Great for providing inline help without cluttering the UI.

```tsx
import { Tooltip } from '@/components';

// Simple text tooltip
<Tooltip content="This is helpful information">
  <span>Hover over me</span>
</Tooltip>

// Rich content tooltip
<Tooltip
  content={
    <div>
      <p><strong>Title:</strong> Description</p>
      <ul className="list-disc list-inside">
        <li>Point 1</li>
        <li>Point 2</li>
      </ul>
    </div>
  }
>
  <span>Rich tooltip</span>
</Tooltip>
```

**Props**:

- `content`: The tooltip content (string or React element)
- `children`: The element to attach the tooltip to
- `className`: Additional CSS classes for styling

### StatusBar

A status bar component that displays application status information.

```tsx
import { StatusBar } from '@/components';

<StatusBar />;
```

### StatusIndicator

A status indicator component for showing different states with icons.

```tsx
import { StatusIndicator } from '@/components';
import { ComputerDesktopIcon } from '@heroicons/react/16/solid';

<StatusIndicator status='success' icon={<ComputerDesktopIcon />} size='sm' />;
```

**Props**:

- `status`: Status type (`success`, `warning`, `error`, `info`)
- `icon`: Icon element to display
- `size`: Size variant (`sm`, `md`, `lg`)
- `className`: Additional CSS classes

### WindowTitle

A window title bar component for the desktop application.

```tsx
import { WindowTitle } from '@/components';

<WindowTitle />;
```

## Usage

All components are exported from the main components index:

```tsx
import {
  Button,
  LoadingSpinner,
  Tooltip,
  StatusBar,
  StatusIndicator,
  WindowTitle,
} from '@/components';
```

## Design Philosophy

These components follow the same design language as the rest of the application:

- **Sharp, angular styling** with minimal rounded corners
- **Dark, fantasy-adjacent color scheme** using zinc colors
- **High contrast** with light text on dark backgrounds
- **Consistent focus states** and hover effects
- **Accessible design** with proper ARIA attributes and keyboard navigation

## Styling

Components use Tailwind CSS classes and follow the established design system:

- **Colors**: zinc-900, zinc-800, zinc-700 for backgrounds and borders
- **Text**: zinc-100, zinc-300, zinc-400, zinc-500 for different text hierarchies
- **Accents**: blue-500 for focus states and primary actions
- **Transitions**: Smooth transitions for hover and focus states
