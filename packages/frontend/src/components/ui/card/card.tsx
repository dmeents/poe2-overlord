import type { ReactNode } from 'react';
import { Button } from '../button/button';
import { type CardAccentColor, cardStyles } from './card.styles';

interface CardProps {
  children: ReactNode;
  title?: string;
  subtitle?: string;
  icon?: ReactNode;
  className?: string;
  accentColor?: CardAccentColor;
  showStatusIndicator?: boolean;
  rightAction?: {
    label: ReactNode;
    onClick: () => void;
    title?: string;
  };
}

export function Card({
  children,
  title,
  subtitle,
  icon,
  className = '',
  accentColor = 'stone',
  showStatusIndicator = false,
  rightAction,
}: CardProps) {
  return (
    <div className={`${cardStyles.base} ${className}`}>
      {title && (
        <div className={`${cardStyles.header} ${cardStyles.accentGradient[accentColor]}`}>
          <div className={cardStyles.headerContent}>
            <div className={cardStyles.headerLeft}>
              {showStatusIndicator && (
                <div className={`${cardStyles.statusDot} ${cardStyles.accentDot[accentColor]}`} />
              )}
              {icon && (
                <span className={`${cardStyles.icon} ${cardStyles.accentText[accentColor]}`}>
                  {icon}
                </span>
              )}
              <span className={`${cardStyles.title} ${cardStyles.accentText[accentColor]}`}>
                {title}
              </span>
            </div>
            {(subtitle || rightAction) && (
              <div className="flex items-center gap-2">
                {subtitle && <span className={cardStyles.subtitle}>{subtitle}</span>}
                {rightAction && (
                  <Button
                    onClick={rightAction.onClick}
                    variant="text"
                    size="xs"
                    title={rightAction.title}
                    className="text-stone-400 hover:text-stone-200 h-auto py-0">
                    {rightAction.label}
                  </Button>
                )}
              </div>
            )}
          </div>
        </div>
      )}
      <div className={cardStyles.body}>{children}</div>
    </div>
  );
}
