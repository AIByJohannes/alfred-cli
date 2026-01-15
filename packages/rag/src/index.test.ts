import { describe, it, expect } from 'vitest';
import { rag } from './index.js';

describe('RAG', () => {
  it('should return rag string', () => {
    expect(rag()).toBe('rag');
  });
});
