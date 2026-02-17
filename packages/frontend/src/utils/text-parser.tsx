import React from 'react';
import type { StepLink } from '../types/walkthrough';

interface ParsedTextProps {
  text: string;
  links: StepLink[];
  onLinkClick: (link: StepLink) => void;
}

/**
 * Parses text and converts link items into clickable links
 * @param text - The text to parse
 * @param links - Array of links with text and URL
 * @param onLinkClick - Callback function when a link is clicked
 * @returns Array of React elements representing the parsed text with links
 */
const parseTextWithLinks = (
  text: string,
  links: StepLink[],
  onLinkClick: (link: StepLink) => void,
): React.ReactNode[] => {
  if (!links.length) {
    return [text];
  }

  // Create a regex pattern that matches any of the link texts
  // Sort by length (longest first) to avoid partial matches
  const sortedLinks = [...links].sort((a, b) => b.text.length - a.text.length);
  const escapedItems = sortedLinks.map(link => link.text.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'));
  const pattern = new RegExp(`\\b(${escapedItems.join('|')})\\b`, 'gi');

  const result: React.ReactNode[] = [];
  let lastIndex = 0;
  let match: RegExpExecArray | null;

  // Reset regex lastIndex to ensure we start from the beginning
  pattern.lastIndex = 0;

  // biome-ignore lint/suspicious/noAssignInExpressions: Standard regex exec loop pattern
  while ((match = pattern.exec(text)) !== null) {
    // Add text before the match
    if (match.index > lastIndex) {
      result.push(text.slice(lastIndex, match.index));
    }

    // Find the exact link that matched
    const link = sortedLinks.find(l => l.text.toLowerCase() === match?.[0].toLowerCase());

    if (link) {
      result.push(
        React.createElement(
          'button',
          {
            key: `link-${match.index}`,
            type: 'button',
            onClick: (e: React.MouseEvent) => {
              e.stopPropagation();
              onLinkClick(link);
            },
            className:
              'text-stone-300 hover:text-stone-200 underline decoration-ember-500/50 hover:decoration-ember-400 cursor-pointer',
          },
          match[0],
        ),
      );
    } else {
      // Fallback if no link found
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
 * Component that renders parsed text with clickable links
 */
export function ParsedText({ text, links, onLinkClick }: ParsedTextProps): React.JSX.Element {
  const parsedElements = parseTextWithLinks(text, links, onLinkClick);

  return (
    <span>
      {parsedElements.map((element, index) => (
        // biome-ignore lint/suspicious/noArrayIndexKey: Parsed text elements lack stable unique identifiers
        <React.Fragment key={index}>{element}</React.Fragment>
      ))}
    </span>
  );
}
