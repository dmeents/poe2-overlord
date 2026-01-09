import type { EconomyType } from '@/types/economy';

/**
 * Icon/image URLs for each economy type
 * TODO: Replace placeholder URLs with actual icon URLs
 */
export const ECONOMY_TYPE_ICONS: Record<EconomyType, string> = {
  Currency:
    'https://web.poecdn.com/gen/image/WzI1LDE0LHsiZiI6IjJESXRlbXMvQ3VycmVuY3kvQ3VycmVuY3lNb2RWYWx1ZXMiLCJzY2FsZSI6MSwicmVhbG0iOiJwb2UyIn1d/2986e220b3/CurrencyModValues.png',
  Fragments:
    'https://web.poecdn.com/gen/image/WzI1LDE0LHsiZiI6IjJESXRlbXMvQ3VycmVuY3kvQnJlYWNoL0JyZWFjaHN0b25lIiwic2NhbGUiOjEsInJlYWxtIjoicG9lMiJ9XQ/d60587d724/Breachstone.png',
  Abyss:
    'https://web.poecdn.com/gen/image/WzI1LDE0LHsiZiI6IjJESXRlbXMvQ3VycmVuY3kvQWJ5c3NhbEV5ZVNvY2tldGFibGVzL1RlY3JvZHNHYXplIiwic2NhbGUiOjEsInJlYWxtIjoicG9lMiJ9XQ/ef2a9355b4/TecrodsGaze.png',
  UncutGems:
    'https://web.poecdn.com/gen/image/WzI1LDE0LHsiZiI6IjJESXRlbXMvR2Vtcy9VbmN1dFN1cHBvcnRHZW0iLCJzY2FsZSI6MSwicmVhbG0iOiJwb2UyIn1d/d1ffe1c951/UncutSupportGem.png',
  LineageSupportGems:
    'https://web.poecdn.com/gen/image/WzI1LDE0LHsiZiI6IjJESXRlbXMvR2Vtcy9OZXcvTmV3U3VwcG9ydC9MaW5lYWdlL1dpbGRzaGFyZHMiLCJzY2FsZSI6MSwicmVhbG0iOiJwb2UyIn1d/6d700adf17/Wildshards.png',
  Essences:
    'https://web.poecdn.com/gen/image/WzI1LDE0LHsiZiI6IjJESXRlbXMvQ3VycmVuY3kvRXNzZW5jZS9HcmVhdGVyQXR0cmlidXRlRXNzZW5jZSIsInNjYWxlIjoxLCJyZWFsbSI6InBvZTIifV0/8a8cb823af/GreaterAttributeEssence.png',
  SoulCores:
    'https://web.poecdn.com/gen/image/WzI1LDE0LHsiZiI6IjJESXRlbXMvQ3VycmVuY3kvU291bENvcmVzL0dyZWF0ZXJTb3VsQ29yZU1hbmEiLCJzY2FsZSI6MSwicmVhbG0iOiJwb2UyIn1d/1437190de2/GreaterSoulCoreMana.png',
  Idols:
    'https://web.poecdn.com/gen/image/WzI1LDE0LHsiZiI6IjJESXRlbXMvQ3VycmVuY3kvVG9ybWVudGVkU3Bpcml0U29ja2V0YWJsZXMvQXptZXJpU29ja2V0YWJsZU1vbmtleVNwZWNpYWwiLCJzY2FsZSI6MSwicmVhbG0iOiJwb2UyIn1d/8ffc9986a0/AzmeriSocketableMonkeySpecial.png',
  Runes:
    'https://web.poecdn.com/gen/image/WzI1LDE0LHsiZiI6IjJESXRlbXMvQ3VycmVuY3kvUnVuZXMvTGlnaHRuaW5nUnVuZSIsInNjYWxlIjoxLCJyZWFsbSI6InBvZTIifV0/98319b3998/LightningRune.png',
  Ritual:
    'https://web.poecdn.com/gen/image/WzI1LDE0LHsiZiI6IjJESXRlbXMvQ3VycmVuY3kvT21lbnMvVm9vZG9vT21lbnMzUmVkIiwic2NhbGUiOjEsInJlYWxtIjoicG9lMiJ9XQ/9cfdcc9e1a/VoodooOmens3Red.png',
  Expedition:
    'https://web.poecdn.com/gen/image/WzI1LDE0LHsiZiI6IjJESXRlbXMvQ3VycmVuY3kvRXhwZWRpdGlvbi9CYXJ0ZXJSZWZyZXNoQ3VycmVuY3kiLCJzY2FsZSI6MSwicmVhbG0iOiJwb2UyIn1d/8a4fe1f468/BarterRefreshCurrency.png',
  Delirium:
    'https://web.poecdn.com/gen/image/WzI1LDE0LHsiZiI6IjJESXRlbXMvQ3VycmVuY3kvRGlzdGlsbGVkRW1vdGlvbnMvRGlzdGlsbGVkUGFyYW5vaWEiLCJzY2FsZSI6MSwicmVhbG0iOiJwb2UyIn1d/279e807e8f/DistilledParanoia.png',
  Breach:
    'https://web.poecdn.com/gen/image/WzI1LDE0LHsiZiI6IjJESXRlbXMvQ3VycmVuY3kvQnJlYWNoL0JyZWFjaENhdGFseXN0TWFuYSIsInNjYWxlIjoxLCJyZWFsbSI6InBvZTIifV0/61d3a7a832/BreachCatalystMana.png',
};

/**
 * Display names for economy types (formatted for UI)
 */
export const ECONOMY_TYPE_LABELS: Record<EconomyType, string> = {
  Currency: 'Currency',
  Fragments: 'Fragments',
  Abyss: 'Abyss',
  UncutGems: 'Uncut Gems',
  LineageSupportGems: 'Lineage Support Gems',
  Essences: 'Essences',
  SoulCores: 'Soul Cores',
  Idols: 'Idols',
  Runes: 'Runes',
  Ritual: 'Ritual',
  Expedition: 'Expedition',
  Delirium: 'Delirium',
  Breach: 'Breach',
};

/**
 * All available economy types in display order
 */
export const ECONOMY_TYPES: EconomyType[] = [
  'Currency',
  'Fragments',
  'Abyss',
  'UncutGems',
  'LineageSupportGems',
  'Essences',
  'SoulCores',
  'Idols',
  'Runes',
  'Ritual',
  'Expedition',
  'Delirium',
  'Breach',
];
