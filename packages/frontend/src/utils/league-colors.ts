import { getThemeHexColor } from '@poe2-overlord/theme';

const LEAGUE_TO_THEME: Record<string, string> = {
  Standard: 'arcane',
  'Rise of the Abyssal': 'verdant',
  'The Fate of the Vaal': 'blood',
};

const DEFAULT_THEME = 'ash';

function getLeagueTheme(league: string): string {
  return LEAGUE_TO_THEME[league] ?? DEFAULT_THEME;
}

export function getLeagueHexColor(league: string): string {
  const theme = getLeagueTheme(league);
  return getThemeHexColor(`${theme}-500`);
}
