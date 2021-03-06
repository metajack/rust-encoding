// AUTOGENERATED FROM index-iso-8859-16.txt, ORIGINAL COMMENT FOLLOWS:
//
// Any copyright is dedicated to the Public Domain.
// http://creativecommons.org/publicdomain/zero/1.0/
//
// For details on index-iso-8859-16.txt see the Encoding Standard
// http://encoding.spec.whatwg.org/

static FORWARD_TABLE: &'static [u16] = &[
    128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142,
    143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157,
    158, 159, 160, 260, 261, 321, 8364, 8222, 352, 167, 353, 169, 536, 171,
    377, 173, 378, 379, 176, 177, 268, 322, 381, 8221, 182, 183, 382, 269, 537,
    187, 338, 339, 376, 380, 192, 193, 194, 258, 196, 262, 198, 199, 200, 201,
    202, 203, 204, 205, 206, 207, 272, 323, 210, 211, 212, 336, 214, 346, 368,
    217, 218, 219, 220, 280, 538, 223, 224, 225, 226, 259, 228, 263, 230, 231,
    232, 233, 234, 235, 236, 237, 238, 239, 273, 324, 242, 243, 244, 337, 246,
    347, 369, 249, 250, 251, 252, 281, 539, 255,
];

#[inline]
pub fn forward(code: u8) -> u16 {
    FORWARD_TABLE[(code - 0x80) as uint]
}

pub fn backward(code: u16) -> u8 {
    match code {
        128 => 128, 129 => 129, 130 => 130, 131 => 131, 132 => 132, 133 => 133,
        134 => 134, 135 => 135, 136 => 136, 137 => 137, 138 => 138, 139 => 139,
        140 => 140, 141 => 141, 142 => 142, 143 => 143, 144 => 144, 145 => 145,
        146 => 146, 147 => 147, 148 => 148, 149 => 149, 150 => 150, 151 => 151,
        152 => 152, 153 => 153, 154 => 154, 155 => 155, 156 => 156, 157 => 157,
        158 => 158, 159 => 159, 160 => 160, 260 => 161, 261 => 162, 321 => 163,
        8364 => 164, 8222 => 165, 352 => 166, 167 => 167, 353 => 168,
        169 => 169, 536 => 170, 171 => 171, 377 => 172, 173 => 173, 378 => 174,
        379 => 175, 176 => 176, 177 => 177, 268 => 178, 322 => 179, 381 => 180,
        8221 => 181, 182 => 182, 183 => 183, 382 => 184, 269 => 185,
        537 => 186, 187 => 187, 338 => 188, 339 => 189, 376 => 190, 380 => 191,
        192 => 192, 193 => 193, 194 => 194, 258 => 195, 196 => 196, 262 => 197,
        198 => 198, 199 => 199, 200 => 200, 201 => 201, 202 => 202, 203 => 203,
        204 => 204, 205 => 205, 206 => 206, 207 => 207, 272 => 208, 323 => 209,
        210 => 210, 211 => 211, 212 => 212, 336 => 213, 214 => 214, 346 => 215,
        368 => 216, 217 => 217, 218 => 218, 219 => 219, 220 => 220, 280 => 221,
        538 => 222, 223 => 223, 224 => 224, 225 => 225, 226 => 226, 259 => 227,
        228 => 228, 263 => 229, 230 => 230, 231 => 231, 232 => 232, 233 => 233,
        234 => 234, 235 => 235, 236 => 236, 237 => 237, 238 => 238, 239 => 239,
        273 => 240, 324 => 241, 242 => 242, 243 => 243, 244 => 244, 337 => 245,
        246 => 246, 347 => 247, 369 => 248, 249 => 249, 250 => 250, 251 => 251,
        252 => 252, 281 => 253, 539 => 254, 255 => 255, _ => 0
    }
}

#[cfg(test)]
mod tests {
    use super::{forward, backward};

    #[test]
    fn test_correct_table() {
        for i in range(128, 256) {
            let i = i as u8;
            let j = forward(i);
            if j != 0xffff { assert_eq!(backward(j), i); }
        }
    }
}
