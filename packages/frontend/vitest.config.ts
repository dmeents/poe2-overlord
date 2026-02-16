import tailwindcss from '@tailwindcss/vite';
import react from '@vitejs/plugin-react';
import tsconfigPaths from 'vite-tsconfig-paths';
import { defineConfig } from 'vitest/config';

export default defineConfig({
  plugins: [tsconfigPaths(), tailwindcss(), react()],
  test: {
    environment: 'jsdom',
    globals: true,
    setupFiles: ['./src/test/setup.ts'],
    include: ['src/**/*.{spec,test}.{ts,tsx}'],
    exclude: ['node_modules', 'dist'],
    coverage: {
      provider: 'v8',
      reporter: ['text', 'html'],
      exclude: [
        'node_modules',
        'src/test',
        'src/routeTree.gen.ts',
        '**/*.d.ts',
        '**/*.config.*',
        '**/*.styles.ts',
        'src/utils/ascendency-assets.ts',
        'src/utils/economy-icons.ts',
        'src/utils/name-generator/name-parts.ts',
      ],
      thresholds: {
        statements: 60,
        branches: 55,
        functions: 60,
        lines: 60,
      },
    },
  },
});
