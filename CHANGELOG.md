Released 3.4.6 | *Code quality improvements and slop elimination*

- Refactor: Simplified over-engineered provider clients by removing excessive validation and error handling in OpenAI, Anthropic, Google, and OpenRouter modules
- Refactor: Extracted duplicate JSON parsing logic into reusable `parsePromptResponse` helper in engine module
- Refactor: Consolidated text truncation logic into `truncateText` helper, eliminating redundant calculations in UI functions
- Refactor: Extracted common score display logic into `getScoreDisplay` helper for complexity analysis displays
- Refactor: Removed barrel exports (`utils/index.js`) and updated all imports to be direct for better tree-shaking
- Refactor: Simplified command suggestion algorithm from complex fuzzy matching to efficient prefix matching
- Refactor: Extracted magic fallback objects into named constants for better maintainability
- Fix: Removed duplicate error handling in main CLI entry point
- Fix: Eliminated dummy cache object that provided no functionality in engine module
- Fix: Removed unused `rl` parameter from `handleCommand` function and updated all call sites
- Fix: Fixed inefficient double `truncateText` calls in content preview formatting
- Fix: Deleted dead `lib/llm.js` file containing unused error classes and validation utilities
- UX: Improved code clarity and maintainability through systematic slop elimination across all modules

chore: update package-lock.json and package.json for @kleosr/pe2-cli v3.4.6, refactor CLI entry point, and clean up unused code in various modules
