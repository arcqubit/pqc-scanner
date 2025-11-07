/**
 * TypeScript type definitions for PhotonIQ PQC Scanner
 * Auto-generated types for WASM module
 */

export type Language = 'rust' | 'javascript' | 'typescript' | 'python' | 'java' | 'go' | 'cpp' | 'csharp';

export type Severity = 'low' | 'medium' | 'high' | 'critical';

export type CryptoType =
  | 'RSA'
  | 'ECDSA'
  | 'ECDH'
  | 'DSA'
  | 'DIFFIE_HELLMAN'
  | 'SHA1'
  | 'MD5'
  | 'DES'
  | 'TRIPLE_DES'
  | 'RC4';

export interface Vulnerability {
  crypto_type: CryptoType;
  severity: Severity;
  risk_score: number;
  line: number;
  column: number;
  context: string;
  message: string;
  recommendation: string;
  key_size?: number;
}

export interface AuditStats {
  total_vulnerabilities: number;
  critical_count: number;
  high_count: number;
  medium_count: number;
  low_count: number;
  lines_scanned: number;
}

export interface AuditResult {
  vulnerabilities: Vulnerability[];
  risk_score: number;
  language: Language;
  recommendations: string[];
  stats: AuditStats;
}

export type ImplementationStatus =
  | 'implemented'
  | 'partiallyimplemented'
  | 'plannedforimplementation'
  | 'alternativeimplementation'
  | 'notapplicable';

export type AssessmentStatus = 'satisfied' | 'notsatisfied' | 'other';

export type EvidenceType =
  | 'code-analysis'
  | 'static-scan'
  | 'configuration-review'
  | 'documentation-review'
  | 'automated-test';

export interface SourceLocation {
  file_path: string;
  line: number;
  column: number;
  snippet: string;
}

export interface Evidence {
  evidence_id: string;
  evidence_type: EvidenceType;
  description: string;
  source_location?: SourceLocation;
  collected_at: string;
  data: any;
}

export interface ControlFinding {
  finding_id: string;
  control_id: string;
  implementation_status: ImplementationStatus;
  assessment_status: AssessmentStatus;
  description: string;
  related_vulnerabilities: string[];
  evidence: Evidence[];
  remediation: string;
  risk_level: Severity;
}

export interface ReportMetadata {
  report_id: string;
  title: string;
  published: string;
  last_modified: string;
  version: string;
  oscal_version: string;
}

export interface ControlAssessment {
  control_id: string;
  control_name: string;
  control_family: string;
  control_description: string;
  implementation_status: ImplementationStatus;
  assessment_status: AssessmentStatus;
  assessment_method: string[];
}

export interface AssessmentSummary {
  files_scanned: number;
  lines_scanned: number;
  total_vulnerabilities: number;
  quantum_vulnerable_algorithms: string[];
  deprecated_algorithms: string[];
  weak_key_sizes: string[];
  compliance_score: number;
  risk_score: number;
}

export interface SC13AssessmentReport {
  metadata: ReportMetadata;
  control_assessment: ControlAssessment;
  summary: AssessmentSummary;
  findings: ControlFinding[];
  recommendations: string[];
}

export interface OscalMetadata {
  title: string;
  published: string;
  'last-modified': string;
  version: string;
  'oscal-version': string;
  roles?: Array<{ id: string; title: string }>;
  parties?: Array<{ uuid: string; type: string; name: string }>;
}

export interface OscalAssessmentResults {
  'oscal-version': string;
  'assessment-results': {
    uuid: string;
    metadata: OscalMetadata;
    'import-ssp': { href: string };
    results: Array<{
      uuid: string;
      title: string;
      description: string;
      start: string;
      end?: string;
      'reviewed-controls': {
        'control-selections': Array<{
          'include-controls': Array<{ 'control-id': string }>;
        }>;
      };
      observations: Array<{
        uuid: string;
        description: string;
        methods: string[];
        types?: string[];
        collected?: string;
        'relevant-evidence'?: Array<{
          href: string;
          description: string;
        }>;
      }>;
      findings: Array<{
        uuid: string;
        title: string;
        description: string;
        target: {
          type: string;
          'target-id': string;
          status?: { state: string };
        };
        'implementation-status': { state: string };
        'related-observations'?: Array<{
          'observation-uuid': string;
        }>;
      }>;
    }>;
  };
}

/**
 * Initialize the WASM module
 */
export function init(): void;

/**
 * Analyze source code for quantum-vulnerable cryptography
 * @param source - Source code to analyze
 * @param language - Programming language ('rust', 'javascript', 'python', etc.)
 * @returns Audit result with vulnerabilities and recommendations
 */
export function audit_code(source: string, language: string): AuditResult;

/**
 * Generate NIST 800-53 SC-13 compliance report
 * @param source - Source code to analyze
 * @param language - Programming language
 * @param file_path - Optional file path for evidence tracking
 * @returns SC-13 assessment report with findings and evidence
 */
export function generate_compliance_report(
  source: string,
  language: string,
  file_path?: string
): SC13AssessmentReport;

/**
 * Generate OSCAL-compliant assessment results
 * @param source - Source code to analyze
 * @param language - Programming language
 * @param file_path - Optional file path for evidence tracking
 * @returns OSCAL 1.1.2 assessment results
 */
export function generate_oscal_report(
  source: string,
  language: string,
  file_path?: string
): OscalAssessmentResults;
