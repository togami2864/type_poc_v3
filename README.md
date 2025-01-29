# type_poc_v3

## Overview

![image](https://private-user-images.githubusercontent.com/62130798/408098082-a4e46fc8-254a-4d5e-9b95-fb08251ae977.png?jwt=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3MzgyMzYwMzYsIm5iZiI6MTczODIzNTczNiwicGF0aCI6Ii82MjEzMDc5OC80MDgwOTgwODItYTRlNDZmYzgtMjU0YS00ZDVlLTliOTUtZmIwODI1MWFlOTc3LnBuZz9YLUFtei1BbGdvcml0aG09QVdTNC1ITUFDLVNIQTI1NiZYLUFtei1DcmVkZW50aWFsPUFLSUFWQ09EWUxTQTUzUFFLNFpBJTJGMjAyNTAxMzAlMkZ1cy1lYXN0LTElMkZzMyUyRmF3czRfcmVxdWVzdCZYLUFtei1EYXRlPTIwMjUwMTMwVDExMTUzNlomWC1BbXotRXhwaXJlcz0zMDAmWC1BbXotU2lnbmF0dXJlPWRjMDYxMDk5MzM3ODA0OTE1OGIwZjgzNWIyZjA1NTg4MjFkM2Q2ZTkwZjdlOWQ4OWRlOGRiZjJkZDFlZGViYmYmWC1BbXotU2lnbmVkSGVhZGVycz1ob3N0In0.8GPz4ASjIn8nb1EM4AoCIs7SYzTRpxQ7y9qyaBj7r8M)

1. Firstly, all the built-in types are loaded. Built-in types are defined in the .d.ts files located under the typescript/lib directory. These files do not contain expressions or implementations with values.
2. The type information collected here is stored in a HashMap of `<String, Type>`.
3. Next, the target files for linting are analyzed to gather type declarations. By collecting type information from all files in advance, we aim to accurately resolve cross-references and properly handle type merges in the future.
4. The type information collected here is stored in a HashMap of `<PathBuf, <String, Type>>`.
5. When access to type information is required, the get_type_info function is called to obtain type information from the symbol's name.

## Supported Features
- types
  - number
  - string
  - boolean
  - symbol
  - bigint
  - any
  - unknown
  - never
  - void
- literal
  - string literal
  - number literal
  - boolean literal
  - object literal
- Declaration
  - variable declaration
  - function declaration
  - interface declaration
  - type alias
  - ambient declaration
- union


## Problem
- This PoC did not consider the scope. We think utilizing 'biome_js_semantic' could address this issue.
- The data structure is quite simple. Defining `<PathBuf, <String, Type>>` for the symbol table could easily lead to conflicts or numerous problems.
- Generics resolution is not implemented yet.
- Type merge is not implemented yet.
- The codebase is quite messy. We need to improve the structure of the codebase to enhance clarity and maintainability.

## Sample

In the samples in this repository, the analyzer loads the built-in file

```typescript
// https://github.com/microsoft/TypeScript/blob/caf1aee269d1660b4d2a8b555c2d602c97cb28d7/src/lib/es5.d.ts#L1512-L1540
interface PromiseLike<T> {
  /**
   * Attaches callbacks for the resolution and/or rejection of the Promise.
   * @param onfulfilled The callback to execute when the Promise is resolved.
   * @param onrejected The callback to execute when the Promise is rejected.
   * @returns A Promise for the completion of which ever callback is executed.
   */
  then<TResult1 = T, TResult2 = never>(
    onfulfilled?:
      | ((value: T) => TResult1 | PromiseLike<TResult1>)
      | undefined
      | null,
    onrejected?:
      | ((reason: any) => TResult2 | PromiseLike<TResult2>)
      | undefined
      | null
  ): PromiseLike<TResult1 | TResult2>;
}

/**
 * Represents the completion of an asynchronous operation
 */
interface Promise<T> {
  /**
   * Attaches callbacks for the resolution and/or rejection of the Promise.
   * @param onfulfilled The callback to execute when the Promise is resolved.
   * @param onrejected The callback to execute when the Promise is rejected.
   * @returns A Promise for the completion of which ever callback is executed.
   */
  then<TResult1 = T, TResult2 = never>(
    onfulfilled?:
      | ((value: T) => TResult1 | PromiseLike<TResult1>)
      | undefined
      | null,
    onrejected?:
      | ((reason: any) => TResult2 | PromiseLike<TResult2>)
      | undefined
      | null
  ): Promise<TResult1 | TResult2>;

  /**
   * Attaches a callback for only the rejection of the Promise.
   * @param onrejected The callback to execute when the Promise is rejected.
   * @returns A Promise for the completion of the callback.
   */
  catch<TResult = never>(
    onrejected?:
      | ((reason: any) => TResult | PromiseLike<TResult>)
      | undefined
      | null
  ): Promise<T | TResult>;
}

```

I added `fake-linter`, a simple linter that checks if promises are used correctly and mimics the `no-floating-promise` rule. This analyzer can report issues in sample code.

```typescript
async function test(): Promise<void> {
  Promise.resolve("value");
  Promise.resolve("value").then(() => {});
  Promise.resolve("value").catch();
  Promise.resolve("value").finally();
}

async function returnPromise(): Promise<string> {
  return "value";
}
// invalid
returnPromise();
returnPromise().then(() => {});
returnPromise().catch();
returnPromise().finally();

//valid
await returnPromise2();
(async () => {
  await returnPromise2();
  await returnPromise2().then(() => {});
  await returnPromise2().catch();
  await returnPromise2().finally();
})();

declare const promiseValue: Promise<number>;
async function test4(): Promise<void> {
  promiseValue;
  promiseValue.then(() => {});
  promiseValue.catch();
  await promiseValue.finally();
}
test4();

declare const promiseOrNumber: Promise<number> | number;

async function test() {
  promiseOrNumber;
}

const foo = async (): Promise<void> => {
  Promise.resolve("value");
};
foo();

```