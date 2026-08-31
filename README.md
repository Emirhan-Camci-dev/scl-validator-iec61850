# GridSCL-Validator

**High-Performance IEC 61850 SCL/SCD Validation, Interoperability & Semantic Diff Engine**

GridSCL-Validator (also known as SCLGuard-Core) is an enterprise-grade CLI & SDK engine designed for Protection Relay Manufacturers (ABB, Siemens, SEL, Schneider Electric), Substation System Integrators, and Conformance Test Laboratories. It deterministically parses, validates, and diffs `.SCD`, `.CID`, `.ICD`, `.IID`, and `.SED` files across **IEC 61850-6 Edition 1, Edition 2, and Edition 2.1** schemas with sub-50ms execution speed.

---

## ⚡ Quickstart

Integrate the SDK into your Rust project with just 3 lines of code:

```rust
let parser = SclParser::new();
let doc = parser.parse(std::fs::File::open("substation.scd")?)?;
let diff = DiffEngine::new().compare(&doc, &parser.parse(std::fs::File::open("revised.scd")?)?);
println!("Validation: {:?}, Changes: {:?}", SclValidator::new().validate(&doc).is_valid, diff);
```

---

## 🚀 Performance Benchmarks

GridSCL-Validator is built in Rust to provide zero-allocation streaming for massive XML payloads, ensuring determinism and avoiding memory leaks commonly seen in Java or C# alternatives.

| File Size | IED Count | Execution Time | Peak Memory | Status |
|-----------|-----------|----------------|-------------|--------|
| 10 MB     | ~20       | < 5 ms         | ~8 MB       | ✅ Pass |
| 50 MB     | ~100      | < 25 ms        | ~20 MB      | ✅ Pass |
| 100 MB    | ~200+     | **< 50 ms**    | ~35 MB      | ✅ Pass |

---

## ⚖️ Open-Core & Dual-Licensing

This repository uses an **Open-Core / Dual-Licensing** model to ensure sustainable development.

| Feature | Community Edition (AGPLv3) | Enterprise Pro Tier (Proprietary) |
|---------|---------------------------|-----------------------------------|
| **License** | Open Source (AGPLv3) | Proprietary Commercial B2B |
| **SCL Parsing** | Basic ICD/CID | Massive Multi-IED SCD (100MB+) |
| **XSD Validation** | Standard XML Syntax | Strict Ed.1 / Ed.2 / Ed.2.1 Rules |
| **Semantic Diff Engine** | ❌ No | ✅ AST-Level Structural Diff |
| **Inter-IED Binding Check**| ❌ No | ✅ GOOSE/SV Publisher-Subscriber Matrix |
| **Audit Reporting** | Console Summary | PDF / Markdown / JSON Conformance |
| **CI/CD Integration** | Basic | Advanced Headless Test-Bench |
| **Offline License Auth** | N/A | Ed25519 Cryptographic Verification |
| **Support** | Community / GitHub Issues | SLA & Dedicated Integration Support |

### 💎 Get Enterprise Tier (Test Labs & OEMs)

Ready for production? Unlock the Enterprise Pro Tier for your Conformance Test Laboratory or Engineering Team:

[![Buy Enterprise License on Polar.sh](https://polar.sh/assets/brand/polar-badge-dark.svg)](https://polar.sh/GridSCL-Validator/subscriptions)

---

## 🔒 Offline Ed25519 License Verification

For Enterprise clients operating in **air-gapped substation environments**, GridSCL-Validator Enterprise utilizes an on-device offline cryptographic license validator (`Ed25519`). 
This guarantees that your license key (JWT) is validated safely and securely without any network requests or telemetry.

### License Metadata:
- **Author:** Emirhan CAMCI
- **Contact:** [byemir@live.com](mailto:byemir@live.com)
- **Year:** 2026

## 🛠 Project Structure (Dual-Licensing Hygiene)

To strictly enforce licensing hygiene, the workspace is split into two isolated modules:
- `gridscl-core/`: Contains the AGPLv3 Open Source engine (parser, basic validator, types).
- `gridscl-enterprise/`: (Private Extension) Contains proprietary modules (AST diff, PDF reporter, Ed25519 license validator).

This guarantees that proprietary Enterprise features **never leak** into the Open Source repository while allowing seamless binary compilation.

---
© 2026 Emirhan CAMCI. All Rights Reserved.
