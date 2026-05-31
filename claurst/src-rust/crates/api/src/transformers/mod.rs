// transformers/mod.rs — Concrete MessageTransformer implementations for each
// supported provider wire format.
//
// Phase 4: Each transformer converts ProviderRequest → provider JSON body
// (to_provider) and provider JSON response → ProviderResponse (from_provider).

pub mod anthropic;
pub mod openai_chat;

pub use anthropic::AnthropicTransformer;
pub use openai_chat::OpenAiChatTransformer;
