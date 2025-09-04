import { useEffect, useState } from 'react';
import type {
  Ascendency,
  Character,
  CharacterClass,
  League,
} from '../../types';
import { Button } from '../button';
import { CheckboxInput } from '../form/checkbox-input';
import { FormField } from '../form/form-field';
import { SelectInput } from '../form/select-input';
import { TextInput } from '../form/text-input';

interface CharacterFormProps {
  character?: Character;
  onSubmit: (data: CharacterFormData) => void;
  onCancel: () => void;
  isLoading?: boolean;
}

export interface CharacterFormData {
  name: string;
  class: CharacterClass;
  ascendency: Ascendency;
  league: League;
  hardcore: boolean;
  solo_self_found: boolean;
}

const CHARACTER_CLASSES: { value: CharacterClass; label: string }[] = [
  { value: 'Warrior', label: 'Warrior' },
  { value: 'Sorceress', label: 'Sorceress' },
  { value: 'Ranger', label: 'Ranger' },
  { value: 'Huntress', label: 'Huntress' },
  { value: 'Monk', label: 'Monk' },
  { value: 'Mercenary', label: 'Mercenary' },
  { value: 'Witch', label: 'Witch' },
];

const ASCENDENCIES: Record<
  CharacterClass,
  { value: Ascendency; label: string }[]
> = {
  Warrior: [
    { value: 'Titan', label: 'Titan' },
    { value: 'Warbringer', label: 'Warbringer' },
    { value: 'Smith of Katava', label: 'Smith of Katava' },
  ],
  Sorceress: [
    { value: 'Stormweaver', label: 'Stormweaver' },
    { value: 'Chronomancer', label: 'Chronomancer' },
  ],
  Ranger: [
    { value: 'Deadeye', label: 'Deadeye' },
    { value: 'Pathfinder', label: 'Pathfinder' },
  ],
  Huntress: [
    { value: 'Ritualist', label: 'Ritualist' },
    { value: 'Amazon', label: 'Amazon' },
  ],
  Monk: [
    { value: 'Invoker', label: 'Invoker' },
    { value: 'Acolyte of Chayula', label: 'Acolyte of Chayula' },
  ],
  Mercenary: [
    { value: 'Gemling Legionnaire', label: 'Gemling Legionnaire' },
    { value: 'Tactitian', label: 'Tactitian' },
    { value: 'Witchhunter', label: 'Witchhunter' },
  ],
  Witch: [
    { value: 'Blood Mage', label: 'Blood Mage' },
    { value: 'Infernalist', label: 'Infernalist' },
    { value: 'Lich', label: 'Lich' },
  ],
};

const LEAGUES: { value: League; label: string }[] = [
  { value: 'Standard', label: 'Standard' },
  { value: 'Third Edict', label: 'Third Edict' },
];

export function CharacterForm({
  character,
  onSubmit,
  onCancel,
  isLoading,
}: CharacterFormProps) {
  const [formData, setFormData] = useState<CharacterFormData>({
    name: character?.name || '',
    class: character?.class || 'Warrior',
    ascendency: character?.ascendency || 'Titan',
    league: character?.league || 'Standard',
    hardcore: character?.hardcore || false,
    solo_self_found: character?.solo_self_found || false,
  });

  const [errors, setErrors] = useState<Partial<CharacterFormData>>({});

  // Reset ascendency when class changes
  useEffect(() => {
    const availableAscendencies = ASCENDENCIES[formData.class];

    if (availableAscendencies.length > 0) {
      setFormData(prev => ({
        ...prev,
        ascendency: availableAscendencies[0].value,
      }));
    }
  }, [formData.class, character]);

  const validateForm = (): boolean => {
    const newErrors: Partial<CharacterFormData> = {};

    if (!formData.name.trim()) {
      newErrors.name = 'Character name is required';
    } else if (formData.name.trim().length < 2) {
      newErrors.name = 'Character name must be at least 2 characters';
    } else if (formData.name.trim().length > 50) {
      newErrors.name = 'Character name must be less than 50 characters';
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (validateForm()) onSubmit(formData);
  };

  const handleInputChange = (field: keyof CharacterFormData, value: any) => {
    setFormData(prev => {
      const newData = { ...prev, [field]: value };
      return newData;
    });

    // Clear error when user starts typing
    if (errors[field]) {
      setErrors(prev => ({ ...prev, [field]: undefined }));
    }
  };

  const availableAscendencies = ASCENDENCIES[formData.class];

  return (
    <form onSubmit={handleSubmit} className='space-y-6'>
      <div className='space-y-4'>
        <FormField label='Character Name'>
          <TextInput
            id='character-name'
            value={formData.name}
            onChange={value => handleInputChange('name', value)}
            placeholder='Enter character name'
            isValid={!errors.name}
            warningMessage={errors.name}
          />
        </FormField>
        <FormField label='Class'>
          <SelectInput
            id='character-class'
            value={formData.class}
            onChange={value => handleInputChange('class', value)}
            options={CHARACTER_CLASSES}
          />
        </FormField>
        <FormField label='Ascendency'>
          <SelectInput
            id='character-ascendency'
            value={formData.ascendency}
            onChange={value => handleInputChange('ascendency', value)}
            options={availableAscendencies}
          />
        </FormField>
        <FormField label='League'>
          <SelectInput
            id='character-league'
            value={formData.league}
            onChange={value => handleInputChange('league', value)}
            options={LEAGUES}
          />
        </FormField>
        <CheckboxInput
          id='character-hardcore'
          label='Hardcore'
          checked={formData.hardcore}
          onChange={checked => handleInputChange('hardcore', checked)}
          description='Hardcore characters have permanent death and cannot be revived.'
        />
        <CheckboxInput
          id='character-ssf'
          label='Solo Self-Found (SSF)'
          checked={formData.solo_self_found}
          onChange={checked => handleInputChange('solo_self_found', checked)}
          description='Solo Self-Found characters cannot trade with other players or use shared stash.'
        />
      </div>
      <div className='flex justify-end gap-3 pt-4 border-t border-zinc-700'>
        <Button
          type='button'
          variant='outline'
          onClick={onCancel}
          disabled={isLoading}
        >
          Cancel
        </Button>
        <Button type='submit' variant='primary' disabled={isLoading}>
          {isLoading
            ? 'Saving...'
            : character
              ? 'Update Character'
              : 'Create Character'}
        </Button>
      </div>
    </form>
  );
}
