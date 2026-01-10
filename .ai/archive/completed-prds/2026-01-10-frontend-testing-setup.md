# PRD: Frontend Testing Setup

## Goal
Set up Jest + React Testing Library for frontend testing

## Requirements
1. Install dependencies (Jest, RTL, @testing-library/react, etc.)
2. Create jest.config.js matching our Vite setup
3. Add test scripts to package.json
4. Create test utilities/helpers in src/test/
5. Write one example test for an existing component
6. Update .ai/memory/patterns.md with the testing pattern

## Success Criteria
- `yarn test:frontend` runs without errors
- Example test passes
- Documentation updated

## Tech Stack
- Jest (test runner)
- React Testing Library (component testing)
- @testing-library/user-event (user interactions)
- vitest might be better for Vite projects (investigate)
