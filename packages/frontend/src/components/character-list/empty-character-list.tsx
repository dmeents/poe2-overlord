import { Button } from '../button';
import {
  getEmptyStateClasses,
  getEmptyStateDescriptionClasses,
  getEmptyStateIconClasses,
  getEmptyStateIconSvgClasses,
  getEmptyStateTitleClasses,
} from './character-list.styles';

interface EmptyCharacterListProps {
  onCreateCharacter: () => void;
}

export function EmptyCharacterList({
  onCreateCharacter,
}: EmptyCharacterListProps) {
  return (
    <div className={getEmptyStateClasses()}>
      <div className={getEmptyStateIconClasses()}>
        <svg
          className={getEmptyStateIconSvgClasses()}
          fill='none'
          viewBox='0 0 24 24'
          stroke='currentColor'
        >
          <path
            strokeLinecap='round'
            strokeLinejoin='round'
            strokeWidth={1.5}
            d='M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z'
          />
        </svg>
      </div>
      <h3 className={getEmptyStateTitleClasses()}>No Characters</h3>
      <p className={getEmptyStateDescriptionClasses()}>
        Create your first character to start tracking your adventures.
      </p>
      <Button onClick={onCreateCharacter} variant='primary'>
        Create Character
      </Button>
    </div>
  );
}
