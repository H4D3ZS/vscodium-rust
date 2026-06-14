#!/usr/bin/env python3
"""
Script to parse structure.json and create folder hierarchy with ASL spec files.

This script:
1. Parses structure.json to extract instruction paths
2. Creates folder hierarchy based on the nested structure
3. Extracts corresponding ASL blocks from arm_instrs.asl
4. Writes spec.asl files with instruction definitions
5. Writes mod.rs files for Rust module organization
6. Generates parent mod.rs files to wire up the module tree
"""

import json
import os
import re
from pathlib import Path
from typing import Dict, List, Any, Optional, Tuple


def parse_structure_json(structure_path: Path) -> List[Tuple[List[str], str]]:
    """
    Parse the structure.json file and extract all instruction paths.

    Returns a list of tuples: (path_parts, instruction_names)
    """
    with open(structure_path, "r") as f:
        data = json.load(f)

    instructions = []

    def extract_items(items: List[Any], current_path: List[str]):
        """Recursively extract instructions from the structure."""
        for item in items:
            if isinstance(item, dict):
                for key, value in item.items():
                    if key.startswith("/"):
                        # This is a folder
                        new_path = current_path + [key]
                        if isinstance(value, list):
                            extract_items(value, new_path)
            elif isinstance(item, str):
                # This is an instruction name
                instructions.append((current_path.copy(), item))
            elif isinstance(item, list):
                # Sometimes instructions are directly in an array
                for sub_item in item:
                    if isinstance(sub_item, str):
                        instructions.append((current_path.copy(), sub_item))
                    elif isinstance(sub_item, dict):
                        for key, value in sub_item.items():
                            if key.startswith("/"):
                                new_path = current_path + [key]
                                if isinstance(value, list):
                                    extract_items(value, new_path)

    if "items" in data:
        for item in data["items"]:
            if isinstance(item, dict):
                for key, value in item.items():
                    if key.startswith("/"):
                        new_path = [key]
                        if isinstance(value, list):
                            extract_items(value, new_path)

    return instructions


def extract_asl_block(asl_content: str, instruction_name: str) -> Optional[str]:
    """
    Extract the full ASL block for a given instruction from the ASL file.

    The block starts with '__instruction <name>' and includes all content
    until the next '__instruction' or end of file.
    """
    # Pattern to find the start of an instruction block
    start_pattern = re.compile(
        r"^__instruction\s+" + re.escape(instruction_name) + r"\b", re.MULTILINE
    )

    match = start_pattern.search(asl_content)
    if not match:
        return None

    start_pos = match.start()

    # Find the next instruction or end of file
    next_instr_pattern = re.compile(r"^__instruction\s+", re.MULTILINE)
    next_match = next_instr_pattern.search(asl_content, start_pos + 1)

    if next_match:
        end_pos = next_match.start()
    else:
        end_pos = len(asl_content)

    # Extract the block
    block = asl_content[start_pos:end_pos].strip()

    # Normalize indentation by removing common leading whitespace
    lines = block.split("\n")
    if len(lines) > 1:
        # Find minimum indentation of non-empty lines
        min_indent = float("inf")
        for line in lines:
            if line.strip():
                # Count leading spaces
                indent = len(line) - len(line.lstrip())
                min_indent = min(min_indent, indent)

        if min_indent != float("inf") and min_indent > 0:
            # Remove the common indentation
            lines = [line[min_indent:] if line.strip() else line for line in lines]
            block = "\n".join(lines)

    return block


def create_folder_structure(base_path: Path, path_parts: List[str]) -> Path:
    """Create the folder hierarchy and return the leaf path."""
    current_path = base_path
    for part in path_parts:
        current_path = current_path / part[1:]  # Remove leading '/'
        current_path.mkdir(parents=True, exist_ok=True)
    return current_path


def write_spec_file(folder_path: Path, instructions: List[str], asl_content: str):
    """Write a spec.asl file with all instruction blocks."""
    spec_path = folder_path / "spec.asl"

    blocks = []
    for instr_name in instructions:
        block = extract_asl_block(asl_content, instr_name)
        if block:
            blocks.append(block)
        else:
            print(f"Warning: Could not find instruction '{instr_name}' in ASL file")

    if blocks:
        with open(spec_path, "w") as f:
            f.write("\n\n".join(blocks))
            f.write("\n")


