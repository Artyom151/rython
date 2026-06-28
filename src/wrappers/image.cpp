#include <png.h>
#include <jpeglib.h>
#include <webp/decode.h>
#include <webp/encode.h>
#include <cstring>
#include <cstdint>
#include <cstdlib>
#include <csetjmp>
#include <vector>
#include <algorithm>

extern "C" {

void png_load(const char* path, int* out_w, int* out_h, int* out_channels, unsigned char** out_data) {
    *out_w = 0; *out_h = 0; *out_channels = 0; *out_data = nullptr;
    FILE* fp = fopen(path, "rb");
    if (!fp) return;
    unsigned char header[8];
    fread(header, 1, 8, fp);
    if (png_sig_cmp(header, 0, 8)) { fclose(fp); return; }
    png_structp png = png_create_read_struct(PNG_LIBPNG_VER_STRING, nullptr, nullptr, nullptr);
    if (!png) { fclose(fp); return; }
    png_infop info = png_create_info_struct(png);
    if (!info) { png_destroy_read_struct(&png, nullptr, nullptr); fclose(fp); return; }
    if (setjmp(png_jmpbuf(png))) { png_destroy_read_struct(&png, &info, nullptr); fclose(fp); return; }
    png_init_io(png, fp);
    png_set_sig_bytes(png, 8);
    png_read_info(png, info);
    *out_w = png_get_image_width(png, info);
    *out_h = png_get_image_height(png, info);
    png_byte color_type = png_get_color_type(png, info);
    png_byte bit_depth = png_get_bit_depth(png, info);
    if (bit_depth == 16) png_set_strip_16(png);
    if (color_type == PNG_COLOR_TYPE_PALETTE) png_set_palette_to_rgb(png);
    if (color_type == PNG_COLOR_TYPE_GRAY && bit_depth < 8) png_set_expand_gray_1_2_4_to_8(png);
    if (png_get_valid(png, info, PNG_INFO_tRNS)) png_set_tRNS_to_alpha(png);
    png_read_update_info(png, info);
    *out_channels = png_get_channels(png, info);
    size_t row_bytes = png_get_rowbytes(png, info);
    *out_data = (unsigned char*)malloc((size_t)(*out_h) * row_bytes);
    std::vector<png_bytep> rows(*out_h);
    for (int y = 0; y < *out_h; y++) rows[y] = *out_data + y * row_bytes;
    png_read_image(png, rows.data());
    png_destroy_read_struct(&png, &info, nullptr);
    fclose(fp);
}

void png_save(const char* path, int w, int h, int channels, const unsigned char* data) {
    FILE* fp = fopen(path, "wb");
    if (!fp) return;
    png_structp png = png_create_write_struct(PNG_LIBPNG_VER_STRING, nullptr, nullptr, nullptr);
    if (!png) { fclose(fp); return; }
    png_infop info = png_create_info_struct(png);
    if (!info) { png_destroy_write_struct(&png, nullptr); fclose(fp); return; }
    if (setjmp(png_jmpbuf(png))) { png_destroy_write_struct(&png, &info); fclose(fp); return; }
    png_init_io(png, fp);
    int color_type = (channels == 4) ? PNG_COLOR_TYPE_RGBA : (channels == 2) ? PNG_COLOR_TYPE_GRAY_ALPHA : PNG_COLOR_TYPE_RGB;
    if (channels == 1) color_type = PNG_COLOR_TYPE_GRAY;
    png_set_IHDR(png, info, w, h, 8, color_type, PNG_INTERLACE_NONE,
                 PNG_COMPRESSION_TYPE_DEFAULT, PNG_FILTER_TYPE_DEFAULT);
    png_write_info(png, info);
    std::vector<png_bytep> rows(h);
    for (int y = 0; y < h; y++) rows[y] = (png_bytep)(data + y * w * channels);
    png_write_image(png, rows.data());
    png_write_end(png, nullptr);
    png_destroy_write_struct(&png, &info);
    fclose(fp);
}

void jpeg_load(const char* path, int* out_w, int* out_h, int* out_channels, unsigned char** out_data) {
    *out_w = 0; *out_h = 0; *out_channels = 0; *out_data = nullptr;
    FILE* fp = fopen(path, "rb");
    if (!fp) return;
    struct jpeg_decompress_struct cinfo;
    struct jpeg_error_mgr jerr;
    cinfo.err = jpeg_std_error(&jerr);
    jpeg_create_decompress(&cinfo);
    jpeg_stdio_src(&cinfo, fp);
    jpeg_read_header(&cinfo, TRUE);
    jpeg_start_decompress(&cinfo);
    *out_w = cinfo.output_width;
    *out_h = cinfo.output_height;
    *out_channels = cinfo.output_components;
    size_t row_stride = *out_w * *out_channels;
    *out_data = (unsigned char*)malloc((size_t)(*out_h) * row_stride);
    while (cinfo.output_scanline < cinfo.output_height) {
        unsigned char* row = *out_data + cinfo.output_scanline * row_stride;
        jpeg_read_scanlines(&cinfo, &row, 1);
    }
    jpeg_finish_decompress(&cinfo);
    jpeg_destroy_decompress(&cinfo);
    fclose(fp);
}

void jpeg_save(const char* path, int w, int h, int channels, const unsigned char* data, int quality) {
    FILE* fp = fopen(path, "wb");
    if (!fp) return;
    struct jpeg_compress_struct cinfo;
    struct jpeg_error_mgr jerr;
    cinfo.err = jpeg_std_error(&jerr);
    jpeg_create_compress(&cinfo);
    jpeg_stdio_dest(&cinfo, fp);
    cinfo.image_width = w;
    cinfo.image_height = h;
    cinfo.input_components = (channels == 4) ? 4 : 3;
    if (channels == 1) cinfo.input_components = 1;
    cinfo.in_color_space = (cinfo.input_components == 1) ? JCS_GRAYSCALE : (cinfo.input_components == 4) ? JCS_EXT_RGBA : JCS_RGB;
    jpeg_set_defaults(&cinfo);
    jpeg_set_quality(&cinfo, quality, TRUE);
    jpeg_start_compress(&cinfo, TRUE);
    while (cinfo.next_scanline < cinfo.image_height) {
        unsigned char* row = (unsigned char*)(data + cinfo.next_scanline * w * channels);
        jpeg_write_scanlines(&cinfo, &row, 1);
    }
    jpeg_finish_compress(&cinfo);
    jpeg_destroy_compress(&cinfo);
    fclose(fp);
}

void webp_load(const char* path, int* out_w, int* out_h, int* out_channels, unsigned char** out_data) {
    *out_w = 0; *out_h = 0; *out_channels = 0; *out_data = nullptr;
    FILE* fp = fopen(path, "rb");
    if (!fp) return;
    fseek(fp, 0, SEEK_END);
    long fsize = ftell(fp);
    fseek(fp, 0, SEEK_SET);
    if (fsize <= 0) { fclose(fp); return; }
    std::vector<unsigned char> buf(fsize);
    fread(buf.data(), 1, fsize, fp);
    fclose(fp);
    WebPBitstreamFeatures features;
    if (WebPGetFeatures(buf.data(), fsize, &features) != VP8_STATUS_OK) return;
    *out_w = features.width;
    *out_h = features.height;
    *out_channels = features.has_alpha ? 4 : 3;
    if (features.has_alpha) {
        *out_data = WebPDecodeRGBA(buf.data(), fsize, out_w, out_h);
    } else {
        *out_data = WebPDecodeRGB(buf.data(), fsize, out_w, out_h);
    }
    if (!*out_data) { *out_w = 0; *out_h = 0; *out_channels = 0; }
}

void webp_save(const char* path, int w, int h, int channels, const unsigned char* data, int quality) {
    uint8_t* out = nullptr;
    size_t out_size;
    if (channels == 4) {
        out_size = WebPEncodeRGBA(data, w, h, w * channels, quality, &out);
    } else if (channels == 3) {
        out_size = WebPEncodeRGB(data, w, h, w * channels, quality, &out);
    } else {
        out_size = WebPEncodeLosslessRGB(data, w, h, w * channels, &out);
    }
    if (!out) return;
    FILE* fp = fopen(path, "wb");
    if (fp) { fwrite(out, 1, out_size, fp); fclose(fp); }
    WebPFree(out);
}

void image_resize(const unsigned char* src, int src_w, int src_h, int channels, int dst_w, int dst_h, unsigned char** out_data) {
    *out_data = (unsigned char*)malloc((size_t)dst_w * dst_h * channels);
    for (int dy = 0; dy < dst_h; dy++) {
        float sy = (float)dy / dst_h * src_h;
        int sy0 = (int)sy;
        int sy1 = std::min(sy0 + 1, src_h - 1);
        float fy = sy - sy0;
        for (int dx = 0; dx < dst_w; dx++) {
            float sx = (float)dx / dst_w * src_w;
            int sx0 = (int)sx;
            int sx1 = std::min(sx0 + 1, src_w - 1);
            float fx = sx - sx0;
            for (int c = 0; c < channels; c++) {
                float v00 = src[(sy0 * src_w + sx0) * channels + c];
                float v10 = src[(sy0 * src_w + sx1) * channels + c];
                float v01 = src[(sy1 * src_w + sx0) * channels + c];
                float v11 = src[(sy1 * src_w + sx1) * channels + c];
                float top = v00 + (v10 - v00) * fx;
                float bot = v01 + (v11 - v01) * fx;
                float val = top + (bot - top) * fy;
                (*out_data)[(dy * dst_w + dx) * channels + c] = (unsigned char)(val + 0.5f);
            }
        }
    }
}

void image_data_free(unsigned char* data) {
    free(data);
}

}
