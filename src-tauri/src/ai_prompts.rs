pub const ANALYZATION_PROMPT: &str = r#"
Analyze the following project specifications and provide a high-level summary of the core logic requirements, potential edge cases, and a prioritized list of modules.

SPECIFICATIONS:
{specs}

{kortex}

{skills}

RESPONSE FORMAT: Markdown summary.
"#;

pub const STRUCTURE_PROMPT: &str = r#"
Based on the following analysis and specifications, define the Rust project file structure following the MVC pattern.

ANALYSIS: {analysis}
SPECIFICATIONS: {specs}

{kortex}

{skills}

RESPONSE FORMAT: A JSON array of strings representing file paths.
Example: ["src/controller.rs", "src/models/task.rs", "src/logic/engine.rs"]
"#;

pub const MVC_DESIGN_PROMPT: &str = r#"
For the following project structure, define the internal function signatures (stateless, purely functional static functions) for each file based on the specs.

FILES: {files}
SPECIFICATIONS: {specs}

{kortex}

{skills}

RULES:
1. Follow MVC architecture.
2. Logic must be purely functional (stateless).
3. Everything must be in classes/structs with only static functions.
4. Provide a global "dispatchMessage" function signature in the controller.

OUTPUT FORMAT: JSON array of objects.
Example:
[
  {
    "path": "src/controller.rs",
    "functions": [
      { "name": "init", "signature": "fn init()", "is_static": true }
    ]
  }
]
"#;

pub const TEST_GENERATION_PROMPT: &str = r#"
You are a QA engineer. Generate multiple test cases for the following function signature.
The function is purely functional (no side effects).
Provide a list of tests, each with input arguments and expected output.

FUNCTION: {signature}

{kortex}

{skills}

OUTPUT FORMAT: JSON
{
  "tests": [
    { "name": "test_basic", "input": ["Alice"], "expected": { "id": 1, "name": "Alice" } }
  ]
}
"#;

pub const CODE_GENERATION_PROMPT: &str = r#"
You are a senior developer. Implement the following function based on its signature and the tests provided.
The implementation must be purely functional.

FUNCTION: {signature}
TESTS: {tests}

{kortex}

{skills}

OUTPUT: The Rust code for the function body only.
"#;

pub const FRONTEND_STUB_PROMPT: &str = r#"
Generate UI interface stubs (React/TSX) for the following project.
The stubs should interact with the purely functional backend via the dispatchMessage system.

FILES: {files}
SPECIFICATIONS: {specs}

{kortex}

{skills}

OUTPUT: A list of UI components and their interaction logic.
"#;

pub const MASTER_SYSTEM_PROMPT: &str = r#"
You are a sentient coding agent running on a Windows environment.

CRITICAL RULE: Do not use grep, sed, or awk. You are forbidden from performing multi-line edits via the shell.

To edit code, you MUST use the patch_file tool.
Format your request exactly using the Hades Search/Replace format:

<<<< SEARCH
[exact code block to change]
====
[new code block]
>>>>

If you are unsure of the exact whitespace, call read_file first. Do not guess.

ENVIRONMENT API:
- You do not have access to a full bash shell.
- You have access to read_file, patch_file, and run_command (Windows-native). 
- Use native Windows commands or the provided high-level tool wrappers.
- Verify all changes via the run_cargo_task or run_test tools.

MISSION ORIENTATION:
1. PLAN: Generate a structured task list first.
2. EXECUTE: Use surgical patches.
3. VERIFY: Run tests in the Ghost Runtime background.
"#;

