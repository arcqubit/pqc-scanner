# Quantum Threat Analysis with MITRE ATLAS Integration

## Executive Summary

This document outlines a comprehensive plan to integrate **MITRE ATLAS (Adversarial Threat Landscape for AI Systems)** framework into the pqc-scanner to provide **quantum threat analysis** capabilities. This integration adds a value-added, non-destructive feature that maps cryptographic vulnerabilities to adversarial AI/ML attack tactics targeting quantum-vulnerable systems.

**Key Value Proposition:**
- Map quantum-vulnerable cryptography to ATLAS threat tactics
- Identify AI/ML model risks in cryptographic implementations
- Provide adversarial threat intelligence for quantum computing era
- Enable threat modeling for quantum-ready AI systems
- Generate ATLAS-aligned threat reports alongside compliance reports

**Implementation Status:** Planning Phase
**Target Timeline:** 8-10 weeks
**Backward Compatibility:** 100% (fully additive feature)

---

## Table of Contents

1. [Introduction](#1-introduction)
2. [MITRE ATLAS Framework Overview](#2-mitre-atlas-framework-overview)
3. [Quantum-ATLAS Threat Model](#3-quantum-atlas-threat-model)
4. [Architecture Design](#4-architecture-design)
5. [Implementation Plan](#5-implementation-plan)
6. [Threat Scoring Engine](#6-threat-scoring-engine)
7. [Attack Path Modeling](#7-attack-path-modeling)
8. [Integration Points](#8-integration-points)
9. [Use Cases](#9-use-cases)
10. [Deliverables](#10-deliverables)
11. [Success Metrics](#11-success-metrics)

---

## 1. Introduction

### 1.1 Background

As organizations integrate AI/ML systems with cryptographic components, they face a convergence of two threat landscapes:

1. **Quantum Computing Threats**: CRQC (Cryptographically Relevant Quantum Computer) breaking classical cryptography
2. **AI/ML Adversarial Threats**: Attacks targeting machine learning systems

The intersection of these domains creates unique attack surfaces:
- **AI-powered cryptanalysis**: ML models accelerating quantum-vulnerable crypto attacks
- **Quantum-enhanced ML attacks**: Quantum algorithms targeting AI model extraction
- **Hybrid system vulnerabilities**: Crypto protecting AI models, AI protecting crypto keys
- **Supply chain risks**: Compromised ML models in crypto implementations

### 1.2 MITRE ATLAS Relevance to PQC-Scanner

MITRE ATLAS provides a knowledge base of adversarial tactics and techniques targeting AI/ML systems. When combined with pqc-scanner's cryptographic inventory capabilities, we can:

- **Map crypto vulnerabilities to ML attack vectors**
- **Identify AI-assisted quantum threats**
- **Model adversarial attack paths through crypto+AI systems**
- **Prioritize migration based on threat intelligence**
- **Enhance risk scoring with adversarial context**

### 1.3 Non-Destructive Integration Philosophy

This feature is designed as a **value-added overlay** that:
- ✅ Does not modify existing audit/compliance modules
- ✅ Operates as an optional analysis layer
- ✅ Enriches existing reports with threat intelligence
- ✅ Provides standalone ATLAS-formatted reports
- ✅ Maintains 100% backward compatibility

---

## 2. MITRE ATLAS Framework Overview

### 2.1 ATLAS Matrix Structure

**MITRE ATLAS** (Adversarial Threat Landscape for Artificial-Intelligence Systems) defines **14 tactics** and **56 techniques** for attacking AI/ML systems.

#### 2.1.1 The 14 ATLAS Tactics

| Tactic ID | Tactic Name | Description | Quantum-Crypto Relevance |
|-----------|-------------|-------------|--------------------------|
| **AML.TA0001** | Reconnaissance | Gather information about ML systems | Identify crypto implementations protecting AI models |
| **AML.TA0002** | Resource Development | Create/acquire resources for attacks | Develop quantum simulators, acquire QPU time |
| **AML.TA0003** | Initial Access | Gain foothold in ML system | Exploit weak crypto in AI API authentication |
| **AML.TA0004** | ML Model Access | Interact with AI models | Query models to extract crypto key patterns |
| **AML.TA0005** | Execution | Run malicious code in AI systems | Execute quantum algorithms via compromised ML pipelines |
| **AML.TA0006** | Persistence | Maintain access to ML infrastructure | Backdoor crypto libraries used in ML training |
| **AML.TA0007** | Privilege Escalation | Gain higher-level permissions | Escalate via weak cryptographic access controls |
| **AML.TA0008** | Defense Evasion | Avoid detection in ML systems | Evade crypto-based integrity checks |
| **AML.TA0009** | Credential Access | Obtain authentication credentials | Extract crypto keys via model inversion attacks |
| **AML.TA0010** | Discovery | Explore ML system environment | Discover cryptographic algorithms in use |
| **AML.TA0011** | Lateral Movement | Move through ML infrastructure | Pivot via weak inter-service crypto |
| **AML.TA0012** | Collection | Gather data from ML systems | Collect training data containing crypto keys |
| **AML.TA0013** | Exfiltration | Steal ML models or data | Exfiltrate models protected by weak encryption |
| **AML.TA0014** | Impact | Disrupt/destroy ML systems | Poison crypto parameters in ML training |

#### 2.1.2 Key ATLAS Techniques (Relevant to Crypto)

| Technique ID | Technique Name | Crypto Relevance |
|--------------|----------------|------------------|
| **AML.T0000** | ML Model Inference API Access | Crypto securing API endpoints |
| **AML.T0002** | Obtain Capabilities | Quantum computing capabilities |
| **AML.T0010** | ML Artifact Collection | Crypto keys in model artifacts |
| **AML.T0015** | Evade ML Model | Bypass crypto-based ML protections |
| **AML.T0017** | Backdoor ML Model | Inject crypto backdoors during training |
| **AML.T0018** | Poison Training Data | Corrupt cryptographic parameters |
| **AML.T0020** | Adversarial Perturbation | Attack crypto key generation randomness |
| **AML.T0024** | Exfiltrate ML Artifacts | Steal models encrypted with weak crypto |
| **AML.T0025** | Exfiltrate via ML Inference API | Extract keys via model queries |
| **AML.T0034** | Transfer Learning Attack | Compromise crypto in transferred models |
| **AML.T0040** | ML Model Inversion | Infer cryptographic secrets from outputs |
| **AML.T0043** | Craft Adversarial Data | Craft inputs to break crypto-AI systems |

### 2.2 ATLAS for Quantum-Era Threats

ATLAS originally focused on AI/ML attacks, but extends naturally to quantum-era scenarios:

**Quantum-Enhanced ML Attacks:**
- Quantum algorithms accelerating model extraction (AML.T0024)
- Quantum machine learning for cryptanalysis (AML.T0043)
- Harvest-now-decrypt-later targeting AI models (AML.T0013)

**ML-Enhanced Quantum Attacks:**
- AI-powered quantum circuit optimization for crypto breaking
- ML models learning cryptographic vulnerabilities
- Adversarial ML targeting post-quantum crypto implementations

**Hybrid Vulnerabilities:**
- Weak classical crypto protecting quantum-resistant AI models
- ML models with embedded cryptographic weaknesses
- Supply chain attacks on PQC libraries used in AI

---

## 3. Quantum-ATLAS Threat Model

### 3.1 Threat Taxonomy

We define a **three-layer threat model** combining quantum risks with ATLAS tactics:

```
Layer 1: Quantum Computing Threats (QCT)
├── QCT-1: CRQC Cryptanalysis (Shor's, Grover's algorithms)
├── QCT-2: Harvest-Now-Decrypt-Later (HNDL)
├── QCT-3: Quantum Side-Channel Attacks
└── QCT-4: Post-Quantum Crypto Downgrade Attacks

Layer 2: AI/ML Adversarial Threats (AAT)
├── AAT-1: Model Extraction Attacks (ATLAS AML.T0024)
├── AAT-2: Data Poisoning (ATLAS AML.T0018)
├── AAT-3: Model Inversion (ATLAS AML.T0040)
├── AAT-4: Adversarial Examples (ATLAS AML.T0043)
└── AAT-5: Backdoor Injection (ATLAS AML.T0017)

Layer 3: Quantum-AI Convergence Threats (QACT)
├── QACT-1: Quantum-Enhanced Model Extraction
├── QACT-2: AI-Accelerated Quantum Cryptanalysis
├── QACT-3: Crypto-AI Supply Chain Compromise
├── QACT-4: Quantum Backdoors in ML Crypto Libraries
└── QACT-5: PQC Downgrade via ML Evasion
```

### 3.2 Threat Mapping to pqc-scanner Detections

| pqc-scanner Detection | Quantum Threat | ATLAS Tactic | ATLAS Technique | Risk Level |
|-----------------------|----------------|--------------|-----------------|------------|
| RSA-1024 detected | QCT-1 (Shor's) | AML.TA0014 (Impact) | AML.T0043 (Craft Adversarial Data) | **Critical** |
| MD5 hash detected | QCT-1 + AAT-4 | AML.TA0014 (Impact) | AML.T0018 (Poison Training) | **Critical** |
| ECDH P-256 detected | QCT-2 (HNDL) | AML.TA0013 (Exfiltration) | AML.T0024 (Exfiltrate Artifacts) | **High** |
| RSA-2048 detected | QCT-2 (HNDL) + QACT-2 | AML.TA0012 (Collection) | AML.T0010 (ML Artifact Collection) | **High** |
| SHA-1 signature | QCT-1 + AAT-1 | AML.TA0009 (Credential Access) | AML.T0040 (Model Inversion) | **High** |
| No PQC detected | QACT-5 (Downgrade) | AML.TA0008 (Defense Evasion) | AML.T0015 (Evade ML Model) | **Medium** |
| Hardcoded key | AAT-5 (Backdoor) | AML.TA0006 (Persistence) | AML.T0017 (Backdoor ML Model) | **High** |

### 3.3 Attack Path Scenarios

**Scenario 1: Harvest-Now-Decrypt-Later on AI Model**
```
1. [AML.TA0001 Reconnaissance] → Identify ML model API using ECDH P-256
2. [AML.TA0003 Initial Access] → Access model inference API
3. [AML.TA0012 Collection] → Collect encrypted model artifacts
4. [AML.TA0013 Exfiltration] → Exfiltrate encrypted data
5. [QCT-2 HNDL] → Store for future quantum decryption
6. [Future: QCT-1 CRQC] → Decrypt with Shor's algorithm
7. [AML.TA0014 Impact] → Use extracted model for competitive advantage
```

**Scenario 2: AI-Accelerated Cryptanalysis**
```
1. [AML.TA0002 Resource Development] → Train ML model on crypto weaknesses
2. [AML.TA0010 Discovery] → Discover RSA-2048 in production
3. [QACT-2 AI-Quantum] → Use ML to optimize quantum circuit
4. [QCT-1 CRQC] → Execute Shor's algorithm efficiently
5. [AML.TA0009 Credential Access] → Extract private keys
6. [AML.TA0011 Lateral Movement] → Pivot to protected systems
```

**Scenario 3: Supply Chain Backdoor**
```
1. [AML.TA0002 Resource Development] → Compromise crypto library repo
2. [AML.TA0017 Backdoor] → Inject weak RNG in PQC implementation
3. [AML.TA0006 Persistence] → Maintain access to CI/CD pipeline
4. [AML.TA0012 Collection] → Collect keys generated with weak RNG
5. [QACT-4 Quantum Backdoor] → Exploit quantum-specific weakness
```

---

## 4. Architecture Design

### 4.1 Module Structure

```
src/
├── atlas/
│   ├── mod.rs                      # ATLAS module root
│   ├── types.rs                    # ATLAS types (tactics, techniques, threats)
│   ├── threat_mapper.rs            # Map crypto vulns to ATLAS tactics
│   ├── quantum_atlas_model.rs      # Quantum-ATLAS threat model
│   ├── attack_path_analyzer.rs     # Attack path/chain construction
│   ├── threat_scoring.rs           # ATLAS-based threat scoring
│   ├── technique_matcher.rs        # Match detections to techniques
│   └── report_generator.rs         # ATLAS-formatted reports
├── compliance.rs (EXISTING)
├── canadian_compliance.rs (EXISTING)
├── p1943_compliance.rs (PLANNED)
└── atlas_integration.rs (NEW)      # Integration orchestrator
```

### 4.2 Data Flow

```
┌─────────────────┐
│  Source Code   │
│   (Crypto)     │
└────────┬────────┘
         │
         ▼
┌─────────────────────────┐
│  Existing Audit Engine  │ (No changes)
│  (detector.rs)          │
└────────┬────────────────┘
         │
         ▼
┌──────────────────────────┐
│   AuditResult            │
│   (vulnerabilities[])    │
└────────┬─────────────────┘
         │
         ├──────────────────────────────────┬──────────────────┐
         │                                  │                  │
         ▼                                  ▼                  ▼
┌─────────────────┐          ┌──────────────────┐   ┌─────────────────┐
│ Compliance      │          │ ATLAS Threat     │   │  Existing       │
│ Reports         │          │ Analysis (NEW)   │   │  Outputs        │
│ (NIST/ITSG33)  │          └────────┬─────────┘   └─────────────────┘
└─────────────────┘                   │
                                      │
                           ┌──────────▼────────────┐
                           │  Threat Mapper        │
                           │  (quantum_atlas_model)│
                           └──────────┬────────────┘
                                      │
                           ┌──────────▼────────────┐
                           │  ATLAS Report         │
                           │  - Tactics identified │
                           │  - Techniques mapped  │
                           │  - Attack paths       │
                           │  - Threat scores      │
                           └───────────────────────┘
```

### 4.3 Core Types (`src/atlas/types.rs`)

```rust
// ATLAS Tactic Identifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ATLASTactic {
    TA0001Reconnaissance,
    TA0002ResourceDevelopment,
    TA0003InitialAccess,
    TA0004MLModelAccess,
    TA0005Execution,
    TA0006Persistence,
    TA0007PrivilegeEscalation,
    TA0008DefenseEvasion,
    TA0009CredentialAccess,
    TA0010Discovery,
    TA0011LateralMovement,
    TA0012Collection,
    TA0013Exfiltration,
    TA0014Impact,
}

impl ATLASTactic {
    pub fn id(&self) -> &str {
        match self {
            Self::TA0001Reconnaissance => "AML.TA0001",
            Self::TA0002ResourceDevelopment => "AML.TA0002",
            // ... etc
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::TA0001Reconnaissance => "Reconnaissance",
            Self::TA0014Impact => "Impact",
            // ... etc
        }
    }
}

// ATLAS Technique
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ATLASTechnique {
    pub id: String,                          // "AML.T0040"
    pub name: String,                        // "ML Model Inversion"
    pub tactic: ATLASTactic,
    pub description: String,
    pub quantum_relevance: QuantumRelevance,
    pub detection_confidence: f32,           // 0.0 - 1.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuantumRelevance {
    DirectQuantumThreat,                     // Shor's, Grover's
    HarvestNowDecryptLater,
    AIEnhancedQuantum,                       // ML accelerating quantum attacks
    QuantumEnhancedAI,                       // Quantum accelerating ML attacks
    SupplyChainCrypto,
    NotQuantumRelated,
}

// Quantum Threat Classification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuantumThreatType {
    QCT1_CRQCCryptanalysis,                  // Shor's, Grover's
    QCT2_HarvestNowDecryptLater,
    QCT3_QuantumSideChannel,
    QCT4_PQCDowngrade,
    AAT1_ModelExtraction,
    AAT2_DataPoisoning,
    AAT3_ModelInversion,
    AAT4_AdversarialExamples,
    AAT5_BackdoorInjection,
    QACT1_QuantumEnhancedExtraction,
    QACT2_AIAcceleratedCryptanalysis,
    QACT3_CryptoAISupplyChain,
    QACT4_QuantumBackdoor,
    QACT5_PQCDowngradeML,
}

// Threat Detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatDetection {
    pub vulnerability: Vulnerability,         // From existing audit
    pub atlas_tactics: Vec<ATLASTactic>,
    pub atlas_techniques: Vec<ATLASTechnique>,
    pub quantum_threats: Vec<QuantumThreatType>,
    pub threat_score: ThreatScore,
    pub attack_paths: Vec<AttackPath>,
}

// Attack Path
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackPath {
    pub path_id: String,
    pub name: String,                         // "Harvest-Now-Decrypt-Later on AI Model"
    pub steps: Vec<AttackStep>,
    pub likelihood: AttackLikelihood,
    pub impact: AttackImpact,
    pub timeframe: AttackTimeframe,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackStep {
    pub step_number: usize,
    pub tactic: ATLASTactic,
    pub technique: Option<ATLASTechnique>,
    pub threat_type: QuantumThreatType,
    pub description: String,
    pub prerequisites: Vec<String>,
    pub indicators: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AttackLikelihood {
    VeryLow,                                  // <10% in next 10 years
    Low,                                      // 10-30%
    Medium,                                   // 30-60%
    High,                                     // 60-85%
    VeryHigh,                                 // >85%
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AttackTimeframe {
    Immediate,                                // <1 year
    NearTerm,                                 // 1-3 years
    MidTerm,                                  // 3-7 years
    LongTerm,                                 // 7-15 years
    Speculative,                              // >15 years
}

// ATLAS Threat Report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ATLASThreatReport {
    pub metadata: ReportMetadata,
    pub summary: ThreatSummary,
    pub threat_detections: Vec<ThreatDetection>,
    pub attack_paths: Vec<AttackPath>,
    pub mitigations: Vec<ATLASMitigation>,
    pub threat_matrix: ThreatMatrix,          // Visual matrix representation
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatSummary {
    pub total_threats: usize,
    pub critical_threats: usize,
    pub high_threats: usize,
    pub medium_threats: usize,
    pub low_threats: usize,
    pub tactics_identified: Vec<ATLASTactic>,
    pub techniques_matched: Vec<String>,      // Technique IDs
    pub quantum_threat_breakdown: HashMap<QuantumThreatType, usize>,
    pub overall_threat_score: f32,            // 0-100
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatMatrix {
    pub tactics: Vec<ATLASTactic>,
    pub technique_counts: HashMap<ATLASTactic, usize>,
    pub coverage_percentage: f32,             // % of ATLAS matrix covered
}
```

---

## 5. Implementation Plan

### 5.1 Phase Breakdown

#### Phase 1: Type System & Data Model (Week 1-2)

**Deliverables:**
- Create `src/atlas/types.rs` with all ATLAS types
- Define `QuantumThreatType`, `ATLASTactic`, `ATLASTechnique` enums
- Implement `ThreatDetection`, `AttackPath`, `ATLASThreatReport` structures
- Add JSON serialization/deserialization

**Validation:**
- All types compile without errors
- JSON serialization roundtrip works
- Type safety enforced

#### Phase 2: Threat Mapper (Week 3-4)

**Deliverables:**
- Implement `threat_mapper.rs` to map `Vulnerability` → `ATLASTactic`
- Create threat mapping database (JSON/YAML)
- Implement confidence scoring for technique matches
- Add quantum relevance classification

**Example Mapping Logic:**

```rust
// src/atlas/threat_mapper.rs

pub struct ThreatMapper {
    mapping_db: ThreatMappingDatabase,
}

impl ThreatMapper {
    pub fn map_vulnerability_to_threats(
        &self,
        vuln: &Vulnerability,
    ) -> ThreatDetection {
        let tactics = self.identify_tactics(vuln);
        let techniques = self.identify_techniques(vuln, &tactics);
        let quantum_threats = self.classify_quantum_threats(vuln);
        let threat_score = self.calculate_threat_score(vuln, &quantum_threats);

        ThreatDetection {
            vulnerability: vuln.clone(),
            atlas_tactics: tactics,
            atlas_techniques: techniques,
            quantum_threats,
            threat_score,
            attack_paths: Vec::new(), // Populated in next phase
        }
    }

    fn identify_tactics(&self, vuln: &Vulnerability) -> Vec<ATLASTactic> {
        match vuln.crypto_type {
            CryptoType::Rsa if vuln.key_size < 2048 => {
                vec![
                    ATLASTactic::TA0009CredentialAccess,
                    ATLASTactic::TA0013Exfiltration,
                    ATLASTactic::TA0014Impact,
                ]
            },
            CryptoType::Md5 | CryptoType::Sha1 => {
                vec![
                    ATLASTactic::TA0008DefenseEvasion,
                    ATLASTactic::TA0014Impact,
                ]
            },
            CryptoType::Ecdh | CryptoType::Ecdsa => {
                vec![
                    ATLASTactic::TA0012Collection,
                    ATLASTactic::TA0013Exfiltration,
                ]
            },
            _ => Vec::new(),
        }
    }

    fn classify_quantum_threats(&self, vuln: &Vulnerability) -> Vec<QuantumThreatType> {
        let mut threats = Vec::new();

        // Quantum-vulnerable crypto
        if matches!(vuln.crypto_type, CryptoType::Rsa | CryptoType::Ecdsa | CryptoType::Ecdh | CryptoType::DiffieHellman) {
            threats.push(QuantumThreatType::QCT1_CRQCCryptanalysis);
            threats.push(QuantumThreatType::QCT2_HarvestNowDecryptLater);
        }

        // Broken crypto (AI-assisted attacks)
        if matches!(vuln.crypto_type, CryptoType::Md5 | CryptoType::Sha1 | CryptoType::Des | CryptoType::Rc4) {
            threats.push(QuantumThreatType::QACT2_AIAcceleratedCryptanalysis);
        }

        // Convergence threats
        if vuln.severity >= Severity::High {
            threats.push(QuantumThreatType::QACT3_CryptoAISupplyChain);
        }

        threats
    }
}
```

**Validation:**
- Accurate mapping for all 10 existing crypto types
- Confidence scores >80% for critical vulnerabilities
- No false positives in technique matching

#### Phase 3: Attack Path Analyzer (Week 5-6)

**Deliverables:**
- Implement `attack_path_analyzer.rs`
- Define attack path templates for common scenarios
- Build attack chain constructor
- Add likelihood and timeframe estimation

**Example Attack Paths:**

```rust
// src/atlas/attack_path_analyzer.rs

pub struct AttackPathAnalyzer {
    path_templates: Vec<AttackPathTemplate>,
}

impl AttackPathAnalyzer {
    pub fn generate_attack_paths(
        &self,
        threat_detection: &ThreatDetection,
    ) -> Vec<AttackPath> {
        let mut paths = Vec::new();

        // Path 1: Harvest-Now-Decrypt-Later
        if threat_detection.quantum_threats.contains(&QuantumThreatType::QCT2_HarvestNowDecryptLater) {
            paths.push(self.build_hndl_path(threat_detection));
        }

        // Path 2: AI-Accelerated Cryptanalysis
        if threat_detection.quantum_threats.contains(&QuantumThreatType::QACT2_AIAcceleratedCryptanalysis) {
            paths.push(self.build_ai_cryptanalysis_path(threat_detection));
        }

        // Path 3: Supply Chain Backdoor
        if threat_detection.quantum_threats.contains(&QuantumThreatType::QACT3_CryptoAISupplyChain) {
            paths.push(self.build_supply_chain_path(threat_detection));
        }

        paths
    }

    fn build_hndl_path(&self, detection: &ThreatDetection) -> AttackPath {
        AttackPath {
            path_id: "HNDL-001".to_string(),
            name: "Harvest-Now-Decrypt-Later on Encrypted Data".to_string(),
            steps: vec![
                AttackStep {
                    step_number: 1,
                    tactic: ATLASTactic::TA0001Reconnaissance,
                    technique: Some(ATLASTechnique {
                        id: "AML.T0000".to_string(),
                        name: "ML Model Inference API Access".to_string(),
                        tactic: ATLASTactic::TA0001Reconnaissance,
                        description: "Identify API endpoints using quantum-vulnerable crypto".to_string(),
                        quantum_relevance: QuantumRelevance::HarvestNowDecryptLater,
                        detection_confidence: 0.85,
                    }),
                    threat_type: QuantumThreatType::QCT2_HarvestNowDecryptLater,
                    description: "Adversary identifies system using ECDH P-256 for key exchange".to_string(),
                    prerequisites: vec!["Public API endpoint".to_string()],
                    indicators: vec![
                        "API documentation mentions ECDH".to_string(),
                        "TLS handshake reveals P-256".to_string(),
                    ],
                },
                AttackStep {
                    step_number: 2,
                    tactic: ATLASTactic::TA0003InitialAccess,
                    technique: None,
                    threat_type: QuantumThreatType::QCT2_HarvestNowDecryptLater,
                    description: "Access API endpoint via normal authentication".to_string(),
                    prerequisites: vec!["Valid credentials or public access".to_string()],
                    indicators: vec!["API access logs".to_string()],
                },
                AttackStep {
                    step_number: 3,
                    tactic: ATLASTactic::TA0012Collection,
                    technique: Some(ATLASTechnique {
                        id: "AML.T0010".to_string(),
                        name: "ML Artifact Collection".to_string(),
                        tactic: ATLASTactic::TA0012Collection,
                        description: "Collect encrypted artifacts for future decryption".to_string(),
                        quantum_relevance: QuantumRelevance::HarvestNowDecryptLater,
                        detection_confidence: 0.90,
                    }),
                    threat_type: QuantumThreatType::QCT2_HarvestNowDecryptLater,
                    description: "Collect encrypted model artifacts, traffic dumps".to_string(),
                    prerequisites: vec!["Access to encrypted data streams".to_string()],
                    indicators: vec![
                        "Large data downloads".to_string(),
                        "Unusual API query patterns".to_string(),
                    ],
                },
                AttackStep {
                    step_number: 4,
                    tactic: ATLASTactic::TA0013Exfiltration,
                    technique: Some(ATLASTechnique {
                        id: "AML.T0024".to_string(),
                        name: "Exfiltrate ML Artifacts".to_string(),
                        tactic: ATLASTactic::TA0013Exfiltration,
                        description: "Exfiltrate encrypted data for future quantum decryption".to_string(),
                        quantum_relevance: QuantumRelevance::HarvestNowDecryptLater,
                        detection_confidence: 0.75,
                    }),
                    threat_type: QuantumThreatType::QCT2_HarvestNowDecryptLater,
                    description: "Store collected data for decryption when CRQC available".to_string(),
                    prerequisites: vec!["Successful collection".to_string()],
                    indicators: vec!["Data egress to external storage".to_string()],
                },
                AttackStep {
                    step_number: 5,
                    tactic: ATLASTactic::TA0014Impact,
                    technique: None,
                    threat_type: QuantumThreatType::QCT1_CRQCCryptanalysis,
                    description: "Future: Decrypt with Shor's algorithm on CRQC (2030-2035)".to_string(),
                    prerequisites: vec!["CRQC availability".to_string()],
                    indicators: vec!["N/A (future threat)".to_string()],
                },
            ],
            likelihood: AttackLikelihood::High, // 70% likelihood in next 10 years
            impact: AttackImpact::Critical,
            timeframe: AttackTimeframe::MidTerm, // 3-7 years
        }
    }
}
```

**Validation:**
- Attack paths are logically coherent
- Prerequisites correctly identified
- Timeframes align with NIST/MOSCA estimates

#### Phase 4: Threat Scoring Engine (Week 7)

**Deliverables:**
- Implement `threat_scoring.rs`
- Define threat scoring algorithm combining:
  - Vulnerability severity
  - ATLAS tactic criticality
  - Quantum threat urgency
  - Attack path likelihood
- Add configurable weights

**Scoring Algorithm:**

```rust
// src/atlas/threat_scoring.rs

pub struct ThreatScoringEngine {
    config: ScoringConfig,
}

#[derive(Debug, Clone)]
pub struct ScoringConfig {
    pub vulnerability_weight: f32,    // 0.4
    pub tactic_weight: f32,           // 0.2
    pub quantum_weight: f32,          // 0.3
    pub likelihood_weight: f32,       // 0.1
}

impl ThreatScoringEngine {
    pub fn calculate_threat_score(
        &self,
        vuln: &Vulnerability,
        tactics: &[ATLASTactic],
        quantum_threats: &[QuantumThreatType],
        attack_paths: &[AttackPath],
    ) -> ThreatScore {
        let vuln_score = self.score_vulnerability(vuln);
        let tactic_score = self.score_tactics(tactics);
        let quantum_score = self.score_quantum_threats(quantum_threats);
        let likelihood_score = self.score_attack_likelihood(attack_paths);

        let weighted_score =
            (vuln_score * self.config.vulnerability_weight) +
            (tactic_score * self.config.tactic_weight) +
            (quantum_score * self.config.quantum_weight) +
            (likelihood_score * self.config.likelihood_weight);

        ThreatScore {
            overall: weighted_score,
            vulnerability: vuln_score,
            tactics: tactic_score,
            quantum: quantum_score,
            likelihood: likelihood_score,
            severity: self.classify_severity(weighted_score),
        }
    }

    fn score_vulnerability(&self, vuln: &Vulnerability) -> f32 {
        match vuln.severity {
            Severity::Critical => 100.0,
            Severity::High => 80.0,
            Severity::Medium => 50.0,
            Severity::Low => 20.0,
        }
    }

    fn score_tactics(&self, tactics: &[ATLASTactic]) -> f32 {
        if tactics.is_empty() {
            return 0.0;
        }

        // Weight critical tactics higher
        let tactic_weights: HashMap<ATLASTactic, f32> = [
            (ATLASTactic::TA0014Impact, 100.0),
            (ATLASTactic::TA0013Exfiltration, 90.0),
            (ATLASTactic::TA0009CredentialAccess, 85.0),
            (ATLASTactic::TA0012Collection, 75.0),
            (ATLASTactic::TA0008DefenseEvasion, 70.0),
            // ... other tactics
        ].iter().cloned().collect();

        let sum: f32 = tactics.iter()
            .map(|t| tactic_weights.get(t).unwrap_or(&50.0))
            .sum();

        (sum / tactics.len() as f32).min(100.0)
    }

    fn score_quantum_threats(&self, threats: &[QuantumThreatType]) -> f32 {
        if threats.is_empty() {
            return 0.0;
        }

        let threat_weights: HashMap<QuantumThreatType, f32> = [
            (QuantumThreatType::QCT1_CRQCCryptanalysis, 100.0),
            (QuantumThreatType::QCT2_HarvestNowDecryptLater, 95.0),
            (QuantumThreatType::QACT2_AIAcceleratedCryptanalysis, 90.0),
            (QuantumThreatType::QACT1_QuantumEnhancedExtraction, 85.0),
            // ... other threats
        ].iter().cloned().collect();

        let sum: f32 = threats.iter()
            .map(|t| threat_weights.get(t).unwrap_or(&50.0))
            .sum();

        (sum / threats.len() as f32).min(100.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatScore {
    pub overall: f32,                 // 0-100
    pub vulnerability: f32,
    pub tactics: f32,
    pub quantum: f32,
    pub likelihood: f32,
    pub severity: ThreatSeverity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThreatSeverity {
    Critical,   // 85-100
    High,       // 70-84
    Medium,     // 50-69
    Low,        // 25-49
    Info,       // 0-24
}
```

**Validation:**
- Scores align with expert threat assessments
- Critical vulnerabilities score >85
- Scores are reproducible and consistent

#### Phase 5: Report Generator (Week 8)

**Deliverables:**
- Implement `report_generator.rs`
- Generate ATLAS-formatted JSON reports
- Create visual threat matrix
- Add executive summary generation

**Report Structure:**

```json
{
  "atlas_threat_report": {
    "version": "1.0.0",
    "generated_at": "2025-11-18T15:30:00Z",
    "scanner_version": "2025.11.18",
    "summary": {
      "total_threats": 42,
      "critical_threats": 8,
      "high_threats": 15,
      "medium_threats": 12,
      "low_threats": 7,
      "tactics_identified": [
        "AML.TA0001",
        "AML.TA0009",
        "AML.TA0012",
        "AML.TA0013",
        "AML.TA0014"
      ],
      "techniques_matched": [
        "AML.T0010",
        "AML.T0024",
        "AML.T0040"
      ],
      "quantum_threat_breakdown": {
        "QCT1_CRQCCryptanalysis": 12,
        "QCT2_HarvestNowDecryptLater": 18,
        "QACT2_AIAcceleratedCryptanalysis": 8,
        "QACT3_CryptoAISupplyChain": 4
      },
      "overall_threat_score": 78.5
    },
    "threat_detections": [
      {
        "vulnerability": {
          "crypto_type": "RSA",
          "key_size": 2048,
          "severity": "High",
          "line": 42,
          "file": "src/auth.rs"
        },
        "atlas_tactics": ["AML.TA0012", "AML.TA0013"],
        "atlas_techniques": [
          {
            "id": "AML.T0010",
            "name": "ML Artifact Collection",
            "quantum_relevance": "HarvestNowDecryptLater",
            "detection_confidence": 0.85
          }
        ],
        "quantum_threats": [
          "QCT1_CRQCCryptanalysis",
          "QCT2_HarvestNowDecryptLater"
        ],
        "threat_score": {
          "overall": 82.3,
          "vulnerability": 80.0,
          "tactics": 82.5,
          "quantum": 95.0,
          "likelihood": 70.0,
          "severity": "High"
        }
      }
    ],
    "attack_paths": [
      {
        "path_id": "HNDL-001",
        "name": "Harvest-Now-Decrypt-Later on Encrypted Data",
        "likelihood": "High",
        "impact": "Critical",
        "timeframe": "MidTerm",
        "steps": [
          {
            "step_number": 1,
            "tactic": "AML.TA0001",
            "technique": "AML.T0000",
            "description": "Identify API using quantum-vulnerable crypto"
          }
        ]
      }
    ],
    "mitigations": [
      {
        "mitigation_id": "M-QT-001",
        "name": "Migrate to Post-Quantum Cryptography",
        "description": "Replace RSA-2048 with ML-KEM-768 for key exchange",
        "atlas_techniques_mitigated": ["AML.T0010", "AML.T0024"],
        "quantum_threats_mitigated": ["QCT1", "QCT2"],
        "priority": "High",
        "estimated_effort": "Medium"
      }
    ]
  }
}
```

**Validation:**
- JSON schema validates
- Reports are human-readable
- All data correctly populated

#### Phase 6: Integration & Testing (Week 9-10)

**Deliverables:**
- Integrate ATLAS module with existing audit flow
- Add CLI command: `pqc-scanner atlas-threat-scan`
- Write comprehensive tests (50+ unit, 20+ integration)
- Add GitHub Actions workflow
- Create documentation

**CLI Usage:**

```bash
# Run ATLAS threat analysis
pqc-scanner atlas-threat-scan \
  --source-dir ./src \
  --output atlas-threat-report.json \
  --include-attack-paths \
  --threat-level high

# Combined compliance + threat analysis
pqc-scanner unified-scan \
  --source-dir ./src \
  --standards nist,itsg33,p1943,atlas \
  --output unified-report.json
```

**Validation:**
- All integration tests pass
- CLI commands functional
- Performance overhead <15%

---

## 6. Threat Scoring Engine

### 6.1 Scoring Methodology

**Composite Threat Score Formula:**

```
ThreatScore = (V × 0.4) + (T × 0.2) + (Q × 0.3) + (L × 0.1)

Where:
V = Vulnerability Severity Score (0-100)
T = ATLAS Tactic Criticality Score (0-100)
Q = Quantum Threat Urgency Score (0-100)
L = Attack Likelihood Score (0-100)
```

### 6.2 Component Scoring Tables

**Vulnerability Severity (V):**
| Crypto Type | Key Size | Score | Rationale |
|-------------|----------|-------|-----------|
| MD5, SHA-1, DES, RC4 | N/A | 100 | Broken, immediate risk |
| RSA | <2048 | 95 | Quantum-breakable now |
| RSA | 2048 | 80 | HNDL risk, migrate by 2030 |
| RSA | 3072+ | 65 | HNDL risk, migrate by 2035 |
| ECDSA, ECDH | P-256 | 75 | HNDL risk, migrate by 2030 |
| ECDSA, ECDH | P-384+ | 60 | HNDL risk, migrate by 2035 |
| 3DES | N/A | 70 | Deprecated, weak |

**ATLAS Tactic Criticality (T):**
| Tactic | Score | Rationale |
|--------|-------|-----------|
| Impact (TA0014) | 100 | Direct system compromise |
| Exfiltration (TA0013) | 90 | Data theft |
| Credential Access (TA0009) | 85 | Key compromise |
| Collection (TA0012) | 75 | Data harvesting |
| Defense Evasion (TA0008) | 70 | Detection bypass |
| Reconnaissance (TA0001) | 40 | Early stage |

**Quantum Threat Urgency (Q):**
| Threat Type | Score | Timeframe |
|-------------|-------|-----------|
| QCT1 (CRQC Cryptanalysis) | 100 | 2030-2035 |
| QCT2 (HNDL) | 95 | Active now |
| QACT2 (AI-Accelerated) | 90 | Active now |
| QACT1 (Quantum-Enhanced ML) | 85 | 2028-2033 |
| QACT3 (Supply Chain) | 80 | Active now |

**Attack Likelihood (L):**
| Likelihood | Score | Probability |
|------------|-------|-------------|
| Very High | 95 | >85% in 10 years |
| High | 80 | 60-85% |
| Medium | 60 | 30-60% |
| Low | 35 | 10-30% |
| Very Low | 15 | <10% |

### 6.3 Threat Severity Classification

| Overall Score | Severity | Action Required |
|---------------|----------|-----------------|
| 85-100 | **Critical** | Immediate mitigation (0-6 months) |
| 70-84 | **High** | Urgent mitigation (6-12 months) |
| 50-69 | **Medium** | Planned mitigation (12-24 months) |
| 25-49 | **Low** | Monitor and plan (24-36 months) |
| 0-24 | **Info** | Track for awareness |

---

## 7. Attack Path Modeling

### 7.1 Pre-Defined Attack Path Templates

**Template 1: Harvest-Now-Decrypt-Later (HNDL)**
```yaml
path_id: HNDL-001
name: "Harvest-Now-Decrypt-Later on AI Model"
triggers:
  - crypto_type: [ECDH, ECDSA, RSA]
  - key_size: [<4096]
likelihood: High
timeframe: MidTerm
steps:
  - step: 1
    tactic: TA0001
    description: "Identify system using quantum-vulnerable crypto"
  - step: 2
    tactic: TA0003
    description: "Access system via normal authentication"
  - step: 3
    tactic: TA0012
    description: "Collect encrypted data for future decryption"
  - step: 4
    tactic: TA0013
    description: "Exfiltrate encrypted artifacts"
  - step: 5
    tactic: TA0014
    description: "Future: Decrypt with CRQC"
```

**Template 2: AI-Accelerated Cryptanalysis**
```yaml
path_id: AAC-001
name: "AI-Accelerated Quantum Cryptanalysis"
triggers:
  - crypto_type: [RSA, DH]
  - severity: [High, Critical]
likelihood: Medium
timeframe: NearTerm
steps:
  - step: 1
    tactic: TA0002
    description: "Train ML model on crypto weaknesses"
  - step: 2
    tactic: TA0010
    description: "Discover RSA implementation details"
  - step: 3
    tactic: TA0005
    description: "Execute ML-optimized quantum circuit"
  - step: 4
    tactic: TA0009
    description: "Extract private keys"
```

### 7.2 Dynamic Path Construction

The attack path analyzer dynamically constructs paths based on:
- Detected vulnerabilities
- System architecture hints (if available)
- Threat intelligence feeds (future enhancement)
- Historical attack patterns

---

## 8. Integration Points

### 8.1 Integration with Existing Modules

**Non-Destructive Integration:**
- ATLAS module is **completely optional**
- Existing audit flow unchanged
- Reports are generated independently
- Can be enabled/disabled via CLI flag

**Integration Touch Points:**

```rust
// src/lib.rs (WASM exports)

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn generate_atlas_threat_report(
    source: &str,
    language: &str,
    file_path: Option<String>,
) -> Result<JsValue, JsValue> {
    // 1. Run existing audit (unchanged)
    let audit_result = audit::analyze(source, language)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    // 2. NEW: Generate ATLAS threat analysis
    let atlas_report = atlas_integration::generate_atlas_report(&audit_result, file_path.as_deref());

    serde_wasm_bindgen::to_value(&atlas_report)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

// Unified report including all standards
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn generate_unified_threat_report(
    source: &str,
    language: &str,
    config: UnifiedConfig,
) -> Result<JsValue, JsValue> {
    let audit_result = audit::analyze(source, language)?;

    let report = UnifiedThreatReport {
        nist_sc13: generate_sc13_report(&audit_result, config.file_path),
        itsg33: generate_itsg33_report(&audit_result, config.classification, config.file_path),
        p1943: generate_p1943_report(&audit_result, &config.p1943_config), // If available
        atlas_threats: generate_atlas_report(&audit_result, config.file_path), // NEW
    };

    serde_wasm_bindgen::to_value(&report)
}
```

### 8.2 Data Flow Integration

```
┌──────────────────┐
│  Source Code     │
└────────┬─────────┘
         │
         ▼
┌────────────────────┐
│  Audit Engine      │ (No changes)
│  audit::analyze()  │
└────────┬───────────┘
         │
         │ AuditResult
         │
         ├─────────────┬──────────────┬────────────────┐
         │             │              │                │
         ▼             ▼              ▼                ▼
    ┌────────┐   ┌─────────┐   ┌──────────┐    ┌──────────┐
    │ NIST   │   │ ITSG33  │   │  P1943   │    │  ATLAS   │
    │ SC-13  │   │ Report  │   │  Report  │    │ Threats  │ (NEW)
    └────────┘   └─────────┘   └──────────┘    └──────────┘
```

### 8.3 Configuration Integration

Add ATLAS configuration to existing config files:

```yaml
# .pqc-scanner.yml
analysis:
  enabled_modules:
    - nist_sc13
    - itsg33
    - p1943
    - atlas_threats   # NEW: Optional ATLAS analysis

atlas_config:
  enabled: true
  include_attack_paths: true
  threat_scoring:
    vulnerability_weight: 0.4
    tactic_weight: 0.2
    quantum_weight: 0.3
    likelihood_weight: 0.1
  reporting:
    format: json
    include_mitigations: true
    min_threat_level: medium
```

---

## 9. Use Cases

### 9.1 Use Case 1: AI/ML Platform Security Assessment

**Scenario:**
A company runs an AI/ML platform with cryptographic components (API auth, model encryption, data protection). They want to assess adversarial threats.

**Workflow:**
1. Run pqc-scanner ATLAS analysis on codebase
2. Identify quantum-vulnerable crypto protecting AI models
3. Review ATLAS threat report showing:
   - Tactic: TA0013 (Exfiltration)
   - Technique: AML.T0024 (Exfiltrate ML Artifacts)
   - Threat: QCT2 (Harvest-Now-Decrypt-Later)
   - Attack path: 5-step HNDL scenario
4. Prioritize PQC migration for high-value AI models
5. Implement mitigations (ML-KEM for API, ML-DSA for signatures)

**Value:**
- Understand AI-specific threat landscape
- Prioritize based on adversarial tactics
- Justify PQC migration with threat intelligence

### 9.2 Use Case 2: Supply Chain Risk Assessment

**Scenario:**
Organization uses third-party ML libraries with embedded crypto. Need to assess supply chain risks.

**Workflow:**
1. Scan dependencies with ATLAS analysis enabled
2. Identify crypto libraries used in ML packages
3. Detect potential backdoor vectors (ATLAS Technique: AML.T0017)
4. Review attack path: "Supply Chain Crypto Backdoor"
5. Validate CMVP certifications
6. Assess quantum supply chain threats (QACT3)

**Value:**
- Identify hidden crypto dependencies
- Assess adversarial supply chain risks
- Validate third-party security

### 9.3 Use Case 3: Quantum Threat Modeling for Security Teams

**Scenario:**
Security team needs to model quantum threats for executive briefing.

**Workflow:**
1. Run unified scan (NIST + ITSG33 + P1943 + ATLAS)
2. Generate executive summary with:
   - ATLAS tactics identified
   - Attack paths with timelines
   - Threat scores by system component
   - Mitigation roadmap
3. Present to leadership with ATLAS matrix visualization
4. Align mitigation priorities with business risk

**Value:**
- Executive-friendly threat intelligence
- Risk-based prioritization
- Business case for PQC investment

### 9.4 Use Case 4: Incident Response Preparedness

**Scenario:**
Prepare incident response playbooks for quantum-era attacks.

**Workflow:**
1. Run ATLAS analysis to identify attack paths
2. Map each path to detection indicators
3. Create IR playbooks per ATLAS tactic:
   - TA0012 (Collection) → Monitor data exfiltration
   - TA0013 (Exfiltration) → Alert on large encrypted transfers
   - TA0014 (Impact) → Detect crypto downgrade attempts
4. Integrate with SIEM for automated detection

**Value:**
- Proactive threat detection
- Tactical IR playbooks
- SIEM integration for early warning

---

## 10. Deliverables

### 10.1 Code Deliverables (~2,500 LOC)

1. **`src/atlas/` module** (8 files, ~1,800 LOC)
   - `mod.rs`, `types.rs`, `threat_mapper.rs`, `quantum_atlas_model.rs`
   - `attack_path_analyzer.rs`, `threat_scoring.rs`, `technique_matcher.rs`, `report_generator.rs`

2. **Integration module** (~300 LOC)
   - `src/atlas_integration.rs`: Orchestrator connecting ATLAS to audit engine

3. **WASM exports** (~200 LOC)
   - `src/lib.rs`: New WASM functions for ATLAS analysis

4. **CLI enhancements** (~200 LOC)
   - `src/bin/pqc-scanner.rs`: New `atlas-threat-scan` command

### 10.2 Data Assets

1. **Threat Mapping Database** (`data/atlas_threat_mappings.json`)
   - Crypto type → ATLAS tactic mappings
   - Vulnerability → Technique mappings
   - Quantum threat classifications

2. **Attack Path Templates** (`data/atlas_attack_paths.yaml`)
   - Pre-defined attack scenarios
   - Step-by-step tactics and techniques
   - Likelihood and timeframe data

3. **ATLAS Taxonomy** (`data/atlas_taxonomy.json`)
   - All 14 tactics with descriptions
   - 56 techniques with metadata
   - Quantum relevance classifications

### 10.3 Test Deliverables (~1,200 LOC)

1. **Unit tests** (`tests/atlas_tests.rs`, ~600 LOC)
   - 40+ tests covering all ATLAS components
   - Threat mapping accuracy tests
   - Scoring algorithm validation
   - Attack path generation tests

2. **Integration tests** (`tests/integration_atlas.rs`, ~400 LOC)
   - End-to-end ATLAS scanning
   - Unified report generation
   - CLI command testing

3. **Test fixtures** (~200 LOC)
   - Sample code with quantum-vulnerable crypto
   - Expected ATLAS threat reports
   - Attack path validation data

### 10.4 CI/CD Deliverables

**`.github/workflows/atlas-threat-scan.yml`**
```yaml
name: ATLAS Threat Analysis

on:
  push:
    branches: [ main, develop ]
  pull_request:
  schedule:
    - cron: '0 0 * * 1'  # Weekly on Monday

jobs:
  atlas-scan:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v5
      - name: Build pqc-scanner
        run: cargo build --release
      - name: Run ATLAS Threat Scan
        run: |
          ./target/release/pqc-scanner atlas-threat-scan \
            --source-dir . \
            --output reports/atlas-threat-report.json \
            --threat-level high
      - name: Upload ATLAS Report
        uses: actions/upload-artifact@v4
        with:
          name: atlas-threat-report
          path: reports/atlas-threat-report.json
      - name: Threat Analysis Summary
        run: |
          echo "## ATLAS Threat Analysis" >> $GITHUB_STEP_SUMMARY
          echo "Critical Threats: $(jq '.summary.critical_threats' reports/atlas-threat-report.json)" >> $GITHUB_STEP_SUMMARY
          echo "High Threats: $(jq '.summary.high_threats' reports/atlas-threat-report.json)" >> $GITHUB_STEP_SUMMARY
```

### 10.5 Documentation Deliverables

```
docs/atlas/
├── README.md                        # ATLAS integration overview
├── threat-modeling-guide.md         # How to use ATLAS for threat modeling
├── attack-paths-reference.md        # Complete attack path catalog
├── scoring-methodology.md           # Threat scoring explained
├── mitre-atlas-mapping.md          # Crypto → ATLAS mappings
└── examples/
    ├── atlas-threat-report-sample.json
    ├── attack-path-visualization.md
    └── use-case-scenarios.md
```

---

## 11. Success Metrics

### 11.1 Functional Metrics

- ✅ All 14 ATLAS tactics represented
- ✅ 20+ techniques accurately mapped to crypto vulns
- ✅ Attack path generation for all critical vulnerabilities
- ✅ Threat scoring accuracy >85% vs expert assessment
- ✅ Test coverage >90% for ATLAS module

### 11.2 Performance Metrics

- ✅ ATLAS analysis overhead <15% of base scan time
- ✅ Report generation <500ms for 1000 LOC
- ✅ Memory overhead <10MB
- ✅ No impact on existing compliance reports

### 11.3 Usability Metrics

- ✅ ATLAS reports are human-readable
- ✅ Attack paths clearly explained with indicators
- ✅ Executive summaries generated automatically
- ✅ Integration with existing workflows seamless

### 11.4 Adoption Metrics

- GitHub Action usage
- ATLAS report downloads
- Community feedback on threat accuracy
- Integration with SIEM/threat intelligence platforms

---

## 12. Timeline & Milestones

### Overall Timeline: 8-10 weeks

| Week | Phase | Deliverables | Validation |
|------|-------|--------------|------------|
| **1-2** | Type System & Data Model | ATLAS types, enums, structures | Types compile, JSON serialization works |
| **3-4** | Threat Mapper | Vulnerability → ATLAS mapping, confidence scoring | Mapping accuracy >80%, no false positives |
| **5-6** | Attack Path Analyzer | Attack path templates, dynamic construction | Paths logically coherent, timeframes accurate |
| **7** | Threat Scoring Engine | Scoring algorithm, severity classification | Scores align with expert assessment |
| **8** | Report Generator | JSON reports, matrix visualization | Reports validate, data complete |
| **9-10** | Integration & Testing | CLI, tests, docs, CI/CD | All tests pass, workflows functional |

### Key Milestones

- ✅ **Week 2**: ATLAS type system complete
- ✅ **Week 4**: Threat mapping functional
- ✅ **Week 6**: Attack paths generated for all critical vulns
- ✅ **Week 7**: Threat scoring accurate
- ✅ **Week 8**: ATLAS reports ready
- ✅ **Week 10**: Production-ready, fully tested

---

## 13. Risk Assessment

### 13.1 Technical Risks

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| ATLAS framework updates | Medium | Medium | Design for extensibility, version mapping DB |
| Attack path complexity | Medium | Low | Start with templates, iterate based on feedback |
| Threat scoring accuracy | Medium | High | Validate with security experts, allow custom weights |
| Performance overhead | Low | Medium | Optimize algorithms, lazy evaluation |

### 13.2 Adoption Risks

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Users don't understand ATLAS | Medium | Medium | Comprehensive docs, examples, tooltips |
| Too much information overload | Medium | Medium | Executive summaries, filterable reports |
| Integration friction | Low | Low | 100% optional, default disabled |

---

## 14. Future Enhancements

### Phase 2 (Post-Initial Release)

1. **Threat Intelligence Integration**
   - Ingest real-world quantum threat feeds
   - Update attack likelihoods based on intelligence
   - STIX/TAXII integration

2. **Interactive Visualizations**
   - Web-based ATLAS matrix heatmap
   - Attack path graph visualization (D3.js)
   - Timeline projections for quantum threats

3. **Automated Mitigations**
   - Link ATLAS threats to auto-remediation
   - Generate PQC migration plans per attack path
   - CI/CD integration for automated fixes

4. **ML-Powered Threat Prediction**
   - Train ML model on historical attack patterns
   - Predict emerging ATLAS techniques
   - Anomaly detection for novel attack paths

5. **Extended Quantum Threat Coverage**
   - Side-channel attack modeling
   - Quantum algorithm simulation
   - Hardware security module (HSM) analysis

---

## 15. Conclusion

The integration of **MITRE ATLAS framework** into pqc-scanner provides a powerful, value-added capability for **quantum-era threat analysis**. This feature:

✅ **Non-Destructive**: Fully optional, no impact on existing functionality
✅ **Value-Added**: Unique threat intelligence combining quantum + AI/ML domains
✅ **Actionable**: Clear attack paths, threat scores, and mitigations
✅ **Standards-Aligned**: Leverages established MITRE ATLAS framework
✅ **Future-Proof**: Extensible design for evolving threat landscape

By mapping cryptographic vulnerabilities to adversarial AI/ML tactics, organizations gain unprecedented visibility into the intersection of quantum computing and machine learning threats—enabling proactive defense in the quantum era.

---

**Document Version**: 1.0
**Last Updated**: 2025-11-18
**Author**: ArcQubit Engineering Team
**Status**: Planning Phase
**Next Steps**: Review and approve for implementation
