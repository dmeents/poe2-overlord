import type { ReactNode } from 'react';

import { CharacterProvider } from './contexts/CharacterContext';
import { ConfigurationProvider } from './contexts/ConfigurationContext';
import { EconomyProvider } from './contexts/EconomyContext';
import { GameProcessProvider } from './contexts/GameProcessContext';
import { ServerStatusProvider } from './contexts/ServerStatusContext';
import { WalkthroughProvider } from './contexts/WalkthroughContext';
import { ZoneProvider } from './contexts/ZoneContext';

interface ProvidersProps {
  children: ReactNode;
}

/**
 * Application-wide context providers wrapper.
 *
 * CRITICAL: Provider order matters! Providers are nested from outermost to innermost.
 * Do NOT reorder without understanding the dependency graph below.
 *
 * Dependency Graph:
 * ```
 * ConfigurationProvider (independent)
 *   └─ GameProcessProvider (independent)
 *       └─ ServerStatusProvider (independent)
 *           └─ CharacterProvider (root of character tree)
 *               ├─ ZoneProvider (depends on Character)
 *               ├─ EconomyProvider (depends on Character)
 *               └─ WalkthroughProvider (depends on Character)
 * ```
 *
 * Dependencies:
 * - ZoneProvider → CharacterProvider (uses activeCharacter.zones)
 * - EconomyProvider → CharacterProvider (uses activeCharacter.league, hardcore, ssf)
 * - WalkthroughProvider → CharacterProvider (uses activeCharacter.walkthrough_progress)
 *
 * Breaking this order will cause runtime errors like:
 * "useCharacter must be used within CharacterProvider"
 *
 * @example
 * // WRONG - ZoneProvider outside CharacterProvider
 * <ZoneProvider>
 *   <CharacterProvider>...</CharacterProvider>
 * </ZoneProvider>
 *
 * @example
 * // CORRECT - ZoneProvider inside CharacterProvider
 * <CharacterProvider>
 *   <ZoneProvider>...</ZoneProvider>
 * </CharacterProvider>
 */
export function Providers({ children }: ProvidersProps): React.JSX.Element {
  return (
    <ConfigurationProvider>
      <GameProcessProvider>
        <ServerStatusProvider>
          <CharacterProvider>
            <ZoneProvider>
              <EconomyProvider>
                <WalkthroughProvider>{children}</WalkthroughProvider>
              </EconomyProvider>
            </ZoneProvider>
          </CharacterProvider>
        </ServerStatusProvider>
      </GameProcessProvider>
    </ConfigurationProvider>
  );
}
