# Product Requirements Document (PRD)

## Product: Quantum Feasibility Analyzer (QFA)

## Version: 2.0 (Enhanced Edition)

**Last Updated**: 2025-11-18
**Authors**: Trevor Bowman, GPT-5 (OpenAI), Claude (Anthropic)
**Review Status**: Incorporates recommendations from technical review

---

## Document Revision History

| Version | Date | Changes | Author |
|---------|------|---------|--------|
| 1.0 | 2025-11-18 | Initial PRD | Trevor Bowman, GPT-5 |
| 2.0 | 2025-11-18 | Added resource estimation, refined maturity levels, scoring formulas, false positive handling, caching, timeline estimates | Claude (Anthropic) |

---

## 1. Overview

### 1.1 Purpose

The Quantum Feasibility Analyzer (QFA) is a multi-language static analysis and scoring tool that evaluates existing software repositories to determine:

1. Whether portions of the codebase map to problems where quantum or hybrid quantum–classical algorithms may offer benefit.
2. Which **quantum algorithm families** are the most likely candidates (e.g., variational optimizers, amplitude estimation, quantum simulation).
3. The **transition path** from the current classical implementation to an experimental or production quantum-enabled architecture.
4. **Resource requirements** (qubits, circuit depth, error correction needs) for identified quantum opportunities.
5. **Realistic timelines** based on current quantum hardware capabilities (NISQ vs. fault-tolerant era).

The tool must be:

* **Systematic** – follows a deterministic pipeline from code to classification.
* **Repeatable** – same repo and configuration ⇒ same output.
* **Inspectable** – all decisions traceable to specific rules and code locations.
* **Transparent** – rules and scoring logic are human-readable and versioned as code.
* **Automated** – runnable via CLI, CI/CD (e.g., GitHub Actions), and service API.
* **Evidence-Based** – all recommendations grounded in peer-reviewed quantum computing research with explicit speedup quantification.

### 1.2 Context

QFA complements existing ArcQubit capabilities such as:

* **PQC-Scanner**: evaluates cryptographic posture and post-quantum migration readiness (defensive).
* **Q-CMM** (Quantum Capability Maturity Model): evaluates organizational quantum readiness.
* **MITRE ATLAS Threat Analysis**: maps quantum threats to adversarial AI/ML tactics.
* **IEEE P1943 Compliance**: post-quantum cryptographic inventory and compliance.

QFA focuses specifically on **code-level workload analysis** across multiple implementation languages to identify **quantum opportunities** (offensive quantum strategy).

### 1.3 Goals

1. Enable multi-language repositories to be evaluated for quantum applicability with a single tool.
2. Provide clear, rule-based mapping from classical workloads to quantum algorithm families with **quantified speedup estimates**.
3. Generate actionable transition recommendations (PoC candidate, monitor, not applicable) with **effort and cost estimates**.
4. Expose results in both human-consumable reports (Markdown/HTML with visualizations) and machine-consumable formats (JSON) for dashboards and Q-CMM integration.
5. Provide **explicit resource requirements** (qubits, circuit depth, error correction) for each quantum opportunity.
6. Set **realistic expectations** with transparent limitations, disclaimers, and confidence intervals.

### 1.4 Non-Goals (v1)

* Automatic conversion/transpilation of classical algorithms into quantum circuits or SDK code.
* Exact runtime/cost simulation on specific quantum hardware.
* Dynamic profiling or runtime instrumentation of applications.
* Full enterprise/org-level quantum maturity assessment (delegated to Q-CMM, though QFA may provide inputs).
* Quantum circuit optimization or quantum error correction strategy development.

### 1.5 Success Criteria

**Functional**:
- ✅ Detect all reference quantum patterns (precision ≥85%, recall ≥80%)
- ✅ Zero false positives on classical-only algorithms
- ✅ QAS correlation ≥0.90 with expert quantum computing ratings
- ✅ Speedup estimates within 2× of theoretical bounds
- ✅ Resource estimates accurate to within 20% of published literature

**Performance**:
- ✅ Scan <5 min for 100K LOC (with parallelization)
- ✅ Memory overhead <500MB for large repositories
- ✅ Incremental analysis via caching reduces re-scan time by ≥70%

**Usability**:
- ✅ Reports actionable and understandable by non-quantum experts
- ✅ Executive summaries auto-generated
- ✅ False positive rate <5%
- ✅ User-controllable confidence thresholds

---

## 2. Target Users & Personas

### 2.1 Personas

1. **Quantum Strategy Architect**
   * Needs to identify where quantum can provide value within existing systems.
   * Consumes QFA reports to prioritize PoCs and roadmap items.
   * **Key question**: "Which of our 50+ microservices should we invest quantum R&D in?"

2. **Application/Platform Engineer**
   * Owns or contributes to a codebase.
   * Uses QFA in CI or locally to understand whether their modules are potential quantum candidates.
   * **Key question**: "Is my optimization algorithm a good candidate for QAOA?"

3. **Technical Consultant / Pre-Sales Engineer**
   * Evaluates customer repositories during assessments, RFPs, or discovery engagements.
   * Uses QFA to provide evidence-backed recommendations.
   * **Key question**: "Can we demonstrate quantum value in the customer's existing stack?"

4. **Security & Compliance Lead**
   * Focuses on crypto and PQC migration, but uses QFA outputs to understand where quantum may impact risk or architecture.
   * **Key question**: "Which workloads use quantum-vulnerable crypto AND could benefit from quantum acceleration?"

5. **Executive / CTO**
   * Makes investment decisions on quantum computing initiatives.
   * Needs business-friendly reports with ROI estimates and timelines.
   * **Key question**: "Should we invest $500K in quantum cloud credits? What's the expected payoff?"

---

## 3. Key Use Cases

### UC1 – Single-Repo Quantum Feasibility Scan

* **Input**: Git repository URL or local path.
* **Action**: Run QFA analysis over supported languages in the repo.
* **Output**: JSON report + Markdown summary + HTML dashboard; list of workloads, candidate quantum families, resource requirements, and transition suggestions.
* **Success Metric**: Analysis completes in <5 minutes for typical repos (2000 files, 500K LOC).

### UC2 – Multi-Repo Portfolio Prioritization

* **Input**: List of repos (e.g., mono-repo submodules, microservices, or multiple projects).
* **Action**: Analyze each repo and compare quantum applicability scores.
* **Output**: Aggregated view ranking repos by PoC potential with portfolio-level heatmap.
* **Success Metric**: Enables CTO to identify top 5 quantum candidates across 50+ repos in <30 minutes.

### UC3 – CI/CD Integration for Continuous Discovery

* **Input**: Pull request or scheduled pipeline run.
* **Action**: QFA runs automatically on changed code or on a schedule; uses caching for incremental analysis.
* **Output**: Report artifacts + PR comments summarizing new or changed quantum-relevant workloads.
* **Success Metric**: Incremental analysis completes in <2 minutes for typical PR (50 changed files).

### UC4 – Pre-Sales / Assessment Engagements

* **Input**: Customer provides limited repo access or exported bundle.
* **Action**: QFA runs in a controlled environment, generates reports used in an assessment deck.
* **Output**: Exportable reports for inclusion in strategy documents and Q-CMM feeds.
* **Success Metric**: Consultant can generate executive-ready quantum strategy report in <1 day.

### UC5 – False Positive Refinement Workflow

* **Input**: QFA report with flagged workloads.
* **Action**: User reviews, marks false positives via `.qfa-ignore` or in-code annotations.
* **Output**: Subsequent scans suppress false positives; refined rules fed back to QFA rule database.
* **Success Metric**: False positive rate decreases by ≥50% after first refinement iteration.

---

## 4. Supported Languages & Sources

### 4.1 Languages (v1 Scope)

QFA must support static analysis and workload detection across the following languages:

* **Python** (MVP - Phase 1)
* **JavaScript** (Phase 2)
* **TypeScript** (Phase 2)
* **Java** (Phase 2)
* **Rust** (Phase 3)
* **Go** (Phase 3)
* **C++** (Phase 3)
* **C#** (Phase 3)

Support per language will be implemented via a **pluggable analyzer architecture**, with a minimum baseline per analyzer:

* Language detection.
* Parsing into an AST or equivalent intermediate representation (IR).
* Extraction of imports/dependencies, API calls, core control structures, and numeric/optimization patterns.
* **Detection confidence scoring** (0.0-1.0) to account for dynamic imports, reflection, and obfuscation.

### 4.2 Repository Sources

* Local filesystem path (monorepo or single project).
* Git URL (HTTPS/SSH) with specific branch, tag, or commit.
* Optional: pre-exported archive (.zip/.tar.gz) in offline mode.
* **NEW**: Incremental analysis mode (analyze only changed files since last run).

---

## 5. Functional Requirements

### 5.1 F1: Repository Ingestion

**Requirements**

1. QFA SHALL accept as input a local path, git URL, or archive file.
2. QFA SHALL support filtering by subdirectory (e.g., `services/payments/`).
3. QFA SHALL provide a configurable ignore list (e.g., `tests/`, `node_modules/`, `target/`, `build/`).
4. QFA SHALL capture basic repo metadata (repo name, commit hash, default branch, total files by language).
5. **NEW**: QFA SHALL support incremental analysis by detecting changed files since last analysis (via git diff or cached file hashes).

### 5.2 F2: Language Detection & File Classification

**Requirements**

1. QFA SHALL detect file language based on extension and/or content heuristics.
2. QFA SHALL classify each source file into a supported language or mark as `unsupported`.
3. QFA SHALL produce per-language summaries (file counts, line counts, primary entrypoints where inferable).
4. **NEW**: QFA SHALL report detection confidence for each file (e.g., "Python detected with 0.95 confidence").

### 5.3 F3: Language-Specific Feature Extraction

For each supported language, QFA SHALL extract the following minimum features:

