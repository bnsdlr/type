use crate::monkeytype::Language;

#[derive(Clone, Copy, Debug)]
pub enum PunctuationKind {
    AfterNumber,
    BetweenWordsWithSpace,
    EndOfWord,
    Upcase,
    AroundWord,
    BetweenWordsWithoutSpace,
    OtherKinds,
}

pub enum PunctuationLanguage {
    Latin,
    Chinese,
    Japanese,
    Korean,
    ArabicPersian,
    Hebrew,
    Devanagari,
    Thai,
    Burmese,
    Khmer,
    Ethiopic,
    Armenian,
    Greek,
    Mongolian,
    None,
}

impl From<Language> for PunctuationLanguage {
    fn from(value: Language) -> Self {
        match value {
            Language::Afrikaans10k => Self::Latin,
            Language::Afrikaans1k => Self::Latin,
            Language::Afrikaans => Self::Latin,
            Language::Albanian1k => Self::Latin,
            Language::Albanian => Self::Latin,
            Language::Amharic1k => Self::Ethiopic,
            Language::Amharic5k => Self::Ethiopic,
            Language::Amharic => Self::Ethiopic,
            Language::Arabic10k => Self::ArabicPersian,
            Language::ArabicEgypt1k => Self::ArabicPersian,
            Language::ArabicEgypt => Self::ArabicPersian,
            Language::ArabicMorocco => Self::ArabicPersian,
            Language::Arabic => Self::ArabicPersian,
            Language::Armenian1k => Self::Armenian,
            Language::ArmenianWestern1k => Self::Armenian,
            Language::ArmenianWestern => Self::Armenian,
            Language::Armenian => Self::Armenian,
            Language::Azerbaijani1k => Self::Latin,
            Language::Azerbaijani => Self::Latin,
            Language::Bangla10k => Self::Devanagari, // Bengali uses danda/।। similar to Devanagari
            Language::BanglaLetters => Self::Devanagari,
            Language::Bangla => Self::Devanagari,
            Language::Belarusian100k => Self::Latin,
            Language::Belarusian10k => Self::Latin,
            Language::Belarusian1k => Self::Latin,
            Language::Belarusian25k => Self::Latin,
            Language::Belarusian50k => Self::Latin,
            Language::Belarusian5k => Self::Latin,
            Language::BelarusianLacinka1k => Self::Latin,
            Language::BelarusianLacinka => Self::Latin,
            Language::Belarusian => Self::Latin,
            Language::Bosnian4k => Self::Latin,
            Language::Bosnian => Self::Latin,
            Language::BulgarianLatin => Self::Latin,
            Language::Bulgarian => Self::Latin,
            Language::Catalan1k => Self::Latin,
            Language::Catalan => Self::Latin,
            Language::ChineseSimplified10k => Self::Chinese,
            Language::ChineseSimplified1k => Self::Chinese,
            Language::ChineseSimplified50k => Self::Chinese,
            Language::ChineseSimplified5k => Self::Chinese,
            Language::ChineseSimplified => Self::Chinese,
            Language::ChineseTraditional => Self::Chinese,
            Language::Croatian1k => Self::Latin,
            Language::Croatian => Self::Latin,
            Language::Czech10k => Self::Latin,
            Language::Czech1k => Self::Latin,
            Language::Czech => Self::Latin,
            Language::Danish10k => Self::Latin,
            Language::Danish1k => Self::Latin,
            Language::Danish => Self::Latin,
            Language::Dutch10k => Self::Latin,
            Language::Dutch1k => Self::Latin,
            Language::Dutch => Self::Latin,
            Language::English10k => Self::Latin,
            Language::English1k => Self::Latin,
            Language::English25k => Self::Latin,
            Language::English450k => Self::Latin,
            Language::English5k => Self::Latin,
            Language::EnglishCommonlyMisspelled => Self::Latin,
            Language::EnglishContractions => Self::Latin,
            Language::EnglishDoubleletter => Self::Latin,
            Language::EnglishMedical => Self::Latin,
            Language::EnglishOld => Self::Latin,
            Language::EnglishShakespearean => Self::Latin,
            Language::English => Self::Latin,
            Language::Esperanto10k => Self::Latin,
            Language::Esperanto1k => Self::Latin,
            Language::Esperanto25k => Self::Latin,
            Language::Esperanto36k => Self::Latin,
            Language::EsperantoHSistemo10k => Self::Latin,
            Language::EsperantoHSistemo1k => Self::Latin,
            Language::EsperantoHSistemo25k => Self::Latin,
            Language::EsperantoHSistemo36k => Self::Latin,
            Language::EsperantoHSistemo => Self::Latin,
            Language::EsperantoXSistemo10k => Self::Latin,
            Language::EsperantoXSistemo1k => Self::Latin,
            Language::EsperantoXSistemo25k => Self::Latin,
            Language::EsperantoXSistemo36k => Self::Latin,
            Language::EsperantoXSistemo => Self::Latin,
            Language::Esperanto => Self::Latin,
            Language::Estonian10k => Self::Latin,
            Language::Estonian1k => Self::Latin,
            Language::Estonian5k => Self::Latin,
            Language::Estonian => Self::Latin,
            Language::Euskera => Self::Latin,
            Language::Filipino1k => Self::Latin,
            Language::Filipino => Self::Latin,
            Language::Finnish10k => Self::Latin,
            Language::Finnish1k => Self::Latin,
            Language::Finnish => Self::Latin,
            Language::French10k => Self::Latin,
            Language::French1k => Self::Latin,
            Language::French2k => Self::Latin,
            Language::French600k => Self::Latin,
            Language::FrenchBitoduc => Self::Latin,
            Language::French => Self::Latin,
            Language::Frisian1k => Self::Latin,
            Language::Frisian => Self::Latin,
            Language::Galician => Self::Latin,
            Language::Georgian => Self::Latin,
            Language::German10k => Self::Latin,
            Language::German1k => Self::Latin,
            Language::German250k => Self::Latin,
            Language::German => Self::Latin,
            Language::Greek10k => Self::Greek,
            Language::Greek1k => Self::Greek,
            Language::Greek25k => Self::Greek,
            Language::Greek5k => Self::Greek,
            Language::Greek => Self::Greek,
            Language::Gujarati1k => Self::Devanagari,
            Language::Gujarati => Self::Devanagari,
            Language::Hausa1k => Self::Latin,
            Language::Hausa => Self::Latin,
            Language::Hawaiian1k => Self::Latin,
            Language::Hawaiian => Self::Latin,
            Language::Hebrew10k => Self::Hebrew,
            Language::Hebrew1k => Self::Hebrew,
            Language::Hebrew5k => Self::Hebrew,
            Language::Hebrew => Self::Hebrew,
            Language::Hindi1k => Self::Devanagari,
            Language::Hindi => Self::Devanagari,
            Language::Hungarian2k => Self::Latin,
            Language::Hungarian => Self::Latin,
            Language::Icelandic1k => Self::Latin,
            Language::Icelandic => Self::Latin,
            Language::Indonesian10k => Self::Latin,
            Language::Indonesian1k => Self::Latin,
            Language::Indonesian => Self::Latin,
            Language::Irish => Self::Latin,
            Language::Italian1k => Self::Latin,
            Language::Italian280k => Self::Latin,
            Language::Italian60k => Self::Latin,
            Language::Italian7k => Self::Latin,
            Language::Italian => Self::Latin,
            Language::JapaneseHiragana => Self::Japanese,
            Language::JapaneseKatakana => Self::Japanese,
            Language::JapaneseRomaji1k => Self::Latin,
            Language::JapaneseRomaji => Self::Latin,
            Language::Kazakh1k => Self::Latin,
            Language::Kazakh => Self::Latin,
            Language::Khmer => Self::Khmer,
            Language::Korean1k => Self::Korean,
            Language::Korean5k => Self::Korean,
            Language::Korean => Self::Korean,
            Language::KurdishCentral2k => Self::Latin,
            Language::KurdishCentral4k => Self::Latin,
            Language::KurdishCentral => Self::Latin,
            Language::Kyrgyz1k => Self::Latin,
            Language::Kyrgyz => Self::Latin,
            Language::Latin => Self::Latin,
            Language::Latvian1k => Self::Latin,
            Language::Latvian => Self::Latin,
            Language::Lithuanian1k => Self::Latin,
            Language::Lithuanian3k => Self::Latin,
            Language::Lithuanian => Self::Latin,
            Language::Macedonian10k => Self::Latin,
            Language::Macedonian1k => Self::Latin,
            Language::Macedonian75k => Self::Latin,
            Language::Macedonian => Self::Latin,
            Language::Malay1k => Self::Latin,
            Language::Malay => Self::Latin,
            Language::Maltese1k => Self::Latin,
            Language::Maltese => Self::Latin,
            Language::Maori1k => Self::Latin,
            Language::Marathi => Self::Devanagari,
            Language::Mongolian10k => Self::Mongolian,
            Language::Mongolian => Self::Mongolian,
            Language::MyanmarBurmese => Self::Burmese,
            Language::Nepali1k => Self::Devanagari,
            Language::NepaliRomanized => Self::Latin,
            Language::Nepali => Self::Devanagari,
            Language::NorwegianBokmal10k => Self::Latin,
            Language::NorwegianBokmal150k => Self::Latin,
            Language::NorwegianBokmal1k => Self::Latin,
            Language::NorwegianBokmal5k => Self::Latin,
            Language::NorwegianBokmal600k => Self::Latin,
            Language::NorwegianBokmal => Self::Latin,
            Language::NorwegianNynorsk100k => Self::Latin,
            Language::NorwegianNynorsk10k => Self::Latin,
            Language::NorwegianNynorsk1k => Self::Latin,
            Language::NorwegianNynorsk400k => Self::Latin,
            Language::NorwegianNynorsk5k => Self::Latin,
            Language::NorwegianNynorsk => Self::Latin,
            Language::Occitan10k => Self::Latin,
            Language::Occitan1k => Self::Latin,
            Language::Occitan2k => Self::Latin,
            Language::Occitan5k => Self::Latin,
            Language::Occitan => Self::Latin,
            Language::Pashto => Self::ArabicPersian,
            Language::Persian1k => Self::ArabicPersian,
            Language::Persian20k => Self::ArabicPersian,
            Language::Persian5k => Self::ArabicPersian,
            Language::PersianRomanized => Self::Latin,
            Language::Persian => Self::ArabicPersian,
            Language::Polish10k => Self::Latin,
            Language::Polish200k => Self::Latin,
            Language::Polish20k => Self::Latin,
            Language::Polish2k => Self::Latin,
            Language::Polish40k => Self::Latin,
            Language::Polish5k => Self::Latin,
            Language::Polish => Self::Latin,
            Language::Portuguese1k => Self::Latin,
            Language::Portuguese320k => Self::Latin,
            Language::Portuguese3k => Self::Latin,
            Language::Portuguese550k => Self::Latin,
            Language::Portuguese5k => Self::Latin,
            Language::PortugueseAcentosECedilha => Self::Latin,
            Language::Portuguese => Self::Latin,
            Language::Romanian100k => Self::Latin,
            Language::Romanian10k => Self::Latin,
            Language::Romanian1k => Self::Latin,
            Language::Romanian200k => Self::Latin,
            Language::Romanian25k => Self::Latin,
            Language::Romanian50k => Self::Latin,
            Language::Romanian5k => Self::Latin,
            Language::Romanian => Self::Latin,
            Language::Russian10k => Self::Latin,
            Language::Russian1k => Self::Latin,
            Language::Russian25k => Self::Latin,
            Language::Russian375k => Self::Latin,
            Language::Russian50k => Self::Latin,
            Language::Russian5k => Self::Latin,
            Language::RussianAbbreviations => Self::Latin,
            Language::RussianContractions1k => Self::Latin,
            Language::RussianContractions => Self::Latin,
            Language::Russian => Self::Latin,
            Language::SanskritRoman => Self::Latin,
            Language::Sanskrit => Self::Devanagari,
            Language::Serbian10k => Self::Latin,
            Language::SerbianLatin10k => Self::Latin,
            Language::SerbianLatin => Self::Latin,
            Language::Serbian => Self::Latin,
            Language::Slovak10k => Self::Latin,
            Language::Slovak1k => Self::Latin,
            Language::Slovak => Self::Latin,
            Language::Slovenian1k => Self::Latin,
            Language::Slovenian5k => Self::Latin,
            Language::Slovenian => Self::Latin,
            Language::Spanish10k => Self::Latin,
            Language::Spanish1k => Self::Latin,
            Language::Spanish650k => Self::Latin,
            Language::Spanish => Self::Latin,
            Language::Swahili1k => Self::Latin,
            Language::Swedish1k => Self::Latin,
            Language::SwedishDiacritics => Self::Latin,
            Language::Swedish => Self::Latin,
            Language::SwissGerman1k => Self::Latin,
            Language::SwissGerman2k => Self::Latin,
            Language::SwissGerman => Self::Latin,
            Language::Tamil1k => Self::Devanagari,
            Language::TamilOld => Self::Devanagari,
            Language::Tamil => Self::Devanagari,
            Language::Tanglish => Self::Latin,
            Language::Tatar1k => Self::Latin,
            Language::Tatar5k => Self::Latin,
            Language::Tatar9k => Self::Latin,
            Language::TatarCrimean10k => Self::Latin,
            Language::TatarCrimean15k => Self::Latin,
            Language::TatarCrimean1k => Self::Latin,
            Language::TatarCrimean5k => Self::Latin,
            Language::TatarCrimeanCyrillic10k => Self::Latin,
            Language::TatarCrimeanCyrillic15k => Self::Latin,
            Language::TatarCrimeanCyrillic1k => Self::Latin,
            Language::TatarCrimeanCyrillic5k => Self::Latin,
            Language::TatarCrimeanCyrillic => Self::Latin,
            Language::TatarCrimean => Self::Latin,
            Language::Tatar => Self::Latin,
            Language::Telugu1k => Self::Devanagari,
            Language::Telugu => Self::Devanagari,
            Language::Thai10k => Self::Thai,
            Language::Thai1k => Self::Thai,
            Language::Thai20k => Self::Thai,
            Language::Thai50k => Self::Thai,
            Language::Thai5k => Self::Thai,
            Language::Thai60k => Self::Thai,
            Language::Thai => Self::Thai,
            Language::Tibetan1k => Self::Devanagari,
            Language::Tibetan => Self::Devanagari,
            Language::Turkish1k => Self::Latin,
            Language::Turkish5k => Self::Latin,
            Language::Turkish => Self::Latin,
            Language::Udmurt => Self::Latin,
            Language::Ukrainian10k => Self::Latin,
            Language::Ukrainian1k => Self::Latin,
            Language::Ukrainian50k => Self::Latin,
            Language::UkrainianEndings => Self::Latin,
            Language::UkrainianLatynka10k => Self::Latin,
            Language::UkrainianLatynka1k => Self::Latin,
            Language::UkrainianLatynka50k => Self::Latin,
            Language::UkrainianLatynkaEndings => Self::Latin,
            Language::UkrainianLatynka => Self::Latin,
            Language::Ukrainian => Self::Latin,
            Language::Urdish => Self::ArabicPersian,
            Language::Urdu1k => Self::ArabicPersian,
            Language::Urdu5k => Self::ArabicPersian,
            Language::Urdu => Self::ArabicPersian,
            Language::Uzbek1k => Self::Latin,
            Language::Uzbek70k => Self::Latin,
            Language::Uzbek => Self::Latin,
            Language::Vietnamese1k => Self::Latin,
            Language::Vietnamese5k => Self::Latin,
            Language::Vietnamese => Self::Latin,
            Language::Welsh1k => Self::Latin,
            Language::Welsh => Self::Latin,
            Language::Xhosa3k => Self::Latin,
            Language::Xhosa => Self::Latin,
            Language::Yiddish => Self::Hebrew,
            Language::Yoruba1k => Self::Latin,
            Language::Zulu => Self::Latin,
            _ => Self::None,
        }
    }
}