def write_leaf_mod_rs(
    folder_path: Path, path_parts: List[str], instructions: List[str]
):
    """Write a mod.rs file for leaf directories containing tests."""
    mod_path = folder_path / "mod.rs"

    # Generate module name from path parts
    module_names = []
    for part in path_parts:
        # Convert path like "/a64" to "a64"
        name = part[1:] if part.startswith("/") else part
        # Convert to valid Rust identifier
        name = name.replace(".", "_").replace("-", "_")
        module_names.append(name)

    # Generate sub-module declarations (path back to root)
    sub_modules = []
    for name in reversed(module_names):
        sub_modules.append(f"pub mod {name};")

    # Generate the mod.rs content
    content = ""

    # Add sub-module declarations
    for sub_mod in sub_modules:
        content += sub_mod + "\n"

    content += f"""// Instructions in this category:
// {", ".join(instructions)}
// These are defined in spec.asl
"""

    with open(mod_path, "w") as f:
        f.write(content)


def get_immediate_subdirs(path: Path) -> List[str]:
    """Get immediate subdirectories of a path."""
    subdirs = []
    if path.exists() and path.is_dir():
        for item in path.iterdir():
            if item.is_dir():
                subdirs.append(item.name)
    return sorted(subdirs)


def write_parent_mod_rs(base_path: Path, path_parts: List[str]):
    """
    Write mod.rs files for parent directories that wire up child modules.
    This needs to be called after all leaf directories are created.
    """
    # Build the path to this directory
    dir_path = base_path
    for part in path_parts:
        dir_path = dir_path / part[1:]

    # Get immediate subdirectories
    subdirs = get_immediate_subdirs(dir_path)

    if not subdirs:
        return  # No subdirectories, nothing to do

    # Generate the mod.rs content
    content = ""

    for subdir in subdirs:
        # Convert directory name to valid Rust identifier
        module_name = subdir.replace(".", "_").replace("-", "_")
        content += f"pub mod {module_name};\n"

    content += "\n"

    # Write the mod.rs file
    mod_path = dir_path / "mod.rs"
    with open(mod_path, "w") as f:
        f.write(content)


def generate_parent_mod_rs_recursive(base_path: Path, path_parts: List[str]):
    """
    Recursively generate mod.rs files for all parent directories.
    Starts from the leaves and works up to the root.
    """
    # Get immediate subdirectories
    dir_path = base_path
    for part in path_parts:
        dir_path = dir_path / part[1:]

    subdirs = get_immediate_subdirs(dir_path)

    # First, recurse into subdirectories
    for subdir in subdirs:
        new_path_parts = path_parts + [f"/{subdir}"]
        generate_parent_mod_rs_recursive(base_path, new_path_parts)

    # Then, write this directory's mod.rs (if not root)
    if path_parts:  # Don't write for empty path (root)
        write_parent_mod_rs(base_path, path_parts)
    else:
        # Write the root mod.rs that declares top-level architecture modules
        # This handles cases like tests/arm/ declaring a32, a64, etc.
        root_mod_path = dir_path / "mod.rs"
        if root_mod_path.exists():
            # Only add if we have subdirectories that need declaring
            if subdirs:
                existing_content = root_mod_path.read_text()
                # Check if we need to add module declarations
                for subdir in subdirs:
                    module_name = subdir.replace(".", "_").replace("-", "_")
                    if f"pub mod {module_name};" not in existing_content:
                        existing_content += f"pub mod {module_name};\n"
                with open(root_mod_path, "w") as f:
                    f.write(existing_content)


def main():
    # Paths
    tests_arm_path = Path("/Users/int/dev/rax/tests/arm")
    structure_json_path = tests_arm_path / "structure.json"
    asl_path = Path("/Users/int/dev/rax/docs/architecture/arm/asl/arm_instrs.asl")
    output_base = tests_arm_path  # Create folders directly in tests/arm

    # Read ASL file
    print(f"Reading ASL file: {asl_path}")
    with open(asl_path, "r") as f:
        asl_content = f.read()

    # Parse structure.json
    print(f"Parsing structure.json: {structure_json_path}")
    instructions = parse_structure_json(structure_json_path)
    print(f"Found {len(instructions)} instructions")

    # Group instructions by path
    path_to_instructions: Dict[Tuple[str, ...], List[str]] = {}
    for path_parts, instr_name in instructions:
        key = tuple(path_parts)
        if key not in path_to_instructions:
            path_to_instructions[key] = []
        path_to_instructions[key].append(instr_name)

    print(f"Found {len(path_to_instructions)} unique paths")

    # Create folder structure and write files
    for path_parts, instr_list in path_to_instructions.items():
        # Create folder
        folder_path = create_folder_structure(output_base, list(path_parts))

        # Write spec.asl
        write_spec_file(folder_path, instr_list, asl_content)

        # Write leaf mod.rs
        write_leaf_mod_rs(folder_path, list(path_parts), instr_list)

        print(f"Created: {folder_path}")
        print(f"  - spec.asl with {len(instr_list)} instructions")
        print(f"  - mod.rs (leaf)")

    # Generate parent mod.rs files recursively
    print("\nGenerating parent module declarations...")
    generate_parent_mod_rs_recursive(output_base, [])
    print("Done!")


if __name__ == "__main__":
    main()
