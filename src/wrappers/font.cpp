#include <ft2build.h>
#include FT_FREETYPE_H
#include FT_GLYPH_H
#include <hb.h>
#include <hb-ft.h>
#include <cstring>
#include <cstdint>
#include <cstdlib>

static FT_Library g_ft_library = nullptr;

struct GlyphInfo {
    uint32_t glyph_id;
    uint32_t cluster;
    int32_t x_advance;
    int32_t y_advance;
    int32_t x_offset;
    int32_t y_offset;
};

extern "C" {

int font_init() {
    if (g_ft_library) return 0;
    return FT_Init_FreeType(&g_ft_library) == 0 ? 0 : -1;
}

void* font_load_face(const char* path, int index) {
    if (!g_ft_library) font_init();
    if (!g_ft_library) return nullptr;
    FT_Face face;
    if (FT_New_Face(g_ft_library, path, index, &face) != 0) return nullptr;
    return (void*)face;
}

void font_done_face(void* face) {
    if (face) FT_Done_Face((FT_Face)face);
}

int font_set_size(void* face, int size, int dpi) {
    if (!face) return -1;
    return FT_Set_Char_Size((FT_Face)face, 0, size * 64, dpi, dpi) == 0 ? 0 : -1;
}

int font_get_glyph(void* face, uint32_t charcode, int32_t* width, int32_t* height,
                   int32_t* bearing_x, int32_t* bearing_y, int32_t* advance,
                   unsigned char** bitmap, int32_t* bitmap_size) {
    if (!face) return -1;
    FT_Face f = (FT_Face)face;
    if (FT_Load_Char(f, charcode, FT_LOAD_RENDER) != 0) return -1;
    *width = f->glyph->bitmap.width;
    *height = f->glyph->bitmap.rows;
    *bearing_x = f->glyph->bitmap_left;
    *bearing_y = f->glyph->bitmap_top;
    *advance = (int32_t)(f->glyph->advance.x >> 6);
    if (f->glyph->bitmap.buffer && f->glyph->bitmap.width > 0 && f->glyph->bitmap.rows > 0) {
        int sz = f->glyph->bitmap.width * f->glyph->bitmap.rows;
        unsigned char* buf = (unsigned char*)malloc(sz);
        if (buf) {
            memcpy(buf, f->glyph->bitmap.buffer, sz);
            *bitmap = buf;
            *bitmap_size = sz;
        } else {
            *bitmap = nullptr;
            *bitmap_size = 0;
        }
    } else {
        *bitmap = nullptr;
        *bitmap_size = 0;
    }
    return 0;
}

int font_get_kerning(void* face, uint32_t left, uint32_t right, int32_t* x, int32_t* y) {
    if (!face) return -1;
    FT_Face f = (FT_Face)face;
    FT_Vector kerning;
    if (FT_Get_Kerning(f, left, right, FT_KERNING_DEFAULT, &kerning) != 0) return -1;
    *x = kerning.x >> 6;
    *y = kerning.y >> 6;
    return 0;
}

const char* font_get_name(void* face) {
    if (!face) return "";
    return ((FT_Face)face)->family_name ? ((FT_Face)face)->family_name : "";
}

int font_get_num_glyphs(void* face) {
    if (!face) return 0;
    return (int)((FT_Face)face)->num_glyphs;
}

void* ry_hb_create_font(void* face) {
    if (!face) return nullptr;
    return (void*)hb_ft_font_create((FT_Face)face, nullptr);
}

void ry_hb_destroy_font(void* font) {
    if (font) hb_font_destroy((hb_font_t*)font);
}

void* ry_hb_buffer_create() {
    return (void*)hb_buffer_create();
}

void ry_hb_buffer_destroy(void* buf) {
    if (buf) hb_buffer_destroy((hb_buffer_t*)buf);
}

void ry_hb_buffer_add_utf8(void* buf, const char* text) {
    if (buf && text) {
        hb_buffer_add_utf8((hb_buffer_t*)buf, text, -1, 0, -1);
    }
}

void ry_hb_buffer_set_script(void* buf, int script) {
    if (buf) {
        hb_buffer_set_script((hb_buffer_t*)buf, (hb_script_t)script);
    }
}

void ry_hb_buffer_set_language(void* buf, const char* lang) {
    if (buf && lang) {
        hb_language_t hl = hb_language_from_string(lang, -1);
        hb_buffer_set_language((hb_buffer_t*)buf, hl);
    }
}

void ry_hb_buffer_set_direction(void* buf, int dir) {
    if (buf) {
        hb_direction_t d;
        switch (dir) {
            case 0: d = HB_DIRECTION_LTR; break;
            case 1: d = HB_DIRECTION_RTL; break;
            case 2: d = HB_DIRECTION_TTB; break;
            default: d = HB_DIRECTION_LTR;
        }
        hb_buffer_set_direction((hb_buffer_t*)buf, d);
    }
}

void ry_hb_shape_text(void* font, void* buf) {
    if (font && buf) {
        hb_shape((hb_font_t*)font, (hb_buffer_t*)buf, nullptr, 0);
    }
}

GlyphInfo* ry_hb_buffer_get_glyph_infos(void* buf, int* out_count) {
    if (!buf) { *out_count = 0; return nullptr; }
    hb_buffer_t* hb_buf = (hb_buffer_t*)buf;
    unsigned int count = 0;
    hb_glyph_info_t* infos = hb_buffer_get_glyph_infos(hb_buf, &count);
    hb_glyph_position_t* positions = hb_buffer_get_glyph_positions(hb_buf, &count);
    if (!infos || count == 0) { *out_count = 0; return nullptr; }
    GlyphInfo* result = (GlyphInfo*)malloc(count * sizeof(GlyphInfo));
    for (unsigned int i = 0; i < count; i++) {
        result[i].glyph_id = infos[i].codepoint;
        result[i].cluster = infos[i].cluster;
        if (positions) {
            result[i].x_advance = positions[i].x_advance >> 6;
            result[i].y_advance = positions[i].y_advance >> 6;
            result[i].x_offset = positions[i].x_offset >> 6;
            result[i].y_offset = positions[i].y_offset >> 6;
        } else {
            result[i].x_advance = 0;
            result[i].y_advance = 0;
            result[i].x_offset = 0;
            result[i].y_offset = 0;
        }
    }
    *out_count = (int)count;
    return result;
}

void font_free_bitmap(unsigned char* bitmap) {
    if (bitmap) free(bitmap);
}

void font_free_glyph_infos(GlyphInfo* infos) {
    if (infos) free(infos);
}

}
