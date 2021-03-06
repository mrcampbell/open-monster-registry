# The Open Monster Registry

Having grown up loving certain RPG's with monsters resembling water tortoises, fire lizards, and poisonous vegetary dinosaurs, and having played many of the official and fan made games - but at the end of the day, these beloved characters are Intellectual Property of a Freaky Game company.

## Motivation

Out of respect for them and what they created, as well as the law, along with the increasingly controversial releases as of late, I think it's time we branch out as an open community to see what we can create.

There are dozens of "Fakemon" games, each with their unique, but ultimately "single title locked" characters, I think the community could benefit from the following things:

1. **Open-Sourced Monsters, Attacks, Items, and Facets** in a simple, consumable form (JSON, CSV, SQL Migrations and Statements)
2. **Embeddable Game Logic and Mechanics** that can be used in any framework or ecosystem (compiled to WebAssembly or C, or a Dockerized REST/GraphQL API)
3. **Development, Deployment, and Distribution Options**.  The current options are ROM (extremely difficult), executable/binaries (not safe for distribution), Game Engines (Unity and Unreal Engine, which are fantastic, but not accessible for the uninitiated).

## Open-Sourced Monsters, Attacks Items, and Facets

We could create a first, "Alpha" generation of Monsters.  Perhaps an arbitrary amount, like 151.  These Monsters would be free use, having agreed upon stats, images, and a list of (recommended) Attacks, and other Facets such as personality, abilities, etc.

Other "generations" ("Beta", "Gamma", etc.) could be released upon options available and demand.

An agreed upon pool of Moves, Abilities, items that are collectable, consumable, and equipable, and other attributes inpsired by successful games could be incorporated.


## Embeddable Game Logic and Mechanics

Some popular mechanics in games include Elemental Advantage, synergetic strategies, damage calculation, status conditions of varying volitility, inventory systems, merchant interfaces (currency, purchasing, selling of items), experience gaining, leveling up, evolution, and other aspects of growth.  These could all, or portions, be bundled in a program that compiles to WebAssembly or C, or callable via a REST/GraphQL API in a Docker container.  

Each format has its benefits; Embeddable, specifically WebAssembly, solutions are able to be used in any framework that works with wasm, including Browser-only applications.  REST/GraphQL API's can be used by experienced and learning developers, but also used in No-code or Less-code applications.  This is not an all inclusive list, and with strategic planning, we could go both directions simultaneously.  

## Development, Deployment, and Distribution Options

Where silo'd solutions have silo'd distrubution channels (executables, ROM, WebApp), we could enable deployment to any of the mainstream options from a single codebase, depending on the project's priority.  

Rather than lists of obscure tools, we could create a WebUI Interface, where it's nearly drag and drop, with the allowance of pluggable custom code (Javascript for example), and wrap around already existing opensource libraries to create installable apps, ROM formatted, or distribute through WebApp.

All build systems should be available via Docker/Docker-Compose.  None of this "install version x.y.z of this, and ensure that N is configured as such...", if it's not in the Dockerfile, it's not getting into the `main` branch.

That goes with tests as well.  The threshold is flexible, but starting at 80% test coverage.  

The first approach is going to be exposing a single function that takes in a GraphQL request (mutations and queries) and returns a string that can be parsed into JSON.  There are a few reasons for this, but mainly - it creates a clean, single entrypoint API that is manageable, rather than maintaining multiple functions.  If we create linker code to different languages, this reduces that workload (and liability) to a single API.  And GraphQL, like REST-ful API's, are not limited to HTTP uses.

# Legal

We need to ensure that this can be used for fun, and for profit, and do not collide with any Intellectual Property or other Trademark/Copyright licensing.

## Who am I?

I'm someone who has studied the algorithms, data structures, and built out multiple "Fakemon" games to varying completion and detail in 7-8 languages, and dozens of frameworks.  

And I'm looking forward to figuring this out!

# Basic explanation of where we're at so far: 

Here is a basic explanation:

1. https://github.com/mrcampbell/open-monster-registry/blob/main/compiler/src/main.rs.
  - This is the compiler that reads the input files, and generates Rust code
  - Here is actual generated output: https://github.com/mrcampbell/open-monster-registry/blob/main/engine/src/generated/monsters.rs
  - And the super basic input: https://github.com/mrcampbell/open-monster-registry/blob/main/data/pokemon/species/1.json
2. The validator (https://github.com/mrcampbell/open-monster-registry/tree/main/validator) probably will be deleted, and I???ll validate at ???compile time???
3. `smoketest` is where I???m actually importing it and implementing it: https://github.com/mrcampbell/open-monster-registry/blob/main/smoketest/index.js
  - To smoke test, you run this cd smoketest && yarn upgrade omr-engine && yarn start.
4. And last, but most certainly not least, engine is the only part that is exported: https://github.com/mrcampbell/open-monster-registry/tree/main/engine
  - Right now, I???m just replicating Pokemon. I???ll build in forking logic if this whimsical project even makes it that far haha
