const LEAGUE_TO_THEME: Record<string, string> = {
  Standard: 'arcane',
  'Rise of the Abyssal': 'verdant',
  'The Fate of the Vaal': 'blood',
};

const DEFAULT_THEME = 'ash';

export function getLeagueTheme(league: string): string {
  return LEAGUE_TO_THEME[league] ?? DEFAULT_THEME;
}

export function getLeagueHexColor(league: string): string {
  const theme = getLeagueTheme(league);
  return getComputedStyle(document.documentElement).getPropertyValue(`--color-${theme}-500`).trim();
}
