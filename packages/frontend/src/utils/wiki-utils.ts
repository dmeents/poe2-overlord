import { open } from '@tauri-apps/plugin-shell';

/**
 * Converts an item name to a capitalized snake case format for wiki URL
 * @param itemName - The name of the item to convert
 * @returns The formatted item name for the wiki URL
 */
const getWikiUrl = (itemName: string): string => {
  const capitalizedSnakeCase = itemName
    .replace(/\s+/g, '_') // Replace spaces with underscores
    .replace(/[^a-zA-Z0-9_'-.]/g, '') // Remove special characters except underscores, apostrophes, hyphens, and periods
    .split('_')
    .map(word => word.charAt(0).toUpperCase() + word.slice(1).toLowerCase())
    .join('_');

  return `https://www.poe2wiki.net/wiki/${capitalizedSnakeCase}`;
};

/**
 * Opens a wiki page for the given item name
 * @param itemName - The name of the item to open the wiki page for
 */
export const handleWikiClick = async (itemName: string): Promise<void> => {
  const url = getWikiUrl(itemName);
  try {
    await open(url);
  } catch (error) {
    console.error('Failed to open wiki link:', error);
  }
};
