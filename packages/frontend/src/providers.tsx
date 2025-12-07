import { GameProcessProvider } from './contexts/GameProcessContext';
import { ServerStatusProvider } from './contexts/ServerStatusContext';
import { CharacterProvider } from './contexts/CharacterContext';
import { ZoneProvider } from './contexts/ZoneContext';
import { EconomyProvider } from './contexts/EconomyContext';
import { WalkthroughProvider } from './contexts/WalkthroughContext';

export const Providers: React.FC<React.PropsWithChildren> = ({ children }) => {
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
};
