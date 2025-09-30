import React from 'react';

interface ParsedTextProps {
  text: string;
  wikiItems: string[];
  onWikiClick: (itemName: string) => void;
}

/**
 * Parses text and converts wiki items into clickable links
 * @param text - The text to parse
 * @param wikiItems - Array of wiki item names to convert to links
 * @param onWikiClick - Callback function when a wiki link is clicked
 * @returns Array of React elements representing the parsed text with links
 */
const parseTextWithWikiLinks = (
  text: string,
  wikiItems: string[],
  onWikiClick: (itemName: string) => void
): React.ReactNode[] => {
  if (!wikiItems.length) {
    return [text];
  }

  // Create a regex pattern that matches any of the wiki items
  // Sort by length (longest first) to avoid partial matches
  const sortedWikiItems = [...wikiItems].sort((a, b) => b.length - a.length);
  const escapedItems = sortedWikiItems.map(item =>
    item.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')
  );
  const pattern = new RegExp(`\\b(${escapedItems.join('|')})\\b`, 'gi');

  const result: React.ReactNode[] = [];
  let lastIndex = 0;
  let match: RegExpExecArray | null;

  // Reset regex lastIndex to ensure we start from the beginning
  pattern.lastIndex = 0;

  while ((match = pattern.exec(text)) !== null) {
    // Add text before the match
    if (match.index > lastIndex) {
      result.push(text.slice(lastIndex, match.index));
    }

    // Find the exact wiki item that matched
    const wikiItem = sortedWikiItems.find(
      item => item.toLowerCase() === match![0].toLowerCase()
    );

    if (wikiItem) {
      result.push(
        React.createElement(
          'button',
          {
            key: `wiki-${match.index}`,
            onClick: (e: React.MouseEvent) => {
              e.stopPropagation();
              onWikiClick(wikiItem);
            },
            className:
              'text-zinc-300 hover:text-zinc-200 underline decoration-blue-400 hover:decoration-blue-300 cursor-pointer',
          },
          match[0]
        )
      );
    } else {
      // Fallback if no wiki item found
      result.push(match[0]);
    }

    lastIndex = match.index + match[0].length;
  }

  // Add remaining text after the last match
  if (lastIndex < text.length) {
    result.push(text.slice(lastIndex));
  }

  return result;
};

/**
 * Component that renders parsed text with wiki links
 */
export const ParsedText: React.FC<ParsedTextProps> = ({
  text,
  wikiItems,
  onWikiClick,
}) => {
  const parsedElements = parseTextWithWikiLinks(text, wikiItems, onWikiClick);

  return (
    <span>
      {parsedElements.map((element, index) => (
        <React.Fragment key={index}>{element}</React.Fragment>
      ))}
    </span>
  );
};
