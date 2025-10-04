use crate::domain::character::models::{Ascendency, CharacterClass};
use crate::infrastructure::parsing::manager::CharacterClassOrAscendency;
use crate::infrastructure::parsing::manager::ParserResult;
use crate::infrastructure::parsing::ParsersConfig;
use crate::infrastructure::parsing::{LogParser, ParseError};
use log::debug;
use regex::Regex;

#[derive(Clone)]
pub struct CharacterLevelParser {
    config: ParsersConfig,
    level_regex: Regex,
}

impl CharacterLevelParser {
    pub fn new() -> Self {
        Self {
            config: ParsersConfig::default(),
            level_regex: Self::create_level_regex(),
        }
    }

    pub fn with_config(config: ParsersConfig) -> Self {
        Self {
            config,
            level_regex: Self::create_level_regex(),
        }
    }

    fn create_level_regex() -> Regex {
        Regex::new(r"\[INFO Client \d+\]\s*:\s*(.+?)\s+\((.+?)\)\s+is\s+now\s+level\s+(\d+)$")
            .expect("Failed to compile character level regex")
    }

    fn extract_character_info(
        &self,
        line: &str,
    ) -> Result<(String, CharacterClassOrAscendency, u32), ParseError> {
        debug!("Attempting to extract character info from: {}", line.trim());

        if let Some(captures) = self.level_regex.captures(line.trim()) {
            if captures.len() == 4 {
                let character_name = captures.get(1).unwrap().as_str().trim().to_string();
                let class_or_ascendency_str = captures.get(2).unwrap().as_str().trim();
                let level_str = captures.get(3).unwrap().as_str().trim();

                let level = level_str.parse::<u32>().map_err(|_| {
                    ParseError::content_extraction_failed(&format!(
                        "Failed to parse level '{}' as number",
                        level_str
                    ))
                })?;

                // Try parsing as ascendency first, then fall back to character class
                let class_or_ascendency = match self.parse_ascendency(class_or_ascendency_str) {
                    Ok(ascendency) => {
                        debug!("Parsed as ascendency: {}", class_or_ascendency_str);
                        CharacterClassOrAscendency::Ascendency(ascendency)
                    }
                    Err(_) => {
                        // Fall back to character class parsing
                        let character_class = self.parse_character_class(class_or_ascendency_str)?;
                        debug!("Parsed as character class: {}", class_or_ascendency_str);
                        CharacterClassOrAscendency::Class(character_class)
                    }
                };

                debug!(
                    "Extracted character info: name='{}', class_or_ascendency='{}', level={}",
                    character_name, class_or_ascendency_str, level
                );

                Ok((character_name, class_or_ascendency, level))
            } else {
                Err(ParseError::content_extraction_failed(
                    "Regex matched but wrong number of capture groups",
                ))
            }
        } else {
            Err(ParseError::content_extraction_failed(
                "Line does not match character level-up pattern",
            ))
        }
    }

    fn parse_ascendency(&self, ascendency_str: &str) -> Result<Ascendency, ParseError> {
        match ascendency_str {
            "Titan" => Ok(Ascendency::Titan),
            "Warbringer" => Ok(Ascendency::Warbringer),
            "Smith of Katava" => Ok(Ascendency::SmithOfKatava),
            "Stormweaver" => Ok(Ascendency::Stormweaver),
            "Chronomancer" => Ok(Ascendency::Chronomancer),
            "Deadeye" => Ok(Ascendency::Deadeye),
            "Pathfinder" => Ok(Ascendency::Pathfinder),
            "Ritualist" => Ok(Ascendency::Ritualist),
            "Amazon" => Ok(Ascendency::Amazon),
            "Invoker" => Ok(Ascendency::Invoker),
            "Acolyte of Chayula" => Ok(Ascendency::AcolyteOfChayula),
            "Gemling Legionnaire" => Ok(Ascendency::GemlingLegionnaire),
            "Tactitian" => Ok(Ascendency::Tactitian),
            "Witchhunter" => Ok(Ascendency::Witchhunter),
            "Blood Mage" => Ok(Ascendency::BloodMage),
            "Infernalist" => Ok(Ascendency::Infernalist),
            "Lich" => Ok(Ascendency::Lich),
            _ => Err(ParseError::content_extraction_failed(&format!(
                "Invalid ascendency: '{}'",
                ascendency_str
            ))),
        }
    }

