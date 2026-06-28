#include <libavcodec/avcodec.h>
#include <libavformat/avformat.h>
#include <libavutil/avutil.h>
#include <libavutil/imgutils.h>
#include <libavutil/dict.h>
#include <libswscale/swscale.h>
#include <cstring>
#include <string>
#include <cstdint>
#include <cstdlib>

struct FFmpegState {
    AVFormatContext* fmt_ctx;
    AVCodecContext* codec_ctx;
    int video_stream;
    int audio_stream;
};

extern "C" {

int ffmpeg_init() {
    avformat_network_init();
    return 0;
}

void* ffmpeg_open_input(const char* path) {
    AVFormatContext* fmt_ctx = nullptr;
    if (avformat_open_input(&fmt_ctx, path, nullptr, nullptr) != 0) {
        return nullptr;
    }
    if (avformat_find_stream_info(fmt_ctx, nullptr) < 0) {
        avformat_close_input(&fmt_ctx);
        return nullptr;
    }
    int video_stream = -1;
    int audio_stream = -1;
    for (unsigned i = 0; i < fmt_ctx->nb_streams; i++) {
        if (fmt_ctx->streams[i]->codecpar->codec_type == AVMEDIA_TYPE_VIDEO && video_stream < 0) {
            video_stream = (int)i;
        }
        if (fmt_ctx->streams[i]->codecpar->codec_type == AVMEDIA_TYPE_AUDIO && audio_stream < 0) {
            audio_stream = (int)i;
        }
    }
    AVCodecContext* codec_ctx = nullptr;
    if (video_stream >= 0) {
        const AVCodec* codec = avcodec_find_decoder(fmt_ctx->streams[video_stream]->codecpar->codec_id);
        if (codec) {
            codec_ctx = avcodec_alloc_context3(codec);
            if (codec_ctx) {
                avcodec_parameters_to_context(codec_ctx, fmt_ctx->streams[video_stream]->codecpar);
                avcodec_open2(codec_ctx, codec, nullptr);
            }
        }
    }
    FFmpegState* state = (FFmpegState*)malloc(sizeof(FFmpegState));
    state->fmt_ctx = fmt_ctx;
    state->codec_ctx = codec_ctx;
    state->video_stream = video_stream;
    state->audio_stream = audio_stream;
    return (void*)state;
}

void ffmpeg_close_input(void* ctx) {
    if (!ctx) return;
    FFmpegState* state = (FFmpegState*)ctx;
    if (state->codec_ctx) {
        avcodec_free_context(&state->codec_ctx);
    }
    if (state->fmt_ctx) {
        avformat_close_input(&state->fmt_ctx);
    }
    free(state);
}

int ffmpeg_find_stream(void* ctx, int type) {
    if (!ctx) return -1;
    FFmpegState* state = (FFmpegState*)ctx;
    if (type == 0) return state->video_stream;
    if (type == 1) return state->audio_stream;
    for (unsigned i = 0; i < state->fmt_ctx->nb_streams; i++) {
        if (state->fmt_ctx->streams[i]->codecpar->codec_type == type) {
            return (int)i;
        }
    }
    return -1;
}

void ffmpeg_get_codec_params(void* ctx, int stream_idx, int* out_w, int* out_h, char** out_codec_name, int* out_pix_fmt) {
    if (!ctx) return;
    FFmpegState* state = (FFmpegState*)ctx;
    if (stream_idx < 0 || stream_idx >= (int)state->fmt_ctx->nb_streams) return;
    AVCodecParameters* par = state->fmt_ctx->streams[stream_idx]->codecpar;
    if (out_w) *out_w = par->width;
    if (out_h) *out_h = par->height;
    if (out_codec_name) {
        const AVCodec* codec = avcodec_find_decoder(par->codec_id);
        if (codec && codec->name) {
            *out_codec_name = strdup(codec->name);
        } else {
            *out_codec_name = strdup("unknown");
        }
    }
    if (out_pix_fmt) *out_pix_fmt = par->format;
}

double ffmpeg_get_duration(void* ctx) {
    if (!ctx) return 0.0;
    FFmpegState* state = (FFmpegState*)ctx;
    if (state->fmt_ctx->duration != AV_NOPTS_VALUE) {
        return state->fmt_ctx->duration / (double)AV_TIME_BASE;
    }
    return 0.0;
}

int ffmpeg_seek(void* ctx, double timestamp) {
    if (!ctx) return -1;
    FFmpegState* state = (FFmpegState*)ctx;
    int64_t ts = (int64_t)(timestamp * AV_TIME_BASE);
    return av_seek_frame(state->fmt_ctx, -1, ts, AVSEEK_FLAG_BACKWARD);
}

int ffmpeg_read_frame(void* ctx, int* out_w, int* out_h, int* out_channels, uint8_t** out_data) {
    if (!ctx) return -1;
    FFmpegState* state = (FFmpegState*)ctx;
    if (state->video_stream < 0) return -1;

    AVPacket* pkt = av_packet_alloc();
    AVFrame* frame = av_frame_alloc();
    SwsContext* sws_ctx = nullptr;
    int ret = -1;

    while (av_read_frame(state->fmt_ctx, pkt) >= 0) {
        if (pkt->stream_index == state->video_stream) {
            if (avcodec_send_packet(state->codec_ctx, pkt) == 0) {
                if (avcodec_receive_frame(state->codec_ctx, frame) == 0) {
                    int w = frame->width;
                    int h = frame->height;
                    uint8_t* rgb_data = (uint8_t*)av_malloc((size_t)w * h * 4);
                    if (!rgb_data) break;

                    uint8_t* dst[4] = {rgb_data, nullptr, nullptr, nullptr};
                    int dst_linesize[4] = {w * 4, 0, 0, 0};

                    sws_ctx = sws_getContext(w, h, state->codec_ctx->pix_fmt,
                                              w, h, AV_PIX_FMT_RGB0,
                                              SWS_BILINEAR, nullptr, nullptr, nullptr);
                    if (sws_ctx) {
                        sws_scale(sws_ctx, frame->data, frame->linesize, 0, h, dst, dst_linesize);
                        sws_freeContext(sws_ctx);
                    }

                    if (out_w) *out_w = w;
                    if (out_h) *out_h = h;
                    if (out_channels) *out_channels = 4;
                    if (out_data) *out_data = rgb_data;

                    ret = 1;
                    break;
                }
            }
        }
        av_packet_unref(pkt);
    }

    av_frame_free(&frame);
    av_packet_free(&pkt);
    return ret;
}

int ffmpeg_extract_thumbnail(void* ctx, double time_sec, int* out_w, int* out_h, uint8_t** out_data) {
    if (!ctx) return -1;
    FFmpegState* state = (FFmpegState*)ctx;
    if (state->video_stream < 0) return -1;

    int64_t ts = (int64_t)(time_sec * AV_TIME_BASE);
    av_seek_frame(state->fmt_ctx, -1, ts, AVSEEK_FLAG_BACKWARD);
    if (state->codec_ctx) {
        avcodec_flush_buffers(state->codec_ctx);
    }

    AVPacket* pkt = av_packet_alloc();
    AVFrame* frame = av_frame_alloc();
    int ret = -1;

    while (av_read_frame(state->fmt_ctx, pkt) >= 0) {
        if (pkt->stream_index == state->video_stream) {
            if (avcodec_send_packet(state->codec_ctx, pkt) == 0) {
                if (avcodec_receive_frame(state->codec_ctx, frame) == 0) {
                    int w = frame->width;
                    int h = frame->height;
                    uint8_t* rgb_data = (uint8_t*)av_malloc((size_t)w * h * 4);
                    if (!rgb_data) break;

                    uint8_t* dst[4] = {rgb_data, nullptr, nullptr, nullptr};
                    int dst_linesize[4] = {w * 4, 0, 0, 0};

                    SwsContext* sws_ctx = sws_getContext(w, h, state->codec_ctx->pix_fmt,
                                                          w, h, AV_PIX_FMT_RGB0,
                                                          SWS_BILINEAR, nullptr, nullptr, nullptr);
                    if (sws_ctx) {
                        sws_scale(sws_ctx, frame->data, frame->linesize, 0, h, dst, dst_linesize);
                        sws_freeContext(sws_ctx);
                    }

                    if (out_w) *out_w = w;
                    if (out_h) *out_h = h;
                    if (out_data) *out_data = rgb_data;
                    ret = 0;
                    break;
                }
            }
        }
        av_packet_unref(pkt);
    }

    av_frame_free(&frame);
    av_packet_free(&pkt);
    return ret;
}

char* ffmpeg_get_metadata(void* ctx, const char* key) {
    if (!ctx) return nullptr;
    FFmpegState* state = (FFmpegState*)ctx;
    AVDictionaryEntry* entry = av_dict_get(state->fmt_ctx->metadata, key, nullptr, 0);
    if (entry && entry->value) {
        return strdup(entry->value);
    }
    return nullptr;
}

char* ffmpeg_get_all_metadata(void* ctx) {
    if (!ctx) return nullptr;
    FFmpegState* state = (FFmpegState*)ctx;
    AVDictionaryEntry* entry = nullptr;
    std::string result;
    while ((entry = av_dict_get(state->fmt_ctx->metadata, "", entry, AV_DICT_IGNORE_SUFFIX))) {
        result += entry->key;
        result += "=";
        result += entry->value;
        result += "\n";
    }
    if (result.empty()) return strdup("");
    return strdup(result.c_str());
}

void ffmpeg_free_string(char* s) {
    if (s) free(s);
}

void ffmpeg_free_data(uint8_t* data) {
    if (data) av_free(data);
}

}
