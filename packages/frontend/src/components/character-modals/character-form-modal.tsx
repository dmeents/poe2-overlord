import { useEffect, useState } from 'react';
import { CheckboxInput, FormField, SelectInput, TextInput } from '../';
import { CHARACTER_FORM_VALIDATION } from '../../config';
import { useCharacterConfig } from '../../hooks';
import type {
  Ascendency,
  CharacterClass,
  CharacterData,
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
    isLoading: configLoading,
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

  // Reset form data when character prop changes or config loads
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
    } else if (!configLoading && characterClasses.length > 0) {
      setFormData(getDefaultFormData());
    }
    setErrors({});
  }, [character, configLoading, characterClasses.length]);

  // Load ascendencies when class changes
  useEffect(() => {
    const loadAscendencies = async () => {
      if (formData.class && characterClasses.length > 0) {
        const ascendencyOptions = await getAscendenciesForClass(formData.class);
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
    };

    loadAscendencies();
  }, [formData.class, characterClasses, getAscendenciesForClass]);

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
      disabled={isLoading || configLoading}
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
              options={characterClasses}
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
              options={leagues}
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
          <Button
            type='submit'
            variant='primary'
            disabled={isLoading || configLoading}
          >
            {isLoading
              ? 'Saving...'
              : configLoading
                ? 'Loading...'
                : character
                  ? 'Update Character'
                  : 'Create Character'}
          </Button>
        </div>
      </form>
    </Modal>
  );
}
