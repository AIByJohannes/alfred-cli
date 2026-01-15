import { describe, it, expect } from 'vitest';
import { core } from './index.js';

describe('Core', () => {
  it('should return core string', () => {
    expect(core()).toBe('core');
  });
});
