#include <vulkan/vulkan.h>
#include <cstring>
#include <vector>
#include <cstdint>

extern "C" {

int vk_init_() {
    return 0;
}

void* vk_create_instance(const char* app_name, const char* engine_name) {
    VkApplicationInfo appInfo = {};
    appInfo.sType = VK_STRUCTURE_TYPE_APPLICATION_INFO;
    appInfo.pApplicationName = app_name;
    appInfo.applicationVersion = VK_MAKE_VERSION(1, 0, 0);
    appInfo.pEngineName = engine_name;
    appInfo.engineVersion = VK_MAKE_VERSION(1, 0, 0);
    appInfo.apiVersion = VK_API_VERSION_1_0;

    VkInstanceCreateInfo createInfo = {};
    createInfo.sType = VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO;
    createInfo.pApplicationInfo = &appInfo;

    VkInstance instance = VK_NULL_HANDLE;
    VkResult res = vkCreateInstance(&createInfo, nullptr, &instance);
    if (res != VK_SUCCESS) return nullptr;
    return (void*)instance;
}

void vk_destroy_instance(void* instance) {
    if (instance) {
        vkDestroyInstance((VkInstance)instance, nullptr);
    }
}

void vk_enumerate_physical_devices(void* instance, void*** out_devices, int* out_count) {
    if (!instance || !out_devices || !out_count) return;
    uint32_t count = 0;
    vkEnumeratePhysicalDevices((VkInstance)instance, &count, nullptr);
    if (count == 0) { *out_count = 0; *out_devices = nullptr; return; }
    std::vector<VkPhysicalDevice> devices(count);
    vkEnumeratePhysicalDevices((VkInstance)instance, &count, devices.data());
    *out_count = (int)count;
    void** arr = new void*[count];
    for (uint32_t i = 0; i < count; i++) {
        arr[i] = (void*)(uintptr_t)devices[i];
    }
    *out_devices = arr;
}

void vk_free_device_list(void*** out_devices, int) {
    if (!out_devices || !*out_devices) return;
    delete[] *out_devices;
    *out_devices = nullptr;
}

void vk_get_device_properties(void* device, char** out_name, int* out_type,
                               int* out_api_major, int* out_api_minor, int* out_api_patch,
                               int* out_driver_major, int* out_driver_minor, int* out_driver_patch) {
    if (!device || !out_name) return;
    VkPhysicalDeviceProperties props;
    vkGetPhysicalDeviceProperties((VkPhysicalDevice)(uintptr_t)device, &props);
    *out_name = strdup(props.deviceName);
    if (out_type) *out_type = (int)props.deviceType;
    if (out_api_major) *out_api_major = VK_VERSION_MAJOR(props.apiVersion);
    if (out_api_minor) *out_api_minor = VK_VERSION_MINOR(props.apiVersion);
    if (out_api_patch) *out_api_patch = VK_VERSION_PATCH(props.apiVersion);
    if (out_driver_major) *out_driver_major = VK_VERSION_MAJOR(props.driverVersion);
    if (out_driver_minor) *out_driver_minor = VK_VERSION_MINOR(props.driverVersion);
    if (out_driver_patch) *out_driver_patch = VK_VERSION_PATCH(props.driverVersion);
}

void* vk_create_device(void* device, int queue_family_index) {
    if (!device) return nullptr;
    float queuePriority = 1.0f;
    VkDeviceQueueCreateInfo queueCreateInfo = {};
    queueCreateInfo.sType = VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO;
    queueCreateInfo.queueFamilyIndex = (uint32_t)queue_family_index;
    queueCreateInfo.queueCount = 1;
    queueCreateInfo.pQueuePriorities = &queuePriority;

    VkDeviceCreateInfo createInfo = {};
    createInfo.sType = VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO;
    createInfo.queueCreateInfoCount = 1;
    createInfo.pQueueCreateInfos = &queueCreateInfo;

    VkDevice logicalDevice = VK_NULL_HANDLE;
    VkResult res = vkCreateDevice((VkPhysicalDevice)(uintptr_t)device, &createInfo, nullptr, &logicalDevice);
    if (res != VK_SUCCESS) return nullptr;
    return (void*)logicalDevice;
}

void vk_destroy_device(void* device) {
    if (device) {
        vkDestroyDevice((VkDevice)device, nullptr);
    }
}

void* vk_create_swapchain(void* device, void* surface, int width, int height, int format) {
    if (!device) return nullptr;
    VkSwapchainCreateInfoKHR createInfo = {};
    createInfo.sType = VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR;
    createInfo.surface = (VkSurfaceKHR)(uintptr_t)surface;
    createInfo.minImageCount = 2;
    createInfo.imageFormat = (VkFormat)format;
    createInfo.imageColorSpace = VK_COLOR_SPACE_SRGB_NONLINEAR_KHR;
    createInfo.imageExtent = { (uint32_t)width, (uint32_t)height };
    createInfo.imageArrayLayers = 1;
    createInfo.imageUsage = VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT;
    createInfo.imageSharingMode = VK_SHARING_MODE_EXCLUSIVE;
    createInfo.preTransform = VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR;
    createInfo.compositeAlpha = VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR;
    createInfo.presentMode = VK_PRESENT_MODE_FIFO_KHR;
    createInfo.clipped = VK_TRUE;

    VkSwapchainKHR swapchain = VK_NULL_HANDLE;
    VkResult res = vkCreateSwapchainKHR((VkDevice)device, &createInfo, nullptr, &swapchain);
    if (res != VK_SUCCESS) return nullptr;
    return (void*)(uintptr_t)swapchain;
}

void vk_destroy_swapchain(void* device, void* swapchain) {
    if (device && swapchain) {
        vkDestroySwapchainKHR((VkDevice)device, (VkSwapchainKHR)(uintptr_t)swapchain, nullptr);
    }
}

void* vk_create_shader_module(void* device, const uint32_t* code, int size) {
    if (!device) return nullptr;
    VkShaderModuleCreateInfo createInfo = {};
    createInfo.sType = VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO;
    createInfo.codeSize = (size_t)size;
    createInfo.pCode = code;

    VkShaderModule module = VK_NULL_HANDLE;
    VkResult res = vkCreateShaderModule((VkDevice)device, &createInfo, nullptr, &module);
    if (res != VK_SUCCESS) return nullptr;
    return (void*)module;
}

void vk_destroy_shader_module(void* device, void* shader) {
    if (device && shader) {
        vkDestroyShaderModule((VkDevice)device, (VkShaderModule)shader, nullptr);
    }
}

void* vk_create_pipeline(void* device, void* vert_shader, void* frag_shader, int width, int height) {
    if (!device) return nullptr;

    VkPipelineShaderStageCreateInfo shaderStages[2] = {};
    shaderStages[0].sType = VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO;
    shaderStages[0].stage = VK_SHADER_STAGE_VERTEX_BIT;
    shaderStages[0].module = (VkShaderModule)vert_shader;
    shaderStages[0].pName = "main";

    shaderStages[1].sType = VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO;
    shaderStages[1].stage = VK_SHADER_STAGE_FRAGMENT_BIT;
    shaderStages[1].module = (VkShaderModule)frag_shader;
    shaderStages[1].pName = "main";

    VkPipelineVertexInputStateCreateInfo vertexInput = {};
    vertexInput.sType = VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO;

    VkPipelineInputAssemblyStateCreateInfo inputAssembly = {};
    inputAssembly.sType = VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO;
    inputAssembly.topology = VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST;

    VkViewport viewport = {};
    viewport.x = 0; viewport.y = 0;
    viewport.width = (float)width; viewport.height = (float)height;
    viewport.minDepth = 0.0f; viewport.maxDepth = 1.0f;

    VkRect2D scissor = {};
    scissor.offset = {0, 0};
    scissor.extent = { (uint32_t)width, (uint32_t)height };

    VkPipelineViewportStateCreateInfo viewportState = {};
    viewportState.sType = VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO;
    viewportState.viewportCount = 1;
    viewportState.pViewports = &viewport;
    viewportState.scissorCount = 1;
    viewportState.pScissors = &scissor;

    VkPipelineRasterizationStateCreateInfo rasterizer = {};
    rasterizer.sType = VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO;
    rasterizer.polygonMode = VK_POLYGON_MODE_FILL;
    rasterizer.cullMode = VK_CULL_MODE_BACK_BIT;
    rasterizer.frontFace = VK_FRONT_FACE_CLOCKWISE;
    rasterizer.lineWidth = 1.0f;

    VkPipelineMultisampleStateCreateInfo multisampling = {};
    multisampling.sType = VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO;
    multisampling.rasterizationSamples = VK_SAMPLE_COUNT_1_BIT;

    VkPipelineColorBlendAttachmentState colorBlendAttachment = {};
    colorBlendAttachment.colorWriteMask = VK_COLOR_COMPONENT_R_BIT | VK_COLOR_COMPONENT_G_BIT | VK_COLOR_COMPONENT_B_BIT | VK_COLOR_COMPONENT_A_BIT;

    VkPipelineColorBlendStateCreateInfo colorBlending = {};
    colorBlending.sType = VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO;
    colorBlending.attachmentCount = 1;
    colorBlending.pAttachments = &colorBlendAttachment;

    VkGraphicsPipelineCreateInfo pipelineInfo = {};
    pipelineInfo.sType = VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO;
    pipelineInfo.stageCount = 2;
    pipelineInfo.pStages = shaderStages;
    pipelineInfo.pVertexInputState = &vertexInput;
    pipelineInfo.pInputAssemblyState = &inputAssembly;
    pipelineInfo.pViewportState = &viewportState;
    pipelineInfo.pRasterizationState = &rasterizer;
    pipelineInfo.pMultisampleState = &multisampling;
    pipelineInfo.pColorBlendState = &colorBlending;

    VkPipeline pipeline = VK_NULL_HANDLE;
    VkResult res = vkCreateGraphicsPipelines((VkDevice)device, VK_NULL_HANDLE, 1, &pipelineInfo, nullptr, &pipeline);
    if (res != VK_SUCCESS) return nullptr;
    return (void*)pipeline;
}

void vk_destroy_pipeline(void* device, void* pipeline) {
    if (device && pipeline) {
        vkDestroyPipeline((VkDevice)device, (VkPipeline)pipeline, nullptr);
    }
}

void* vk_create_command_buffer(void* device) {
    if (!device) return nullptr;
    VkCommandPoolCreateInfo poolInfo = {};
    poolInfo.sType = VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO;
    poolInfo.queueFamilyIndex = 0;

    VkCommandPool pool = VK_NULL_HANDLE;
    VkResult res = vkCreateCommandPool((VkDevice)device, &poolInfo, nullptr, &pool);
    if (res != VK_SUCCESS) return nullptr;

    VkCommandBufferAllocateInfo allocInfo = {};
    allocInfo.sType = VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO;
    allocInfo.commandPool = pool;
    allocInfo.level = VK_COMMAND_BUFFER_LEVEL_PRIMARY;
    allocInfo.commandBufferCount = 1;

    VkCommandBuffer cmd = VK_NULL_HANDLE;
    res = vkAllocateCommandBuffers((VkDevice)device, &allocInfo, &cmd);
    if (res != VK_SUCCESS) { vkDestroyCommandPool((VkDevice)device, pool, nullptr); return nullptr; }
    return (void*)cmd;
}

void vk_begin_command_buffer(void* cmd) {
    if (!cmd) return;
    VkCommandBufferBeginInfo beginInfo = {};
    beginInfo.sType = VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO;
    vkBeginCommandBuffer((VkCommandBuffer)cmd, &beginInfo);
}

void vk_cmd_bind_pipeline(void* cmd, void* pipeline) {
    if (!cmd || !pipeline) return;
    vkCmdBindPipeline((VkCommandBuffer)cmd, VK_PIPELINE_BIND_POINT_GRAPHICS, (VkPipeline)pipeline);
}

void vk_cmd_draw(void* cmd, int vertex_count, int instance_count) {
    if (!cmd) return;
    vkCmdDraw((VkCommandBuffer)cmd, (uint32_t)vertex_count, (uint32_t)instance_count, 0, 0);
}

void vk_end_command_buffer(void* cmd) {
    if (!cmd) return;
    vkEndCommandBuffer((VkCommandBuffer)cmd);
}

void vk_queue_submit(void* device, void* cmd) {
    if (!device || !cmd) return;
    VkQueue queue = VK_NULL_HANDLE;
    vkGetDeviceQueue((VkDevice)device, 0, 0, &queue);
    if (queue == VK_NULL_HANDLE) return;

    VkSubmitInfo submitInfo = {};
    submitInfo.sType = VK_STRUCTURE_TYPE_SUBMIT_INFO;
    submitInfo.commandBufferCount = 1;
    submitInfo.pCommandBuffers = (const VkCommandBuffer*)&cmd;

    vkQueueSubmit(queue, 1, &submitInfo, VK_NULL_HANDLE);
}

void vk_device_wait_idle(void* device) {
    if (device) {
        vkDeviceWaitIdle((VkDevice)device);
    }
}

void vk_get_physical_device_memory_properties(void* device, int* out_heap_count,
                                               int* out_heap_sizes, int max_heaps) {
    if (!device) return;
    VkPhysicalDeviceMemoryProperties memProps;
    vkGetPhysicalDeviceMemoryProperties((VkPhysicalDevice)(uintptr_t)device, &memProps);
    if (out_heap_count) *out_heap_count = (int)memProps.memoryHeapCount;
    if (out_heap_sizes && max_heaps > 0) {
        for (int i = 0; i < (int)memProps.memoryHeapCount && i < max_heaps; i++) {
            out_heap_sizes[i] = (int)(memProps.memoryHeaps[i].size / (1024 * 1024));
        }
    }
}

} // extern "C"
