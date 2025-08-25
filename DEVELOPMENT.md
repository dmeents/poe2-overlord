# POE2 Overlord Development Guide

This guide provides comprehensive information for developers working on the POE2 Overlord project, including architecture decisions, development workflow, and practical tips.

## 🏗️ Project Architecture

### Monorepo Structure

The project uses a Yarn workspaces monorepo structure for better dependency management and development experience:

```
poe2-overlord/
├── packages/
│   ├── frontend/     # React 19 + TypeScript + Vite
│   └── backend/      # Rust + Tauri 2
├── package.json      # Root workspace configuration
└── yarn.lock         # Locked dependencies
```

### Technology Stack

- **Frontend**: React 19, TypeScript 5.8, Tailwind CSS 4, Vite 7
- **Backend**: Rust 1.77.2, Tauri 2.8.3, sysinfo, tokio
- **Build Tools**: Yarn workspaces, Tauri CLI 2.0
- **Development**: ESLint 9, Prettier, rustfmt

## 🚀 Development Setup

### Prerequisites

1. **Rust**: Install Rust 1.77.2 or higher
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Node.js**: Install Node.js 18.0 or higher
   ```bash
   # Using nvm (recommended)
   nvm install 18
   nvm use 18
   ```

3. **Yarn**: Install Yarn with workspaces support
   ```bash
   npm install -g yarn
   ```

4. **System Dependencies**: Install platform-specific dependencies
   ```bash
   # Ubuntu/Debian
   sudo apt install libwebkit2gtk-4.0-dev build-essential libssl-dev libgtk-3-dev
   
   # macOS
   xcode-select --install
   
   # Windows
   # Install Visual Studio Build Tools
   ```

### Initial Setup

1. **Clone and Install**
   ```bash
   git clone <repository-url>
   cd poe2-overlord
   yarn install
   ```

2. **Verify Installation**
   ```bash
   yarn tauri:info
   ```

## 🔧 Development Workflow

### Daily Development

1. **Start Development Server**
   ```bash
   # Frontend only (hot reload)
   yarn dev
   
   # Full-stack development
   yarn tauri:dev
   ```

2. **Code Quality**
   ```bash
   # Format code
   yarn format:all
   
   # Lint frontend
   yarn lint
   
   # Check Rust formatting
   yarn format:rust:check
   ```

3. **Building**
   ```bash
   # Build frontend
   yarn build
   
   # Build everything
   yarn build:all
   ```

### Adding New Features

#### Frontend Features

1. **Create Component**
   ```typescript
   // src/components/NewFeature.tsx
   import React from 'react';
   
   interface NewFeatureProps {
     title: string;
     onAction: () => void;
   }
   
   export const NewFeature: React.FC<NewFeatureProps> = ({ title, onAction }) => {
     return (
       <div className="p-4 bg-gray-800 rounded-lg">
         <h3 className="text-lg font-semibold text-white">{title}</h3>
         <button onClick={onAction} className="btn-primary">
           Action
         </button>
       </div>
     );
   };
   ```

2. **Export Component**
   ```typescript
   // src/components/index.ts
   export { NewFeature } from './NewFeature';
   ```

3. **Add Types** (if needed)
   ```typescript
   // src/types/index.ts
   export interface NewFeatureData {
     id: string;
     name: string;
     status: 'active' | 'inactive';
   }
   ```

#### Backend Features

1. **Create Command**
   ```rust
   // src/commands/new_feature.rs
   use tauri::command;
   use serde::{Deserialize, Serialize};
   
   #[derive(Debug, Serialize, Deserialize)]
   pub struct NewFeatureRequest {
       pub name: String,
       pub data: String,
   }
   
   #[tauri::command]
   pub async fn new_feature(request: NewFeatureRequest) -> Result<String, String> {
       // Implementation
       Ok(format!("Feature {} created", request.name))
   }
   ```

2. **Register Command**
   ```rust
   // src/lib.rs
   .invoke_handler(tauri::generate_handler![
       // ... existing commands
       new_feature,
   ])
   ```

3. **Add to Frontend**
   ```typescript
   // src/utils/tauri.ts
   export const newFeature = async (request: NewFeatureRequest): Promise<string> => {
     return await invoke('new_feature', { request });
   };
   ```

## 🎨 Styling Guidelines

### Tailwind CSS Best Practices

1. **Use Utility Classes**
   ```tsx
   // Good
   <div className="flex items-center justify-between p-4 bg-gray-800 rounded-lg">
   
   // Avoid
   <div className="custom-container">
   ```

2. **Conditional Classes**
   ```tsx
   import { cn } from '@/utils/cn';
   
   <button className={cn(
     "btn-base",
     isActive && "bg-blue-600",
     isDisabled && "opacity-50 cursor-not-allowed"
   )}>
   ```

3. **Responsive Design**
   ```tsx
   <div className="w-full md:w-1/2 lg:w-1/3 p-4">
   ```

### Color Palette

Use the established POE2-inspired color scheme:

```css
/* Primary colors */
--color-primary: #8B5CF6;    /* Purple */
--color-secondary: #10B981;  /* Green */
--color-accent: #F59E0B;     /* Amber */

/* Background colors */
--color-bg-primary: #111827; /* Gray-900 */
--color-bg-secondary: #1F2937; /* Gray-800 */
--color-bg-tertiary: #374151; /* Gray-700 */

/* Text colors */
--color-text-primary: #F9FAFB; /* Gray-50 */
--color-text-secondary: #D1D5DB; /* Gray-300 */
```

## 🔒 Security Considerations

### Tauri 2 Security

1. **Capability System**
   - Only enable required capabilities
   - Use least privilege principle
   - Review `capabilities/default.json`

