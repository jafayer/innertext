#!/usr/bin/env python3
"""
Sync version across all package configuration files.

Usage:
  python scripts/sync-version.py 0.2.0

This updates:
  - Cargo.toml (workspace version)
  - bindings/innertext-python/pyproject.toml
  - bindings/innertext-node/package.json
  - bindings/innertext-java/build.gradle
"""

import sys
import re
from pathlib import Path

def update_file(path: Path, pattern: str, replacement: str, flags: int = 0) -> bool:
    """Update a file using regex substitution."""
    content = path.read_text()
    new_content = re.sub(pattern, replacement, content, flags=flags)
    if content != new_content:
        path.write_text(new_content)
        return True
    return False

def main():
    if len(sys.argv) < 2:
        print("Usage: python sync-version.py <version>")
        print("Example: python sync-version.py 0.2.0")
        sys.exit(1)
    
    version = sys.argv[1]
    
    # Validate version format
    if not re.match(r'^\d+\.\d+\.\d+$', version):
        print(f"Error: Invalid version format '{version}'")
        print("Expected format: MAJOR.MINOR.PATCH (e.g., 0.2.0)")
        sys.exit(1)
    
    root = Path(__file__).parent.parent
    
    updates = [
        (
            root / "Cargo.toml",
            r'(^\[workspace\.package\]\nversion = )"[^"]*"',
            rf'\1"{version}"',
            "Cargo.toml"
        ),
        (
            root / "bindings/innertext-python/pyproject.toml",
            r'(^version = )"[^"]*"',
            rf'\1"{version}"',
            "pyproject.toml"
        ),
        (
            root / "bindings/innertext-node/package.json",
            r'("version": )"[^"]*"',
            rf'\1"{version}"',
            "package.json"
        ),
        (
            root / "bindings/innertext-java/build.gradle",
            r"(^version = )'[^']*'",
            rf"\1'{version}'",
            "build.gradle"
        ),
    ]
    
    updated_files = []
    
    for file_path, pattern, replacement, name in updates:
        if not file_path.exists():
            print(f"Warning: {file_path} not found, skipping")
            continue
        
        try:
            if update_file(file_path, pattern, replacement, flags=re.MULTILINE):
                updated_files.append(name)
                print(f"✓ Updated {name} to version {version}")
            else:
                print(f"⚠ No changes needed in {name}")
        except Exception as e:
            print(f"✗ Error updating {name}: {e}")
            sys.exit(1)
    
    if updated_files:
        print(f"\n✓ Successfully updated {len(updated_files)} file(s)")
        print(f"\nNext steps:")
        print(f"  1. Review the changes:")
        print(f"     git diff")
        print(f"  2. Commit the version bump:")
        print(f"     git add .")
        print(f"     git commit -m 'chore: bump version to {version}'")
        print(f"  3. Tag and push:")
        print(f"     git tag v{version}")
        print(f"     git push origin v{version}")
    else:
        print("No files were updated")

if __name__ == "__main__":
    main()
