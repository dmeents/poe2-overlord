import type { CharacterClass } from '../types/character';

const CLASS_TO_THEME: Record<CharacterClass, string> = {
  Warrior: 'blood',
  Sorceress: 'arcane',
  Ranger: 'verdant',
  Huntress: 'molten',
  Monk: 'spirit',
  Mercenary: 'ember',
  Witch: 'hex',
  Druid: 'primal',
};

const DEFAULT_THEME = 'ash';

export function getClassTheme(characterClass: string): string {
  return CLASS_TO_THEME[characterClass as CharacterClass] ?? DEFAULT_THEME;
}

export function getClassTextColor(characterClass: string): string {
  return `text-${getClassTheme(characterClass)}-400`;
}

export function getClassBorderColor(characterClass: string): string {
  return `border-${getClassTheme(characterClass)}-500`;
}

export function getClassBgGradient(characterClass: string): string {
  const theme = getClassTheme(characterClass);
  return `from-${theme}-500/10 to-${theme}-600/5`;
}

export function getClassLevelColors(characterClass: string): {
  bg: string;
  border: string;
  text: string;
} {
  const theme = getClassTheme(characterClass);
  return {
    bg: `from-${theme}-500/20 to-${theme}-600/20`,
    border: `border-${theme}-500/30`,
    text: `text-${theme}-400`,
  };
}

export function getClassHexColor(characterClass: string): string {
  const theme = getClassTheme(characterClass);
  return getComputedStyle(document.documentElement).getPropertyValue(`--color-${theme}-500`).trim();
}

export function getClassSecondaryHexColor(characterClass: string): string {
  const theme = getClassTheme(characterClass);
  return getComputedStyle(document.documentElement).getPropertyValue(`--color-${theme}-600`).trim();
}
