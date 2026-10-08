//! The plugins this site presents.

/// Where a plugin stands.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Availability {
    /// Released, with its documentation at this path on this domain, which is also the name of
    /// the product's documentation repository.
    Available {
        /// Site-absolute path of the documentation, e.g. `/gantry/`.
        docs: &'static str,
    },
    /// Announced but not released.
    ComingSoon,
}

/// One plugin.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Product {
    /// The product name, as the store and the docs use it.
    pub name: &'static str,
    /// One or two sentences for a card.
    pub summary: &'static str,
    /// Whether it is out yet.
    pub availability: Availability,
}

/// Every plugin, in the order the site shows them.
pub const PRODUCTS: &[Product] = &[
    Product {
        name: "Gantry",
        summary: "Experiences composed into a declared dependency tree and activated in observable \
                  stages, with game feature plugins reference counted across swaps. Shipped content \
                  and mods travel the same path, and every load is a named scope you can watch.",
        availability: Availability::Available { docs: "/gantry/" },
    },
    Product {
        name: "TerraVoxel",
        summary: "Planet-scale voxel terrain for Unreal Engine: spherical bodies with streaming, \
                  editable, collidable terrain.",
        availability: Availability::ComingSoon,
    },
    Product {
        name: "Firmament",
        summary: "A multi-frame star system to scale inside Unreal's world bounds: bodies on \
                  Keplerian rails, an epoch clock and exact rotating-frame physics, lit by one \
                  directional light that follows the world frame.",
        availability: Availability::ComingSoon,
    },
];
