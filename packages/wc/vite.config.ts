/// <reference types="vitest/config" />

import { playwright } from '@vitest/browser-playwright'
import { defineConfig } from 'vite'

export default defineConfig({
  test: {
    browser: {
      enabled: true,
      provider: playwright(),
      headless: false,
      instances: [
        { browser: 'chromium' },
      ],
    },
    setupFiles: ['./src/test/setup.ts'],
    api: {
      port: 51234,
    },
  },
})