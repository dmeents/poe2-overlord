/// Cumulative XP required to reach each level in Path of Exile 2.
/// Index = level, value = total cumulative XP required to be at that level.
/// Level 0 is a sentinel (characters start at level 1).
/// Source: POE2 community datamining.
pub const EXPERIENCE_TABLE: [u64; 101] = [
    0,             // 0 (sentinel)
    0,             // 1
    492,           // 2
    1_576,         // 3
    3_288,         // 4
    5_988,         // 5
    9_836,         // 6
    15_284,        // 7
    23_424,        // 8
    34_952,        // 9
    50_792,        // 10
    72_672,        // 11
    101_912,       // 12
    141_376,       // 13
    192_100,       // 14
    257_572,       // 15
    341_380,       // 16
    447_980,       // 17
    581_784,       // 18
    747_688,       // 19
    950_440,       // 20
    1_197_380,     // 21
    1_495_800,     // 22
    1_854_204,     // 23
    2_282_428,     // 24
    2_791_776,     // 25
    3_390_512,     // 26
    4_088_496,     // 27
    4_899_520,     // 28
    5_841_644,     // 29
    6_929_712,     // 30
    8_179_028,     // 31
    9_606_060,     // 32
    11_224_504,    // 33
    13_056_036,    // 34
    15_115_948,    // 35
    17_420_412,    // 36
    19_987_452,    // 37
    22_839_432,    // 38
    26_000_828,    // 39
    29_493_960,    // 40
    33_342_348,    // 41
    37_566_032,    // 42
    42_190_872,    // 43
    47_241_256,    // 44
    52_748_800,    // 45
    58_748_472,    // 46
    65_279_840,    // 47
    72_382_612,    // 48
    80_102_680,    // 49
    88_487_372,    // 50
    97_583_716,    // 51
    107_494_192,   // 52
    118_232_712,   // 53
    129_929_232,   // 54
    142_649_148,   // 55
    156_485_044,   // 56
    171_538_808,   // 57
    187_916_968,   // 58
    205_745_032,   // 59
    225_143_584,   // 60
    246_228_164,   // 61
    269_120_612,   // 62
    293_948_444,   // 63
    320_817_856,   // 64
    349_998_212,   // 65
    381_603_920,   // 66
    415_922_304,   // 67
    453_139_704,   // 68  <- death penalty begins (level >= 68)
    493_418_524,   // 69
    536_928_900,   // 70
    583_873_000,   // 71
    634_446_876,   // 72
    688_994_396,   // 73
    747_741_904,   // 74
    810_909_024,   // 75
    878_693_656,   // 76
    951_346_892,   // 77
    1_029_142_104, // 78
    1_112_269_632, // 79
    1_200_949_336, // 80
    1_295_476_872, // 81
    1_395_993_108, // 82
    1_502_849_692, // 83
    1_616_388_636, // 84
    1_737_012_656, // 85
    1_865_228_464, // 86
    2_001_511_252, // 87
    2_146_506_108, // 88
    2_300_864_028, // 89
    2_465_371_536, // 90
    2_640_900_608, // 91
    2_828_427_716, // 92
    3_028_952_644, // 93
    3_243_589_240, // 94
    3_473_462_880, // 95
    3_719_706_976, // 96
    3_983_462_484, // 97
    4_266_085_076, // 98
    4_569_245_812, // 99
    4_894_421_836, // 100
];

/// Returns the XP delta needed to go from `level` to `level + 1`.
pub fn xp_for_level(level: u32) -> u64 {
    if level == 0 || level >= 100 {
        return 0;
    }
    let idx = level as usize;
    EXPERIENCE_TABLE[idx + 1].saturating_sub(EXPERIENCE_TABLE[idx])
}

/// Returns the XP lost on death at the given level.
/// 10% of the level's XP requirement if level >= 68, else 0.
pub fn death_penalty_xp(level: u32) -> u64 {
    if level < 68 {
        return 0;
    }
    xp_for_level(level) / 10
}

/// Computes the effective XP that must be earned (including re-grinding after deaths)
/// to complete the given level with the given number of deaths.
pub fn effective_xp_earned(level: u32, deaths: u32) -> u64 {
    let base_xp = xp_for_level(level);
    let penalty = death_penalty_xp(level);
    base_xp + (deaths as u64) * penalty
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xp_for_level_1_is_nonzero() {
        assert!(xp_for_level(1) > 0);
    }

    #[test]
    fn xp_for_level_0_is_zero() {
        assert_eq!(xp_for_level(0), 0);
    }

    #[test]
    fn xp_for_level_100_is_zero() {
        assert_eq!(xp_for_level(100), 0);
    }

    #[test]
    fn death_penalty_below_68_is_zero() {
        assert_eq!(death_penalty_xp(67), 0);
        assert_eq!(death_penalty_xp(1), 0);
        assert_eq!(death_penalty_xp(50), 0);
    }

    #[test]
    fn death_penalty_at_68_is_nonzero() {
        assert!(death_penalty_xp(68) > 0);
        assert_eq!(death_penalty_xp(68), xp_for_level(68) / 10);
    }

    #[test]
    fn effective_xp_no_deaths_equals_base() {
        assert_eq!(effective_xp_earned(50, 0), xp_for_level(50));
    }

    #[test]
    fn effective_xp_with_deaths_below_68_no_penalty() {
        // Below level 68, deaths don't add extra XP
        assert_eq!(effective_xp_earned(60, 3), xp_for_level(60));
    }

    #[test]
    fn effective_xp_with_deaths_at_68_adds_penalty() {
        let base = xp_for_level(68);
        let penalty = death_penalty_xp(68);
        assert_eq!(effective_xp_earned(68, 2), base + 2 * penalty);
    }

    #[test]
    fn experience_table_is_monotonically_increasing() {
        for i in 1..100 {
            assert!(
                EXPERIENCE_TABLE[i + 1] > EXPERIENCE_TABLE[i],
                "XP table not monotonic at level {}",
                i
            );
        }
    }
}
