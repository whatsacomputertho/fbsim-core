#!/usr/bin/env node

/**
 * inject-jsdoc.mjs
 *
 * Extracts `///` doc comments from Rust source files for Tsify-derived types
 * and injects them as JSDoc comments into the generated `.d.ts` files.
 *
 * wasm-bindgen preserves doc comments for opaque `#[wasm_bindgen]` classes,
 * but Tsify-generated interfaces/enums/type aliases do not get JSDoc in the
 * output `.d.ts`. This script bridges that gap.
 *
 * Usage:
 *   node scripts/inject-jsdoc.mjs
 *
 * Expects `pkg/web/fbsim_core.d.ts` and `pkg/node/fbsim_core.d.ts` to exist.
 */

import { readFileSync, writeFileSync, readdirSync, statSync } from "node:fs";
import { join, extname } from "node:path";

const ROOT = new URL("..", import.meta.url).pathname.replace(/^\/([A-Z]:)/, "$1");
const SRC_DIR = join(ROOT, "src");
const DTS_PATHS = [
  join(ROOT, "pkg", "web", "fbsim_core.d.ts"),
  join(ROOT, "pkg", "node", "fbsim_core.d.ts"),
];

/**
 * Recursively collect all `.rs` files under a directory.
 */
function collectRsFiles(dir) {
  const results = [];
  for (const entry of readdirSync(dir)) {
    const full = join(dir, entry);
    const stat = statSync(full);
    if (stat.isDirectory()) {
      results.push(...collectRsFiles(full));
    } else if (extname(entry) === ".rs") {
      results.push(full);
    }
  }
  return results;
}

/**
 * Parse a Rust source file and extract doc comments for Tsify-derived types.
 *
 * Returns a Map<string, string> where key = type name, value = JSDoc block.
 *
 * We look for patterns like:
 *   /// Some doc comment
 *   /// More doc comment
 *   #[cfg_attr(feature = "wasm", derive(Tsify))]   // or derive(tsify_next::Tsify)
 *   #[tsify(...)]
 *   pub struct FooBar {
 *
 * Or in wasm/ files:
 *   /// Some doc comment
 *   #[derive(Clone, Debug, Serialize, Tsify)]
 *   #[tsify(...)]
 *   pub struct FooBar {
 */
function extractTsifyDocs(filePath) {
  const content = readFileSync(filePath, "utf-8");
  const lines = content.split("\n");
  const docs = new Map();

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i].trim();

    // Check if this line has a Tsify derive
    const isTsifyDerive =
      /derive\(.*Tsify.*\)/.test(line) ||
      /derive\(.*tsify_next::Tsify.*\)/.test(line);

    if (!isTsifyDerive) continue;

    // Walk backwards from the derive line to collect doc comments.
    // Skip over other #[...] attribute lines.
    let docStart = i - 1;
    while (docStart >= 0) {
      const prev = lines[docStart].trim();
      if (prev.startsWith("///")) {
        docStart--;
      } else if (prev.startsWith("#[")) {
        // Another attribute above the doc comments - skip it
        docStart--;
      } else {
        break;
      }
    }
    docStart++; // move back to the first doc/attr line

    // Collect only the `///` lines
    const docLines = [];
    for (let j = docStart; j < i; j++) {
      const l = lines[j].trim();
      if (l.startsWith("///")) {
        // Strip the `/// ` or `///` prefix
        const text = l.replace(/^\/\/\/\s?/, "");
        docLines.push(text);
      }
    }

    if (docLines.length === 0) continue;

    // Walk forward from the derive line to find the type declaration
    let typeName = null;
    for (let j = i + 1; j < lines.length && j < i + 10; j++) {
      const decl = lines[j].trim();
      const match = decl.match(
        /^pub\s+(?:struct|enum|type)\s+(\w+)/
      );
      if (match) {
        typeName = match[1];
        break;
      }
    }

    if (!typeName) continue;

    // Build JSDoc block
    const jsdoc = [
      "/**",
      ...docLines.map((l) => (l ? ` * ${l}` : " *")),
      " */",
    ].join("\n");

    docs.set(typeName, jsdoc);
  }

  return docs;
}

/**
 * Inject JSDoc comments into a `.d.ts` file.
 *
 * For each type in `docs`, find the matching `export interface X`,
 * `export type X`, or `export enum X` declaration and prepend the JSDoc.
 */
