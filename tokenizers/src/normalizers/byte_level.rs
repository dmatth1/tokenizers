use crate::processors::byte_level::bytes_char;
use crate::tokenizer::{NormalizedString, Normalizer, Result};
use crate::utils::macro_rules_attribute;
use ahash::AHashSet;
use std::sync::LazyLock;

#[derive(Clone, Debug)]
#[macro_rules_attribute(impl_serde_type!)]
pub struct ByteLevel;

// `u8 -> char` is a dense 256-entry map, so a flat array (O(1) index, no hashing) is much
// faster than a hash map, and lets `normalize` use the single-pass
// `NormalizedString::apply_byte_map` fast path instead of the general transform machinery.
static BYTES_CHAR: LazyLock<[char; 256]> = LazyLock::new(|| {
    let mut table = ['\0'; 256];
    for (b, c) in bytes_char() {
        table[b as usize] = c;
    }
    table
});

impl Default for ByteLevel {
    fn default() -> Self {
        Self::new()
    }
}

impl ByteLevel {
    pub fn new() -> Self {
        Self {}
    }

    pub fn alphabet() -> AHashSet<char> {
        BYTES_CHAR.iter().copied().collect()
    }
}

impl Normalizer for ByteLevel {
    /// Strip the normalized string inplace
    fn normalize(&self, normalized: &mut NormalizedString) -> Result<()> {
        normalized.apply_byte_map(&BYTES_CHAR);
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_byte_level_normalize() {
        let original = "Hello 我今天能为你做什么";
        let normalized = "HelloĠæĪĳä»Ĭå¤©èĥ½ä¸ºä½łåģļä»Ģä¹Ī";
        assert_ne!(original, normalized);
        let mut n = NormalizedString::from(original);
        let byte_level = ByteLevel::new();
        byte_level.normalize(&mut n).unwrap();
        assert_eq!(&n.get(), &normalized);
        assert_eq!(
            n,
            NormalizedString::new(
                original.to_string(),
                normalized.to_string(),
                vec![
                    (0, 1),
                    (1, 2),
                    (2, 3),
                    (3, 4),
                    (4, 5),
                    (5, 6),
                    (5, 6),
                    (6, 9),
                    (6, 9),
                    (6, 9),
                    (6, 9),
                    (6, 9),
                    (6, 9),
                    (9, 12),
                    (9, 12),
                    (9, 12),
                    (9, 12),
                    (9, 12),
                    (9, 12),
                    (12, 15),
                    (12, 15),
                    (12, 15),
                    (12, 15),
                    (12, 15),
                    (12, 15),
                    (15, 18),
                    (15, 18),
                    (15, 18),
                    (15, 18),
                    (15, 18),
                    (15, 18),
                    (18, 21),
                    (18, 21),
                    (18, 21),
                    (18, 21),
                    (18, 21),
                    (18, 21),
                    (21, 24),
                    (21, 24),
                    (21, 24),
                    (21, 24),
                    (21, 24),
                    (21, 24),
                    (24, 27),
                    (24, 27),
                    (24, 27),
                    (24, 27),
                    (24, 27),
                    (24, 27),
                    (27, 30),
                    (27, 30),
                    (27, 30),
                    (27, 30),
                    (27, 30),
                    (27, 30),
                    (30, 33),
                    (30, 33),
                    (30, 33),
                    (30, 33),
                    (30, 33),
                    (30, 33)
                ],
                0
            )
        );
        assert_eq!(
            n.alignments_original(),
            vec![
                (0, 1),
                (1, 2),
                (2, 3),
                (3, 4),
                (4, 5),
                (5, 7),
                (7, 13),
                (7, 13),
                (7, 13),
                (13, 19),
                (13, 19),
                (13, 19),
                (19, 25),
                (19, 25),
                (19, 25),
                (25, 31),
                (25, 31),
                (25, 31),
                (31, 37),
                (31, 37),
                (31, 37),
                (37, 43),
                (37, 43),
                (37, 43),
                (43, 49),
                (43, 49),
                (43, 49),
                (49, 55),
                (49, 55),
                (49, 55),
                (55, 61),
                (55, 61),
                (55, 61)
            ]
        );
    }
}
