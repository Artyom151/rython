#include <curl/curl.h>
#include <cstring>
#include <cstdint>
#include <string>
#include <cstdlib>

struct CurlBuffer { std::string data; };

static size_t write_cb(void *contents, size_t size, size_t nmemb, void *userp) {
    ((CurlBuffer*)userp)->data.append((char*)contents, size * nmemb);
    return size * nmemb;
}

extern "C" {

void* curl_easy_init_() {
    return curl_easy_init();
}

void curl_easy_cleanup_(void* handle) {
    curl_easy_cleanup((CURL*)handle);
}

void curl_easy_setopt_url(void* handle, const char* url) {
    curl_easy_setopt((CURL*)handle, CURLOPT_URL, url);
}

void curl_easy_setopt_post(void* handle, const char* postdata) {
    curl_easy_setopt((CURL*)handle, CURLOPT_POSTFIELDS, postdata);
}

void curl_easy_setopt_timeout(void* handle, int64_t secs) {
    curl_easy_setopt((CURL*)handle, CURLOPT_TIMEOUT, secs);
}

void curl_easy_setopt_follow_location(void* handle, int64_t enable) {
    curl_easy_setopt((CURL*)handle, CURLOPT_FOLLOWLOCATION, enable);
}

void curl_easy_setopt_user_agent(void* handle, const char* ua) {
    curl_easy_setopt((CURL*)handle, CURLOPT_USERAGENT, ua);
}

void curl_easy_setopt_verbose(void* handle, int64_t enable) {
    curl_easy_setopt((CURL*)handle, CURLOPT_VERBOSE, enable);
}

void curl_easy_setopt_header(void* handle, int64_t enable) {
    curl_easy_setopt((CURL*)handle, CURLOPT_HEADER, enable);
}

void curl_easy_setopt_customrequest(void* handle, const char* method) {
    curl_easy_setopt((CURL*)handle, CURLOPT_CUSTOMREQUEST, method);
}

int64_t curl_easy_perform_(void* handle, char** out_body) {
    CurlBuffer buf;
    curl_easy_setopt((CURL*)handle, CURLOPT_WRITEFUNCTION, write_cb);
    curl_easy_setopt((CURL*)handle, CURLOPT_WRITEDATA, &buf);
    CURLcode res = curl_easy_perform((CURL*)handle);
    if (res != CURLE_OK) {
        *out_body = strdup(curl_easy_strerror(res));
        return -1;
    }
    long http_code = 0;
    curl_easy_getinfo((CURL*)handle, CURLINFO_RESPONSE_CODE, &http_code);
    *out_body = strdup(buf.data.c_str());
    return http_code;
}

void* curl_build_slist(const char* const* headers) {
    struct curl_slist* list = nullptr;
    for (int i = 0; headers[i] != nullptr; i++) {
        list = curl_slist_append(list, headers[i]);
    }
    return list;
}

void curl_free_slist(void* slist) {
    curl_slist_free_all((struct curl_slist*)slist);
}

void curl_easy_setopt_slist(void* handle, void* slist) {
    curl_easy_setopt((CURL*)handle, CURLOPT_HTTPHEADER, (struct curl_slist*)slist);
}

void curl_free_string(char* s) {
    free(s);
}

}