function injectJsdoc(dtsPath, docs) {
  let content;
  try {
    content = readFileSync(dtsPath, "utf-8");
  } catch {
    console.warn(`  Skipping ${dtsPath} (file not found)`);
    return 0;
  }

  let injected = 0;

  for (const [typeName, jsdoc] of docs) {
    // Match export declarations that don't already have a JSDoc comment
    // Handles: export interface X, export type X, export enum X
    const pattern = new RegExp(
      `^(export\\s+(?:interface|type|enum)\\s+${typeName}\\b)`,
      "m"
    );

    if (!pattern.test(content)) continue;

    // Check if there's already a JSDoc comment before this declaration
    const alreadyDocumented = new RegExp(
      `\\*/\\s*\\n\\s*export\\s+(?:interface|type|enum)\\s+${typeName}\\b`
    );
    if (alreadyDocumented.test(content)) continue;

    content = content.replace(pattern, `${jsdoc}\n$1`);
    injected++;
  }

  if (injected > 0) {
    writeFileSync(dtsPath, content, "utf-8");
  }

  return injected;
}

/**
 * Fix readonly modifier mismatches between Tsify interfaces and
 * wasm-bindgen classes.
 *
 * When a type has both `export interface Foo { bar: T; }` (from Tsify)
 * and `export class Foo { readonly bar: T; }` (from wasm-bindgen),
 * TypeScript declaration merging requires identical modifiers. This
 * function adds `readonly` to interface fields that have a corresponding
 * `readonly` property in the class.
 */
function fixReadonlyModifiers(dtsPath) {
  let content;
  try {
    content = readFileSync(dtsPath, "utf-8");
  } catch {
    return 0;
  }

  // Find all type names that have both an interface and a class declaration
  const interfaceNames = new Set(
    [...content.matchAll(/^export\s+interface\s+(\w+)/gm)].map((m) => m[1])
  );
  const classNames = new Set(
    [...content.matchAll(/^export\s+class\s+(\w+)/gm)].map((m) => m[1])
  );
  const conflicts = [...interfaceNames].filter((n) => classNames.has(n));

  if (conflicts.length === 0) return 0;

  let fixed = 0;

  for (const name of conflicts) {
    // Extract readonly property names from the class declaration
    const classPattern = new RegExp(
      `^export\\s+class\\s+${name}\\b[^}]*?^}`,
      "ms"
    );
    const classMatch = content.match(classPattern);
    if (!classMatch) continue;

    const readonlyProps = new Set(
      [...classMatch[0].matchAll(/^\s+readonly\s+(\w+)\s*:/gm)].map(
        (m) => m[1]
      )
    );
    if (readonlyProps.size === 0) continue;

    // Find the interface block and add readonly to matching fields.
    // We match the full interface block, then do field-level replacements
    // inside it.
    const ifacePattern = new RegExp(
      `(^export\\s+interface\\s+${name}\\b[\\s\\S]*?^})`,
      "m"
    );
    const ifaceMatch = content.match(ifacePattern);
    if (!ifaceMatch) continue;

    let ifaceBlock = ifaceMatch[0];
    let changed = false;

    for (const prop of readonlyProps) {
      // Match a non-readonly field with this property name inside the
      // interface (camelCase in the class may differ from snake_case in
      // the interface, so we need to match by converting).  However the
      // TS errors only fire when the names are identical, so we only
      // need to fix exact-name matches.
      const fieldPattern = new RegExp(
        `^(\\s+)(?!readonly\\s)(${prop}\\s*[?]?\\s*:)`,
        "m"
      );
      if (fieldPattern.test(ifaceBlock)) {
        ifaceBlock = ifaceBlock.replace(fieldPattern, "$1readonly $2");
        changed = true;
      }
    }

    if (changed) {
      content = content.replace(ifaceMatch[0], ifaceBlock);
      fixed++;
    }
  }

  if (fixed > 0) {
    writeFileSync(dtsPath, content, "utf-8");
    console.log(
      `Fixed readonly modifiers in ${fixed} interface(s) in ${dtsPath}`
    );
  }

  return fixed;
}

// ── Main ────────────────────────────────────────────────────────────────

const rsFiles = collectRsFiles(SRC_DIR);
const allDocs = new Map();

for (const file of rsFiles) {
  const docs = extractTsifyDocs(file);
  for (const [name, jsdoc] of docs) {
    allDocs.set(name, jsdoc);
  }
}

console.log(`Found ${allDocs.size} Tsify-derived types with doc comments:`);
for (const name of allDocs.keys()) {
  console.log(`  - ${name}`);
}

let totalInjected = 0;
for (const dtsPath of DTS_PATHS) {
  const count = injectJsdoc(dtsPath, allDocs);
  if (count > 0) {
    console.log(`Injected ${count} JSDoc comments into ${dtsPath}`);
  }
  totalInjected += count;
}

if (totalInjected === 0) {
  console.log("No JSDoc comments injected (types may already be documented or .d.ts files not found).");
}

// Fix readonly modifier mismatches between Tsify interfaces and
// wasm-bindgen classes so TypeScript declaration merging succeeds.
for (const dtsPath of DTS_PATHS) {
  fixReadonlyModifiers(dtsPath);
}
