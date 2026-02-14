import { NAME_STYLES, type NameStyleType } from './name-parts';

export type Gender = 'male' | 'female';

export interface NameGeneratorOptions {
  style?: NameStyleType;
  minLength?: number;
  maxLength?: number;
  gender?: Gender;
  allowBlending?: boolean;
}

/**
 * Generates a random fantasy name based on the selected style
 */
export function generateFantasyName(options: NameGeneratorOptions = {}): string {
  const {
    style = getRandomStyle(),
    minLength = 3,
    maxLength = 15,
    gender,
    allowBlending = true,
  } = options;

  const nameStyle = NAME_STYLES[style];
  let name = '';
  let attempts = 0;
  const maxAttempts = 50;

  // Decide if we should blend styles (15% chance if allowed)
  const shouldBlend = allowBlending && Math.random() < 0.15;
  const blendStyle = shouldBlend ? NAME_STYLES[getRandomStyle()] : null;

  // Try to generate a valid name within constraints
  while (attempts < maxAttempts) {
    // Randomly choose generation method
    const method = Math.random();

    if (method < 0.7) {
      // Method 1: Prefix + Suffix (70% of the time)
      name = generateFromPrefixSuffix(nameStyle, gender, blendStyle);
    } else if (method < 0.9) {
      // Method 2: Prefix + Middle + Suffix (20% of the time)
      name = generateFromThreeParts(nameStyle, gender, blendStyle);
    } else {
      // Method 3: Single prefix or suffix (10% of the time)
      name = generateSinglePart(nameStyle, gender);
    }

    // Check if name meets length requirements
    if (name.length >= minLength && name.length <= maxLength) {
      break;
    }

    attempts++;
  }

  // If we couldn't generate a valid name, just use a prefix
  if (name.length < minLength || name.length > maxLength) {
    const prefixes =
      gender === 'female' && nameStyle.femalePrefixes
        ? nameStyle.femalePrefixes
        : nameStyle.prefixes;
    name = getRandomElement(prefixes);
  }

  // Capitalize the first letter
  return capitalizeFirstLetter(name);
}

/**
 * Capitalizes the first letter of a string
 */
function capitalizeFirstLetter(str: string): string {
  if (!str) return str;
  return str.charAt(0).toUpperCase() + str.slice(1);
}

/**
 * Generates a name from prefix and suffix
 */
function generateFromPrefixSuffix(
  nameStyle: (typeof NAME_STYLES)[NameStyleType],
  gender?: Gender,
  blendStyle?: (typeof NAME_STYLES)[NameStyleType] | null,
): string {
  const prefixes =
    gender === 'female' && nameStyle.femalePrefixes ? nameStyle.femalePrefixes : nameStyle.prefixes;

  // Use blend style for suffix if provided
  const suffixSource = blendStyle || nameStyle;
  const suffixes =
    gender === 'female' && suffixSource.femaleSuffixes
      ? suffixSource.femaleSuffixes
      : suffixSource.suffixes;

  const prefix = getRandomElement(prefixes);
  const suffix = getRandomElement(suffixes);

  // Avoid awkward repeated syllables
  if (shouldCombine(prefix, suffix)) {
    return prefix + suffix;
  }

  // Try another combination
  const newSuffix = getRandomElement(suffixes);
  return prefix + newSuffix;
}

/**
 * Generates a name from prefix, middle, and suffix
 */
function generateFromThreeParts(
  nameStyle: (typeof NAME_STYLES)[NameStyleType],
  gender?: Gender,
  blendStyle?: (typeof NAME_STYLES)[NameStyleType] | null,
): string {
  const prefixes =
    gender === 'female' && nameStyle.femalePrefixes ? nameStyle.femalePrefixes : nameStyle.prefixes;

  // Use blend style for middle and suffix if provided
  const middleSource = blendStyle || nameStyle;
  const suffixSource = blendStyle || nameStyle;

  const suffixes =
    gender === 'female' && suffixSource.femaleSuffixes
      ? suffixSource.femaleSuffixes
      : suffixSource.suffixes;

  const prefix = getRandomElement(prefixes);
  const middle = getRandomElement(middleSource.middles);
  const suffix = getRandomElement(suffixes);

  // Check for awkward combinations
  if (shouldCombine(prefix, middle) && shouldCombine(middle, suffix)) {
    return prefix + middle + suffix;
  }

  // Fallback to prefix + suffix
  return prefix + suffix;
}

/**
 * Generates a single-part name
 */
function generateSinglePart(
  nameStyle: (typeof NAME_STYLES)[NameStyleType],
  gender?: Gender,
): string {
  const prefixes =
    gender === 'female' && nameStyle.femalePrefixes ? nameStyle.femalePrefixes : nameStyle.prefixes;
  const suffixes =
    gender === 'female' && nameStyle.femaleSuffixes ? nameStyle.femaleSuffixes : nameStyle.suffixes;

  const useSuffix = Math.random() > 0.5;
  return useSuffix ? getRandomElement(suffixes) : getRandomElement(prefixes);
}

/**
 * Checks if two parts should be combined (avoids awkward repetition)
 */
function shouldCombine(part1: string, part2: string): boolean {
  const end1 = part1.slice(-2).toLowerCase();
  const start2 = part2.slice(0, 2).toLowerCase();

  // Avoid identical endings/beginnings
  if (end1 === start2) {
    return false;
  }

  // Avoid too many consonants in a row
  const combined = part1 + part2;
  if (/[bcdfghjklmnpqrstvwxyz]{5,}/i.test(combined)) {
    return false;
  }

  return true;
}

/**
 * Gets a random element from an array
 */
function getRandomElement<T>(array: T[]): T {
  return array[Math.floor(Math.random() * array.length)];
}

/**
 * Gets a random name style
 */
function getRandomStyle(): NameStyleType {
  const styles: NameStyleType[] = Object.keys(NAME_STYLES) as NameStyleType[];
  return getRandomElement(styles);
}