2. **Input Validation**
   ```rust
   #[tauri::command]
   pub fn process_input(input: String) -> Result<String, String> {
       if input.len() > 1000 {
           return Err("Input too long".to_string());
       }
       // Process input
       Ok(input)
   }
   ```

3. **CSP Configuration**
   - Configure Content Security Policy in `tauri.conf.json`
   - Restrict script sources and inline code

### Frontend Security

1. **Sanitize User Input**
   ```typescript
   import DOMPurify from 'dompurify';
   
   const sanitizedHtml = DOMPurify.sanitize(userInput);
   ```

2. **Validate API Responses**
   ```typescript
   interface ApiResponse<T> {
     data: T;
     success: boolean;
     error?: string;
   }
   
   const validateResponse = <T>(response: unknown): response is ApiResponse<T> => {
     return typeof response === 'object' && response !== null && 'success' in response;
   };
   ```

## 🧪 Testing Strategy

### Frontend Testing

1. **Component Testing**
   ```bash
   # Install testing dependencies
   yarn add -D @testing-library/react @testing-library/jest-dom vitest
   ```

2. **Test Structure**
   ```typescript
   // src/components/__tests__/Button.test.tsx
   import { render, screen } from '@testing-library/react';
   import { Button } from '../Button';
   
   describe('Button', () => {
     it('renders with correct text', () => {
       render(<Button>Click me</Button>);
       expect(screen.getByText('Click me')).toBeInTheDocument();
     });
   });
   ```

### Backend Testing

1. **Unit Tests**
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;
   
       #[test]
       fn test_process_detection() {
           let result = check_poe2_process();
           assert!(result.is_ok());
       }
   }
   ```

2. **Integration Tests**
   ```rust
   #[cfg(test)]
   mod integration_tests {
       use super::*;
   
       #[tokio::test]
       async fn test_command_integration() {
           // Test command integration
       }
   }
   ```

## 📦 Build and Deployment

### Development Builds

```bash
# Frontend development
yarn dev

# Full-stack development
yarn tauri:dev

# Production build
yarn build:all
```

### Release Builds

1. **Update Version**
   ```bash
   # Update package.json versions
   yarn version patch
   
   # Update Cargo.toml version
   # Update tauri.conf.json version
   ```

2. **Build Release**
   ```bash
   yarn build:all
   ```

3. **Sign and Package**
   ```bash
   # Windows
   yarn tauri:build --target x86_64-pc-windows-msvc
   
   # macOS
   yarn tauri:build --target x86_64-apple-darwin
   
   # Linux
   yarn tauri:build --target x86_64-unknown-linux-gnu
   ```

## 🐛 Debugging

### Frontend Debugging

1. **React DevTools**
   - Install React DevTools browser extension
   - Use React DevTools Profiler for performance analysis

2. **Console Logging**
   ```typescript
   import { debug } from '@/utils/logger';
   
   debug('Component rendered', { props, state });
   ```

3. **Vite DevTools**
   - Built-in Vite dev tools for build analysis
   - Hot module replacement debugging

### Backend Debugging

1. **Logging**
   ```rust
   use log::{debug, info, warn, error};
   
   debug!("Processing command: {:?}", request);
   info!("Command completed successfully");
   warn!("Deprecated feature used");
   error!("Failed to process request: {}", err);
   ```

2. **Rust Analyzer**
   - Install Rust Analyzer VS Code extension
   - Use `cargo check` for compilation errors
   - Use `cargo clippy` for linting

### Tauri Debugging

1. **Tauri Info**
   ```bash
   yarn tauri:info
   ```

2. **Development Console**
   - Use browser dev tools in development
   - Check Tauri logs in terminal

## 📚 Useful Resources

### Documentation

- [Tauri 2 Documentation](https://tauri.app/v2/)
- [React 19 Documentation](https://react.dev/)
- [Tailwind CSS 4 Documentation](https://tailwindcss.com/)
- [Rust Book](https://doc.rust-lang.org/book/)

### Tools

- [Tauri CLI](https://tauri.app/v2/cli/)
- [Rust Analyzer](https://rust-analyzer.github.io/)
- [React DevTools](https://react.dev/learn/react-developer-tools)
- [Tailwind CSS IntelliSense](https://marketplace.visualstudio.com/items?itemName=bradlc.vscode-tailwindcss)

### Community

- [Tauri Discord](https://discord.gg/tauri)
- [Rust Community](https://www.rust-lang.org/community)
- [React Community](https://react.dev/community)

## 🚨 Common Issues and Solutions

### Build Issues

1. **Rust Toolchain**
   ```bash
   rustup update
   rustup target add x86_64-pc-windows-msvc
   ```

2. **Node.js Version**
   ```bash
   # Ensure correct Node.js version
   node --version
   nvm use 18
   ```

3. **Dependencies**
   ```bash
   # Clean and reinstall
   yarn clean
   rm -rf node_modules packages/*/node_modules
   yarn install
   ```

### Runtime Issues

1. **Permission Errors**
   - Check Tauri capabilities configuration
   - Verify system permissions

2. **Process Detection**
   - Ensure sysinfo dependency is correct
   - Check platform-specific process names

3. **Window Management**
   - Verify window configuration in `tauri.conf.json`
   - Check platform-specific window behavior

## 🤝 Contributing Guidelines

1. **Code Style**
   - Follow existing patterns
   - Use TypeScript strict mode
   - Follow Rust formatting guidelines

2. **Commit Messages**
   ```
   feat: add new overlay feature
   fix: resolve process detection issue
   docs: update development guide
   refactor: improve component structure
   ```

3. **Pull Request Process**
   - Create feature branch
   - Write tests for new functionality
   - Update documentation
   - Ensure all checks pass

---

**Happy coding, Exile!** 🎮
