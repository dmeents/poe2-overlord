import type { ReactNode } from 'react';
import { sectionHeaderStyles } from './section-header.styles';

interface SectionHeaderProps {
  title: string;
  icon?: ReactNode;
  className?: string;
}

export function SectionHeader({ title, icon, className = '' }: SectionHeaderProps) {
  return (
    <div className={`${sectionHeaderStyles.container} ${className}`}>
      <div className={sectionHeaderStyles.header}>
        {icon && <span className={sectionHeaderStyles.icon}>{icon}</span>}
        <h3 className={sectionHeaderStyles.title}>{title}</h3>
      </div>
    </div>
  );
}
