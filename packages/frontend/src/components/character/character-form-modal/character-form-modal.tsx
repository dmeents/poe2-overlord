import { useEffect, useState } from 'react';
import { CheckboxInput } from '../../forms/form-checkbox-input/form-checkbox-input';
import { FormField } from '../../forms/form-field/form-field';
import { Input } from '../../forms/form-input/form-input';
import { Select } from '../../forms/form-select/form-select';
import { CHARACTER_FORM_VALIDATION } from '../../../config/form-config';
import { useCharacterConfig } from '../../../hooks/useCharacterConfig';
import type {
  Ascendency,
  CharacterClass,
  CharacterData,
  League,
} from '../../../types/character';
import { Button } from '../../ui/button/button';
import { Modal } from '../../ui/modal/modal';
import {
  getFormActionsClasses,
  getFormFieldClasses,
} from './character-form-modal.styles';

interface CharacterFormModalProps {
  isOpen: boolean;
  character?: CharacterData;
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
  const {
    characterClasses,
    leagues,
    getAscendenciesForClass,
    getDefaultFormData,
  } = useCharacterConfig();

  const [formData, setFormData] = useState<CharacterFormData>({
    name: '',
    class: 'Warrior',
    ascendency: 'Titan',
    league: 'Standard',
    hardcore: false,
    solo_self_found: false,
  });

  const [errors, setErrors] = useState<Partial<CharacterFormData>>({});
  const [availableAscendencies, setAvailableAscendencies] = useState<
    { value: Ascendency; label: string }[]
  >([]);

  // Reset form data when character prop changes
  useEffect(() => {
    if (character) {
      setFormData({
        name: character.name,
        class: character.class,
        ascendency: character.ascendency,
        league: character.league,
        hardcore: character.hardcore,
        solo_self_found: character.solo_self_found,
      });
    } else {
      setFormData(getDefaultFormData());
    }
    setErrors({});
  }, [character, getDefaultFormData]);

  // Update ascendencies when class changes
  useEffect(() => {
    if (formData.class) {
      const ascendencyOptions = getAscendenciesForClass(formData.class);
      setAvailableAscendencies(ascendencyOptions);

      // Reset ascendency if current one is not valid for the new class
      if (
        ascendencyOptions.length > 0 &&
        !ascendencyOptions.some(opt => opt.value === formData.ascendency)
      ) {
        setFormData(prev => ({
          ...prev,
          ascendency: ascendencyOptions[0].value,
        }));
      }
    }
  }, [formData.class, getAscendenciesForClass, formData.ascendency]);

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
            <Input
              id='character-name'
              value={formData.name}
              onChange={value => handleInputChange('name', value as string)}
              type='text'
              placeholder='Enter character name'
              isValid={!errors.name}
              warningMessage={errors.name}
            />
          </FormField>
          <FormField label='Class'>
            <Select
              id='character-class'
              value={formData.class}
              onChange={value => handleInputChange('class', value)}
              options={characterClasses}
              variant='basic'
            />
          </FormField>
          <FormField label='Ascendency'>
            <Select
              id='character-ascendency'
              value={formData.ascendency}
              onChange={value => handleInputChange('ascendency', value)}
              options={availableAscendencies}
              variant='basic'
            />
          </FormField>
          <FormField label='League'>
            <Select
              id='character-league'
              value={formData.league}
              onChange={value => handleInputChange('league', value)}
              options={leagues}
              variant='basic'
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
