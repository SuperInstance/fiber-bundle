# fiber-bundle

**A Rust library for fiber bundles and principal bundles** — the mathematical structures that generalize vector bundles, tangent bundles, and gauge theories. Provides constructions for total space, base space, fiber, structure group, and associated bundle formation.

## Why It Matters

Fiber bundles are the geometric language of modern physics and differential geometry. They describe:

- **Tangent bundles** (TM → M): the collection of all tangent spaces on a manifold. Essential for Lagrangian/Hamiltonian mechanics and general relativity.
- **Principal bundles** (P → M with fiber G): the geometric framework for gauge theories (Yang-Mills, electromagnetism, Standard Model). The structure group G is the symmetry group (U(1) for EM, SU(2)×U(1) for electroweak, SU(3) for QCD).
- **Hopf fibration** (S³ → S² with fiber S¹): the most famous non-trivial bundle, demonstrating that topology can be globally non-trivial even when locally trivial.
- **Frame bundles**: the principal bundle underlying all Riemannian geometry.

This crate models these structures as Rust types, enabling symbolic computation of bundle properties (dimensionality, principality, associated bundles) and serving as a teaching tool for differential geometry courses.

## How It Works

### Fiber Bundle Definition

A fiber bundle is a quadruple (E, B, F, G) where:

- **E** — Total space (what the bundle "is")
- **B** — Base space (what the bundle sits over)
- **F** — Fiber (what's attached at each point of B)
- **G** — Structure group (symmetries of the fiber, acting as transition functions)

Locally, E looks like B × F (cartesian product). Globally, it may be twisted — the transition functions g_αβ: U_α ∩ U_β → G determine how fibers are glued together across chart overlaps.

### Dimension Formula

$$\dim(E) = \dim(B) + \dim(F)$$

Example: Tangent bundle of S² has dim(TS²) = dim(S²) + dim(R²) = 2 + 2 = 4.

### Principality Check

A bundle is **principal** if the fiber equals the structure group (F = G) and G acts on itself freely and transitively (left multiplication).

$$\text{is\_principal}(F, G) = (F = G)$$

Examples:
- Hopf fibration: F = S¹, G = U(1) → Principal ✓
- Tangent bundle: F = R², G = GL(2,R) → Not principal (F ≠ G)
- Frame bundle: F = GL(n,R), G = GL(n,R) → Principal ✓

### Vector Bundle Check

A bundle is a **vector bundle** if the fiber is a vector space (R^n or C^n). This crate checks by inspecting the fiber string for 'R' or 'C' prefixes.

### Associated Bundle Construction

Given a principal G-bundle P → M and a representation ρ: G → Aut(F), the associated bundle is:

$$P \times_\rho F = (P \times F) / \sim, \quad (p \cdot g, f) \sim (p, \rho(g) \cdot f)$$

This is how vector bundles arise from principal bundles: the tangent bundle TM is associated to the frame bundle via the standard representation of GL(n,R) on R^n.

### Key Examples

| Bundle | Base | Fiber | Group | Principal? | Dim(E) |
|--------|------|-------|-------|-----------|--------|
| TS² | S² | R² | GL(2,R) | No | 4 |
| Hopf S³→S² | S² | S¹ | U(1) | Yes | 3 |
| Möbius strip | S¹ | R | Z₂ | No | 2 |
| Frame F(S²) | S² | GL(2,R) | GL(2,R) | Yes | 6 |

### Complexity Analysis

All operations are O(1) — they involve string comparisons and arithmetic on dimension integers. This is a symbolic/type-level library, not a numerical one.

## Quick Start

```rust
use fiber_bundle::FiberBundle;

// Tangent bundle of S²: TS² → S² with fiber R² and structure group GL(2,R)
let ts2 = FiberBundle::new("TS²", "S²", "R²", "GL(2,R)");
assert!(ts2.is_vector_bundle());      // Fiber is R² ✓
assert!(!ts2.is_principal());          // F ≠ G
assert_eq!(ts2.total_dim(2, 2), 4);   // dim(S²) + dim(R²)

// Hopf fibration: S³ → S² with fiber S¹ and structure group U(1)
let hopf = FiberBundle::new("S³", "S²", "S¹", "U(1)");
assert!(hopf.is_principal());          // F = G = U(1)... well, S¹ ≅ U(1)

// Associated bundle: construct a vector bundle from a principal bundle
let associated = hopf.associated("C");
```

## API

### `FiberBundle`
- `new(total: &str, base: &str, fiber: &str, group: &str) -> Self`
- `is_vector_bundle(&self) -> bool` — True if fiber starts with 'R' or 'C'
- `is_principal(&self) -> bool` — True if fiber equals structure group
- `total_dim(&self, base_dim: usize, fiber_dim: usize) -> usize` — dim(E) = dim(B) + dim(F)
- `associated(&self, new_fiber: &str) -> FiberBundle` — Construct associated bundle

## Architecture Notes

This crate provides the geometric framework for the γ + η = C conservation link:

- **γ** (gamma) = the global topology of the bundle (transition functions)
- **η** (eta) = the local trivializations (fiber coordinates)
- **C** (constant) = the total bundle E

The local-to-global relationship γ + η = C is the content of the Čech-de Rham theorem: local data (trivializations + transition functions) determines global topology (characteristic classes).

See the full architecture: [ARCHITECTURE.md](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md)

## References

1. Husemoller, D. (1994). *Fiber Bundles,* 3rd ed. Springer GTM 20. — Standard reference.
2. Kobayashi, S. & Nomizu, K. (1963). *Foundations of Differential Geometry,* Vol. I. Wiley. — Principal bundles and connections.
3. Nakahara, M. (2003). *Geometry, Topology and Physics,* 2nd ed. Taylor & Francis. — Physics-oriented introduction.
4. Steenrod, N. (1951). *The Topology of Fibre Bundles.* Princeton. — The founding text.
5. Atiyah, M.F. (1979). *Geometry on Yang-Mills Fields.* Scuola Normale Superiore, Pisa. — Gauge theory and bundles.

## License

MIT