    fn parse_character_class(&self, class_str: &str) -> Result<CharacterClass, ParseError> {
        match class_str.to_lowercase().as_str() {
            "warrior" => Ok(CharacterClass::Warrior),
            "sorceress" => Ok(CharacterClass::Sorceress),
            "ranger" => Ok(CharacterClass::Ranger),
            "huntress" => Ok(CharacterClass::Huntress),
            "monk" => Ok(CharacterClass::Monk),
            "mercenary" => Ok(CharacterClass::Mercenary),
            "witch" => Ok(CharacterClass::Witch),
            _ => Err(ParseError::content_extraction_failed(&format!(
                "Invalid character class: '{}'",
                class_str
            ))),
        }
    }
}

impl LogParser for CharacterLevelParser {
    type Event = ParserResult;

    fn should_parse(&self, line: &str) -> bool {
        self.config
            .matches_patterns("character_level", line)
            .unwrap_or(false)
    }

    fn parse_line(&self, line: &str) -> Result<Self::Event, ParseError> {
        debug!(
            "Character level parser attempting to parse line: {}",
            line.trim()
        );

        if !self.should_parse(line) {
            debug!("Line does not match character level patterns");
            return Err(ParseError::no_pattern_match("character_level"));
        }

        let (character_name, class_or_ascendency, level) = self.extract_character_info(line)?;

        if !(1..=100).contains(&level) {
            return Err(ParseError::content_extraction_failed(&format!(
                "Level {} is outside valid range (1-100)",
                level
            )));
        }

        debug!(
            "Successfully parsed character level-up: {} ({:?}) -> level {}",
            character_name, class_or_ascendency, level
        );

        Ok(ParserResult::CharacterLevel((
            character_name,
            class_or_ascendency,
            level,
        )))
    }

    fn parser_name(&self) -> &'static str {
        "character_level"
    }
}

impl Default for CharacterLevelParser {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub struct CharacterDeathParser {
    config: ParsersConfig,
    death_regex: Regex,
}

impl CharacterDeathParser {
    pub fn new() -> Self {
        Self {
            config: ParsersConfig::default(),
            death_regex: Self::create_death_regex(),
        }
    }

    pub fn with_config(config: ParsersConfig) -> Self {
        Self {
            config,
            death_regex: Self::create_death_regex(),
        }
    }

    fn create_death_regex() -> Regex {
        Regex::new(r"\[INFO Client \d+\]\s*:\s*(\S.*?)\s+has\s+been\s+slain\.$")
            .expect("Failed to compile character death regex")
    }

    fn extract_character_name(&self, line: &str) -> Result<String, ParseError> {
        debug!("Attempting to extract character name from: {}", line.trim());

        if let Some(captures) = self.death_regex.captures(line.trim()) {
            if captures.len() == 2 {
                let character_name = captures.get(1).unwrap().as_str().trim().to_string();

                debug!("Extracted character name: '{}'", character_name);

                Ok(character_name)
            } else {
                Err(ParseError::content_extraction_failed(
                    "Regex matched but wrong number of capture groups",
                ))
            }
        } else {
            Err(ParseError::content_extraction_failed(
                "Line does not match character death pattern",
            ))
        }
    }
}

impl LogParser for CharacterDeathParser {
    type Event = ParserResult;

    fn should_parse(&self, line: &str) -> bool {
        self.config
            .matches_patterns("character_death", line)
            .unwrap_or(false)
    }

    fn parse_line(&self, line: &str) -> Result<Self::Event, ParseError> {
        debug!(
            "Character death parser attempting to parse line: {}",
            line.trim()
        );

        if !self.should_parse(line) {
            debug!("Line does not match character death patterns");
            return Err(ParseError::no_pattern_match("character_death"));
        }

        let character_name = self.extract_character_name(line)?;

        debug!(
            "✅ DEATH PARSER: Successfully parsed character death: '{}' has been slain",
            character_name
        );

        Ok(ParserResult::CharacterDeath(character_name))
    }

    fn parser_name(&self) -> &'static str {
        "character_death"
    }
}

impl Default for CharacterDeathParser {
    fn default() -> Self {
        Self::new()
    }
}
