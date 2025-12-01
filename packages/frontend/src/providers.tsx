import { GameProcessProvider } from './contexts/GameProcessContext';
import { ServerStatusProvider } from './contexts/ServerStatusContext';

export const Providers: React.FC<React.PropsWithChildren> = ({ children }) => {
  return (
    <GameProcessProvider>
      <ServerStatusProvider>{children}</ServerStatusProvider>
    </GameProcessProvider>
  );
};
