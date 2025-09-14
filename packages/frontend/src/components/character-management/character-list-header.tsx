import { Button } from '../button';

interface CharacterListHeaderProps {
  onCreateCharacter: () => void;
}

export function CharacterListHeader({ onCreateCharacter }: CharacterListHeaderProps) {
  return (
    <div className='flex justify-between items-center'>
      <h2 className='text-xl font-semibold text-white'>Your Characters</h2>
      <Button onClick={onCreateCharacter} variant='primary' size='sm'>
        Create Character
      </Button>
    </div>
  );
}
