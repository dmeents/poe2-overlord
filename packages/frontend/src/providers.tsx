import { GameProcessProvider } from './contexts/GameProcessContext';
import { ServerStatusProvider } from './contexts/ServerStatusContext';
import { CharacterProvider } from './contexts/CharacterContext';
import { WalkthroughProvider } from './contexts/WalkthroughContext';

export const Providers: React.FC<React.PropsWithChildren> = ({ children }) => {
  return (
    <GameProcessProvider>
      <ServerStatusProvider>
        <CharacterProvider>
          <WalkthroughProvider>{children}</WalkthroughProvider>
        </CharacterProvider>
      </ServerStatusProvider>
    </GameProcessProvider>
  );
};
