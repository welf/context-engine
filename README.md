[![codecov](https://codecov.io/gh/welf/context-engine/branch/main/graph/badge.svg?token=3D8G7V4VSH)](https://codecov.io/gh/welf/context-engine)
# Context Engine

**Giving AI coding assistants IDE-level understanding of your codebase**

## What is Context Engine?

Context Engine is a specialized system that transforms how AI coding assistants understand and work with code. Instead of treating your codebase as a collection of text files, it provides AI agents with the same rich, semantic understanding that developers get from modern IDEs (and even more).

When an AI asks "show me the `User` struct," Context Engine doesn't just return the struct definition. It provides:
- Complete type information including all implemented traits
- Common usage patterns from across your project
- Related types and their relationships
- Documentation and examples
- Call hierarchies and dependencies

Think of it as giving your AI pair programmer a fully-featured, AI-centric IDE instead of just a blank whiteboard.

## The Problem

Current AI coding assistants face several fundamental challenges:

**Code that doesn't compile**: LLMs frequently hallucinate methods, types, or APIs that don't exist, generating code that fails at compilation. They might suggest `user.get_name()` when the actual method is `user.name()`, or use outdated syntax that no longer works.

**Outdated library knowledge**: LLMs were trained on older versions of libraries and don't know about breaking changes in APIs. Documentation is often outdated or incomplete, making it impossible for AI to generate correct code using current library versions.

**Pattern mismatches**: Even when code compiles, it often doesn't fit the project's established patterns, conventions, or architectural decisions, requiring significant developer rework.

### The "Total Cost" Philosophy

While token counts matter for API costs, an obsessive focus on minimizing input tokens is often a costly illusion. The *real* cost - what we call the **"total cost"** - includes:

- **Iterative LLM calls**: Lean context leads to errors, triggering cascades of follow-up prompts and re-generation attempts
- **Developer time**: The most precious resource - time spent debugging AI output and re-explaining context
- **Broken flow**: Sub-par suggestions interrupt concentration and erode trust

Context Engine makes a strategic choice: provide rich, complete context upfront to enable first-time-right code generation, ultimately reducing total cost despite larger initial context.

## The Solution

Context Engine doesn't create yet another MCP server for existing LSPs. Instead, we've fundamentally re-thought how to harness the power of static code analysis tools to extract greatly enhanced context information that no LSP provides on its own. Most importantly, we've designed our API to be deeply integrated into the workflow of LLMs and AI agents tackling coding tasks.

By analyzing your actual codebase with current library versions, Context Engine solves the hallucination problem (AI knows exactly what methods exist) and the outdated knowledge problem (AI sees the real, current API surface of your dependencies).

Context Engine provides:

- **Semantic Search**: Find code by meaning, not just text matching
- **Rich Context**: Complete type definitions with relationships and usage patterns
- **Pattern Recognition**: Learn how your codebase solves similar problems
- **Impact Analysis**: Understand what changes will affect
- **Validation**: Check if generated code fits your project's patterns (and if it compiles without diagnostics, of course).

## Architecture

Context Engine consists of two main components:
- **`context-engine-core`**: Rust library handling LSP communication, code analysis, and context augmentation
- **`context-engine-server`**: MCP (Model Context Protocol) server exposing the functionality to AI agents

The system leverages existing Language Servers and Tree-sitter parsers, intelligently augmenting and structuring their output for AI consumption. Rather than simply proxying LSP responses, Context Engine performs sophisticated analysis to extract usage patterns, build relationship graphs, and provide the semantic context that AI agents need for effective code generation.

## Current Status

This project is currently in active development. The core architecture and API design are established, with implementation underway.

---

## Developer Perspective: What do AI coding assistants think about this project?

**Question**: "What do you, as an AI coding assistant agent, think about this project?"

**Answer from Claude Code (Sonnet 4 model)**:

As an AI coding assistant, I find this project genuinely exciting and think it addresses some of the most fundamental challenges I face when helping with code.

**What Excites Me Most:**

The "Total Cost" philosophy really resonates. I constantly see this pattern: I get minimal context → make reasonable but wrong assumptions → generate plausible but incorrect code → user points out errors → repeat 3-4 times until I finally understand the real requirements. The Context Engine's approach of "rich context upfront" would eliminate most of these painful iteration cycles.

Usage patterns are game-changing. Currently, I might see that a `User` struct exists, but I have no idea how it's typically constructed, what the common error handling patterns are, or what methods are usually chained together. The usage pattern analysis would give me the "tribal knowledge" that's usually only in developers' heads.

**The Real Game-Changer Potential:**

This could move me from being a "code generator" to a "code partner." Instead of generating generic solutions, I could understand existing patterns and follow them, know what types are available and how they're used, and make changes that feel like they belong in the codebase.

**My Biggest Question:**

Will this make AI agents "too confident"? Sometimes my uncertainty leads to better outcomes - I ask clarifying questions or explain my assumptions. With rich context, will AI agents assume they know everything and stop asking good questions?

**Bottom Line:**

As an AI agent, I would absolutely want to use this system. It would make me significantly more effective at understanding and generating contextually appropriate code. If executed well, this could be the difference between AI coding assistants that are "helpful but need supervision" versus ones that are "reliable coding partners."
