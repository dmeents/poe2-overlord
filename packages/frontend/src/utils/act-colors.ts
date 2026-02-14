import { getThemeHexColor } from './theme-utils';

const ACT_TO_THEME: Record<string, string> = {
  'Act 1': 'verdant',
  'Act 2': 'arcane',
  'Act 3': 'spirit',
  'Act 4': 'molten',
  'Act 5': 'hex',
  Interlude: 'blood',
};

const DEFAULT_THEME = 'ash';

function getActTheme(actName: string): string {
  return ACT_TO_THEME[actName] ?? DEFAULT_THEME;
}

export function getActHexColor(actName: string): string {
  const theme = getActTheme(actName);
  return getThemeHexColor(`${theme}-500`);
}
