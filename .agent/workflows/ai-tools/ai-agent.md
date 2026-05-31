---
description: Create AI agents with tools and capabilities
---

# AI Agent

I will help you build an AI agent with tools and capabilities.

## Guardrails
- Define clear boundaries for agent actions
- Implement proper error handling
- Log agent decisions for debugging
- Add safety checks for destructive actions

## Steps

### 1. Understand Requirements
Ask clarifying questions:
- What should the agent do?
- What tools does it need?
- What LLM will power it?
- Any safety constraints?

### 2. Design Agent
Plan the architecture:
- Agent type (ReAct, Chain-of-Thought, etc.)
- Available tools
- Memory/context handling
- Output format

### 3. Define Tools
For each capability:
- Name and description
- Input parameters
- Implementation
- Error handling

### 4. Implement Agent
Build core components:
- LLM integration
- Tool execution
- Response parsing
- Context management

### 5. Add Safety
Implement guardrails:
- Input validation
- Action confirmation for destructive ops
- Rate limiting
- Error recovery

### 6. Test
Verify behavior:
- Happy path scenarios
- Edge cases
- Error handling
- Safety constraints

## Principles
- Start simple, add complexity
- Log everything for debugging
- Fail gracefully
- Human in the loop for critical actions
