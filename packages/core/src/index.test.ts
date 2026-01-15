import { describe, it, expect } from 'vitest';
import { core, LLMClient } from './index.js';

describe('Core', () => {
  it('should return core string', () => {
    expect(core()).toBe('core');
  });

  it('should instantiate LLMClient', () => {
    const client = new LLMClient({ apiKey: 'test-key' });
    expect(client).toBeDefined();
    expect(client).toBeInstanceOf(LLMClient);
  });
});
