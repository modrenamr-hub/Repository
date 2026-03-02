use ahash::AHashMap;

#[derive(Debug)]
pub struct LayoutConverter {
    en_to_ar: AHashMap<char, char>,
    ar_to_en: AHashMap<char, char>,
}

impl Default for LayoutConverter {
    fn default() -> Self {
        let mapping: &[(char, char)] = &[
            ('q', 'ض'),
            ('w', 'ص'),
            ('e', 'ث'),
            ('r', 'ق'),
            ('t', 'ف'),
            ('y', 'غ'),
            ('u', 'ع'),
            ('i', 'ه'),
            ('o', 'خ'),
            ('p', 'ح'),
            ('a', 'ش'),
            ('s', 'س'),
            ('d', 'ي'),
            ('f', 'ب'),
            ('g', 'ل'),
            ('h', 'ا'),
            ('j', 'ت'),
            ('k', 'ن'),
            ('l', 'م'),
            ('z', 'ئ'),
            ('x', 'ء'),
            ('c', 'ؤ'),
            ('v', 'ر'),
            ('b', 'ل'),
            ('n', 'ى'),
            ('m', 'ة'),
        ];
        let mut en_to_ar = AHashMap::new();
        let mut ar_to_en = AHashMap::new();

        for (en, ar) in mapping {
            en_to_ar.insert(*en, *ar);
            en_to_ar.insert(en.to_ascii_uppercase(), *ar);
            ar_to_en.insert(*ar, *en);
        }

        Self { en_to_ar, ar_to_en }
    }
}

impl LayoutConverter {
    pub fn convert_en_to_ar(&self, input: &str) -> String {
        input
            .chars()
            .map(|c| self.en_to_ar.get(&c).copied().unwrap_or(c))
            .collect()
    }

    pub fn convert_ar_to_en(&self, input: &str) -> String {
        input
            .chars()
            .map(|c| self.ar_to_en.get(&c).copied().unwrap_or(c))
            .collect()
    }
}
