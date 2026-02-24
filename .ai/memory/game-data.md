# Game Data: Classes & Ascendencies

## Adding a New Ascendency

**Checklist (5 files, no DB changes needed — ascendency stored as TEXT):**

### 1. Backend Enum — `packages/backend/src/domain/character/models.rs`

Add variant to `Ascendency` enum (multi-word names use PascalCase variant + serde rename):
```rust
#[serde(rename = "Disciple of Varashta")]
DiscipleOfVarashta,
```

Add to the class's match arm in `is_valid_ascendency_for_class()`:
```rust
CharacterClass::Sorceress => matches!(
    ascendency,
    Ascendency::Stormweaver | Ascendency::Chronomancer | Ascendency::DiscipleOfVarashta
),
```

### 2. Backend Tests — `packages/backend/src/domain/character/models_test.rs`

Add assertion in `test_valid_<class>_ascendencies()`:
```rust
assert!(is_valid_ascendency_for_class(
    &Ascendency::DiscipleOfVarashta,
    &CharacterClass::Sorceress
));
```

### 3. Frontend Types — `packages/frontend/src/types/character.ts`

Add to `ALL_ASCENDENCIES` array (order matters — group by class):
```ts
'Disciple of Varashta',
```

Add to `ASCENDENCIES_BY_CLASS` for the relevant class:
```ts
Sorceress: ['Stormweaver', 'Chronomancer', 'Disciple of Varashta'],
```

**Note:** `ASCENDENCY_IMAGES` in `ascendency-assets.ts` uses `Record<Ascendency, string | null>`,
which means TypeScript will error at build time if any ascendency is missing from the map. This
enforces completeness automatically — `pnpm typecheck` will catch omissions.

### 4. Frontend Asset Map — `packages/frontend/src/utils/ascendency-assets.ts`

Add import (use snake_case filename):
```ts
import discipleOfVarashtaImage from '../assets/ascendencies/disciple_of_varashta.webp';
```

Add to `ASCENDENCY_IMAGES` record:
```ts
'Disciple of Varashta': discipleOfVarashtaImage,
```

### 5. Image Asset

Place image at: `packages/frontend/src/assets/ascendencies/<snake_case_name>.webp`
Supported formats: `.webp`, `.jpeg`, `.png` (webp preferred).

### Verification
```bash
pnpm test:backend   # confirms Rust tests pass
pnpm typecheck      # TS Record<Ascendency,...> enforces completeness
```

---

## Adding a New Class

**Checklist (6 files, no DB changes needed — class stored as TEXT):**

### 1. Backend Enum — `packages/backend/src/domain/character/models.rs`

Add variant to `CharacterClass` enum:
```rust
#[serde(rename = "ClassName")]
ClassName,
```

Add a new match arm in `is_valid_ascendency_for_class()` listing all ascendencies for the class:
```rust
CharacterClass::ClassName => matches!(
    ascendency,
    Ascendency::FirstAscendency | Ascendency::SecondAscendency
),
```

### 2. Backend Tests — `packages/backend/src/domain/character/models_test.rs`

Add a new test function `test_valid_<classname>_ascendencies()` asserting each valid ascendency
and at least one invalid ascendency.

### 3. Frontend Types — `packages/frontend/src/types/character.ts`

Add to `CHARACTER_CLASSES` array:
```ts
export const CHARACTER_CLASSES = [
  // ...existing...
  'ClassName',
] as const;
```

Add all ascendencies for the new class first to `ALL_ASCENDENCIES`, then add the class entry to
`ASCENDENCIES_BY_CLASS`:
```ts
ClassName: ['FirstAscendency', 'SecondAscendency'],
```

### 4. Frontend Class Colors — `packages/frontend/src/utils/class-colors.ts`

Add to `CLASS_TO_THEME` map (choose from existing theme colors or add new tokens to theme package):
```ts
ClassName: 'ember',  // pick a theme color token
```

Available theme color tokens: `blood`, `arcane`, `verdant`, `molten`, `spirit`, `ember`, `hex`, `primal`, `stone`, `ash`, `bone`.

If a new color is needed, add it to `packages/theme/src/css/tokens.css` under `@theme`.

### 5. Frontend Asset Map — `packages/frontend/src/utils/ascendency-assets.ts`

Add imports and entries for all new ascendencies (same as ascendency checklist above).

### 6. Image Assets

Place images at: `packages/frontend/src/assets/ascendencies/<snake_case_name>.webp`

### Verification
```bash
pnpm test:backend   # confirms Rust enum + match arm coverage
pnpm typecheck      # Record<CharacterClass,...> and Record<Ascendency,...> enforce completeness
```

---

## Current Class/Ascendency Map

| Class      | Ascendencies                                              | Theme Color |
|------------|-----------------------------------------------------------|-------------|
| Warrior    | Titan, Warbringer, Smith of Katava                        | `blood`     |
| Sorceress  | Stormweaver, Chronomancer, Disciple of Varashta           | `arcane`    |
| Ranger     | Deadeye, Pathfinder                                       | `verdant`   |
| Huntress   | Ritualist, Amazon                                         | `molten`    |
| Monk       | Invoker, Acolyte of Chayula                               | `spirit`    |
| Mercenary  | Gemling Legionnaire, Tactician, Witchhunter               | `ember`     |
| Witch      | Blood Mage, Infernalist, Lich                             | `hex`       |
| Druid      | Shaman, Oracle                                            | `primal`    |

**Keep this table updated** whenever a class or ascendency is added.

---

## Key Locations

| Concern                  | File                                                              |
|--------------------------|-------------------------------------------------------------------|
| Backend enums + validation | `packages/backend/src/domain/character/models.rs`               |
| Backend tests            | `packages/backend/src/domain/character/models_test.rs`           |
| Frontend types           | `packages/frontend/src/types/character.ts`                       |
| Ascendency images map    | `packages/frontend/src/utils/ascendency-assets.ts`               |
| Class theme colors       | `packages/frontend/src/utils/class-colors.ts`                    |
| Image assets             | `packages/frontend/src/assets/ascendencies/`                     |
| Theme color tokens       | `packages/theme/src/css/tokens.css`                              |
