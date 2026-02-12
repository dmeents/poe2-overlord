import { render, screen } from '@testing-library/react';
import type { ReactNode } from 'react';
import { describe, expect, it, vi } from 'vitest';

import { Providers } from './providers';

// Mock all context providers to prevent actual initialization
vi.mock('./contexts/GameProcessContext', () => ({
  GameProcessProvider: ({ children }: { children: ReactNode }) => (
    <div data-testid="game-process-provider">{children}</div>
  ),
}));

vi.mock('./contexts/ServerStatusContext', () => ({
  ServerStatusProvider: ({ children }: { children: ReactNode }) => (
    <div data-testid="server-status-provider">{children}</div>
  ),
}));

vi.mock('./contexts/CharacterContext', () => ({
  CharacterProvider: ({ children }: { children: ReactNode }) => (
    <div data-testid="character-provider">{children}</div>
  ),
}));

vi.mock('./contexts/ZoneContext', () => ({
  ZoneProvider: ({ children }: { children: ReactNode }) => (
    <div data-testid="zone-provider">{children}</div>
  ),
}));

vi.mock('./contexts/EconomyContext', () => ({
  EconomyProvider: ({ children }: { children: ReactNode }) => (
    <div data-testid="economy-provider">{children}</div>
  ),
}));

vi.mock('./contexts/WalkthroughContext', () => ({
  WalkthroughProvider: ({ children }: { children: ReactNode }) => (
    <div data-testid="walkthrough-provider">{children}</div>
  ),
}));

describe('Providers', () => {
  it('renders all providers in correct nesting order', () => {
    render(
      <Providers>
        <div data-testid="app-content">App Content</div>
      </Providers>,
    );

    // Get all provider elements
    const gameProcessProvider = screen.getByTestId('game-process-provider');
    const serverStatusProvider = screen.getByTestId('server-status-provider');
    const characterProvider = screen.getByTestId('character-provider');
    const zoneProvider = screen.getByTestId('zone-provider');
    const economyProvider = screen.getByTestId('economy-provider');
    const walkthroughProvider = screen.getByTestId('walkthrough-provider');
    const appContent = screen.getByTestId('app-content');

    // Verify nesting order by checking DOM hierarchy
    expect(gameProcessProvider).toContainElement(serverStatusProvider);
    expect(serverStatusProvider).toContainElement(characterProvider);
    expect(characterProvider).toContainElement(zoneProvider);
    expect(zoneProvider).toContainElement(economyProvider);
    expect(economyProvider).toContainElement(walkthroughProvider);
    expect(walkthroughProvider).toContainElement(appContent);
  });

  it('ensures CharacterProvider wraps all dependent providers', () => {
    render(
      <Providers>
        <div data-testid="app-content">App Content</div>
      </Providers>,
    );

    const characterProvider = screen.getByTestId('character-provider');
    const zoneProvider = screen.getByTestId('zone-provider');
    const economyProvider = screen.getByTestId('economy-provider');
    const walkthroughProvider = screen.getByTestId('walkthrough-provider');

    // All character-dependent providers should be children of CharacterProvider
    expect(characterProvider).toContainElement(zoneProvider);
    expect(characterProvider).toContainElement(economyProvider);
    expect(characterProvider).toContainElement(walkthroughProvider);
  });

  it('renders children correctly', () => {
    render(
      <Providers>
        <div data-testid="test-child">Test Child</div>
      </Providers>,
    );

    expect(screen.getByTestId('test-child')).toBeInTheDocument();
    expect(screen.getByTestId('test-child')).toHaveTextContent('Test Child');
  });

  it('renders the complete provider hierarchy', () => {
    const { container } = render(
      <Providers>
        <span>Nested Content</span>
      </Providers>,
    );

    // All providers should be present in the DOM
    expect(screen.getByTestId('game-process-provider')).toBeInTheDocument();
    expect(screen.getByTestId('server-status-provider')).toBeInTheDocument();
    expect(screen.getByTestId('character-provider')).toBeInTheDocument();
    expect(screen.getByTestId('zone-provider')).toBeInTheDocument();
    expect(screen.getByTestId('economy-provider')).toBeInTheDocument();
    expect(screen.getByTestId('walkthrough-provider')).toBeInTheDocument();

    // Content should be rendered
    expect(container).toHaveTextContent('Nested Content');
  });
});
