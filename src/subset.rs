use harfbuzz_bindings::{hb_face_reference_blob, hb_set_add, hb_subset_input_create_or_fail, hb_subset_input_destroy, hb_subset_input_unicode_set, hb_subset_or_fail};

use crate::{Blob, Font, HarfbuzzObject};

// TODO Pass in a set of glyph indices to subset + further settings
// TODO Check https://github.com/sile-typesetter/sile/blob/master/src/justenoughharfbuzz.c for an example on how to pin axes of variable fonts (or https://github.com/ImageMagick/harfbuzz/blob/ad59dba8ad7be4ebbd58de287898aaee7c1f74ef/test/api/test-instance-cff2.c)
pub fn subset(font: &Font<'_>, codepoints: Vec<u32>) -> Vec<u8> {
    let font_face = font.face();

    unsafe {
        // Adding glyph indices and further subsetting settings
        let input = hb_subset_input_create_or_fail();
        let unicode_set = hb_subset_input_unicode_set(input);
        for codepoint in codepoints {
            hb_set_add(unicode_set, codepoint);
        }

        // TODO Pin axis of variable fonts

        let raw_subset_font_face = hb_subset_or_fail(font_face.as_raw(), input);
        let raw_blob = hb_face_reference_blob(raw_subset_font_face);
        let blob = Blob::from_raw(raw_blob);

        // Cleanup
        hb_subset_input_destroy(input);

        return blob.to_vec();
    }
}
