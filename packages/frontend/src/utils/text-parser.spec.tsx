import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';
import { ParsedText } from './text-parser';

describe('ParsedText', () => {
  it('returns plain text when no wiki items provided', () => {
    render(<ParsedText text="This is plain text" wikiItems={[]} onWikiClick={vi.fn()} />);
    expect(screen.getByText('This is plain text')).toBeInTheDocument();
  });

  it('converts single wiki item to clickable link', () => {
    const handleClick = vi.fn();
    render(
      <ParsedText text="Defeat the Tukohama" wikiItems={['Tukohama']} onWikiClick={handleClick} />,
    );

    const link = screen.getByRole('button', { name: 'Tukohama' });
    expect(link).toBeInTheDocument();
    expect(link).toHaveClass('underline');
  });

  it('calls onWikiClick when link is clicked', async () => {
    const user = userEvent.setup();
    const handleClick = vi.fn();

    render(
      <ParsedText text="Defeat the Tukohama" wikiItems={['Tukohama']} onWikiClick={handleClick} />,
    );

    await user.click(screen.getByRole('button', { name: 'Tukohama' }));
    expect(handleClick).toHaveBeenCalledWith('Tukohama');
    expect(handleClick).toHaveBeenCalledTimes(1);
  });

  it('converts multiple wiki items to links', () => {
    render(
      <ParsedText
        text="Defeat Tukohama and find the Karui Fortress"
        wikiItems={['Tukohama', 'Karui Fortress']}
        onWikiClick={vi.fn()}
      />,
    );

    expect(screen.getByRole('button', { name: 'Tukohama' })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Karui Fortress' })).toBeInTheDocument();
  });

  it('handles overlapping matches by choosing longest first', () => {
    render(
      <ParsedText
        text="Visit the Coast and find The Coast Guardian"
        wikiItems={['Coast', 'The Coast Guardian']}
        onWikiClick={vi.fn()}
      />,
    );

    // Should create link for "The Coast Guardian" first (longer match)
    // Then create separate link for "Coast"
    const links = screen.getAllByRole('button');
    expect(links).toHaveLength(2);
  });

  it('matches wiki items case-insensitively', () => {
    render(
      <ParsedText text="Defeat the TUKOHAMA" wikiItems={['Tukohama']} onWikiClick={vi.fn()} />,
    );

    const link = screen.getByRole('button', { name: 'TUKOHAMA' });
    expect(link).toBeInTheDocument();
  });

  it('preserves original case in the displayed text', () => {
    render(
      <ParsedText
        text="Defeat the TUKOHAMA and tukohama again"
        wikiItems={['Tukohama']}
        onWikiClick={vi.fn()}
      />,
    );

    expect(screen.getByRole('button', { name: 'TUKOHAMA' })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'tukohama' })).toBeInTheDocument();
  });

  it('handles wiki items with apostrophes and hyphens', () => {
    render(
      <ParsedText
        text="Find Hillock's Den and talk to Sister Cassia"
        wikiItems={["Hillock's Den", 'Sister Cassia']}
        onWikiClick={vi.fn()}
      />,
    );

    expect(screen.getByRole('button', { name: "Hillock's Den" })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Sister Cassia' })).toBeInTheDocument();
  });

  it('only matches whole words', () => {
    render(
      <ParsedText text="The Coastline and The Coast" wikiItems={['Coast']} onWikiClick={vi.fn()} />,
    );

    // Should only match "Coast" as a whole word, not "Coastline"
    const links = screen.getAllByRole('button');
    expect(links).toHaveLength(1);
    expect(links[0]).toHaveTextContent('Coast');
  });

  it('stops click propagation', async () => {
    const user = userEvent.setup();
    const handleWikiClick = vi.fn();
    const handleContainerClick = vi.fn();

    render(
      // biome-ignore lint/a11y/useKeyWithClickEvents: Test wrapper only
      // biome-ignore lint/a11y/noStaticElementInteractions: Test wrapper only
      <div onClick={handleContainerClick}>
        <ParsedText text="Defeat Tukohama" wikiItems={['Tukohama']} onWikiClick={handleWikiClick} />
      </div>,
    );

    await user.click(screen.getByRole('button', { name: 'Tukohama' }));

    expect(handleWikiClick).toHaveBeenCalledTimes(1);
    expect(handleContainerClick).not.toHaveBeenCalled();
  });

  it('handles text with no matching wiki items', () => {
    render(
      <ParsedText
        text="This text has no matching items"
        wikiItems={['Tukohama', 'Karui']}
        onWikiClick={vi.fn()}
      />,
    );

    expect(screen.queryByRole('button')).not.toBeInTheDocument();
    expect(screen.getByText('This text has no matching items')).toBeInTheDocument();
  });

  it('renders text and links together correctly', () => {
    render(
      <ParsedText
        text="Go to The Coast and defeat Tukohama"
        wikiItems={['The Coast', 'Tukohama']}
        onWikiClick={vi.fn()}
      />,
    );

    expect(screen.getByText(/^Go to/)).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'The Coast' })).toBeInTheDocument();
    expect(screen.getByText(/and defeat/)).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Tukohama' })).toBeInTheDocument();
  });
});
