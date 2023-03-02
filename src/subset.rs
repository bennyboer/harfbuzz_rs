use std::collections::HashMap;
use std::ffi::c_uint;
use std::ptr;

use harfbuzz_bindings::{hb_face_reference_blob, hb_ot_var_axis_info_t, hb_ot_var_get_axis_infos, hb_set_add, HB_SUBSET_FLAGS_GLYPH_NAMES, HB_SUBSET_FLAGS_NAME_LEGACY, HB_SUBSET_FLAGS_NO_PRUNE_UNICODE_RANGES, HB_SUBSET_FLAGS_RETAIN_GIDS, hb_subset_input_create_or_fail, hb_subset_input_destroy, hb_subset_input_glyph_set, hb_subset_input_pin_axis_location, hb_subset_input_pin_axis_to_default, hb_subset_input_set_flags, hb_subset_or_fail};

use crate::{Blob, Font, HarfbuzzObject, Tag, Variation};

// TODO Pass in a set of glyph indices to subset + further settings
// TODO Check https://github.com/sile-typesetter/sile/blob/master/src/justenoughharfbuzz.c for an example on how to pin axes of variable fonts (or https://github.com/ImageMagick/harfbuzz/blob/ad59dba8ad7be4ebbd58de287898aaee7c1f74ef/test/api/test-instance-cff2.c)
pub fn subset(font: &Font<'_>, codepoints: &[u32], variations: &[Variation]) -> Vec<u8> {
    let font_face = font.face();

    let mut set_variations_lookup = HashMap::new();
    for variation in variations {
        set_variations_lookup.insert(variation.tag(), variation.value());
    }

    unsafe {
        // Adding glyph indices and further subsetting settings
        let input = hb_subset_input_create_or_fail();

        // Configure subsetter
        hb_subset_input_set_flags(
            input,
            (HB_SUBSET_FLAGS_RETAIN_GIDS |
                HB_SUBSET_FLAGS_NAME_LEGACY |
                HB_SUBSET_FLAGS_GLYPH_NAMES |
                HB_SUBSET_FLAGS_NO_PRUNE_UNICODE_RANGES) as c_uint,
        );

        // Adding codepoints
        let glyph_set = hb_subset_input_glyph_set(input);
        for codepoint in codepoints {
            hb_set_add(glyph_set, *codepoint);
        }

        // Pin axes of variable fonts
        {
            // Fetch all axes in the font
            let mut axes_count = hb_ot_var_get_axis_infos(font_face.as_raw(), 0, ptr::null_mut(), ptr::null_mut());
            let axis_info_size = std::mem::size_of::<hb_ot_var_axis_info_t>();
            let mut axes = Vec::with_capacity(axes_count as usize * axis_info_size);
            hb_ot_var_get_axis_infos(font_face.as_raw(), 0, &mut axes_count, axes.as_mut_ptr());
            axes.set_len(axes_count as usize);

            // Pin all axes
            for axis in axes.iter() {
                let axis_tag = axis.tag;
                let tag = Tag(axis_tag);

                if let Some(value) = set_variations_lookup.get(&tag) {
                    hb_subset_input_pin_axis_location(input, font_face.as_raw(), axis_tag, *value);
                } else {
                    hb_subset_input_pin_axis_to_default(input, font_face.as_raw(), axis_tag);
                }
            }
        }

        let raw_subset_font_face = hb_subset_or_fail(font_face.as_raw(), input);
        let raw_blob = hb_face_reference_blob(raw_subset_font_face);
        let blob = Blob::from_raw(raw_blob);

        // Cleanup
        hb_subset_input_destroy(input);

        return blob.to_vec();
    }
}
