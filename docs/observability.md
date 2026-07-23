# Observability at the Dart ↔ Rust Boundary

[ภาษาไทย](th/observability.md)

## What to measure

For significant native operations capture:

- operation name
- correlation/request ID
- input/output size buckets
- total duration
- compute duration when separable
- transfer/serialization duration when separable
- outcome category
- cancellation count
- queue/concurrency saturation when relevant

## Logging ownership

Rust logs native/domain diagnostics. Dart logs user-flow and presentation context. Correlation IDs connect both sides without duplicating all context.

## Privacy

Log metadata, not raw payloads. Avoid user content, keys, tokens, full file paths, or binary dumps unless a dedicated secure diagnostic mode explicitly requires them.

## Performance budgets

Define budgets around user-visible outcomes: frame responsiveness, preview latency, import duration, memory peak. Native compute time alone is not the product metric.