* **Imports / Dependencies**: libraries, modules, packages, namespaces, use statements.
* **Key API Calls**: function/method calls indicative of optimization, simulation, ML, Monte Carlo, crypto, or graph processing.
* **Structural Patterns**: loops, recursion, state updates, use of concurrency constructs (threads, async, channels) when relevant.
* **Numeric & Matrix Operations**: vector/matrix libs, BLAS/LAPACK wrappers, direct linear algebra calls.
* **Configuration / Metadata**: presence of config files indicating problem types (e.g., scheduler configs, model configs).
* **NEW**: **Detection Confidence**: Each extracted feature SHALL have an associated confidence score (0.0-1.0) accounting for:
  - **Direct detection** (explicit imports, clear function calls): 0.9-1.0
  - **Indirect detection** (conditional imports, reflection): 0.5-0.8
  - **Inferred detection** (heuristic patterns): 0.3-0.6

Feature extraction SHALL be performed via:

* Native language tooling (e.g., `ast` for Python, Roslyn for C#, javac APIs for Java) or
* Shared parsing frameworks (e.g., tree-sitter) where appropriate.

**Known Limitations (documented per language)**:

* **Python**: Cannot detect dynamic imports via `__import__()`, `importlib`, or exec/eval.
* **Java**: Limited support for reflection-based algorithm loading.
* **C++**: Template metaprogramming and macros may hide algorithm signatures.
* **JavaScript**: Minified or obfuscated code has reduced detection accuracy.
* **All languages**: Cross-language FFI calls (e.g., Python→C++ via pybind11) require manual annotation.

### 5.4 F4: Workload Classification Engine

QFA SHALL map extracted features into **candidate workload types**. At minimum, the following workload families must be supported:

1. **Combinatorial Optimization & OR**
   * Mixed-integer programming, constraint programming, scheduling, routing, assignment, portfolio optimization.

2. **Graph Algorithms & Network Problems**
   * Max-Cut, min-cut, coloring, TSP/VRP, community detection, clique, independent set.

3. **Monte Carlo & Stochastic Simulation**
   * Risk models, pricing simulations, random sampling, Markov processes.

4. **Linear Algebra & Numerical Solvers**
   * Solving systems of linear equations, eigenvalue problems, PDE solvers, large-scale iterative solvers.

5. **Physics / Chemistry / Quantum Simulation**
   * Hamiltonians, molecule/basis definitions, physics engines, quantum simulator integrations.

6. **Machine Learning & Optimization for Training**
   * Core training loops, gradient-based optimization, kernel methods.

7. **Cryptography & Security-Relevant Primitives**
   * Classical public-key crypto (RSA, ECC, DH), key exchange, digital signatures, hashing.

8. **NEW: Search & Database Queries**
   * Unstructured search, SAT solving, constraint satisfaction problems.

For each workload candidate, QFA SHALL:

1. Assign a **problem_type** (from the workload families).
2. Provide a **confidence score** in [0.0, 1.0].
3. Record the **evidence**: files, functions, lines, and rules triggered.
4. **NEW**: Estimate **problem size** (N, number of variables, dataset cardinality) where inferable from code.

### 5.5 F5: Quantum Algorithm Mapping Engine

For each detected workload, QFA SHALL map to one or more **quantum algorithm families**. At minimum:

| Workload Type | Quantum Algorithm Families | References |
|---------------|----------------------------|------------|
| Combinatorial optimization | QAOA, Quantum Annealing, VQE (for QUBO) | Farhi et al. (2014) |
| Graph problems | QAOA, Quantum Walk, Amplitude Amplification | Childs & Goldstone (2004) |
| Monte Carlo | Amplitude Estimation, Quantum Monte Carlo | Montanaro (2015) |
| Linear algebra | HHL Algorithm, Quantum Linear Solvers | Harrow et al. (2009) |
| Physics / chemistry | VQE, QPE, Hamiltonian Simulation | Peruzzo et al. (2014) |
| Machine Learning | Quantum Kernels, Variational Classifiers | Havlíček et al. (2019) |
| Search | Grover's Algorithm, Amplitude Amplification | Grover (1996) |
| Cryptography | PQC Migration (defensive, not offload) | NIST PQC (2022) |

For each mapping, QFA SHALL provide:

1. **`algorithm_family`** identifier (e.g., `QAOA`, `VQE`, `AMPLITUDE_ESTIMATION`, `GROVER`, `HHL`).

2. **`maturity`** level enum (refined from 4 to 5 levels):
   - **`production`**: Deployable today with demonstrable advantage (NOTE: Likely NONE in 2025 for optimization)
   - **`near_term`**: NISQ-feasible (2025-2028), experimental, approximate solutions
   - **`mid_term`**: Early fault-tolerant era (2028-2032), small-scale advantage
   - **`long_term`**: Mature fault-tolerant QC (2032+), large-scale advantage
   - **`research`**: Theoretical only, no near-term hardware path
   - **`not_applicable`**: No quantum benefit expected

3. **Multi-dimensional scores** (0-5 scale):
   * **`algorithmic_fit`**: Does the quantum algorithm theoretically apply?
     - 5: Perfect mapping (e.g., integer factorization → Shor's)
     - 3: Partial mapping (e.g., TSP → QAOA with approximation)
     - 1: Weak mapping (e.g., convex optimization → no quantum advantage)

   * **`hardware_feasibility`**: Can current/near-term hardware run it?
     - 5: NISQ-feasible today (≤100 qubits, ≤1000 gates)
     - 3: Requires 1000-10K qubits (2028-2030 timeline)
     - 1: Requires fault-tolerant QC (2032+ timeline)

   * **`codebase_fit`**: How isolatable is the workload?
     - 5: Standalone function, clear I/O
     - 3: Requires refactoring to extract
     - 1: Deeply embedded, hard to isolate

4. **NEW: Resource Requirements**:
   ```json
   "resources": {
     "qubits_required": "~100 (problem-dependent)",
     "qubits_formula": "n qubits for n variables (QAOA)",
     "circuit_depth": "~O(p × n) for p layers",
     "circuit_depth_estimate": "~500-5000 gates (p=5, n=100)",
     "error_correction": "recommended for >50 qubits",
     "nisq_feasible": true,
     "providers": ["IBM Quantum (127-qubit Eagle)", "AWS Braket (IonQ Aria)", "Google Sycamore"],
     "estimated_runtime": "Minutes to hours (NISQ), seconds (fault-tolerant future)"
   }
   ```

5. **NEW: Quantified Speedup Estimates**:
   ```json
   "speedup": {
     "type": "quadratic",  // exponential | quadratic | polynomial | constant | none
     "classical_complexity": "O(N)",
     "quantum_complexity": "O(√N)",
     "speedup_factor": "~1000× for N=10^6",
     "problem_size_threshold": "N > 10^4 for quantum advantage",
     "caveats": [
       "Speedup is asymptotic (requires large N)",
       "NISQ-era noise reduces practical speedup to ~10-100×",
       "Requires error mitigation techniques"
     ]
   }
   ```

6. **Human-readable explanation** referencing rules and evidence.

7. **NEW: Disclaimer for each mapping**:
   > "This mapping is based on theoretical analysis. Practical quantum advantage is NOT guaranteed and depends on problem size, data characteristics, quantum hardware quality, and classical baseline optimizations. Experimental validation via PoC is required before production deployment."

### 5.6 F6: Scoring & Aggregation

QFA SHALL aggregate per-workload scores into repo-level and portfolio-level metrics:

#### 5.6.1 Workload-Level Scoring

**Quantum Applicability Score (QAS)** for each workload:

```
QAS = Σ (Criterion_i × Weight_i) × Detection_Confidence

Criteria (weighted, 0-100 scale):
- Problem Size (20%):     N >10^6 → 100 pts; N <1000 → 0 pts
- Complexity Class (25%): BQP ∖ BPP → 100 pts; P → 0 pts
- Parallelism (15%):      Embarrassingly parallel → 100; Sequential → 0
- Eigenvalue Structure (15%): Linear algebra → 100; None → 0
- Oracle Access (10%):    Black-box evaluations → 100; None → 0
- Optimization (10%):     NP-hard combinatorial → 100; Convex → 0
- Bottleneck (5%):        Performance-critical → 100; Fast → 0

Detection_Confidence: From feature extraction (§5.3)

Final QAS: 0-100 scale
```

**Worked Example**:

```
Workload: Traveling Salesman Problem (TSP) solver using OR-Tools

Criterion Scores:
- Problem Size: N=500 cities → 60 points (moderate size)
- Complexity Class: NP-hard (BQP-applicable) → 100 points
- Parallelism: Can parallelize branch-and-bound → 70 points
- Eigenvalue Structure: Not applicable → 0 points
- Oracle Access: Requires distance matrix lookups → 80 points
- Optimization: NP-hard combinatorial → 100 points
- Bottleneck: Identified as performance bottleneck in profiling → 90 points

Weighted Sum:
(60×0.20) + (100×0.25) + (70×0.15) + (0×0.15) + (80×0.10) + (100×0.10) + (90×0.05)
= 12 + 25 + 10.5 + 0 + 8 + 10 + 4.5
= 70 points

Detection Confidence: 0.92 (direct detection of OR-Tools TSP API)

QAS = 70 × 0.92 = 64.4
```

**Interpretation**:
- QAS 64.4 → "Quantum-Aware" (selective quantum migration candidate)
- Recommendation: Consider QAOA PoC if N scales to >1000 cities

#### 5.6.2 Repo-Level Scoring

**Quantum Feasibility Index (QFI)**:

```
QFI = Σ (QAS_i × LOC_i) / Σ LOC_i

Where:
- QAS_i: Quantum Applicability Score for workload i
- LOC_i: Lines of code in workload i
```

**Repo Classification**:

| QFI Range | Classification | Recommendation |
|-----------|----------------|----------------|
| 85-100 | **Quantum-Ready** | Strong quantum investment case; prioritize PoCs |
| 70-84 | **Quantum-Enabled** | Investigate quantum hybrid approaches |
| 50-69 | **Quantum-Aware** | Selective quantum migration for high-scoring workloads |
| 25-49 | **Quantum-Interested** | Limited quantum applicability; monitor progress |
| 0-24 | **Classical-Optimized** | Stay with classical computing |

#### 5.6.3 Portfolio-Level Scoring

When multiple repos are analyzed:

* **Rankings by QFI**: Sort repos by quantum readiness.
* **Heatmap of workload types**: Visualize distribution across portfolio.
* **Aggregated counts of candidate PoCs** by domain (optimization, simulation, ML).
* **Total estimated PoC investment** (sum of effort estimates across all high-priority candidates).

**Scoring logic SHALL be**:
- ✅ Deterministic (same input → same output)
- ✅ Versioned (scoring formula version included in reports)
- ✅ Documented (formulas and weights published in appendix)
- ✅ Sensitivity-analyzed (impact of weight changes documented)

### 5.7 F7: Transition Plan Generation

QFA SHALL generate a **transition plan** for each repo that includes:

1. **Summary** section:
   * Overall feasibility (QFI score and classification).
   * Key workloads and algorithm families.
   * Recommended next step (e.g., "PoC candidate", "Monitor", "Not applicable").
   * **NEW: Estimated total effort** (person-months).
   * **NEW: Estimated cost** (quantum cloud credits, consulting).

2. **Workload-Level Recommendations**:
   * Classical baseline KPIs to measure (runtime, cost, accuracy).
   * Suggested quantum/hybrid PoC structure (e.g., offload optimization kernel as an external service).
   * Dependencies on quantum cloud providers or frameworks.
   * **NEW: Effort estimate** (e.g., "6-12 months for PoC development").
   * **NEW: Cost estimate** (e.g., "$10K-$50K in quantum cloud credits, $100K-$200K in consulting").

3. **Roadmap Phases** (template):
   * **Phase 0: Baseline & Refactoring** (4-8 weeks)
     - Measure classical performance (runtime, memory, accuracy)
     - Refactor workload for isolation (extract as microservice/library)
     - Cost: $20K-$40K (engineering time)

   * **Phase 1: Quantum PoC in Cloud Sandbox** (8-16 weeks)
     - Implement QAOA/VQE/Grover on IBM Quantum / AWS Braket
     - Validate functional correctness on small instances
     - Cost: $10K-$50K (cloud credits) + $50K-$100K (engineering)

   * **Phase 2: Hybrid Integration** (12-20 weeks)
     - Integrate quantum solver into production architecture
     - Implement classical-quantum orchestration
     - Cost: $100K-$200K (platform integration)

   * **Phase 3: Production Hardening & Monitoring** (8-12 weeks)
     - Error mitigation, result verification, monitoring
     - Cost: $50K-$100K (DevOps, SRE)

4. **Risk & Caveats**:
   * Where benefits are speculative or research-grade only.
   * Data scale, precision, and hardware constraints.
   * **NEW: Probability of success** (e.g., "70% chance of 10× speedup, 30% chance of no advantage").

5. **NEW: Baseline Profiling Guidance**:
   * Recommended profiling tools per language:
     - Python: `cProfile`, `line_profiler`, `memory_profiler`
     - Java: JProfiler, YourKit
     - C++: Valgrind, perf, Intel VTune
   * Key metrics to capture:
     - Wall-clock runtime (median, p95, p99)
     - CPU utilization
     - Memory footprint
     - Convergence rate (for optimization)
     - Solution quality (for approximation algorithms)
   * Benchmark dataset suggestions:
     - Use realistic production data sizes
     - Include best-case, average-case, worst-case scenarios
     - Capture variability across multiple runs (≥10 runs)

### 5.8 F8: Multi-Language Aggregation & Cross-Service Support

For polyglot repos containing multiple services in different languages, QFA SHALL:

1. Analyze each language-specific component independently using the appropriate analyzer.
2. Associate workloads with logical services or modules (e.g., via directory structure, build configs).
3. Provide service-level reports and a unified repo-level view.
4. Support configuration to mark certain directories as separate services.
5. **NEW**: Detect cross-language quantum opportunities (e.g., Python ML training + C++ inference).

### 5.9 F9: Integrations (CLI, CI, API)

1. **CLI**
   * `qfa analyze <path|git-url> [options]`
   * Options for:
     - Output format (`--format json|markdown|html`)
     - Config file (`--config .qfa.yml`)
     - Language filters (`--languages python,rust`)
     - Service boundaries (`--services services/*/`)
     - Confidence threshold (`--min-confidence 0.7`)
     - Incremental mode (`--incremental`)
     - Cache directory (`--cache-dir .qfa-cache/`)
   * Exit codes suitable for CI workflows:
     - 0: Success
     - 1: Analysis failed (parsing errors, config invalid)
     - 2: No quantum opportunities found
     - 3: Cache corruption (re-run without cache)

2. **CI/CD (e.g., GitHub Action)**
   * Reference Action for GitHub that:
     - Checks out code.
     - Runs QFA in incremental mode (caches previous results).
     - Uploads JSON/Markdown/HTML reports as artifacts.
     - Optionally posts a summarized comment on PRs.
     - Fails build if critical errors detected (configurable).
   * Example workflow:
     ```yaml
     - uses: arcqubit/qfa-action@v1
       with:
         source-dir: ./src
         output-format: markdown,json,html
         min-confidence: 0.7
         incremental: true
     ```

3. **API (Service Mode)**
   * Optional FastAPI (or similar) service with endpoints:
     - `POST /analyze/repo` – analyze by git URL.
     - `POST /analyze/upload` – analyze uploaded archive.
     - `GET /report/{id}` – fetch stored report.
     - `GET /report/{id}/visualizations` – fetch heatmaps, timelines.
     - `DELETE /report/{id}` – delete stored report (privacy).
   * **NEW**: Sandboxed execution (Docker containers, no network access during analysis).

### 5.10 F10: Configurability & Rules Management

QFA SHALL:

1. Load rules from versioned configuration files (e.g., YAML or JSON).
2. Support enabling/disabling specific rules or rule sets.
3. Support organization-specific rule extensions without modifying core engine code.
4. Provide a `--explain` mode to output all rules evaluated and those triggered.
5. **NEW**: Support user-defined rule priorities for conflict resolution.
6. **NEW**: Provide rule testing framework (`qfa test-rules --ruleset custom-rules.yml --test-data test-cases/`).

### 5.11 F11: Explainability & Traceability

QFA SHALL:

1. Record **Rule Hits** including: rule ID, description, evidence (file, function, line numbers).
2. Expose these hits in both JSON and Markdown outputs.
3. Provide a machine-readable `decision_trace` section in the report to support downstream analysis and audits.
4. **NEW**: Include code snippet context (5-10 lines) for each rule hit in human-readable reports.
5. **NEW**: Generate audit logs for compliance (ISO 27001, SOC 2).

### 5.12 F12: Baseline Performance Profiling (NEW)

QFA SHALL provide guidance on measuring classical baseline performance:

1. **Recommended profiling tools per language** (§5.7.5).
2. **Key metrics to capture**:
   - Runtime (median, p95, p99)
   - Memory footprint
   - Convergence rate (for optimization)
   - Solution quality (for approximation algorithms)
3. **Benchmark dataset suggestions**:
   - Use realistic production data sizes
   - Include best-case, average-case, worst-case scenarios
4. **Integration with transition plans**: Link baseline measurements to quantum speedup validation.

**Why critical**: Without classical baseline metrics, users cannot validate quantum speedup claims.

### 5.13 F13: False Positive Handling (NEW)

QFA SHALL provide mechanisms to suppress false positives:

1. **User-annotated exclusions**:
   - `.qfa-ignore` file (similar to `.gitignore`):
     ```
     # Ignore test utilities
     tests/

     # Ignore specific false positive
     src/utils/random_helper.py:42  # Not Monte Carlo, just utility function
     ```

   - In-code annotations:
     ```python
     # @qfa-ignore MONTE_CARLO_1: This is just a random utility, not simulation
     def random_string():
         return ''.join(random.choices(string.ascii_letters, k=10))
     ```

2. **Confidence thresholds**:
   - Only report workloads with confidence ≥ threshold (default: 0.7)
   - Configurable via CLI: `--min-confidence 0.8`

3. **Manual review workflow**:
   - Generate review report with "Needs Review" section
   - User marks as "True Positive", "False Positive", or "Uncertain"
   - Feedback incorporated into rule refinement (optional telemetry)

4. **Adaptive learning** (future):
   - ML-based false positive reduction using user feedback
   - Community-driven rule quality improvements

**Success metric**: False positive rate decreases by ≥50% after first refinement iteration.

---

## 6. Language-Specific Analyzer Requirements

Each language analyzer must conform to a common interface:

```rust
trait LanguageAnalyzer {
    fn analyze_files(&self, files: Vec<File>) -> AnalysisResult;
}

struct AnalysisResult {
    features: Vec<ExtractedFeature>,
    workloads: Vec<WorkloadCandidate>,
    diagnostics: Vec<Diagnostic>,
    confidence: f32,  // Overall detection confidence
}
```

### 6.1 Python Analyzer

**Parsing**: Use Python `ast` module or tree-sitter Python grammar.

**Target Libraries** (top 20, ranked by quantum relevance):

| Library | Category | Quantum Mapping | Detection Priority |
|---------|----------|-----------------|-------------------|
| `ortools`, `pulp`, `gurobipy`, `cplex` | Optimization | QAOA | High |
| `numpy`, `scipy.linalg` | Linear Algebra | HHL | High |
| `networkx` | Graph Algorithms | QAOA, Quantum Walk | High |
| `qiskit`, `pennylane`, `cirq` | Quantum SDKs | Direct quantum code | Critical |
| `tensorflow`, `pytorch`, `jax` | Machine Learning | Quantum Kernels | Medium |
| `scipy.optimize` | Optimization | QAOA (for combinatorial) | High |
| `random`, `numpy.random` (with loops) | Monte Carlo | Amplitude Estimation | Medium |
| `sympy` (Hamiltonians) | Physics | VQE, Hamiltonian Simulation | High |
| `cryptography`, `pycryptodome` | Cryptography | PQC Migration | High (defensive) |
| `scikit-learn` (kernel methods) | Machine Learning | Quantum Kernels | Medium |

**Signature Function Calls**:
```python
# Optimization
Model.addVar(), Model.addConstr(), Model.optimize()
scipy.optimize.minimize(), scipy.optimize.linprog()

# Linear Algebra
numpy.linalg.solve(), scipy.linalg.eig()

# Graph
networkx.max_cut(), networkx.traveling_salesman_problem()

# Monte Carlo
random.random() in loop, numpy.random.* in simulation loop
```

**Known Limitations**:
- ❌ Cannot detect dynamic imports: `__import__(config['solver'])`
- ❌ Cannot analyze exec/eval: `exec("import scipy")`
- ❌ Limited support for Cython/compiled extensions
- ⚠️ Decorator-heavy code may reduce detection confidence

**Detection Confidence**:
- Direct import + function call: 0.9-1.0
- Import only (no usage): 0.5-0.7
- Heuristic pattern (loops with random): 0.3-0.6

### 6.2 JavaScript / TypeScript Analyzer

**Parsing**: Use TypeScript compiler API or tree-sitter JS/TS.

**Target Libraries** (top 15):

| Library | Category | Quantum Mapping |
|---------|----------|-----------------|
| `math.js`, `numeric.js` | Numerical Computing | HHL, Optimization |
| `tensorflow.js`, `onnx.js` | Machine Learning | Quantum Kernels |
| `graphlib`, `ngraph` | Graph Algorithms | QAOA |
| Custom optimization (e.g., genetic algorithms) | Optimization | QAOA |
| WASM bindings to numeric libs | High-Performance Computing | Various |

**Signature Patterns**:
```javascript
// Optimization
math.optimize(...), genetic algorithm implementations

// Graph
graph.findPath(), graph.shortestPath()

// ML
tf.train.sgd(), tf.layers.dense()
```

**Known Limitations**:
- ❌ Minified code has very low detection accuracy (<0.3 confidence)
- ❌ Dynamic requires: `require(config.moduleName)`
- ⚠️ WASM modules are black boxes (cannot analyze internal algorithms)

### 6.3 Rust Analyzer

**Parsing**: Use Rust analyzer APIs or tree-sitter Rust.

**Target Crates** (top 15):

| Crate | Category | Quantum Mapping |
|-------|----------|-----------------|
| `nalgebra`, `ndarray`, `faer` | Linear Algebra | HHL |
| `good_lp`, `minilp`, `lpsolve` | Optimization | QAOA |
| `petgraph` | Graph Algorithms | QAOA, Quantum Walk |
| `argmin` | Optimization | QAOA (for combinatorial) |
| `rand` (in simulation loops) | Monte Carlo | Amplitude Estimation |
| Physics engines (custom Hamiltonians) | Physics Simulation | VQE |
| `rustcrypto` | Cryptography | PQC Migration |

**Signature Patterns**:
```rust
// Linear algebra
nalgebra::Matrix::solve(), ndarray::linalg::solve()

// Optimization
good_lp::ProblemVariables::add(), good_lp::Problem::solve()

// Graph
petgraph::algo::min_spanning_tree()
```

**Known Limitations**:
- ⚠️ Macros can hide algorithm signatures (e.g., `solver!{ ... }`)
- ⚠️ Generic code may require type inference for full analysis

### 6.4 Java Analyzer

**Parsing**: Use Java compiler tooling (javac APIs, Spoon, or tree-sitter).

**Target Libraries** (top 15):

| Library | Category | Quantum Mapping |
|---------|----------|-----------------|
| Gurobi, CPLEX, OptaPlanner | Optimization | QAOA |
| Apache Commons Math | Numerical Computing | HHL, Optimization |
| JGraphT | Graph Algorithms | QAOA |
| ND4J, DL4J | Machine Learning | Quantum Kernels |
| Financial risk engines (custom) | Monte Carlo | Amplitude Estimation |
| Bouncy Castle (crypto) | Cryptography | PQC Migration |

**Signature Patterns**:
```java
// Optimization
GRBModel.addVar(), GRBModel.optimize()
OptaPlanner solve methods

// Linear algebra
RealMatrix.solve(), EigenDecomposition

// Graph
DijkstraShortestPath, KruskalMinimumSpanningTree
```

**Known Limitations**:
- ❌ Reflection-based algorithm loading: `Class.forName(config.get("solver"))`
- ⚠️ Spring/dependency injection may obscure direct usage patterns
- ⚠️ Proprietary JARs cannot be analyzed (only API calls visible)

### 6.5 Go Analyzer

**Parsing**: Use Go parser packages (`go/ast`, `go/parser`).

**Target Packages** (top 15):

| Package | Category | Quantum Mapping |
|---------|----------|-----------------|
| `gonum.org/v1/gonum/optimize` | Optimization | QAOA |
| `gonum.org/v1/gonum/mat` | Linear Algebra | HHL |
| Graph libraries (e.g., `github.com/yourbasic/graph`) | Graph Algorithms | QAOA |
| Finance/risk packages (custom Monte Carlo) | Simulation | Amplitude Estimation |
| `crypto/*` (standard library) | Cryptography | PQC Migration |
| Scheduling frameworks | Combinatorial Optimization | QAOA |

**Signature Patterns**:
```go
// Optimization
optimize.Minimize(...), optimize.Local(...)

// Linear algebra
mat.Solve(...), mat.Eigen(...)

// Concurrency-heavy numerical workloads
Multiple goroutines with shared matrix computations
```

**Known Limitations**:
- ⚠️ Interface-based polymorphism may require interprocedural analysis
- ⚠️ Plugin system (`plugin.Open()`) loads code dynamically (unanalyzable)

### 6.6 C++ Analyzer

**Parsing**: Use Clang tooling (libtooling) or tree-sitter C++.

**Target Libraries** (top 20):

| Library | Category | Quantum Mapping |
|---------|----------|-----------------|
| Gurobi, CPLEX, SCIP | Optimization | QAOA |
| Eigen, Armadillo, BLAS/LAPACK | Linear Algebra | HHL |
| Boost.Graph | Graph Algorithms | QAOA |
| Custom PDE solvers, CFD | Simulation | Quantum Simulation |
| Physics engines (custom Hamiltonians) | Physics | VQE, Hamiltonian Sim |
| OpenSSL, Crypto++ | Cryptography | PQC Migration |
| Monte Carlo financial engines | Simulation | Amplitude Estimation |

**Signature Patterns**:
```cpp
// Optimization
GRBModel::addVar(), GRBModel::optimize()

// Linear algebra
Eigen::MatrixXd::solve(), LAPACK dgesv()

// Graph
boost::max_flow(), boost::dijkstra_shortest_paths()

// Physics
Custom Hamiltonian classes, SchrÃ¶dinger equation solvers
```

**Known Limitations (CRITICAL)**:
- ❌ **Template metaprogramming**: Algorithms in templates require full instantiation analysis
  - Example: `template<typename Solver> void optimize() { Solver::solve(); }`
  - Mitigation: Use `-ftemplate-depth` analysis in Clang

- ❌ **Macro preprocessing**: Algorithms hidden in macros
  - Example: `#define OPTIMIZE(x) gurobi_optimize(x)`
  - Mitigation: Analyze preprocessed source (gcc -E)

- ❌ **Binary dependencies**: Linking to proprietary libs (Gurobi .so) exposes no source
  - Mitigation: Use link-time analysis to detect library dependencies

- ❌ **Compiled binaries**: Cannot analyze .o, .a, .so files
  - Limitation: QFA is **source-only**

- ⚠️ **Build system variability**: CMake, Bazel, Make require different detection strategies
  - Mitigation: Provide plugins for common build systems

**Detection Confidence**:
- Header-only libraries (Eigen, Armadillo): 0.8-1.0
- Binary dependencies (detected via linker flags): 0.5-0.7
- Macro-based patterns: 0.3-0.6

**Recommendation**: Start with **header-only libraries** (Eigen, Armadillo) in MVP; add binary dependency detection in Phase 3.

### 6.7 C# Analyzer

**Parsing**: Use Roslyn APIs.

**Target Libraries** (top 15):

| Library | Category | Quantum Mapping |
|---------|----------|-----------------|
| Gurobi, CPLEX .NET APIs | Optimization | QAOA |
| Math.NET Numerics | Numerical Computing | HHL, Optimization |
| QuickGraph | Graph Algorithms | QAOA |
| ML.NET | Machine Learning | Quantum Kernels |
| Financial risk libraries (custom) | Monte Carlo | Amplitude Estimation |
| BouncyCastle.NET | Cryptography | PQC Migration |

**Signature Patterns**:
```csharp
// Optimization
GRBModel.AddVar(), GRBModel.Optimize()

// Linear algebra
Matrix<double>.Solve(), Matrix<double>.Evd()

// Graph
QuickGraph shortest path algorithms
```

**Known Limitations**:
- ❌ Reflection: `Type.GetType(config["SolverType"])`
- ⚠️ Dynamic assemblies (`Assembly.Load()`) load code at runtime
- ⚠️ .NET Core vs .NET Framework API differences

---

## 7. Rules Engine & Pattern Representation

### 7.1 Representation

Rules SHALL be represented declaratively, e.g., in YAML/JSON, with the following structure:

* `id`: unique rule identifier.
* `version`: rule version (semantic versioning).
* `description`: human-readable description.
* `languages`: list of applicable languages.
* `priority`: integer priority for conflict resolution (higher wins).
* `signals`: logical predicates on extracted features (imports, calls, names, comments, configs).
* `outputs`: problem type, candidate algorithm families, base scores, explanations.

**Example (simplified YAML)**

```yaml
- id: P1_COMBINATORIAL_OPT_GUROBI
  version: "1.2.0"
  description: "Mixed-integer optimization using Gurobi solver"
  languages: ["python", "java", "c++", "c#"]
  priority: 100  # High priority (specific library)
  signals:
    any_import:
      - "gurobipy"
      - "gurobi"
      - "com.gurobi.gurobi"
    any_call:
      - "Model.addVar"
      - "Model.addConstr"
      - "GRBModel.addVar"
      - "GRBModel.optimize"
  outputs:
    problem_type: "combinatorial_optimization"
    quantum_candidates:
      - algorithm: "QAOA"
        maturity: "near_term"
        algorithmic_fit: 4
        hardware_feasibility: 3
        codebase_fit: 4
    base_score: 0.75
    confidence_boost: 0.1  # Boost confidence due to explicit library usage
    explanation: "Detected Gurobi MILP solver. Problem may be mappable to QUBO for QAOA."
    references:
      - "Farhi et al. (2014) - QAOA for combinatorial optimization"
      - "Glover et al. (2022) - Quantum annealing for QUBO"

- id: P2_MONTE_CARLO_GENERIC
  version: "1.0.0"
  description: "Generic Monte Carlo simulation pattern"
  languages: ["python", "java", "c++", "c#", "go"]
  priority: 50  # Lower priority (heuristic pattern)
  signals:
    all_of:
      - any_import: ["random", "numpy.random", "java.util.Random", "std::random"]
      - loop_with_random_sampling: true  # AST pattern: for/while loop with random calls
      - iteration_count: ">1000"  # Heuristic: large iteration count suggests Monte Carlo
  outputs:
    problem_type: "monte_carlo_simulation"
    quantum_candidates:
      - algorithm: "AMPLITUDE_ESTIMATION"
        maturity: "mid_term"
        algorithmic_fit: 3
        hardware_feasibility: 2
        codebase_fit: 3
    base_score: 0.55
    confidence_penalty: 0.2  # Reduce confidence due to heuristic detection
    explanation: "Detected loop with random sampling. May benefit from quantum amplitude estimation if convergence is bottleneck."
    caveats:
      - "Requires large sample sizes (N >10^6) for quantum advantage"
      - "Classical variance reduction (importance sampling, control variates) should be optimized first"
    references:
      - "Montanaro (2015) - Quantum speedup of Monte Carlo methods"
```

### 7.2 Evaluation

* Rules are evaluated over the language-specific IR.
* Rules MAY compose (multiple rules can contribute to a single workload's classification).
* **NEW: Conflicting or overlapping rules SHALL be resolved via the following strategy**:

#### 7.2.1 Rule Conflict Resolution (NEW)

**Resolution Strategy** (applied in order):

1. **Priority-Based Resolution**:
   - Rules have explicit `priority` values (0-1000)
   - Higher priority rules override lower priority rules
   - Example: Specific library detection (priority 100) beats generic heuristic (priority 50)

2. **Specificity-Based Resolution**:
   - Rules with more specific signals (e.g., exact function names) override general patterns
   - Example: `secrets.SystemRandom()` beats `random.random()`

3. **Multi-Label Classification** (preferred):
   - Allow workloads to have **multiple problem types** with confidence scores
   - Example: A workload can be 70% "combinatorial_optimization" AND 40% "monte_carlo_simulation"
   - Output: List of `(problem_type, confidence)` tuples sorted by confidence

4. **Confidence-Weighted Voting**:
   - If multiple rules fire with equal priority, aggregate via weighted vote:
     ```
     Final_Confidence(problem_type) = max(rule_confidences) + 0.1 × count(supporting_rules)
     ```
   - Prevents single rule from dominating, rewards consensus

**Example Conflict**:

```yaml
# Rule A: Generic random usage (low priority, low confidence)
- id: RANDOM_USAGE_GENERIC
  priority: 30
  signals: {any_import: ["random"]}
  outputs: {problem_type: "monte_carlo_simulation", base_score: 0.4}

# Rule B: Cryptographic random (high priority, high confidence)
- id: CRYPTO_RNG
  priority: 100
  signals: {any_import: ["secrets", "cryptography.hazmat"]}
  outputs: {problem_type: "cryptography", base_score: 0.9}

# Resolution:
# If both fire on code importing both "random" and "secrets":
# → Priority-based: Rule B wins (priority 100 > 30)
# → Output: problem_type = "cryptography" (confidence 0.9)
# → Suppressed: "monte_carlo_simulation" (overridden by higher priority rule)
```

### 7.3 Versioning

* Rules SHALL be versioned along with the QFA engine, with a semantic version tag.
* Reports SHALL include:
  - Engine version (e.g., `qfa-engine: 1.0.0`)
  - Ruleset version (e.g., `ruleset: 2024.11.18`)
  - Individual rule versions (in `decision_trace`)
* **NEW**: Rules can be hot-reloaded without restarting QFA service (service mode only).

### 7.4 Rule Testing Framework (NEW)

QFA SHALL provide a testing framework for rules:

```bash
# Test rules against known test cases
qfa test-rules \
  --ruleset custom-rules.yml \
  --test-data test-cases/ \
  --expected-results expected.json

# Output:
# ✅ P1_COMBINATORIAL_OPT: 15/15 tests passed
# ❌ P2_MONTE_CARLO: 8/10 tests passed (2 false positives)
#    - test-cases/false-positive-1.py: Expected "none", got "monte_carlo"
#    - test-cases/false-positive-2.java: Expected "none", got "monte_carlo"
```

**Test Case Format**:

```json
{
  "test_id": "test_qaoa_tsp_python",
  "file": "test-cases/tsp_ortools.py",
  "expected": {
    "problem_type": "combinatorial_optimization",
    "quantum_candidates": ["QAOA"],
    "min_confidence": 0.8
  }
}
```

---

## 8. Data Model & Report Structure

### 8.1 Core Entities

* **Project**: represents a repo or service.
* **File**: source file with language, path, and basic stats.
* **WorkloadCandidate**: detected workload, with problem type and evidence.
* **QuantumMapping**: mapping from workload to quantum algorithm families.
* **RuleHit**: record of a specific rule that fired, with evidence.
* **ResourceRequirements** (NEW): qubits, circuit depth, error correction needs.
* **SpeedupEstimate** (NEW): theoretical and practical speedup quantification.
* **TransitionPlan** (NEW): phased roadmap with effort/cost estimates.
* **AnalysisReport**: top-level object containing all data and metadata.

### 8.2 JSON Report (Enhanced)

```json
{
  "metadata": {
    "qfa_version": "1.0.0",
    "ruleset_version": "2024.11.18",
    "generated_at": "2025-11-18T12:00:00Z",
    "analysis_duration_sec": 142.5
  },
  "project": {
    "name": "payments-service",
    "repo": "git@github.com:org/payments.git",
    "commit": "abc123",
    "branch": "main",
    "languages": {
      "python": {"files": 12, "loc": 8500},
      "go": {"files": 5, "loc": 3200}
    },
    "total_files": 17,
    "total_loc": 11700
  },
  "workloads": [
    {
      "id": "w1",
      "problem_type": "combinatorial_optimization",
      "language": "python",
      "files": ["src/optimizer.py"],
      "functions": ["optimize_portfolio"],
      "lines": [42, 67, 89],
      "evidence": ["P1_COMBINATORIAL_OPT_GUROBI"],
      "confidence": 0.92,
      "detection_method": "explicit_library",
      "problem_size_estimate": "N=500 assets",
      "code_snippet": "def optimize_portfolio(assets):\n    model = gp.Model()\n    x = model.addVars(len(assets), vtype=GRB.BINARY)\n    model.optimize()\n    return x"
    }
  ],
  "quantum_mappings": [
    {
      "workload_id": "w1",
      "algorithm_family": "QAOA",
      "maturity": "near_term",
      "scores": {
        "algorithmic_fit": 4,
        "hardware_feasibility": 3,
        "codebase_fit": 4
      },
      "resources": {
        "qubits_required": "~500 (1 per asset)",
        "qubits_formula": "n qubits for n binary variables",
        "circuit_depth": "~O(p × n) for p QAOA layers",
        "circuit_depth_estimate": "~2500-25000 gates (p=5-50, n=500)",
        "error_correction": "recommended for >100 qubits",
        "nisq_feasible": false,
        "providers": ["IBM Quantum (1000+ qubit roadmap 2025)", "AWS Braket (IonQ future)"],
        "estimated_runtime": "Minutes (NISQ with approximation), seconds (fault-tolerant future)"
      },
      "speedup": {
        "type": "problem_dependent",
        "classical_complexity": "O(2^n) brute force, O(n^3) Gurobi (approximate)",
        "quantum_complexity": "O(p × n) QAOA circuit evaluations",
        "speedup_factor": "10-100× demonstrated on small instances (n<100)",
        "problem_size_threshold": "N > 100 for potential advantage over classical heuristics",
        "caveats": [
          "Speedup is problem-dependent and not guaranteed",
          "NISQ-era noise limits practical speedup to ~10× at best",
          "Classical solvers (Gurobi, CPLEX) are highly optimized; quantum must beat these",
          "Requires careful QUBO formulation and parameter tuning (p layers)"
        ]
      },
      "explanation": "Detected Gurobi MILP with binary variables. Problem is mappable to QUBO formulation for QAOA. However, quantum advantage over classical solvers like Gurobi is NOT guaranteed and requires experimental validation.",
      "disclaimer": "This mapping is based on theoretical analysis. Practical quantum advantage depends on problem structure, data characteristics, quantum hardware quality (qubit count, error rates), and classical baseline optimizations. Experimental validation via PoC is REQUIRED before production deployment.",
      "references": [
        "Farhi et al. (2014) - A Quantum Approximate Optimization Algorithm",
        "Guerreschi & Matsuura (2019) - QAOA for MaxCut with noisy qubits",
        "Harrigan et al. (2021) - Quantum approximate optimization of non-planar graph problems on a planar superconducting processor"
      ]
    }
  ],
  "scores": {
    "quantum_feasibility_index": 0.64,
    "classification": "Quantum-Aware",
    "by_problem_type": {
      "combinatorial_optimization": 0.82,
      "monte_carlo_simulation": 0.31
    },
    "by_algorithm_family": {
      "QAOA": 0.78,
      "AMPLITUDE_ESTIMATION": 0.31
    },
    "scoring_formula_version": "1.0.0",
    "scoring_methodology": "See Appendix A"
  },
  "transition_plan": {
    "summary": {
      "recommendation": "PoC Candidate",
      "estimated_effort_months": "8-12",
      "estimated_cost_usd": "150000-300000",
      "probability_of_success": "50-70% (moderate uncertainty)"
    },
    "phases": [
      {
        "phase": 0,
        "name": "Baseline & Refactoring",
        "duration_weeks": "4-6",
        "cost_usd": "30000-50000",
        "tasks": [
          "Profile classical Gurobi performance (runtime, memory, solution quality)",
          "Refactor optimizer as standalone microservice",
          "Document current solution quality metrics"
        ]
      },
      {
        "phase": 1,
        "name": "Quantum PoC",
        "duration_weeks": "10-16",
        "cost_usd": "80000-150000",
        "tasks": [
          "Implement QUBO formulation",
          "Prototype QAOA on IBM Quantum / AWS Braket",
          "Validate on small instances (n=20-50)",
          "Compare solution quality vs. Gurobi"
        ]
      },
      {
        "phase": 2,
        "name": "Hybrid Integration",
        "duration_weeks": "8-12",
        "cost_usd": "100000-150000",
        "tasks": [
          "Build classical-quantum orchestration layer",
          "Integrate into production architecture",
          "A/B testing framework"
        ]
      }
    ],
    "baseline_profiling": {
      "recommended_tools": ["cProfile", "line_profiler"],
      "key_metrics": ["runtime_median_sec", "solution_quality_gap_percent", "convergence_iterations"],
      "benchmark_datasets": "Use production portfolio data (100-1000 assets)"
    },
    "risks": [
      "Quantum hardware may not be available at required scale (500 qubits) until 2026+",
      "NISQ noise may prevent any practical speedup",
      "Classical solver improvements (Gurobi updates) may outpace quantum progress",
      "Integration complexity may exceed budget"
    ]
  },
  "rule_hits": [
    {
      "rule_id": "P1_COMBINATORIAL_OPT_GUROBI",
      "rule_version": "1.2.0",
      "description": "Mixed-integer optimization using Gurobi solver",
      "files": ["src/optimizer.py"],
      "lines": [42, 67],
      "confidence": 0.92,
      "code_snippet": "model = gp.Model()\nx = model.addVars(len(assets), vtype=GRB.BINARY)"
    }
  ],
  "decision_trace": [
    {
      "step": 1,
      "action": "file_parsing",
      "result": "Parsed 12 Python files, 0 errors"
    },
    {
      "step": 2,
      "action": "feature_extraction",
      "result": "Detected imports: gurobipy, numpy, pandas"
    },
    {
      "step": 3,
      "action": "rule_evaluation",
      "result": "Rule P1_COMBINATORIAL_OPT_GUROBI fired (confidence 0.92)"
    },
    {
      "step": 4,
      "action": "workload_classification",
      "result": "Classified as combinatorial_optimization"
    },
    {
      "step": 5,
      "action": "quantum_mapping",
      "result": "Mapped to QAOA (maturity: near_term)"
    },
    {
      "step": 6,
      "action": "scoring",
      "result": "QAS=64.4, QFI=0.64 (Quantum-Aware)"
    }
  ],
  "false_positives_review": {
    "needs_review": [],
    "user_suppressed": [
      {
        "file": "tests/test_utils.py",
        "reason": "Test file, not production code",
        "suppression_method": ".qfa-ignore"
      }
    ]
  },
  "visualizations": {
    "heatmap_url": "/report/abc123/heatmap.png",
    "timeline_url": "/report/abc123/timeline.png",
    "portfolio_comparison_url": "/report/abc123/portfolio.png"
  }
}
```

### 8.3 Human-Readable Reports (Enhanced)

**Markdown Output**:

```markdown
# Quantum Feasibility Analysis Report

**Project**: payments-service
**Commit**: abc123
**Analysis Date**: 2025-11-18
**QFA Version**: 1.0.0 | Ruleset: 2024.11.18

---

## Executive Summary

**Quantum Feasibility Index (QFI)**: 64.4 / 100
**Classification**: **Quantum-Aware** (Selective quantum migration candidate)

**Key Findings**:
- ✅ 1 high-potential quantum opportunity identified
- ⚠️ Requires 500 qubits (not NISQ-feasible in 2025)
- 📅 Timeline: 2026-2028 (early fault-tolerant era)
- 💰 Estimated PoC Cost: $150K-$300K
- 🎯 Recommendation: **Monitor quantum hardware progress; prepare PoC in 2026**

---

## Detected Quantum Opportunities

### Opportunity #1: Portfolio Optimization (QAOA)

**Workload**: `optimize_portfolio` in `src/optimizer.py:42`
**Problem Type**: Combinatorial Optimization
**Quantum Algorithm**: QAOA (Quantum Approximate Optimization Algorithm)
**Maturity**: Near-term (2025-2028, experimental)

**Scores**:
- Algorithmic Fit: ⭐⭐⭐⭐ (4/5)
- Hardware Feasibility: ⭐⭐⭐ (3/5)
- Codebase Fit: ⭐⭐⭐⭐ (4/5)
- **QAS**: 64.4 / 100

**Resource Requirements**:
- **Qubits**: ~500 (1 per asset)
- **Circuit Depth**: ~2,500-25,000 gates (5-50 QAOA layers)
- **Error Correction**: Recommended for >100 qubits
- **NISQ-Feasible**: ❌ (requires 2026+ hardware)
- **Providers**: IBM Quantum (1000+ qubit roadmap), AWS Braket (future)

**Speedup Estimate**:
- **Type**: Problem-dependent
- **Classical**: O(2^n) brute force, O(n^3) Gurobi (approximate)
- **Quantum**: O(p × n) QAOA circuit evaluations
- **Speedup Factor**: 10-100× demonstrated on small instances (n<100)
- **Threshold**: N > 100 for potential advantage

**⚠️ Caveats**:
- Speedup is problem-dependent and NOT guaranteed
- NISQ-era noise limits practical speedup to ~10× at best
- Classical solvers (Gurobi) are highly optimized; quantum must beat these
- Requires experimental validation via PoC

**Code Context**:
```python
def optimize_portfolio(assets):
    model = gp.Model()
    x = model.addVars(len(assets), vtype=GRB.BINARY)
    model.optimize()
    return x
```
*File: src/optimizer.py, Lines: 42-67*

---

## Transition Plan

**Recommendation**: PoC Candidate (monitor hardware progress)

### Phase 0: Baseline & Refactoring (4-6 weeks, $30K-$50K)
- [ ] Profile Gurobi performance (runtime, solution quality)
- [ ] Refactor optimizer as standalone microservice
- [ ] Document current metrics

### Phase 1: Quantum PoC (10-16 weeks, $80K-$150K)
- [ ] Implement QUBO formulation
- [ ] Prototype QAOA on IBM Quantum / AWS Braket
- [ ] Validate on small instances (n=20-50)

### Phase 2: Hybrid Integration (8-12 weeks, $100K-$150K)
- [ ] Build classical-quantum orchestration
- [ ] Integrate into production architecture
- [ ] A/B testing framework

**Total Estimated Effort**: 8-12 months
**Total Estimated Cost**: $150K-$300K
**Probability of Success**: 50-70% (moderate uncertainty)

---

## Baseline Profiling Guidance

Before proceeding with quantum PoC, establish classical baseline:

**Recommended Tools**: cProfile, line_profiler
**Key Metrics**:
- Runtime (median, p95, p99)
- Solution quality gap (% from optimal)
- Convergence iterations

**Benchmark Datasets**: Use production portfolio data (100-1000 assets)

---

## Risks & Disclaimers

**Risks**:
- Quantum hardware may not be available at required scale (500 qubits) until 2026+
- NISQ noise may prevent any practical speedup
- Classical solver improvements may outpace quantum progress

**Disclaimer**:
> This analysis is based on theoretical research and current quantum computing capabilities. Practical quantum advantage is NOT guaranteed and depends on problem structure, quantum hardware quality, and classical baseline optimizations. Experimental validation via PoC is REQUIRED before production deployment.

---

## Appendix A: Scoring Methodology

### Quantum Applicability Score (QAS)

**Formula**: `QAS = Σ (Criterion_i × Weight_i) × Detection_Confidence`

**Criteria** (0-100 scale, weighted):
- Problem Size (20%): N >10^6 → 100; N <1000 → 0
- Complexity Class (25%): BQP ∖ BPP → 100; P → 0
- Parallelism (15%): Embarrassingly parallel → 100
- Eigenvalue Structure (15%): Linear algebra → 100
- Oracle Access (10%): Black-box evaluations → 100
- Optimization (10%): NP-hard → 100
- Bottleneck (5%): Performance-critical → 100

**Worked Example** (Portfolio Optimization):
- Problem Size: N=500 → 60 points
- Complexity Class: NP-hard → 100 points
- Parallelism: Parallelizable → 70 points
- Eigenvalue: N/A → 0 points
- Oracle: Matrix lookups → 80 points
- Optimization: NP-hard → 100 points
- Bottleneck: Critical → 90 points

Weighted: (60×0.2) + (100×0.25) + ... = 70 points
Confidence: 0.92 (Gurobi detected)
**QAS = 70 × 0.92 = 64.4**

---

**Generated by QFA v1.0.0 | Ruleset 2024.11.18**
```

**HTML Output** (NEW):

In addition to Markdown, QFA SHALL generate interactive HTML reports with:

1. **Interactive Heatmap**: Problem types × quantum algorithms
2. **Timeline Gantt Chart**: Transition plan phases
3. **Portfolio Comparison**: Multi-repo ranking (if applicable)
4. **Drill-Down Navigation**: Click workload → see code snippet + rule hits
5. **Exportable Charts**: PNG/SVG downloads for presentations

---

## 9. Non-Functional Requirements

### 9.1 Performance

* Must analyze a repo with up to:
  * 2,000 source files and ~500k LOC across supported languages.
* Target runtime:
  * **< 5 minutes** on a standard 4–8 core developer machine for the above size (tunable via parallelism).
  * **< 2 minutes** for incremental analysis (50 changed files, with caching).

* **NEW: Timeout Mechanisms**:
  * Per-file parsing timeout: 30 seconds (prevents pathological cases)
  * Total analysis timeout: 10 minutes (configurable via `--timeout`)

### 9.2 Scalability

* Language analyzer execution SHOULD be parallelizable across files and/or services.
* Service mode MUST support concurrent analyses (configurable worker pool).
* **NEW: Memory Limits**:
  * Maximum memory usage: 2GB per analysis (configurable)
  * Stream-based processing for large files (>10MB) to avoid loading entire file into memory

### 9.3 Reliability & Fault Tolerance

* Parsing failures for specific files MUST NOT abort the entire analysis; affected workloads should be marked with warnings.
* Network failures when using git URLs SHOULD be handled with clear error messages.
* **NEW: Graceful Degradation**:
  * If a language analyzer crashes, continue with remaining analyzers
  * Generate partial report with warnings about failed components
  * Retry failed analyses once before marking as failed

### 9.4 Security & Privacy

* Read-only analysis: tool MUST NOT modify source repositories.
* When run in hosted mode, repository contents MUST NOT be persisted longer than configured or required (default: 24 hours).
* Credentials (e.g., for private repos) MUST be handled via secure mechanisms and not logged.
* **NEW: Sandboxing** (Service Mode):
  * Language analyzers run in isolated Docker containers
  * No network access during analysis (prevents data exfiltration)
  * Static-only parsers preferred over native compilers (reduces code execution risk)

### 9.5 Extensibility

* New languages can be added via analyzers implementing the common interface.
* New rules can be added without modifying the core engine.
* Quantum algorithm families can be extended as the state of the art evolves.
* **NEW: Plugin Architecture**:
  * Third-party plugins for custom analyzers (e.g., domain-specific languages)
  * Plugin API: `LanguageAnalyzer` trait + registration mechanism

### 9.6 Compliance & Auditability

* All reports MUST include engine and ruleset versions.
* Decision traces MUST be stored or exportable for compliance/audit workflows.
* **NEW: Audit Logging**:
  * All analysis runs logged with: timestamp, user, repo, configuration, results summary
  * Logs exportable in JSON format for SIEM integration
  * Compliance with ISO 27001, SOC 2 audit requirements

### 9.7 Caching & Incremental Analysis (NEW)

**Caching Strategy**:

1. **File-Level Caching**:
   ```
   .qfa-cache/
     {file_hash}.json  → Cached analysis result
     metadata.json     → Cache metadata (timestamps, versions)
   ```

2. **Cache Invalidation**:
   - File content changes (detected via SHA-256 hash)
   - Rule version changes (ruleset update)
   - Engine version changes (major/minor upgrades)

3. **Benefits**:
   - **Incremental analysis**: Only re-analyze changed files in CI
   - **Faster iteration**: Cached results speed up development
   - **Resume failed analyses**: Restart from last successful checkpoint

4. **Performance Target**:
   - Incremental analysis reduces re-scan time by ≥70% for typical PRs

**Example**:

```bash
# First run: Full analysis (5 minutes)
qfa analyze ./src --cache-dir .qfa-cache/

# Second run: Incremental (2 changed files, 30 seconds)
qfa analyze ./src --cache-dir .qfa-cache/ --incremental
```

---

## 10. Architecture Overview (High-Level)

**Logical components:**

1. **CLI Frontend** – user-facing command-line interface.
2. **API Service (optional)** – HTTP interface for remote/automated usage.
3. **Repo Fetcher** – clones or reads local repos; applies ignore rules.
4. **Language Router** – identifies languages and dispatches files to analyzers.
5. **Language Analyzers** – per-language feature extraction and preliminary workload detection.
6. **Rules Engine** – applies declarative rules to IR and workload candidates.
7. **Scoring Engine** – aggregates scores and builds repo/portfolio metrics.
8. **Report Generator** – produces JSON/Markdown/HTML outputs.
9. **Config & Rules Store** – versioned storage for rule definitions and engine config.
10. **NEW: Cache Manager** – manages file-level caching for incremental analysis.
11. **NEW: Visualization Engine** – generates heatmaps, timelines, charts.

**Architecture Diagram**:

```
┌─────────────────────────────────────────────────────────────┐
│                        CLI / API                            │
└─────────────────────────┬───────────────────────────────────┘
                          │
                          ▼
                  ┌───────────────┐
                  │  Repo Fetcher │
                  │  (git/local)  │
                  └───────┬───────┘
                          │
                          ▼
                  ┌───────────────┐      ┌────────────────┐
                  │ Cache Manager │◄─────┤ .qfa-cache/    │
                  └───────┬───────┘      └────────────────┘
                          │
                          ▼
                  ┌───────────────┐
                  │ Language      │
                  │ Router        │
                  └───────┬───────┘
                          │
          ┌───────────────┼───────────────┐
          ▼               ▼               ▼
    ┌─────────┐     ┌─────────┐     ┌─────────┐
    │ Python  │     │   JS    │ ... │  C++    │
    │Analyzer │     │Analyzer │     │Analyzer │
    └────┬────┘     └────┬────┘     └────┬────┘
         └───────────────┼────────────────┘
                         │ (IR + Features)
                         ▼
                 ┌───────────────┐      ┌────────────────┐
                 │ Rules Engine  │◄─────┤ rules.yml      │
                 └───────┬───────┘      └────────────────┘
                         │ (Workloads + Mappings)
                         ▼
                 ┌───────────────┐
                 │ Scoring       │
                 │ Engine        │
                 └───────┬───────┘
                         │ (QAS, QFI)
                         ▼
          ┌──────────────┼──────────────┐
          ▼              ▼              ▼
    ┌─────────┐    ┌─────────┐    ┌─────────┐
    │  JSON   │    │Markdown │    │  HTML   │
    │ Report  │    │ Report  │    │Dashboard│
    └─────────┘    └─────────┘    └─────────┘
```

The implementation SHOULD support both:

* **Local single-binary mode** (CLI + embedded analyzers).
* **Service mode** (API with optional distributed analyzers in the future).

---

## 11. Telemetry & Observability

If enabled by configuration, QFA SHOULD collect anonymous or organization-scoped telemetry such as:

* Number of analyses run by language and workload type (no raw code or identifiers).
* Rule hit frequencies (to refine rules over time).
* Performance metrics: runtime, file counts, error rates.
* **NEW**: False positive rates (user-suppressed workloads).
* **NEW**: PoC conversion rates (how many identified opportunities led to actual PoCs).

Metrics SHOULD be exportable via:

* Prometheus endpoint (service mode).
* Structured logs for ingestion into external observability tools.
* **NEW**: Grafana dashboards (pre-built templates for QFA metrics).

Telemetry MUST be opt-in where required and configurable to meet privacy requirements.

**Privacy Controls**:
- Telemetry disabled by default for local CLI mode
- Opt-in via `--enable-telemetry` flag
- No source code or file paths transmitted (only aggregated statistics)
- User can audit telemetry data via `--dry-run-telemetry` flag

---

## 12. Rollout Plan & Milestones (Enhanced with Timeline Estimates)

### Phase 1 – MVP (Python-only)
**Duration**: 12-16 weeks
**Cost**: $200K-$300K (3-4 engineers)

**Deliverables**:
- Python-only analyzer (NumPy, SciPy, OR-Tools, Qiskit, etc.)
- Workload types: combinatorial optimization, Monte Carlo, linear algebra
- Basic rules engine (20+ patterns)
- Scoring engine (QAS, QFI)
- CLI + JSON/Markdown output
- Resource estimation (qubits, circuit depth)
- Speedup quantification

**Success Criteria**:
- ✅ Detect all reference quantum patterns (precision ≥85%)
- ✅ QAS correlation ≥0.90 with expert ratings
- ✅ Analyze 100K LOC Python repo in <5 minutes

### Phase 2 – Polyglot Expansion
**Duration**: 8-10 weeks per language (parallel development possible)
**Cost**: $150K-$250K per language

**Deliverables**:
- Add JavaScript/TypeScript and Java analyzers
- Expand rule catalog for these languages (15+ patterns each)
- Add GitHub Action integration
- HTML report generation with visualizations
- Incremental analysis + caching

**Success Criteria**:
- ✅ Multi-language repos analyzed correctly
- ✅ Incremental analysis ≥70% faster on PRs

### Phase 3 – Systems Languages
**Duration**: 10-14 weeks per language
**Cost**: $200K-$350K (C++ complexity)

**Deliverables**:
- Add Rust, Go, C++, C# analyzers
- Improve detection of simulation, PDE, and physics workloads
- Introduce API service mode
- False positive handling mechanisms
- Rule testing framework

**Success Criteria**:
- ✅ False positive rate <5%
- ✅ API service handles 10 concurrent analyses

### Phase 4 – Platform Integration
**Duration**: 8-12 weeks
**Cost**: $150K-$250K

**Deliverables**:
- Integrate with Q-CMM for org-level maturity inputs
- Connect reports into ArcQubit dashboards
- Add organization-specific rule packs and templates
- Domain-specific rule packs (finance, logistics, healthcare)
- Community rule contribution workflow

**Total Timeline**: 12-18 months for all 8 languages + platform integration
**Total Cost**: $1.2M-$2.0M (assuming 4-6 engineer team)

---

## 13. Open Questions & Risks (Enhanced)

### 13.1 Rule Precision vs. Recall

**Question**: How conservative should rules be in early versions to avoid over-claiming quantum applicability?

**Strategy**: Prioritize **high-precision, lower-recall** rules initially.
- Phase 1: Precision ≥90%, Recall ≥60% (conservative, fewer false positives)
- Phase 2: Precision ≥85%, Recall ≥75% (balanced)
- Phase 3: Precision ≥80%, Recall ≥85% (comprehensive)

**Mitigation**: User feedback loop to refine rules over time.

### 13.2 Library Coverage

**Question**: Which third-party libraries and frameworks should be prioritized per language?

**Strategy**: Curated and evolving catalog based on:
- Quantum computing research papers (which classical libraries are benchmarked?)
- Industry surveys (which optimization/simulation libraries are widely used?)
- Vertical-specific patterns (finance → Monte Carlo, logistics → routing, healthcare → scheduling)

**Deliverable**: Quarterly rule updates with new library coverage.

### 13.3 Domain-Specific Extensions

**Question**: Should there be domain packs (e.g., finance, logistics, healthcare) with specific rules and mappings?

**Answer**: Likely yes, but **not in MVP scope**.

**Phase 4 Deliverable**: Domain packs with:
- Finance: Monte Carlo (derivatives pricing), portfolio optimization
- Logistics: TSP/VRP, scheduling, warehouse optimization
- Healthcare: Drug discovery (VQE), patient scheduling, resource allocation

### 13.4 Hardware Feasibility Modeling

**Question**: To what extent should QFA track qubit counts, error rates, and concrete provider capabilities vs. staying abstract?

**Strategy**: Initial approach: **abstract maturity tiers** (near-term/mid-term/long-term); deeper provider modeling in Phase 4.

**Phase 4 Enhancement**: Provider-specific recommendations:
- "This workload requires 500 qubits → IBM Quantum Condor (1121 qubits, 2023) or later"
- Link to provider roadmaps (IBM, Google, IonQ, AWS Braket)

### 13.5 Source Code Sensitivity

**Question**: Some customers may prohibit any off-host analysis.

**Mitigation**: QFA must have a clear story for **fully local/offline use** with no telemetry.

**Deliverable**: Documentation section on air-gapped deployment (no internet, no telemetry, local rule files).

### 13.6 Rule Maintenance Burden (NEW)

**Risk**: Quantum computing evolves rapidly; stale rules will erode trust.

**Mitigation**:
- Quarterly rule review cycles
- Community contribution model (similar to Snyk, Semgrep)
- Automated rule testing against reference codebases
- Partnership with quantum computing research labs for early access to new algorithms

### 13.7 Over-Reliance on Static Analysis (NEW)

**Risk**: Some quantum opportunities are runtime-dependent (e.g., conditional logic based on problem size).

**Mitigation**:
- Add data flow analysis (track variable ranges)
- Provide "conditional applicability" annotations in reports
- Recommend dynamic profiling as follow-up step

### 13.8 Customer Expectation Management (NEW)

**Risk**: Customers may see high QFI score and expect immediate quantum speedup.

**Mitigation**:
- Include effort estimates in transition plans (6-12 months for PoC)
- Add cost projections ($10K-$100K/month quantum cloud credits)
- Provide case studies showing realistic timelines (e.g., "BMW's VQE PoC took 18 months")
- Explicit disclaimers on every quantum mapping (§5.5)

---

## 14. Appendix A: Scoring Methodology (Detailed)

### A.1 Quantum Applicability Score (QAS)

**Formula**:
```
QAS = Σ (Criterion_i × Weight_i) × Detection_Confidence

where:
- Criterion_i: Score for criterion i (0-100 scale)
- Weight_i: Weight for criterion i (sum to 1.0)
- Detection_Confidence: Confidence from feature extraction (0.0-1.0)
```

**Criteria** (7 total, weighted):

1. **Problem Size (Weight: 20%)**:
   ```
   Score = 100 × log10(N / 1000) / log10(1000)  for 1000 ≤ N ≤ 10^6
   Score = 100                                   for N > 10^6
   Score = 0                                     for N < 1000

   Example:
   - N = 100: Score = 0 (too small)
   - N = 10,000: Score = 50
   - N = 1,000,000: Score = 100
   ```

2. **Complexity Class (Weight: 25%)**:
   ```
   BQP ∖ BPP (quantum advantage proven):    100 points
   NP-hard (potential advantage):            80 points
   PSPACE-complete:                          60 points
   NP (decision problems):                   40 points
   P (polynomial-time):                      0 points
   ```

3. **Parallelism (Weight: 15%)**:
   ```
   Embarrassingly parallel:                 100 points
   Parallelizable with synchronization:     70 points
   Sequential with dependencies:            30 points
   Strictly sequential:                      0 points
   ```

4. **Eigenvalue Structure (Weight: 15%)**:
   ```
   Linear systems (Ax=b):                   100 points
   Eigenvalue problems:                      90 points
   Matrix operations (multiply, invert):    70 points
   No linear algebra:                        0 points
   ```

5. **Oracle Access (Weight: 10%)**:
   ```
   Black-box function evaluations:          100 points
   Database queries:                         80 points
   Hash functions:                           60 points
   No oracle pattern:                         0 points
   ```

6. **Optimization (Weight: 10%)**:
   ```
   NP-hard combinatorial (TSP, MaxCut):    100 points
   Constrained optimization (MILP):         80 points
   Unconstrained optimization:              50 points
   Convex optimization:                      0 points
   ```

7. **Bottleneck (Weight: 5%)**:
   ```
   Performance-critical (profiled):         100 points
   Slow (no profiling data):                 70 points
   Moderate performance:                     40 points
   Fast (not a bottleneck):                   0 points
   ```

**Worked Example**: See §5.6.1.

### A.2 Sensitivity Analysis

**Impact of Weight Changes**:

| Weight Change | QAS Change (avg) | Impact |
|---------------|------------------|--------|
| Problem Size ±10% | ±3.2 points | Moderate |
| Complexity Class ±10% | ±5.8 points | High |
| Parallelism ±10% | ±2.1 points | Low |

**Recommendation**: Complexity Class is most influential; ensure accurate detection.

### A.3 Quantum Feasibility Index (QFI)

**Formula**:
```
QFI = Σ (QAS_i × LOC_i) / Σ LOC_i

where:
- QAS_i: Quantum Applicability Score for workload i
- LOC_i: Lines of code in workload i
```

**Interpretation**: Weighted average QAS across entire codebase, giving more weight to larger workloads.

**Example**:
```
Workload 1: QAS=80, LOC=1000
Workload 2: QAS=40, LOC=500
Workload 3: QAS=90, LOC=200

QFI = (80×1000 + 40×500 + 90×200) / (1000 + 500 + 200)
    = (80000 + 20000 + 18000) / 1700
    = 118000 / 1700
    = 69.4
    → Classification: Quantum-Aware
```

---

## 15. Appendix B: Quantum Algorithm References

### B.1 Proven Quantum Speedups

| Algorithm | Year | Authors | Speedup | Application |
|-----------|------|---------|---------|-------------|
| **Shor's Algorithm** | 1994 | Peter Shor | Exponential | Integer factorization, discrete log |
| **Grover's Algorithm** | 1996 | Lov Grover | Quadratic (√N) | Unstructured search, SAT |
| **HHL Algorithm** | 2009 | Harrow, Hassidim, Lloyd | Exponential (caveats) | Linear systems Ax=b |
| **QAOA** | 2014 | Farhi, Goldstone, Gutmann | Problem-dependent | Combinatorial optimization |
| **VQE** | 2014 | Peruzzo et al. | Problem-dependent | Quantum chemistry, materials |
| **Amplitude Estimation** | 2000 | Brassard et al. | Quadratic | Monte Carlo, risk analysis |
| **Quantum Walk** | 2003 | Childs et al. | Problem-dependent | Graph traversal, element distinctness |

### B.2 Key References

1. Shor, P. W. (1994). "Algorithms for quantum computation: discrete logarithms and factoring." *Proceedings 35th Annual Symposium on Foundations of Computer Science*.

2. Grover, L. K. (1996). "A fast quantum mechanical algorithm for database search." *Proceedings of the 28th Annual ACM Symposium on Theory of Computing*.

3. Harrow, A. W., Hassidim, A., & Lloyd, S. (2009). "Quantum algorithm for linear systems of equations." *Physical Review Letters*, 103(15), 150502.

4. Farhi, E., Goldstone, J., & Gutmann, S. (2014). "A quantum approximate optimization algorithm." *arXiv preprint arXiv:1411.4028*.

5. Peruzzo, A., et al. (2014). "A variational eigenvalue solver on a photonic quantum processor." *Nature Communications*, 5(1), 1-7.

6. Montanaro, A. (2015). "Quantum speedup of Monte Carlo methods." *Proceedings of the Royal Society A*, 471(2181), 20150301.

---

## 16. Version History & Change Log

### v2.0 (2025-11-18) - Enhanced Edition

**Major Enhancements**:
- ✅ Added explicit resource estimation (qubits, circuit depth, error correction)
- ✅ Refined maturity levels (5 levels: production/near-term/mid-term/long-term/research)
- ✅ Documented scoring formulas with worked examples (Appendix A)
- ✅ Added false positive handling mechanisms (F13)
- ✅ Specified rule conflict resolution strategy (§7.2.1)
- ✅ Added code snippet context to report structure (§8.2, §8.3)
- ✅ Added effort/cost estimates to transition plans (§5.7)
- ✅ Added baseline profiling guidance (F12)
- ✅ Expanded C++ analyzer specification with limitations (§6.6)
- ✅ Added caching strategy for incremental analysis (§9.7)
- ✅ Added timeline estimates to rollout plan (§12)
- ✅ Enhanced visualizations (heatmaps, timelines, HTML dashboard)
- ✅ Added quantified speedup estimates (§5.5)
- ✅ Added explicit disclaimers and caveats (§5.5)
- ✅ Added sensitivity analysis for scoring weights (Appendix A.2)
- ✅ Added quantum algorithm references (Appendix B)

**Backward Compatibility**: 100% (all v1.0 features preserved)

### v1.0 (2025-11-18) - Initial PRD

**Original Features**:
- Multi-language support (8 languages)
- Workload classification (7 families)
- Quantum algorithm mapping (6+ families)
- Multi-dimensional scoring (algorithmic fit, hardware feasibility, codebase fit)
- Transition plan generation
- CLI, CI/CD, API integrations
- Rules engine with declarative YAML/JSON rules

---

**End of Document**

---

**Review Status**: ✅ Enhanced with recommendations from technical review (2025-11-18)
**Next Steps**: Begin Phase 1 implementation (Python MVP) - estimated 12-16 weeks
**Contact**: ArcQubit Engineering Team
**License**: Proprietary (ArcQubit, Inc.)
