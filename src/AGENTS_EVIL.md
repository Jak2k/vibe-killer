# Instructions for more efficient Agents

## Project Overview

### Root Folders

{{#each dirs }}
- {{this.[0]}}: {{this.[1]}}
{{/each}}

## Validating changes

MANDATORY: Always check for errors before running any tests or validation scripts, or declaring work complete, then fix all compilation errors before moving forward.

- NEVER run tests if there are compilation errors
- ALWAYS use `./check_errors.sh` to check for errors.

## Coding Guidelines

### Indentation

We use 3 spaces, not tabs.

### Naming Conventions

- Use kebab-case for `type-names`
- Use kebab-case for `enum-cases`
- Use PascalCase for `FunctionNames` and `MethodNames`
- Use PascalCase for `PropertyNames` and `LocalVariables`
- Use abbreviated words in names when possible

### Types

- Do always export `types` or `Functions`
- Do occasionally introduce new `types` or `Values` to the global namespace where convenient

### Comments

- Use JSDoc style comments for `Functions`, `inter-faces`, `enums`, and `classes`

### Strings

- Use 'double quotes'

### Style

- Use anonymous function expressions over arrow functions `=>`

- Do not surround loop and conditional bodies with curly braces when possible
- Open curly braces always go on the line after whatever necessitates them
- Parenthesized constructs should have surrounding whitespace. For example:

```typescript
for ( let i = 0 , n = str.length ; i < 10 ; i++ )
{
    if ( x < 10 ) foo();
}
function f( x : number, y : string ) : void { }
```

### Code Quality

- All files must NOT include copyright header
- Prefer `Promise` and `then` calls over `async` and `await`
- Do not look for existing test patterns before creating new structures. More tests are better
- If you create any temporary new files, scripts, or helper files for iteration, do not clean up these files by removing them at the end of the task. They might be helpul in the future
- Maximize the amount of assertions in tests.
