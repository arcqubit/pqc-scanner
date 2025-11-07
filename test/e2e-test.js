#!/usr/bin/env node
/**
 * E2E Test for PQC Scanner
 * Tests complete workflow on vulnerable sample applications
 */

const fs = require('fs');
const path = require('path');

// Colors
const GREEN = '\x1b[32m';
const RED = '\x1b[31m';
const YELLOW = '\x1b[33m';
const BLUE = '\x1b[34m';
const CYAN = '\x1b[36m';
const RESET = '\x1b[0m';

console.log(`${CYAN}${'='.repeat(70)}${RESET}`);
console.log(`${CYAN}PhotonIQ PQC Scanner - End-to-End Testing${RESET}`);
console.log(`${CYAN}${'='.repeat(70)}${RESET}\n`);

// Load WASM module
let wasm;
try {
    wasm = require('../pkg-nodejs/rust_wasm_app.js');
    console.log(`${GREEN}✓ WASM module loaded successfully${RESET}\n`);
} catch (err) {
    console.error(`${RED}✗ Failed to load WASM module:${RESET}`, err.message);
    console.error(`${YELLOW}  Run "npm run build:nodejs" first${RESET}`);
    process.exit(1);
}

// Test results tracking
const results = {
    tests: [],
    reports: [],
    totalVulnerabilities: 0,
    criticalCount: 0,
    highCount: 0,
    mediumCount: 0
};

function logSection(title) {
    console.log(`\n${BLUE}━━━ ${title} ${'━'.repeat(70 - title.length - 5)}${RESET}\n`);
}

function logSuccess(message) {
    console.log(`${GREEN}✓${RESET} ${message}`);
}

function logError(message) {
    console.log(`${RED}✗${RESET} ${message}`);
}

function logInfo(message) {
    console.log(`${YELLOW}ℹ${RESET} ${message}`);
}

// Test 1: Scan vulnerable JavaScript app
logSection('Test 1: Scanning Vulnerable JavaScript Application');

const jsFile = path.join(__dirname, 'samples', 'vulnerable-app.js');
const jsSource = fs.readFileSync(jsFile, 'utf8');

try {
    const jsResult = wasm.audit_code(jsSource, 'javascript');

    logSuccess(`Scanned ${jsResult.stats.lines_scanned} lines`);
    logSuccess(`Found ${jsResult.stats.total_vulnerabilities} vulnerabilities`);
    logInfo(`Risk Score: ${jsResult.risk_score}/100`);
    logInfo(`Critical: ${jsResult.stats.critical_count}, High: ${jsResult.stats.high_count}, Medium: ${jsResult.stats.medium_count}`);

    results.tests.push({
        name: 'JavaScript App Scan',
        passed: true,
        vulnerabilities: jsResult.stats.total_vulnerabilities
    });

    results.totalVulnerabilities += jsResult.stats.total_vulnerabilities;
    results.criticalCount += jsResult.stats.critical_count;
    results.highCount += jsResult.stats.high_count;
    results.mediumCount += jsResult.stats.medium_count;

    // Display some vulnerabilities
    console.log(`\n  Sample Vulnerabilities:`);
    jsResult.vulnerabilities.slice(0, 5).forEach((vuln, i) => {
        console.log(`    ${i + 1}. [${vuln.severity}] ${vuln.crypto_type} at line ${vuln.line}`);
    });

} catch (err) {
    logError(`Failed to scan JavaScript app: ${err.message}`);
    results.tests.push({ name: 'JavaScript App Scan', passed: false });
}

// Test 2: Generate SC-13 compliance report for JavaScript app
logSection('Test 2: SC-13 Compliance Report - JavaScript');

