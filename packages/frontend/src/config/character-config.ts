import type { Ascendency, CharacterClass, League } from '../types';

export const CHARACTER_CLASSES: { value: CharacterClass; label: string }[] = [
  { value: 'Warrior', label: 'Warrior' },
  { value: 'Sorceress', label: 'Sorceress' },
  { value: 'Ranger', label: 'Ranger' },
  { value: 'Huntress', label: 'Huntress' },
  { value: 'Monk', label: 'Monk' },
  { value: 'Mercenary', label: 'Mercenary' },
  { value: 'Witch', label: 'Witch' },
];

export const ASCENDENCIES: Record<
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

export const LEAGUES: { value: League; label: string }[] = [
  { value: 'Standard', label: 'Standard' },
  { value: 'Third Edict', label: 'Third Edict' },
];
