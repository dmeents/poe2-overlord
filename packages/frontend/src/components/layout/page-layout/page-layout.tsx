import type { ReactNode } from 'react';

interface PageLayoutProps {
  children?: ReactNode;
  leftColumn: ReactNode;
  rightColumn: ReactNode;
  className?: string;
}

export function PageLayout({ children, leftColumn, rightColumn, className = '' }: PageLayoutProps) {
  return (
    <div className={`min-h-screen text-white ${className}`}>
      <div className="px-6 py-8 pb-16">
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
          {/* Left Column - Takes up 2/3 of the space */}
          <div className="lg:col-span-2 space-y-6">{leftColumn}</div>

          {/* Right Column - Takes up 1/3 of the space */}
          <div className="space-y-6">{rightColumn}</div>
        </div>
        {children}
      </div>
    </div>
  );
}
