import OpenAI from 'openai';
import { Stream } from 'openai/streaming';

export interface ChatMessage {
  role: 'system' | 'user' | 'assistant';
  content: string;
}

export interface LLMOptions {
  apiKey?: string;
  model?: string;
}

export class LLMClient {
  private client: OpenAI;
  private model: string;

  constructor(options: LLMOptions = {}) {
    this.client = new OpenAI({
      apiKey: options.apiKey || process.env.OPENAI_API_KEY,
    });
    this.model = options.model || 'gpt-4o';
  }

  async streamChat(messages: ChatMessage[]): Promise<Stream<OpenAI.Chat.Completions.ChatCompletionChunk>> {
    return await this.client.chat.completions.create({
      model: this.model,
      messages: messages,
      stream: true,
    });
  }

  async chat(messages: ChatMessage[]): Promise<string | null> {
    const response = await this.client.chat.completions.create({
      model: this.model,
      messages: messages,
      stream: false,
    });

    return response.choices[0]?.message?.content || null;
  }
}
