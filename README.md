# Fiber Bundle

A **fiber bundle** is a topological structure consisting of a **total space** E, a **base space** B, a **fiber** F, and a projection π: E → B such that every point of B has a neighborhood U where π⁻¹(U) is homeomorphic to U × F (local triviality). This crate models fiber bundles, their structure groups, and associated bundle constructions.

## Why It Matters

Fiber bundles are the language of modern physics and geometry. The **tangent bundle** TM describes velocity spaces on manifolds — fundamental to Lagrangian and Hamiltonian mechanics. **Principal G-bundles** underpin gauge theory: electromagnetism is a U(1)-bundle, the Standard Model uses SU(3) × SU(2) × U(1). The **Hopf fibration** S³ → S² with fiber S¹ is the canonical example of nontrivial topology, demonstrating that S³ is not globally S² × S¹. Understanding bundles is prerequisite to studying characteristic classes, connections (gauge fields), and curvature — the mathematical foundations of general relativity and quantum field theory.

## How It Works

### Bundle Definition

A fiber bundle (E, B, F, G) consists of:
- **E** — total space
- **B** — base space
- **F** — typical fiber
- **G** — structure group acting on F
- **π: E → B** — continuous projection, locally trivial

```
F → E
    ↓ π
    B
```

### Classification

- **Vector bundle**: fiber is a vector space (ℝⁿ or ℂⁿ)
- **Principal bundle**: fiber equals structure group (G acts on itself)
- **Trivial bundle**: E ≅ B × F globally

### Dimension Formula

For a bundle with base dimension `m` and fiber dimension `k`:

$$\dim(E) = \dim(B) + \dim(F) = m + k$$

### Associated Bundles

Given a principal G-bundle P → B and a left G-space F, the associated bundle is P ×_G F → B. This construction generates vector bundles from principal bundles — the tangent bundle is associated to the frame bundle via the standard GL(n,ℝ) action on ℝⁿ.

### Key Examples

| Bundle | Base | Fiber | Group |
|--------|------|-------|-------|
| TS² (tangent) | S² | ℝ² | GL(2,ℝ) |
| Hopf fibration | S² | S¹ | U(1) |
| Möbius strip | S¹ | [0,1] | ℤ/2 |

## Quick Start

```rust
// Tangent bundle of S²
let ts2 = FiberBundle::new("TS²", "S²", "R²", "GL(2,R)");
assert!(ts2.is_vector_bundle());
assert_eq!(ts2.total_dim(2, 2), 4);

// Hopf fibration S³ → S² with fiber S¹
let hopf = FiberBundle::new("S³", "S²", "S¹", "U(1)");
assert!(hopf.is_principal()); // fiber == structure group

// Associated bundle construction
let associated = ts2.associated("C²");
```

## API

| Type / Function | Description |
|----------------|-------------|
| `FiberBundle` | Struct: `total_space`, `base_space`, `fiber`, `structure_group` |
| `FiberBundle::new` | Constructor |
| `is_vector_bundle()` | True if fiber is ℝⁿ or ℂⁿ |
| `is_principal()` | True if fiber equals structure group |
| `total_dim(base_dim, fiber_dim)` | Dimension of total space |
| `associated(new_fiber)` | Associated bundle construction |

## Architecture Notes

Part of the **SuperInstance** differential geometry toolkit. Fiber bundles model the mathematical structures underlying Fleet's spatial indexing and physics simulations. This contributes to **γ + η = C**: γ (geometric correctness) and η (clean algebraic manipulation) combine for correct topological computation.

See [ARCHITECTURE.md](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md) for the full system design.

## References

1. Husemöller, D. *Fibre Bundles*, 3rd ed., Springer GTM 20, 1994.
2. Steenrod, N. *The Topology of Fibre Bundles*. Princeton UP, 1951.
3. Nakahara, M. *Geometry, Topology and Physics*, 2nd ed., IOP Publishing, 2003.

## License

MIT
