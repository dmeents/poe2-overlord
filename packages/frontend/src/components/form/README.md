# Form Components

This directory contains reusable form components that provide consistent styling and behavior across the application. The components use a sharp, fantasy-adjacent design aesthetic with dark colors, minimal rounded corners, and high contrast.

## Design Philosophy

The form components follow the same design language as the rest of the application:

- **Sharp, angular styling** with minimal rounded corners
- **Dark, fantasy-adjacent color scheme** using zinc-900, zinc-800, and zinc-700
- **High contrast** with light text on dark backgrounds
- **Clean, crisp borders** and edges
- **Consistent focus states** with blue-500 accent colors

## Components

### FormField

A wrapper component that provides consistent label and description styling for form inputs. Labels can be strings or React elements (useful for tooltips). The label and input are displayed inline for a cleaner layout.

```tsx
<FormField
  label='Field Label'
  description='Optional description text'
  htmlFor='input-id'
>
  <input id='input-id' />
</FormField>

// With tooltip in label
<FormField
  label={
    <Tooltip content="Helpful information">
      Field Label
    </Tooltip>
  }
>
  <input id='input-id' />
</FormField>
```

### TextInput

A text input component with validation styling and warning messages.

```tsx
<TextInput
  id='my-input'
  value={value}
  onChange={setValue}
  placeholder='Enter text...'
  isValid={isValid}
  warningMessage='Warning message if invalid'
/>
```

### CheckboxInput

A checkbox input component with label and description. Labels can be strings or React elements.

```tsx
<CheckboxInput
  id='my-checkbox'
  checked={checked}
  onChange={setChecked}
  label='Checkbox label'
  description='Optional description'
/>

// With tooltip in label
<CheckboxInput
  id='my-checkbox'
  checked={checked}
  onChange={setChecked}
  label={
    <Tooltip content="Helpful information">
      Checkbox label
    </Tooltip>
  }
/>
```

### SelectInput

A select dropdown component with options.

```tsx
<SelectInput
  id='my-select'
  value={value}
  onChange={setValue}
  options={[
    { value: 'option1', label: 'Option 1' },
    { value: 'option2', label: 'Option 2' },
  ]}
/>
```

### AlertMessage

A component for displaying error and success messages.

```tsx
<AlertMessage
  type='error'
  message='Error message'
/>
<AlertMessage
  type='success'
  message='Success message'
/>
```

## Styling Details

### Color Palette

- **Backgrounds**: `bg-zinc-900` (primary), `bg-zinc-800` (secondary)
- **Borders**: `border-zinc-700` (default), `border-zinc-600` (subtle)
- **Text**: `text-zinc-100` (primary), `text-zinc-300` (labels), `text-zinc-400` (secondary), `text-zinc-500` (descriptions)
- **Accents**: `text-blue-500` (focus), `text-amber-500` (warnings), `text-red-400` (errors), `text-green-400` (success)

### Focus States

All interactive elements use consistent focus styling with `focus:ring-2 focus:ring-blue-500 focus:border-blue-500`.

### Validation States

- **Valid**: Default zinc-700 borders
- **Invalid**: Amber-500 borders with subtle amber-950/20 backgrounds
- **Warnings**: Amber-500 text for validation messages

### Layout

- **Inline labels**: Labels and inputs are displayed inline for a cleaner, more compact layout
- **Responsive spacing**: Proper spacing between elements with consistent margins
- **Flexbox layout**: Uses flexbox for proper alignment and spacing

## Usage

All form components are exported from the main components index and can be imported as:

```tsx
import { FormField, TextInput, CheckboxInput, SelectInput } from '@/components';
```

Or individually from the form directory:

```tsx
import { FormField } from '@/components/form';
```

## Related Components

For tooltips and loading states, see the general-purpose components:

- **Tooltip**: `import { Tooltip } from '@/components'`
- **LoadingSpinner**: `import { LoadingSpinner } from '@/components'`
