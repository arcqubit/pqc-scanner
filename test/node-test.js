#!/usr/bin/env node
/**
 * Node.js Integration Test for PhotonIQ PQC Scanner
 *
 * Tests the WASM module in Node.js environment
 */

const path = require('path');
const fs = require('fs');

// Colors for output
const GREEN = '\x1b[32m';
const RED = '\x1b[31m';
const YELLOW = '\x1b[33m';
const RESET = '\x1b[0m';

console.log('🧪 Running Node.js Integration Tests...\n');

// Check if pkg-nodejs exists
const pkgPath = path.join(__dirname, '..', 'pkg-nodejs');
if (!fs.existsSync(pkgPath)) {
    console.error(`${RED}✗ Error: pkg-nodejs directory not found${RESET}`);
    console.error('  Run "npm run build:nodejs" first');
    process.exit(1);
}

// Load the WASM module
let wasm;
try {
    wasm = require('../pkg-nodejs/rust_wasm_app.js');
    console.log(`${GREEN}✓ WASM module loaded${RESET}\n`);
} catch (err) {
    console.error(`${RED}✗ Failed to load WASM module:${RESET}`, err.message);
    process.exit(1);
}

// Test suite
const tests = [];
let passed = 0;
let failed = 0;

function test(name, fn) {
    tests.push({ name, fn });
}

function assert(condition, message) {
    if (!condition) {
        throw new Error(message || 'Assertion failed');
    }
}

function assertEqual(actual, expected, message) {
    if (actual !== expected) {
        throw new Error(message || `Expected ${expected}, got ${actual}`);
    }
}

// Test 1: Basic audit functionality
test('Basic crypto detection', () => {
    const source = "const rsa = crypto.generateKeyPair('rsa', { modulusLength: 1024 });";
    const result = wasm.audit_code(source, 'javascript');

    assert(result.vulnerabilities.length > 0, 'Should detect RSA vulnerability');
    assert(result.risk_score > 0, 'Should have non-zero risk score');
    assertEqual(result.language, 'javascript', 'Should preserve language');
});

// Test 2: Multiple vulnerabilities
test('Multiple crypto algorithms', () => {
    const source = `
        const md5 = crypto.createHash('md5');
        const sha1 = crypto.createHash('sha1');
        const rsa = crypto.generateKeyPair('rsa', { modulusLength: 2048 });
    `;
    const result = wasm.audit_code(source, 'javascript');

    assert(result.vulnerabilities.length >= 3, 'Should detect multiple vulnerabilities');
    assert(result.stats.critical_count > 0, 'Should have critical vulnerabilities');
});

// Test 3: Clean code (no vulnerabilities)
test('Clean code detection', () => {
    const source = `
        // Using quantum-safe algorithms
        const safe = 'no crypto here';
    `;
    const result = wasm.audit_code(source, 'javascript');

    assertEqual(result.vulnerabilities.length, 0, 'Should have no vulnerabilities');
    assertEqual(result.risk_score, 0, 'Risk score should be zero');
});

// Test 4: SC-13 compliance report generation
test('SC-13 compliance report', () => {
    const source = "const md5 = crypto.createHash('md5');";
    const report = wasm.generate_compliance_report(source, 'javascript', 'test.js');

    assert(report.metadata, 'Should have metadata');
    assert(report.control_assessment, 'Should have control assessment');
    assertEqual(report.control_assessment.control_id, 'sc-13', 'Should be SC-13 control');
    assert(report.summary.total_vulnerabilities > 0, 'Should detect vulnerabilities');
    assert(report.findings.length > 0, 'Should have findings');
});

// Test 5: OSCAL JSON generation
test('OSCAL JSON output', () => {
    const source = "const rsa = crypto.generateKeyPair('rsa', { modulusLength: 1024 });";
    const oscal = wasm.generate_oscal_report(source, 'javascript', 'test.js');

    assertEqual(oscal.oscal_version, '1.1.2', 'Should use OSCAL 1.1.2');
    assert(oscal.assessment_results, 'Should have assessment results');
    assert(oscal.assessment_results.uuid, 'Should have UUID');
    assert(oscal.assessment_results.results.length > 0, 'Should have results');
});

// Test 6: Multi-language support
test('Multi-language support', () => {
    const languages = [
        { lang: 'rust', code: 'use crypto::md5;' },
        { lang: 'python', code: 'import hashlib; hashlib.md5()' },
        { lang: 'javascript', code: 'crypto.createHash("md5")' }
    ];

    for (const { lang, code } of languages) {
        const result = wasm.audit_code(code, lang);
        assert(result.language === lang, `Should support ${lang}`);
    }
});

// Test 7: Error handling - invalid language
test('Error handling - invalid language', () => {
    try {
        wasm.audit_code('test', 'invalid-lang');
        throw new Error('Should have thrown error for invalid language');
    } catch (err) {
        assert(err.message.includes('Unsupported language'), 'Should error on invalid language');
    }
});

// Test 8: Error handling - empty source
test('Error handling - empty source', () => {
    try {
        wasm.audit_code('', 'javascript');
        throw new Error('Should have thrown error for empty source');
    } catch (err) {
        assert(err.message.includes('Invalid source'), 'Should error on empty source');
    }
});

// Run all tests
console.log('Running tests...\n');

for (const { name, fn } of tests) {
    try {
        fn();
        console.log(`${GREEN}✓${RESET} ${name}`);
        passed++;
    } catch (err) {
        console.log(`${RED}✗${RESET} ${name}`);
        console.log(`  ${RED}${err.message}${RESET}`);
        failed++;
    }
}

// Summary
console.log('\n' + '='.repeat(50));
console.log(`Tests: ${tests.length} total, ${GREEN}${passed} passed${RESET}, ${failed > 0 ? RED : GREEN}${failed} failed${RESET}`);
console.log('='.repeat(50) + '\n');

if (failed > 0) {
    process.exit(1);
} else {
    console.log(`${GREEN}✅ All Node.js tests passed!${RESET}\n`);
    process.exit(0);
}
