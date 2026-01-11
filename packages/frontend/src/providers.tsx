import type { ReactNode } from 'react';

import { CharacterProvider } from './contexts/CharacterContext';
import { EconomyProvider } from './contexts/EconomyContext';
import { GameProcessProvider } from './contexts/GameProcessContext';
import { ServerStatusProvider } from './contexts/ServerStatusContext';
import { WalkthroughProvider } from './contexts/WalkthroughContext';
import { ZoneProvider } from './contexts/ZoneContext';

interface ProvidersProps {
  children: ReactNode;
}

export function Providers({ children }: ProvidersProps): React.JSX.Element {
  return (
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
  );
}
