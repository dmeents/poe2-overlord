import type { ReactNode } from 'react';
import { CharacterStatusCard } from '@/components/character/character-status-card/character-status-card';

interface PageLayoutProps {
  children?: ReactNode;
  leftColumn: ReactNode;
  rightColumn: ReactNode;
  className?: string;
  showCharacterCard?: boolean;
}

export function PageLayout({
  children,
  leftColumn,
  rightColumn,
  className = '',
  showCharacterCard = false,
}: PageLayoutProps) {
  return (
    <div className={`min-h-screen text-stone-50 ${className}`}>
      <div className="px-6 py-8 pb-16">
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
          <div className="lg:col-span-2 space-y-6">
            {showCharacterCard && <CharacterStatusCard />}
            {leftColumn}
          </div>
          <div className="space-y-6">{rightColumn}</div>
        </div>
        {children}
      </div>
    </div>
  );
}