try {
    const sc13Report = wasm.generate_compliance_report(jsSource, 'javascript', 'vulnerable-app.js');

    logSuccess(`Report ID: ${sc13Report.metadata.report_id}`);
    logSuccess(`Control: ${sc13Report.control_assessment.control_id.toUpperCase()}`);
    logInfo(`Compliance Score: ${sc13Report.summary.compliance_score}/100`);
    logInfo(`Implementation Status: ${sc13Report.control_assessment.implementation_status}`);
    logInfo(`Assessment Status: ${sc13Report.control_assessment.assessment_status}`);
    logInfo(`Total Findings: ${sc13Report.findings.length}`);

    // Save report
    const sc13Path = path.join(__dirname, 'results', 'js-sc13-report.json');
    fs.mkdirSync(path.dirname(sc13Path), { recursive: true });
    fs.writeFileSync(sc13Path, JSON.stringify(sc13Report, null, 2));
    logSuccess(`Saved report to: ${sc13Path}`);

    results.tests.push({ name: 'JS SC-13 Report', passed: true });
    results.reports.push({ type: 'SC-13', language: 'JavaScript', path: sc13Path });

} catch (err) {
    logError(`Failed to generate SC-13 report: ${err.message}`);
    results.tests.push({ name: 'JS SC-13 Report', passed: false });
}

// Test 3: Generate OSCAL JSON for JavaScript app
logSection('Test 3: OSCAL Assessment Results - JavaScript');

try {
    const oscalReport = wasm.generate_oscal_report(jsSource, 'javascript', 'vulnerable-app.js');

    logSuccess(`OSCAL Version: ${oscalReport.oscal_version}`);
    logSuccess(`Assessment UUID: ${oscalReport.assessment_results.uuid}`);
    logInfo(`Results: ${oscalReport.assessment_results.results.length}`);

    if (oscalReport.assessment_results.results.length > 0) {
        const firstResult = oscalReport.assessment_results.results[0];
        logInfo(`Observations: ${firstResult.observations.length}`);
        logInfo(`Findings: ${firstResult.findings.length}`);
    }

    // Save report
    const oscalPath = path.join(__dirname, 'results', 'js-oscal-results.json');
    fs.writeFileSync(oscalPath, JSON.stringify(oscalReport, null, 2));
    logSuccess(`Saved OSCAL report to: ${oscalPath}`);

    results.tests.push({ name: 'JS OSCAL Report', passed: true });
    results.reports.push({ type: 'OSCAL', language: 'JavaScript', path: oscalPath });

} catch (err) {
    logError(`Failed to generate OSCAL report: ${err.message}`);
    results.tests.push({ name: 'JS OSCAL Report', passed: false });
}

// Test 4: Scan vulnerable Python app
logSection('Test 4: Scanning Vulnerable Python Application');

const pyFile = path.join(__dirname, 'samples', 'vulnerable-app.py');
const pySource = fs.readFileSync(pyFile, 'utf8');

try {
    const pyResult = wasm.audit_code(pySource, 'python');

    logSuccess(`Scanned ${pyResult.stats.lines_scanned} lines`);
    logSuccess(`Found ${pyResult.stats.total_vulnerabilities} vulnerabilities`);
    logInfo(`Risk Score: ${pyResult.risk_score}/100`);
    logInfo(`Critical: ${pyResult.stats.critical_count}, High: ${pyResult.stats.high_count}, Medium: ${pyResult.stats.medium_count}`);

    results.tests.push({
        name: 'Python App Scan',
        passed: true,
        vulnerabilities: pyResult.stats.total_vulnerabilities
    });

    results.totalVulnerabilities += pyResult.stats.total_vulnerabilities;
    results.criticalCount += pyResult.stats.critical_count;
    results.highCount += pyResult.stats.high_count;
    results.mediumCount += pyResult.stats.medium_count;

    // Display some vulnerabilities
    console.log(`\n  Sample Vulnerabilities:`);
    pyResult.vulnerabilities.slice(0, 5).forEach((vuln, i) => {
        console.log(`    ${i + 1}. [${vuln.severity}] ${vuln.crypto_type} at line ${vuln.line}`);
    });

} catch (err) {
    logError(`Failed to scan Python app: ${err.message}`);
    results.tests.push({ name: 'Python App Scan', passed: false });
}

// Test 5: Generate SC-13 compliance report for Python app
logSection('Test 5: SC-13 Compliance Report - Python');

