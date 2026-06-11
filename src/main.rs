/// Fiber bundles and principal bundles
/// Local triviality, transition functions, structure groups

#[derive(Debug, Clone)]
struct FiberBundle {
    total_space: String,
    base_space: String,
    fiber: String,
    structure_group: String,
}

impl FiberBundle {
    fn new(total: &str, base: &str, fiber: &str, group: &str) -> Self {
        Self {
            total_space: total.to_string(),
            base_space: base.to_string(),
            fiber: fiber.to_string(),
            structure_group: group.to_string(),
        }
    }

    fn is_vector_bundle(&self) -> bool {
        self.fiber.starts_with('R') || self.fiber.starts_with('C')
    }

    fn is_principal(&self) -> bool {
        self.fiber == self.structure_group
    }

    /// Dimension of total space = dim(base) + dim(fiber)
    fn total_dim(&self, base_dim: usize, fiber_dim: usize) -> usize {
        base_dim + fiber_dim
    }

    /// Associated bundle construction
    fn associated(&self, new_fiber: &str) -> FiberBundle {
        FiberBundle::new(
            &format!("Assoc({})", self.total_space),
            &self.base_space,
            new_fiber,
            &self.structure_group,
        )
    }
}

fn main() {
    // Tangent bundle of S^2
    let ts2 = FiberBundle::new("TS²", "S²", "R²", "GL(2,R)");
    println!("Bundle: {} → {} with fiber {}", ts2.total_space, ts2.base_space, ts2.fiber);
    println!("Vector bundle: {}", ts2.is_vector_bundle());
    println!("Total dim(2-manifold): {}", ts2.total_dim(2, 2));

    // Hopf fibration: S^3 → S^2 with fiber S^1
    let hopf = FiberBundle::new("S³", "S²", "S¹", "U(1)");
    println!("\nHopf: {} → {} with fiber {}", hopf.total_space, hopf.base_space, hopf.fiber);
    println!("Principal: {}", hopf.is_principal());
}
