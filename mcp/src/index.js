#!/usr/bin/env node
/**
 * PQC Scanner MCP Server
 * Model Context Protocol server for quantum-safe cryptography analysis
 *
 * Implements MCP 2025-11 specification
 */

import { Server } from '@modelcontextprotocol/sdk/server/index.js';
import { StdioServerTransport } from '@modelcontextprotocol/sdk/server/stdio.js';
import {
  CallToolRequestSchema,
  ListToolsRequestSchema
} from '@modelcontextprotocol/sdk/types.js';
import { spawn } from 'child_process';
import { readFileSync, writeFileSync } from 'fs';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const PROJECT_ROOT = join(__dirname, '../..');

// Load tool definitions
const TOOLS_DIR = join(__dirname, '../tools');
const TOOLS = [
  'scan_code',
  'analyze_file',
  'get_remediation',
  'validate_compliance'
].map(name => JSON.parse(
  readFileSync(join(TOOLS_DIR, `${name}.json`), 'utf-8')
));

/**
 * Execute Rust scanner via cargo
 */
async function executeScanner(args) {
  return new Promise((resolve, reject) => {
    const proc = spawn('cargo', [
      'run',
      '--quiet',
      '--example',
      'scan_directory',
      '--',
      ...args
    ], {
      cwd: PROJECT_ROOT,
      stdio: ['ignore', 'pipe', 'pipe']
    });

    let stdout = '';
    let stderr = '';

    proc.stdout.on('data', (data) => {
      stdout += data.toString();
    });

    proc.stderr.on('data', (data) => {
      stderr += data.toString();
    });

    proc.on('close', (code) => {
      // Code 1 is expected when vulnerabilities are found
      if (code === 0 || code === 1) {
        resolve({ stdout, stderr, code });
      } else {
        reject(new Error(`Scanner failed with code ${code}: ${stderr}`));
      }
    });

    proc.on('error', (err) => {
      reject(err);
    });
  });
}

/**
 * Scan code for vulnerabilities
 */
async function scanCode(args) {
  const { path, format = 'sc13' } = args;
  const outputFile = `/tmp/pqc-scan-${Date.now()}.json`;

  try {
    await executeScanner([
      '--path', path,
      '--output', outputFile,
      '--format', format
    ]);

    const report = JSON.parse(readFileSync(outputFile, 'utf-8'));

    return {
      report,
      summary: report.summary || {
        files_scanned: report.summary?.files_scanned || 0,
        lines_scanned: report.summary?.lines_scanned || 0,
        vulnerabilities_found: report.summary?.total_vulnerabilities || 0,
        compliance_score: report.summary?.compliance_score || 0,
        risk_score: report.summary?.risk_score || 0
      },
      vulnerabilities: report.findings?.map(f => ({
        crypto_type: f.description?.split(' ')[2] || 'Unknown',
        severity: f.risk_level?.toLowerCase() || 'unknown',
        message: f.description || '',
        recommendation: f.remediation_steps?.[0] || ''
      })) || []
    };
  } catch (error) {
    throw new Error(`Scan failed: ${error.message}`);
  }
}

/**
 * Analyze single file
 */
async function analyzeFile(args) {
  const { source_code, language, filename = 'input.code' } = args;
  const tempFile = `/tmp/pqc-input-${Date.now()}.${getExtension(language)}`;

  try {
    writeFileSync(tempFile, source_code);
    const result = await scanCode({ path: tempFile, format: 'sc13' });
    return {
      vulnerabilities: result.vulnerabilities,
      stats: result.summary,
      risk_score: result.summary.risk_score,
      recommendations: result.report.recommendations || []
    };
  } catch (error) {
    throw new Error(`Analysis failed: ${error.message}`);
  }
}

/**
 * Get remediation recommendations
 */
