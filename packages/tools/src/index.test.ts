import { describe, it, expect } from 'vitest';
import { tools } from './index.js';

describe('Tools', () => {
  it('should return tools string', () => {
    expect(tools()).toBe('tools');
  });
});
