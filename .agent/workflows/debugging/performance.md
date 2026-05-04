---
description: Profile and optimize slow code paths
---

# Performance

I will help you identify and fix performance issues in your code.

## Guardrails
- Measure before optimizing
- Focus on actual bottlenecks
- Don't sacrifice readability for micro-optimizations
- Verify improvements with benchmarks

## Steps

### 1. Understand the Issue
Ask clarifying questions:
- What's slow? (page load, API response, etc.)
- How slow is it currently?
- What's the target performance?
- Is it consistent or intermittent?

### 2. Profile the Code
Use appropriate tools:
- **Browser**: DevTools Performance tab
- **Node.js**: `--inspect`, clinic.js
- **Python**: cProfile, py-spy
- **Database**: EXPLAIN queries

### 3. Identify Bottlenecks
Common issues:
- N+1 database queries
- Unindexed database columns
- Large bundle sizes
- Unnecessary re-renders
- Synchronous blocking operations
- Memory leaks

### 4. Optimize
Apply targeted fixes:
- Add database indexes
- Batch queries
- Add caching
- Lazy load resources
- Memoize expensive computations
- Use pagination

### 5. Verify Improvements
- Measure again with same methodology
- Compare before/after metrics
- Ensure no regression in functionality

## Principles
- Don't guess, measure
- Optimize the critical path first
- Cache expensive operations
- Profile in production-like conditions
