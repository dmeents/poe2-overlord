# PRD: Wiki Parsing Robustness System

## Context

This PRD captures the comprehensive architectural improvements needed for wiki parsing robustness. Issue #9 identified that current section parsing is brittle and assumes specific HTML structure.

**Origin**: Issue #9 analysis revealed need for multi-strategy parsing with fallbacks
**Scope**: Full wiki parsing architecture with strategies, fuzzy matching, and graceful degradation
**Priority**: HIGH (reliability improvement)
**Estimated Effort**: ~30 hours (4-5 working days)

## Problem Statement

Current wiki parsing issues:
1. Assumes linear DOM structure (h2/h3 → ul) - breaks with wrapper divs
2. Only looks for specific element names (hardcoded selectors)
3. Fragile section detection with case-insensitive substring matching
4. No fallback strategies when primary parsing fails
5. No HTML structure validation
6. Infobox parser validation is weak (false positives possible)

## Proposed Solution

### Multi-Strategy Parsing Architecture

**Core Components**:
1. `ParsingStrategy` trait - Interface for different parsing approaches
2. `SectionParsingStrategy` - Current approach (improved)
3. `DomTraversalStrategy` - Navigate DOM tree from heading
4. `RegexFallbackStrategy` - Text-based extraction when structure fails
5. `StrategyChain` - Orchestrates fallback sequence with confidence scores

### Key Features

1. **Strategy Pattern**: Try multiple CSS selectors/approaches for same data
2. **Fuzzy Heading Matching**: Handle typos and alternate headings
3. **Synonym Dictionary**: Map "Monsters" → "Creatures", "Enemies", etc.
4. **Confidence Scores**: Each strategy returns quality score
5. **Graceful Degradation**: Return partial data rather than nothing
6. **Observability**: Log which strategy succeeded

### Files to Create

- `parsers/strategies/mod.rs`
- `parsers/strategies/traits.rs`
- `parsers/strategies/section_strategy.rs`
- `parsers/strategies/dom_traversal_strategy.rs`
- `parsers/strategies/regex_fallback_strategy.rs`
- `parsers/strategies/chain.rs`

### Files to Modify

- `parsers/base.rs` - Add fuzzy matching, synonyms
- All individual parsers - Integrate strategy chains
- `parser.rs` - Add quality validation

### Dependencies

```toml
# Add to Cargo.toml
regex = "1.10"    # Fallback text extraction
strsim = "0.11"   # Fuzzy string matching
```

## Phased Rollout

1. **Phase 1**: Deploy strategy infrastructure (no behavior change)
2. **Phase 2**: Integrate MonstersParser first (pilot)
3. **Phase 3**: Roll out to other parsers one at a time
4. **Phase 4**: Add validation and telemetry

## Success Criteria

- Parser handles wrapper divs, ol lists, heading typos
- Strategy fallbacks work correctly
- Confidence scores reflect data quality
- Existing tests pass (backward compatible)
- Wiki scraping completes within 10 seconds (no regression)

## Risk Mitigation

- Conservative fuzzy matching thresholds
- Low confidence for regex strategy
- Comprehensive tests before each parser integration
- Easy rollback per parser

## Notes

This is a deferred architectural improvement. A focused fix for Issue #9 was implemented in Batch 5 with improved validation in base.rs. This full implementation should be scheduled as a separate sprint.

## References

- Issue #9: Wiki section parsing brittleness
- Related: Issue #10 (redirects), Issue #32-34 (other wiki issues)
- Original analysis: Implementation planner session 2026-01-11
