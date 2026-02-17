import { cn } from '@poe2-overlord/theme';
import { memo } from 'react';
import {
  typeBarContainerClasses,
  typeButtonActiveClasses,
  typeButtonClasses,
  typeButtonInactiveClasses,
  typeIconClasses,
} from './economy-type-bar.styles';

interface TypeOption {
  value: string;
  label: string;
  icon?: React.ReactNode;
}

interface EconomyTypeBarProps {
  types: TypeOption[];
  selectedType: string;
  onTypeChange: (type: string) => void;
}

export const EconomyTypeBar = memo(function EconomyTypeBar({
  types,
  selectedType,
  onTypeChange,
}: EconomyTypeBarProps) {
  return (
    <div className={typeBarContainerClasses}>
      {types.map(type => {
        const isActive = selectedType === type.value;
        return (
          <button
            key={type.value}
            type="button"
            onClick={() => onTypeChange(type.value)}
            className={cn(
              typeButtonClasses,
              isActive ? typeButtonActiveClasses : typeButtonInactiveClasses,
            )}>
            {type.icon && <span className={typeIconClasses}>{type.icon}</span>}
            {type.label}
          </button>
        );
      })}
    </div>
  );
});
