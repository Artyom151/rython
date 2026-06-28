#include <SDL2/SDL.h>
#include <SDL2/SDL_image.h>
#include <SDL2/SDL_ttf.h>
#include <SDL2/SDL_mixer.h>
#include <cstdint>
#include <cstring>

extern "C" {

int sdl_init(uint32_t flags) {
    return SDL_Init(flags);
}

void* sdl_create_window(const char* title, int x, int y, int w, int h, uint32_t flags) {
    return SDL_CreateWindow(title, x, y, w, h, flags);
}

void sdl_destroy_window(void* win) {
    SDL_DestroyWindow((SDL_Window*)win);
}

void* sdl_create_renderer(void* window, int index, uint32_t flags) {
    return SDL_CreateRenderer((SDL_Window*)window, index, flags);
}

void sdl_destroy_renderer(void* renderer) {
    SDL_DestroyRenderer((SDL_Renderer*)renderer);
}

void sdl_render_clear(void* renderer) {
    SDL_RenderClear((SDL_Renderer*)renderer);
}

void sdl_render_present(void* renderer) {
    SDL_RenderPresent((SDL_Renderer*)renderer);
}

void sdl_set_render_draw_color(void* renderer, uint8_t r, uint8_t g, uint8_t b, uint8_t a) {
    SDL_SetRenderDrawColor((SDL_Renderer*)renderer, r, g, b, a);
}

void sdl_render_fill_rect(void* renderer, int x, int y, int w, int h) {
    SDL_Rect rect = { (int16_t)x, (int16_t)y, (uint16_t)w, (uint16_t)h };
    SDL_RenderFillRect((SDL_Renderer*)renderer, &rect);
}

int sdl_poll_event(uint32_t* type_out, uint32_t* keycode_out, uint32_t* scancode_out, uint8_t* keymod_out, uint8_t* repeat_out) {
    SDL_Event event;
    if (SDL_PollEvent(&event)) {
        *type_out = (uint32_t)event.type;
        if (event.type == SDL_KEYDOWN || event.type == SDL_KEYUP) {
            *keycode_out = (uint32_t)event.key.keysym.sym;
            *scancode_out = (uint32_t)event.key.keysym.scancode;
            *keymod_out = (uint8_t)event.key.keysym.mod;
            *repeat_out = (uint8_t)event.key.repeat;
        } else if (event.type == SDL_MOUSEBUTTONDOWN || event.type == SDL_MOUSEBUTTONUP) {
            *keycode_out = (uint32_t)event.button.button;
            *scancode_out = (uint32_t)event.button.x;
            *repeat_out = (uint8_t)event.button.y;
        } else if (event.type == SDL_MOUSEMOTION) {
            *keycode_out = (uint32_t)event.motion.x;
            *scancode_out = (uint32_t)event.motion.y;
            *repeat_out = 0;
        } else if (event.type == SDL_WINDOWEVENT) {
            *keycode_out = (uint32_t)event.window.event;
        }
        return 1;
    }
    return 0;
}

void sdl_delay(uint32_t ms) {
    SDL_Delay(ms);
}

uint32_t sdl_get_ticks() {
    return SDL_GetTicks();
}

void* sdl_create_texture_from_surface(void* renderer, void* surface) {
    return SDL_CreateTextureFromSurface((SDL_Renderer*)renderer, (SDL_Surface*)surface);
}

void sdl_render_copy(void* renderer, void* texture, int sx, int sy, int sw, int sh, int dx, int dy, int dw, int dh) {
    SDL_Rect src_rect = { (int16_t)sx, (int16_t)sy, (uint16_t)sw, (uint16_t)sh };
    SDL_Rect dst_rect = { (int16_t)dx, (int16_t)dy, (uint16_t)dw, (uint16_t)dh };
    SDL_RenderCopy((SDL_Renderer*)renderer, (SDL_Texture*)texture, &src_rect, &dst_rect);
}

void sdl_render_copy_full(void* renderer, void* texture) {
    SDL_RenderCopy((SDL_Renderer*)renderer, (SDL_Texture*)texture, NULL, NULL);
}

void sdl_destroy_texture(void* texture) {
    SDL_DestroyTexture((SDL_Texture*)texture);
}

void* sdl_image_load(const char* file) {
    return IMG_Load(file);
}

void sdl_free_surface(void* surface) {
    SDL_FreeSurface((SDL_Surface*)surface);
}

const uint8_t* sdl_get_keyboard_state(int* numkeys) {
    return SDL_GetKeyboardState(numkeys);
}

int sdl_ttf_init() {
    return TTF_Init();
}

void sdl_ttf_quit() {
    TTF_Quit();
}

void* sdl_ttf_open_font(const char* path, int size) {
    return TTF_OpenFont(path, size);
}

void* sdl_ttf_render_text_solid(void* font, const char* text, uint8_t r, uint8_t g, uint8_t b) {
    SDL_Color color = { r, g, b, 255 };
    return TTF_RenderText_Solid((TTF_Font*)font, text, color);
}

int sdl_mixer_init(int freq, uint16_t format, int channels, int chunksize) {
    return Mix_OpenAudio(freq, format, channels, chunksize);
}

void* sdl_mixer_load_music(const char* file) {
    return Mix_LoadMUS(file);
}

void sdl_mixer_play_music(void* music, int loops) {
    Mix_PlayMusic((Mix_Music*)music, loops);
}

void* sdl_mixer_load_chunk(const char* file) {
    return Mix_LoadWAV(file);
}

int sdl_mixer_play_channel(int channel, void* chunk, int loops) {
    return Mix_PlayChannel(channel, (Mix_Chunk*)chunk, loops);
}

void sdl_quit() {
    SDL_Quit();
}

}
