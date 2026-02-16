import { getThemeHexColor } from '@poe2-overlord/theme';
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

export function getClassHexColor(characterClass: string): string {
  const theme = getClassTheme(characterClass);
  return getThemeHexColor(`${theme}-500`);
}
