import { describe, it, expect } from 'vitest';
import { CLI_NAME } from './index.js';

describe('CLI', () => {
  it('should export CLI_NAME', () => {
    expect(CLI_NAME).toBe('alfred-cli');
  });
});
