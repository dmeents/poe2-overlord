import { Button } from '../button';
import {
  getListHeaderClasses,
  getListHeaderTitleClasses,
} from './character-list.styles';

interface CharacterListHeaderProps {
  onCreateCharacter: () => void;
}

export function CharacterListHeader({
  onCreateCharacter,
}: CharacterListHeaderProps) {
  return (
    <div className={getListHeaderClasses()}>
      <h2 className={getListHeaderTitleClasses()}>Your Characters</h2>
      <Button onClick={onCreateCharacter} variant='primary' size='sm'>
        Create Character
      </Button>
    </div>
  );
}
