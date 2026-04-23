/**
 * Converts DDSFile art paths from ItemVisualIdentity into web.poecdn.com image URLs.
 *
 * ItemVisualIdentity stores paths like:
 *   "Art/2DItems/Armours/Boots/BootsStr1.dds"
 *
 * The GGG CDN serves PNG versions at:
 *   "https://web.poecdn.com/image/Art/2DItems/Armours/Boots/BootsStr1.png"
 *
 * These URLs work without any auth headers and are already used in the economy domain.
 */

const POECDN_BASE = 'https://web.poecdn.com/image';

/**
 * @param {string | null | undefined} artPath
 * @returns {string | null}
 */
export function artPathToImageUrl(artPath) {
  if (!artPath) return null;

  // Normalise separators and strip leading slash
  const normalised = artPath.replace(/\\/g, '/').replace(/^\//, '');

  // Replace .dds (or .DDS) extension with .png; keep path otherwise
  const withPng = normalised.replace(/\.dds$/i, '.png');

  return `${POECDN_BASE}/${withPng}`;
}

/**
 * Finds the best art path from an ItemVisualIdentity row.
 * Prefers DDSFile (the 2D inventory art). Falls back to null if absent.
 *
 * @param {Record<string, unknown>} visualIdentityRow
 * @returns {string | null}
 */
export function imageUrlFromVisualIdentity(visualIdentityRow) {
  const ddsFile = visualIdentityRow?.DDSFile ?? visualIdentityRow?.dds_file;
  return artPathToImageUrl(typeof ddsFile === 'string' ? ddsFile : null);
}
