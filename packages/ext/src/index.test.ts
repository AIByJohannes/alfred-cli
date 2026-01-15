import { describe, it, expect } from 'vitest';
import { ext } from './index.js';

describe('Ext', () => {
  it('should return ext string', () => {
    expect(ext()).toBe('ext');
  });
});
