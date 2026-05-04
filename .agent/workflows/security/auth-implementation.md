---
description: Implement authentication patterns for any stack
---

# Auth Implementation

I will help you implement authentication for your application.

## Guardrails
- Never store passwords in plain text
- Use established libraries, don't roll your own crypto
- Detect existing auth setup before adding
- Follow security best practices

## Steps

### 1. Understand Requirements
Ask clarifying questions:
- What auth method? (email/password, OAuth, magic link)
- What providers? (Google, GitHub, etc.)
- Session or JWT-based?
- Need role-based access control?

### 2. Analyze Stack
Check existing setup:
- NextAuth, Auth.js
- Passport.js
- Django auth
- Firebase Auth

### 3. Choose Strategy
Based on requirements:
- **Session-based**: Traditional, server-side
- **JWT**: Stateless, API-friendly
- **OAuth**: Social/third-party login
- **Magic Link**: Passwordless

### 4. Implement
Core components:
- User model/table
- Login/register flows
- Session/token management
- Protected routes
- Logout functionality

### 5. Secure
Add protections:
- Password hashing
- Rate limiting
- CSRF protection
- Secure cookies
- Input validation

### 6. Verify
Test thoroughly:
- Happy path flows
- Error cases
- Session expiry
- Protected routes

## Principles
- Use battle-tested libraries
- Fail securely (deny by default)
- Log security events
- Regular security reviews