async function getRemediation(args) {
  const { vulnerability_type, language, context } = args;

  const remediations = {
    'RSA': {
      pqc_alternative: {
        algorithm: 'CRYSTALS-Kyber (NIST)',
        nist_status: 'Standardized',
        code_example: language === 'python'
          ? 'from pqcrypto.kem.kyber768 import generate_keypair, encrypt, decrypt'
          : 'import { kyber768 } from "@noble/post-quantum/kyber";'
      },
      interim_solution: {
        algorithm: 'RSA-3072 or RSA-4096',
        code_example: language === 'python'
          ? 'from cryptography.hazmat.primitives.asymmetric import rsa\nkey = rsa.generate_private_key(public_exponent=65537, key_size=4096)'
          : 'crypto.generateKeyPairSync("rsa", { modulusLength: 4096 })'
      },
      migration_steps: [
        'Audit all RSA usage in codebase',
        'Upgrade RSA key sizes to 3072+ bits as interim measure',
        'Evaluate PQC library support for your platform',
        'Implement hybrid classical-PQC scheme',
        'Migrate to pure PQC when ready'
      ]
    },
    'MD5': {
      pqc_alternative: {
        algorithm: 'SHA-256 or SHA-3',
        nist_status: 'Recommended',
        code_example: language === 'python'
          ? 'import hashlib\nhash = hashlib.sha256(data).hexdigest()'
          : 'crypto.createHash("sha256").update(data).digest("hex")'
      },
      migration_steps: [
        'Replace MD5 with SHA-256 for integrity checks',
        'Use Argon2id for password hashing',
        'Update all hash comparisons',
        'Test thoroughly before deployment'
      ]
    }
  };

  const remediation = remediations[vulnerability_type] || {
    pqc_alternative: { algorithm: 'Contact security team' },
    migration_steps: ['Consult NIST guidelines']
  };

  return {
    vulnerability: vulnerability_type,
    severity: ['RSA', 'ECDSA', 'DSA'].includes(vulnerability_type) ? 'high' : 'critical',
    recommendation: `Migrate away from ${vulnerability_type}`,
    ...remediation,
    resources: [
      {
        title: 'NIST Post-Quantum Cryptography',
        url: 'https://csrc.nist.gov/projects/post-quantum-cryptography'
      }
    ]
  };
}

/**
 * Validate compliance
 */
async function validateCompliance(args) {
  const { path, target_score = 80, strict_mode = false } = args;

  try {
    const result = await scanCode({ path, format: 'sc13' });
    const compliant = result.summary.compliance_score >= target_score;
    const hasQuantumVulns = result.vulnerabilities.some(v =>
      ['RSA', 'ECDSA', 'ECDH', 'DSA'].includes(v.crypto_type)
    );

    return {
      compliant: strict_mode ? !hasQuantumVulns && compliant : compliant,
      compliance_score: result.summary.compliance_score,
      target_score,
      control_status: result.report.control_assessment?.assessment_status || 'unknown',
      blocking_issues: result.vulnerabilities.filter(v => v.severity === 'critical'),
      recommendations: result.report.recommendations || [],
      validation_timestamp: new Date().toISOString()
    };
  } catch (error) {
    throw new Error(`Validation failed: ${error.message}`);
  }
}

/**
 * Get file extension for language
 */
function getExtension(language) {
  const extensions = {
    'javascript': 'js',
    'python': 'py',
    'java': 'java',
    'cpp': 'cpp',
    'go': 'go',
    'rust': 'rs',
    'csharp': 'cs'
  };
  return extensions[language] || 'txt';
}

/**
 * Create and configure MCP server
 */
const server = new Server(
  {
    name: 'pqc-scanner',
    version: '1.0.0',
  },
  {
    capabilities: {
      tools: {},
    },
  }
);

// List available tools
server.setRequestHandler(ListToolsRequestSchema, async () => {
  return {
    tools: TOOLS.map(t => ({
      name: t.tool_id,
      description: t.description,
      inputSchema: t.input_schema
    }))
  };
});

// Handle tool calls
server.setRequestHandler(CallToolRequestSchema, async (request) => {
  const { name, arguments: args } = request.params;

  try {
    let result;
    switch (name) {
      case 'scan_code':
        result = await scanCode(args);
        break;
      case 'analyze_file':
        result = await analyzeFile(args);
        break;
      case 'get_remediation':
        result = await getRemediation(args);
        break;
      case 'validate_compliance':
        result = await validateCompliance(args);
        break;
      default:
        throw new Error(`Unknown tool: ${name}`);
    }

    return {
      content: [
        {
          type: 'text',
          text: JSON.stringify(result, null, 2)
        }
      ]
    };
  } catch (error) {
    return {
      content: [
        {
          type: 'text',
          text: `Error: ${error.message}`
        }
      ],
      isError: true
    };
  }
});

/**
 * Start server
 */
async function main() {
  const transport = new StdioServerTransport();
  await server.connect(transport);
  console.error('PQC Scanner MCP Server running on stdio');
}

main().catch((error) => {
  console.error('Server error:', error);
  process.exit(1);
});