// END_OF_WORD
const LATIN_END_OF_WORD: &[char] = &['.', ',', ';', ':', '?', '!', '-'];
const CHINESE_END_OF_WORD: &[char] = &['。', '、'];
const JAPANESE_END_OF_WORD: &[char] = &['。', '、'];
const KOREAN_END_OF_WORD: &[char] = &['.', ',', '?', '!'];
const ARABICPERSIAN_END_OF_WORD: &[char] = &['،', '؛', '؟'];
const HEBREW_END_OF_WORD: &[char] = &['׃'];
const DEVANAGARI_END_OF_WORD: &[char] = &['।', '॥'];
const THAI_END_OF_WORD: &[char] = &['ฯ'];
const BURMESE_END_OF_WORD: &[char] = &['၊', '။'];
const KHMER_END_OF_WORD: &[char] = &['។', '៖'];
const ETHIOPIC_END_OF_WORD: &[char] = &['።', '፣'];
const ARMENIAN_END_OF_WORD: &[char] = &['։', '՝', '՞'];
const GREEK_END_OF_WORD: &[char] = &['.', ',', ';', '·'];
const MONGOLIAN_END_OF_WORD: &[char] = &['᠂', '᠃'];

// AROUND_WORD
const LATIN_AROUND_WORD: &[char] = &['\'', '"', '(', ')', '[', ']', '{', '}', '<', '>'];
const CHINESE_AROUND_WORD: &[char] = &['「', '」', '『', '』', '《', '》'];
const JAPANESE_AROUND_WORD: &[char] = &['「', '」', '『', '』', '《', '》'];
const ARABICPERSIAN_AROUND_WORD: &[char] = &['«', '»'];
const HEBREW_AROUND_WORD: &[char] = &['«', '»', '„', '”'];
const DEVANAGARI_AROUND_WORD: &[char] = &['(', ')'];
const ETHIOPIC_AROUND_WORD: &[char] = &['፨'];
const ARMENIAN_AROUND_WORD: &[char] = &['«', '»'];
const GREEK_AROUND_WORD: &[char] = &['«', '»', '“', '”'];
const MONGOLIAN_AROUND_WORD: &[char] = &['᠁'];

