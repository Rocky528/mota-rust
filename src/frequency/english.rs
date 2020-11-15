use crate::xor_with;
use crate::frequency::*;

pub fn get_best_xor_key(v: &[u8], english_char_frequency: &CharFrequency) -> (u8, f64) {
    let mut best_key: u8 = 0;
    let mut best_freq_comparison: f64 = std::f64::MAX;

    for key in 0..=255 {
        let decoded = xor_with(&v, key);
        let char_frequency = get_char_frequency(&decoded);
        let freq_comparison = compare_frequencies(&char_frequency, &english_char_frequency);

        if freq_comparison < best_freq_comparison {
            best_key = key;
            best_freq_comparison = freq_comparison;
        }
    }

    (best_key, best_freq_comparison)
}

pub fn get_english_char_frequency() -> types::CharFrequency {
    let mut result = [0.0; 256];

    result[9] = 0.0057 / 100.0;
    result[23] = 0.0000 / 100.0;
    result[32] = 17.1662 / 100.0;
    result[33] = 0.0072 / 100.0;
    result[34] = 0.2442 / 100.0;
    result[35] = 0.0179 / 100.0;
    result[36] = 0.0561 / 100.0;
    result[37] = 0.0160 / 100.0;
    result[38] = 0.0226 / 100.0;
    result[39] = 0.2447 / 100.0;
    result[40] = 0.2178 / 100.0;
    result[41] = 0.2233 / 100.0;
    result[42] = 0.0628 / 100.0;
    result[43] = 0.0215 / 100.0;
    result[44] = 0.7384 / 100.0;
    result[45] = 1.3734 / 100.0;
    result[46] = 1.5124 / 100.0;
    result[47] = 0.1549 / 100.0;
    result[48] = 0.5516 / 100.0;
    result[49] = 0.4594 / 100.0;
    result[50] = 0.3322 / 100.0;
    result[51] = 0.1847 / 100.0;
    result[52] = 0.1348 / 100.0;
    result[53] = 0.1663 / 100.0;
    result[54] = 0.1153 / 100.0;
    result[55] = 0.1030 / 100.0;
    result[56] = 0.1054 / 100.0;
    result[57] = 0.1024 / 100.0;
    result[58] = 0.4354 / 100.0;
    result[59] = 0.1214 / 100.0;
    result[60] = 0.1225 / 100.0;
    result[61] = 0.0227 / 100.0;
    result[62] = 0.1242 / 100.0;
    result[63] = 0.1474 / 100.0;
    result[64] = 0.0073 / 100.0;
    result[65] = 0.3132 / 100.0;
    result[66] = 0.2163 / 100.0;
    result[67] = 0.3906 / 100.0;
    result[68] = 0.3151 / 100.0;
    result[69] = 0.2673 / 100.0;
    result[70] = 0.1416 / 100.0;
    result[71] = 0.1876 / 100.0;
    result[72] = 0.2321 / 100.0;
    result[73] = 0.3211 / 100.0;
    result[74] = 0.1726 / 100.0;
    result[75] = 0.0687 / 100.0;
    result[76] = 0.1884 / 100.0;
    result[77] = 0.3529 / 100.0;
    result[78] = 0.2085 / 100.0;
    result[79] = 0.1842 / 100.0;
    result[80] = 0.2614 / 100.0;
    result[81] = 0.0316 / 100.0;
    result[82] = 0.2519 / 100.0;
    result[83] = 0.4003 / 100.0;
    result[84] = 0.3322 / 100.0;
    result[85] = 0.0814 / 100.0;
    result[86] = 0.0892 / 100.0;
    result[87] = 0.2527 / 100.0;
    result[88] = 0.0343 / 100.0;
    result[89] = 0.0304 / 100.0;
    result[90] = 0.0076 / 100.0;
    result[91] = 0.0086 / 100.0;
    result[92] = 0.0016 / 100.0;
    result[93] = 0.0088 / 100.0;
    result[94] = 0.0003 / 100.0;
    result[95] = 0.1159 / 100.0;
    result[96] = 0.0009 / 100.0;
    result[97] = 5.1880 / 100.0;
    result[98] = 1.0195 / 100.0;
    result[99] = 2.1129 / 100.0;
    result[100] = 2.5071 / 100.0;
    result[101] = 8.5771 / 100.0;
    result[102] = 1.3725 / 100.0;
    result[103] = 1.5597 / 100.0;
    result[104] = 2.7444 / 100.0;
    result[105] = 4.9019 / 100.0;
    result[106] = 0.0867 / 100.0;
    result[107] = 0.6753 / 100.0;
    result[108] = 3.1750 / 100.0;
    result[109] = 1.6437 / 100.0;
    result[110] = 4.9701 / 100.0;
    result[111] = 5.7701 / 100.0;
    result[112] = 1.5482 / 100.0;
    result[113] = 0.0747 / 100.0;
    result[114] = 4.2586 / 100.0;
    result[115] = 4.3686 / 100.0;
    result[116] = 6.3700 / 100.0;
    result[117] = 2.0999 / 100.0;
    result[118] = 0.8462 / 100.0;
    result[119] = 1.3034 / 100.0;
    result[120] = 0.1950 / 100.0;
    result[121] = 1.1330 / 100.0;
    result[122] = 0.0596 / 100.0;
    result[123] = 0.0026 / 100.0;
    result[124] = 0.0007 / 100.0;
    result[125] = 0.0026 / 100.0;
    result[126] = 0.0003 / 100.0;
    result[131] = 0.0000 / 100.0;
    result[149] = 0.6410 / 100.0;
    result[183] = 0.0010 / 100.0;
    result[223] = 0.0000 / 100.0;
    result[226] = 0.0000 / 100.0;
    result[229] = 0.0000 / 100.0;
    result[230] = 0.0000 / 100.0;
    result[237] = 0.0000 / 100.0;

    result
}
