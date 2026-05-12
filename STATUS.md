# pippyzippy STATUS

**Current focus:** Phase 1 wrapper complete (ppmd-rust backend). Phase 2: native PPMd implementation.

| Piece | Status |
|---|---|
| encoder (wrapper) | ✅ (ppmd-rust backend) |
| decoder (wrapper) | ✅ (ppmd-rust backend) |
| round-trip test | ✅ |
| oracle (7zz round-trip) | ✅ (via 7zippy layer5) |
| streaming | ⬜ |
| decode bench | ⬜ |
| encode bench | ⬜ |
| fuzz | ⬜ |

**Phase 1 backend:** `ppmd-rust v1.4` (pure-Rust PPMd7 port, CC0/MIT-0 license).
**Phase 2:** Replace with pippyzippy's own native PPMd7 implementation.

Symbols: ⬜ not started, 🟡 in progress, ✅ done, ❌ blocked.
