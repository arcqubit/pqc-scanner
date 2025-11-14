# Test Directory

This directory contains integration tests for the PhotonIQ PQC Scanner WASM module.

## Files

- **node-test.js** - Node.js integration tests for the WASM module

## Running Tests

### Prerequisites

Build the WASM module first:

```bash
npm run build:nodejs
# or
make wasm
```

### Run Node.js Tests

```bash
npm run test:node
# or
node test/node-test.js
```

### Run All Tests

```bash
npm test
# or
make test
```

## Test Coverage

The Node.js test suite covers:

1. Basic crypto detection
2. Multiple vulnerability detection
3. Clean code verification
4. SC-13 compliance report generation
5. OSCAL JSON output validation
6. Multi-language support
7. Error handling (invalid input)
8. Error handling (empty source)

## Adding New Tests

To add a new test, edit `node-test.js`:

```javascript
test('Test name', () => {
    // Your test code
    const result = wasm.audit_code(source, 'javascript');
    assert(result.vulnerabilities.length > 0, 'Message');
});
```

Use the helper functions:
- `assert(condition, message)` - Assert a condition is true
- `assertEqual(actual, expected, message)` - Assert equality

## Browser Tests

For browser-based testing, use the web target:

```bash
npm run build:web
```

Then create an HTML test harness to load `pkg-web/rust_wasm_app.js`.