// AFTER_NUMBER
const LATIN_AFTER_NUMBER: &[char] = &['%'];
const ARABICPERSIAN_AFTER_NUMBER: &[char] = &['٪'];

// BETWEEN_WORDS_WITH_SPACE
const LATIN_BETWEEN_WORDS_WITH_SPACE: &[char] = &['–', '&'];
const KOREAN_BETWEEN_WORDS_WITH_SPACE: &[char] = &['·'];

// BETWEEN_WORDS_WITHOUT_SPACE
const LATIN_BETWEEN_WORDS_WITHOUT_SPACE: &[char] = &['-', '/'];
const THAI_BETWEEN_WORDS_WITHOUT_SPACE: &[char] = &['ๆ'];
const KHMER_BETWEEN_WORDS_WITHOUT_SPACE: &[char] = &['々'];

// OTHER_KINDS
const LATIN_OTHER_KINDS: &[char] = &['@', '#', '*', '^', '_'];
const CHINESE_OTHER_KINDS: &[char] = &['〜'];
const ETHIOPIC_OTHER_KINDS: &[char] = &['፨'];
const ARMENIAN_OTHER_KINDS: &[char] = &['՛'];
const GREEK_OTHER_KINDS: &[char] = &['·'];

pub fn get_punctuation(p_kind: PunctuationKind, p_lang: PunctuationLanguage) -> &'static [char] {
    match p_kind {
        PunctuationKind::EndOfWord => match p_lang {
            PunctuationLanguage::Latin => LATIN_END_OF_WORD,
            PunctuationLanguage::Chinese => CHINESE_END_OF_WORD,
            PunctuationLanguage::Japanese => JAPANESE_END_OF_WORD,
            PunctuationLanguage::Korean => KOREAN_END_OF_WORD,
            PunctuationLanguage::ArabicPersian => ARABICPERSIAN_END_OF_WORD,
            PunctuationLanguage::Hebrew => HEBREW_END_OF_WORD,
            PunctuationLanguage::Devanagari => DEVANAGARI_END_OF_WORD,
            PunctuationLanguage::Thai => THAI_END_OF_WORD,
            PunctuationLanguage::Burmese => BURMESE_END_OF_WORD,
            PunctuationLanguage::Khmer => KHMER_END_OF_WORD,
            PunctuationLanguage::Ethiopic => ETHIOPIC_END_OF_WORD,
            PunctuationLanguage::Armenian => ARMENIAN_END_OF_WORD,
            PunctuationLanguage::Greek => GREEK_END_OF_WORD,
            PunctuationLanguage::Mongolian => MONGOLIAN_END_OF_WORD,
            _ => &[],
        },

        PunctuationKind::AroundWord => match p_lang {
            PunctuationLanguage::Latin => LATIN_AROUND_WORD,
            PunctuationLanguage::Chinese => CHINESE_AROUND_WORD,
            PunctuationLanguage::Japanese => JAPANESE_AROUND_WORD,
            PunctuationLanguage::ArabicPersian => ARABICPERSIAN_AROUND_WORD,
            PunctuationLanguage::Hebrew => HEBREW_AROUND_WORD,
            PunctuationLanguage::Devanagari
            | PunctuationLanguage::Thai
            | PunctuationLanguage::Burmese
            | PunctuationLanguage::Khmer => DEVANAGARI_AROUND_WORD,
            PunctuationLanguage::Ethiopic => ETHIOPIC_AROUND_WORD,
            PunctuationLanguage::Armenian => ARMENIAN_AROUND_WORD,
            PunctuationLanguage::Greek => GREEK_AROUND_WORD,
            PunctuationLanguage::Mongolian => MONGOLIAN_AROUND_WORD,
            _ => &[],
        },

        PunctuationKind::AfterNumber => match p_lang {
            PunctuationLanguage::Latin => LATIN_AFTER_NUMBER,
            PunctuationLanguage::ArabicPersian => ARABICPERSIAN_AFTER_NUMBER,
            _ => &[],
        },

        PunctuationKind::BetweenWordsWithSpace => match p_lang {
            PunctuationLanguage::Latin => LATIN_BETWEEN_WORDS_WITH_SPACE,
            PunctuationLanguage::Korean => KOREAN_BETWEEN_WORDS_WITH_SPACE,
            _ => &[],
        },

        PunctuationKind::BetweenWordsWithoutSpace => match p_lang {
            PunctuationLanguage::Latin => LATIN_BETWEEN_WORDS_WITHOUT_SPACE,
            PunctuationLanguage::Thai => THAI_BETWEEN_WORDS_WITHOUT_SPACE,
            PunctuationLanguage::Khmer => KHMER_BETWEEN_WORDS_WITHOUT_SPACE,
            _ => &[],
        },
        PunctuationKind::OtherKinds => match p_lang {
            PunctuationLanguage::Latin => LATIN_OTHER_KINDS,
            PunctuationLanguage::Chinese
            | PunctuationLanguage::Japanese
            | PunctuationLanguage::Korean => CHINESE_OTHER_KINDS,
            PunctuationLanguage::Ethiopic => ETHIOPIC_OTHER_KINDS,
            PunctuationLanguage::Armenian => ARMENIAN_OTHER_KINDS,
            PunctuationLanguage::Greek => GREEK_OTHER_KINDS,
            _ => &[],
        },
        _ => &[],
    }
}

pub fn apply(
    words: Vec<String>,
    language: Language,
    mut kinds: Vec<PunctuationKind>,
    percentage: usize,
) -> Vec<String> {
    let punctuation_language = PunctuationLanguage::from(language);
    let apply_count = words.len() as f32 * percentage.min(100) as f32;
    let mut applied = 0;

    kinds.sort_by_key(|k| *k as usize);

    for kind in kinds {
        match kind {
            PunctuationKind::AfterNumber => todo!(),
            PunctuationKind::BetweenWordsWithSpace => todo!(),
            PunctuationKind::EndOfWord => todo!(),
            PunctuationKind::Upcase => todo!(),
            PunctuationKind::AroundWord => todo!(),
            PunctuationKind::BetweenWordsWithoutSpace => todo!(),
            PunctuationKind::OtherKinds => todo!(),
        }
    }

    Vec::new()
}