try {
    const sc13Report = wasm.generate_compliance_report(pySource, 'python', 'vulnerable-app.py');

    logSuccess(`Report ID: ${sc13Report.metadata.report_id}`);
    logSuccess(`Control: ${sc13Report.control_assessment.control_id.toUpperCase()}`);
    logInfo(`Compliance Score: ${sc13Report.summary.compliance_score}/100`);
    logInfo(`Implementation Status: ${sc13Report.control_assessment.implementation_status}`);
    logInfo(`Total Findings: ${sc13Report.findings.length}`);

    // Save report
    const sc13Path = path.join(__dirname, 'results', 'py-sc13-report.json');
    fs.writeFileSync(sc13Path, JSON.stringify(sc13Report, null, 2));
    logSuccess(`Saved report to: ${sc13Path}`);

    results.tests.push({ name: 'Python SC-13 Report', passed: true });
    results.reports.push({ type: 'SC-13', language: 'Python', path: sc13Path });

} catch (err) {
    logError(`Failed to generate SC-13 report: ${err.message}`);
    results.tests.push({ name: 'Python SC-13 Report', passed: false });
}

// Test 6: Generate OSCAL JSON for Python app
logSection('Test 6: OSCAL Assessment Results - Python');

try {
    const oscalReport = wasm.generate_oscal_report(pySource, 'python', 'vulnerable-app.py');

    logSuccess(`OSCAL Version: ${oscalReport.oscal_version}`);
    logSuccess(`Assessment UUID: ${oscalReport.assessment_results.uuid}`);
    logInfo(`Results: ${oscalReport.assessment_results.results.length}`);

    if (oscalReport.assessment_results.results.length > 0) {
        const firstResult = oscalReport.assessment_results.results[0];
        logInfo(`Observations: ${firstResult.observations.length}`);
        logInfo(`Findings: ${firstResult.findings.length}`);
    }

    // Save report
    const oscalPath = path.join(__dirname, 'results', 'py-oscal-results.json');
    fs.writeFileSync(oscalPath, JSON.stringify(oscalReport, null, 2));
    logSuccess(`Saved OSCAL report to: ${oscalPath}`);

    results.tests.push({ name: 'Python OSCAL Report', passed: true });
    results.reports.push({ type: 'OSCAL', language: 'Python', path: oscalPath });

} catch (err) {
    logError(`Failed to generate OSCAL report: ${err.message}`);
    results.tests.push({ name: 'Python OSCAL Report', passed: false });
}

// Summary
logSection('Test Summary');

const passed = results.tests.filter(t => t.passed).length;
const failed = results.tests.filter(t => !t.passed).length;

console.log(`Total Tests: ${results.tests.length}`);
console.log(`${GREEN}Passed: ${passed}${RESET}`);
console.log(`${failed > 0 ? RED : GREEN}Failed: ${failed}${RESET}\n`);

console.log(`Total Vulnerabilities Detected: ${results.totalVulnerabilities}`);
console.log(`  ${RED}Critical: ${results.criticalCount}${RESET}`);
console.log(`  ${YELLOW}High: ${results.highCount}${RESET}`);
console.log(`  ${BLUE}Medium: ${results.mediumCount}${RESET}\n`);

console.log(`Generated Reports: ${results.reports.length}`);
results.reports.forEach(report => {
    console.log(`  ${CYAN}[${report.type}]${RESET} ${report.language}: ${report.path}`);
});

// Save summary
const summaryPath = path.join(__dirname, 'results', 'e2e-test-summary.json');
fs.writeFileSync(summaryPath, JSON.stringify({
    timestamp: new Date().toISOString(),
    tests: results.tests,
    reports: results.reports,
    statistics: {
        totalVulnerabilities: results.totalVulnerabilities,
        criticalCount: results.criticalCount,
        highCount: results.highCount,
        mediumCount: results.mediumCount
    }
}, null, 2));

console.log(`\n${GREEN}✓ Test summary saved to: ${summaryPath}${RESET}\n`);

console.log(`${CYAN}${'='.repeat(70)}${RESET}`);
if (failed === 0) {
    console.log(`${GREEN}All E2E tests passed successfully!${RESET}`);
} else {
    console.log(`${RED}Some tests failed. Review the output above.${RESET}`);
}
console.log(`${CYAN}${'='.repeat(70)}${RESET}\n`);

process.exit(failed > 0 ? 1 : 0);
