import { useEffect, useState } from 'react';
import { CheckboxInput, FormField, SelectInput, TextInput } from '../';
import {
  ASCENDENCIES,
  CHARACTER_CLASSES,
  CHARACTER_FORM_VALIDATION,
  LEAGUES,
  getDefaultFormData,
} from '../../config';
import type {
  Ascendency,
  Character,
  CharacterClass,
  League,
} from '../../types';
import { Button } from '../button';
import { Modal } from '../modal';
import {
  getFormActionsClasses,
  getFormFieldClasses,
} from './character-form-modal.styles';

interface CharacterFormModalProps {
  isOpen: boolean;
  character?: Character;
  onSubmit: (data: CharacterFormData) => void;
  onClose: () => void;
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

export function CharacterFormModal({
  isOpen,
  character,
  onSubmit,
  onClose,
  isLoading,
}: CharacterFormModalProps) {
  const [formData, setFormData] = useState<CharacterFormData>(
    getDefaultFormData(character)
  );

  const [errors, setErrors] = useState<Partial<CharacterFormData>>({});

  // Reset form data when character prop changes
  useEffect(() => {
    setFormData(getDefaultFormData(character));
    setErrors({});
  }, [character]);

  // Reset ascendency when class changes
  useEffect(() => {
    const availableAscendencies = ASCENDENCIES[formData.class];

    if (availableAscendencies.length > 0) {
      setFormData(prev => ({
        ...prev,
        ascendency: availableAscendencies[0].value,
      }));
    }
  }, [formData.class]);

  const validateForm = (): boolean => {
    const newErrors: Partial<CharacterFormData> = {};
    const { name } = CHARACTER_FORM_VALIDATION;

    if (!formData.name.trim()) {
      newErrors.name = name.messages.required;
    } else if (formData.name.trim().length < name.minLength) {
      newErrors.name = name.messages.minLength;
    } else if (formData.name.trim().length > name.maxLength) {
      newErrors.name = name.messages.maxLength;
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (validateForm()) onSubmit(formData);
  };

  const handleInputChange = (
    field: keyof CharacterFormData,
    value: string | boolean
  ) => {
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
    <Modal
      isOpen={isOpen}
      onClose={onClose}
      size='2xl'
      title={character ? 'Edit Character' : 'Create Character'}
      disabled={isLoading}
    >
      <form onSubmit={handleSubmit} className='space-y-6'>
        <div className={getFormFieldClasses()}>
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
        <div className={getFormActionsClasses()}>
          <Button
            type='button'
            variant='outline'
            onClick={onClose}
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
    </Modal>
  );
}
