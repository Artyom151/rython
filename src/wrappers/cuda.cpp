#include <cuda_runtime.h>
#include <cublas_v2.h>
#include <cstring>
#include <string>
#include <cstdlib>

// Simple built-in CUDA kernel for vector addition
__global__ void vec_add_kernel(float *a, float *b, float *c, int n) {
    int idx = blockIdx.x * blockDim.x + threadIdx.x;
    if (idx < n) c[idx] = a[idx] + b[idx];
}

extern "C" {

int cuda_init() {
    cudaError_t err = cudaFree(0);
    if (err != cudaSuccess) return -1;
    int count = 0;
    cudaGetDeviceCount(&count);
    return count;
}

int cuda_device_count() {
    int count = 0;
    cudaGetDeviceCount(&count);
    return count;
}

const char* cuda_device_name(int device) {
    cudaDeviceProp prop;
    cudaError_t err = cudaGetDeviceProperties(&prop, device);
    if (err != cudaSuccess) {
        char* msg = (char*)malloc(1);
        msg[0] = '\0';
        return msg;
    }
    char* name = (char*)malloc(strlen(prop.name) + 1);
    strcpy(name, prop.name);
    return name;
}

void* cuda_device_props(int device) {
    cudaDeviceProp* prop = (cudaDeviceProp*)malloc(sizeof(cudaDeviceProp));
    cudaError_t err = cudaGetDeviceProperties(prop, device);
    if (err != cudaSuccess) {
        free(prop);
        return nullptr;
    }
    return (void*)prop;
}

int cuda_get_device_props_major(void* props) {
    return ((cudaDeviceProp*)props)->major;
}

int cuda_get_device_props_minor(void* props) {
    return ((cudaDeviceProp*)props)->minor;
}

size_t cuda_get_device_props_total_mem(void* props) {
    return ((cudaDeviceProp*)props)->totalGlobalMem;
}

int cuda_get_device_props_multiprocessors(void* props) {
    return ((cudaDeviceProp*)props)->multiProcessorCount;
}

void cuda_free_device_props(void* props) {
    free(props);
}

void cuda_free_string(char* s) {
    free(s);
}

int cuda_set_device(int device) {
    return (int)cudaSetDevice(device);
}

void* cuda_malloc(size_t size) {
    void* ptr = nullptr;
    cudaError_t err = cudaMalloc(&ptr, size);
    if (err != cudaSuccess) return nullptr;
    return ptr;
}

void cuda_free(void* ptr) {
    cudaFree(ptr);
}

int cuda_memcpy_host_to_device(const void* host, void* dev, size_t size) {
    return (int)cudaMemcpy(dev, host, size, cudaMemcpyHostToDevice);
}

int cuda_memcpy_device_to_host(const void* dev, void* host, size_t size) {
    return (int)cudaMemcpy(host, dev, size, cudaMemcpyDeviceToHost);
}

int cuda_memcpy_device_to_device(const void* src, void* dst, size_t size) {
    return (int)cudaMemcpy(dst, src, size, cudaMemcpyDeviceToDevice);
}

void* cuda_malloc_host(size_t size) {
    void* ptr = nullptr;
    cudaError_t err = cudaMallocHost(&ptr, size);
    if (err != cudaSuccess) return nullptr;
    return ptr;
}

void cuda_free_host(void* ptr) {
    cudaFreeHost(ptr);
}

int cuda_memset(void* ptr, int val, size_t size) {
    return (int)cudaMemset(ptr, val, size);
}

int cuda_synchronize() {
    return (int)cudaDeviceSynchronize();
}

const char* cuda_get_last_error() {
    cudaError_t err = cudaGetLastError();
    return cudaGetErrorString(err);
}

// Vector add kernel launcher
int cuda_launch_vector_add(void* a_dev, void* b_dev, void* c_dev, int n) {
    int blockSize = 256;
    int gridSize = (n + blockSize - 1) / blockSize;
    vec_add_kernel<<<gridSize, blockSize>>>((float*)a_dev, (float*)b_dev, (float*)c_dev, n);
    cudaError_t err = cudaGetLastError();
    return (int)err;
}

// Generic kernel launch via function pointer
int cuda_launch_kernel(void* function_ptr, int grid_dim_x, int grid_dim_y, int grid_dim_z,
                       int block_dim_x, int block_dim_y, int block_dim_z, void** args) {
    dim3 gridDim(grid_dim_x, grid_dim_y, grid_dim_z);
    dim3 blockDim(block_dim_x, block_dim_y, block_dim_z);
    cudaError_t err = cudaLaunchKernel(function_ptr, gridDim, blockDim, args, 0, nullptr);
    return (int)err;
}

// cuBLAS
void* cublas_create() {
    cublasHandle_t handle = nullptr;
    cublasStatus_t status = cublasCreate(&handle);
    if (status != CUBLAS_STATUS_SUCCESS) return nullptr;
    return (void*)handle;
}

void cublas_destroy(void* handle) {
    cublasDestroy((cublasHandle_t)handle);
}

int cublas_sgemm(void* handle, const char* transa, const char* transb,
                 int m, int n, int k, float alpha,
                 const float* A, int lda, const float* B, int ldb,
                 float beta, float* C, int ldc) {
    cublasOperation_t ta = (*transa == 'T' || *transa == 't') ? CUBLAS_OP_T : CUBLAS_OP_N;
    cublasOperation_t tb = (*transb == 'T' || *transb == 't') ? CUBLAS_OP_T : CUBLAS_OP_N;
    return (int)cublasSgemm((cublasHandle_t)handle, ta, tb, m, n, k, &alpha, A, lda, B, ldb, &beta, C, ldc);
}

float cublas_sdot(void* handle, int n, const float* x, int incx, const float* y, int incy) {
    float result = 0.0f;
    cublasSdot((cublasHandle_t)handle, n, x, incx, y, incy, &result);
    return result;
}

// PTX compilation and launch (using nvrtc or manual)
void* cuda_compile_ptx(const char* source, const char* func_name) {
    // Stub - requires nvrtc. Returns nullptr to indicate not supported without nvrtc.
    (void)source;
    (void)func_name;
    return nullptr;
}

void* cuda_get_ptx_function(void* module, const char* func_name) {
    // Stub
    (void)module;
    (void)func_name;
    return nullptr;
}

void cuda_unload_ptx_module(void* module) {
    // Stub
    (void)module;
}

int cuda_launch_ptx(void* function, void** args, int grid_dim_x, int grid_dim_y, int grid_dim_z,
                     int block_dim_x, int block_dim_y, int block_dim_z) {
    // Stub
    (void)function;
    (void)args;
    (void)grid_dim_x;
    (void)grid_dim_y;
    (void)grid_dim_z;
    (void)block_dim_x;
    (void)block_dim_y;
    (void)block_dim_z;
    return -1;
}

} // extern "C"
