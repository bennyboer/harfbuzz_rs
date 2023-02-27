use harfbuzz_bindings::{hb_face_destroy, hb_set_add, hb_subset_input_create_or_fail, hb_subset_input_destroy, hb_subset_input_unicode_set, hb_subset_or_fail};

use crate::{Face, Font, HarfbuzzObject};

// TODO Pass in a set of glyph indices to subset + further settings
// TODO Check https://github.com/sile-typesetter/sile/blob/master/src/justenoughharfbuzz.c for an example on how to pin axes of variable fonts (or https://github.com/ImageMagick/harfbuzz/blob/ad59dba8ad7be4ebbd58de287898aaee7c1f74ef/test/api/test-instance-cff2.c)
pub fn subset<'a, 'b>(font: &Font<'a>) -> Face<'b> {
    let font_face = font.face();

    unsafe {
        // Adding glyph indices and further subsetting settings
        let input = hb_subset_input_create_or_fail();
        let unicode_set = hb_subset_input_unicode_set(input);
        hb_set_add(unicode_set, 97);
        hb_set_add(unicode_set, 98);
        hb_set_add(unicode_set, 99);

        // TODO Pin axis of variable fonts

        let raw_subset_font_face = hb_subset_or_fail(font_face.as_raw(), input);

        // Cleanup input
        hb_subset_input_destroy(input);

        // Fetch resulting BLOB
        let font_face = Face::from_raw(raw_subset_font_face);

        // Cleanup subset
        hb_face_destroy(raw_subset_font_face);

        return font_face;
    }
}
