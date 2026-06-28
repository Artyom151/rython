pub mod wrappers {
    use super::Value;

    pub mod vulkan {
        use super::super::Value;
        use std::ffi::c_void;
        use std::os::raw::c_char;

        extern "C" {
            fn free(ptr: *mut c_void);
            fn vk_init_() -> i32;
            fn vk_create_instance(app_name: *const c_char, engine_name: *const c_char) -> *mut c_void;
            fn vk_destroy_instance(instance: *mut c_void);
            fn vk_enumerate_physical_devices(instance: *mut c_void, out_devices: *mut *mut *mut c_void, out_count: *mut i32);
            fn vk_free_device_list(out_devices: *mut *mut *mut c_void, count: i32);
            fn vk_get_device_properties(device: *mut c_void, out_name: *mut *mut c_char, out_type: *mut i32, out_api_major: *mut i32, out_api_minor: *mut i32, out_api_patch: *mut i32, out_driver_major: *mut i32, out_driver_minor: *mut i32, out_driver_patch: *mut i32);
            fn vk_create_device(device: *mut c_void, queue_family_index: i32) -> *mut c_void;
            fn vk_destroy_device(device: *mut c_void);
            fn vk_create_swapchain(device: *mut c_void, surface: *mut c_void, width: i32, height: i32, format: i32) -> *mut c_void;
            fn vk_destroy_swapchain(device: *mut c_void, swapchain: *mut c_void);
            fn vk_create_shader_module(device: *mut c_void, code: *const u32, size: i32) -> *mut c_void;
            fn vk_destroy_shader_module(device: *mut c_void, shader: *mut c_void);
            fn vk_create_pipeline(device: *mut c_void, vert_shader: *mut c_void, frag_shader: *mut c_void, width: i32, height: i32) -> *mut c_void;
            fn vk_destroy_pipeline(device: *mut c_void, pipeline: *mut c_void);
            fn vk_create_command_buffer(device: *mut c_void) -> *mut c_void;
            fn vk_begin_command_buffer(cmd: *mut c_void);
            fn vk_cmd_bind_pipeline(cmd: *mut c_void, pipeline: *mut c_void);
            fn vk_cmd_draw(cmd: *mut c_void, vertex_count: i32, instance_count: i32);
            fn vk_end_command_buffer(cmd: *mut c_void);
            fn vk_queue_submit(device: *mut c_void, cmd: *mut c_void);
            fn vk_device_wait_idle(device: *mut c_void);
            fn vk_get_physical_device_memory_properties(device: *mut c_void, out_heap_count: *mut i32, out_heap_sizes: *mut i32, max_heaps: i32);
        }

        pub fn init() -> Value {
            Value::Int(unsafe { vk_init_() } as i64)
        }

        pub fn create_instance(app_name: &str, engine_name: &str) -> Value {
            let an = std::ffi::CString::new(app_name).unwrap_or_default();
            let en = std::ffi::CString::new(engine_name).unwrap_or_default();
            let ptr = unsafe { vk_create_instance(an.as_ptr(), en.as_ptr()) };
            if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
        }

        pub fn destroy_instance(instance: &Value) {
            if let Value::Ptr(p) = instance { unsafe { vk_destroy_instance(*p) } }
        }

        pub fn enumerate_physical_devices(instance: &Value) -> Value {
            let ptr = match instance { Value::Ptr(p) => *p, _ => return Value::List(vec![]) };
            let mut devices: *mut *mut c_void = std::ptr::null_mut();
            let mut count: i32 = 0;
            unsafe { vk_enumerate_physical_devices(ptr, &mut devices, &mut count) };
            if devices.is_null() || count == 0 { return Value::List(vec![]) }
            let mut list = Vec::new();
            for i in 0..count {
                let dev = unsafe { *devices.offset(i as isize) };
                list.push(Value::Ptr(dev));
            }
            unsafe { vk_free_device_list(&mut devices, count) };
            Value::List(list)
        }

        pub fn get_device_properties(device: &Value) -> Value {
            let ptr = match device { Value::Ptr(p) => *p, _ => return Value::Tuple(vec![Value::Str(String::new()), Value::Int(0), Value::Int(0), Value::Int(0)]) };
            let mut name: *mut c_char = std::ptr::null_mut();
            let mut dev_type: i32 = 0;
            let mut api_major: i32 = 0; let mut api_minor: i32 = 0; let mut api_patch: i32 = 0;
            let mut drv_major: i32 = 0; let mut drv_minor: i32 = 0; let mut drv_patch: i32 = 0;
            unsafe {
                vk_get_device_properties(ptr, &mut name, &mut dev_type, &mut api_major, &mut api_minor, &mut api_patch, &mut drv_major, &mut drv_minor, &mut drv_patch);
            }
            let name_str = if !name.is_null() {
                let s = unsafe { std::ffi::CStr::from_ptr(name) }.to_string_lossy().into_owned();
                unsafe { free(name as *mut c_void) };
                s
            } else { String::new() };
            let api_ver = format!("{}.{}.{}", api_major, api_minor, api_patch);
            let drv_ver = format!("{}.{}.{}", drv_major, drv_minor, drv_patch);
            Value::Tuple(vec![
                Value::Str(name_str),
                Value::Int(dev_type as i64),
                Value::Str(api_ver),
                Value::Str(drv_ver),
            ])
        }

        pub fn create_device(device: &Value, queue_family_index: i64) -> Value {
            let ptr = match device { Value::Ptr(p) => *p, _ => return Value::None };
            let dev = unsafe { vk_create_device(ptr, queue_family_index as i32) };
            if dev.is_null() { Value::None } else { Value::Ptr(dev) }
        }

        pub fn destroy_device(device: &Value) {
            if let Value::Ptr(p) = device { unsafe { vk_destroy_device(*p) } }
        }

        pub fn create_swapchain(device: &Value, surface: &Value, width: i64, height: i64, format: i64) -> Value {
            let dev = match device { Value::Ptr(p) => *p, _ => return Value::None };
            let surf = match surface { Value::Ptr(p) => *p, _ => return Value::None };
            let sc = unsafe { vk_create_swapchain(dev, surf, width as i32, height as i32, format as i32) };
            if sc.is_null() { Value::None } else { Value::Ptr(sc) }
        }

        pub fn destroy_swapchain(device: &Value, swapchain: &Value) {
            let d = match device { Value::Ptr(p) => *p, _ => return };
            let sc = match swapchain { Value::Ptr(p) => *p, _ => return };
            unsafe { vk_destroy_swapchain(d, sc) }
        }

        pub fn create_shader_module(device: &Value, code: &[u8]) -> Value {
            let dev = match device { Value::Ptr(p) => *p, _ => return Value::None };
            let ptr = unsafe { vk_create_shader_module(dev, code.as_ptr() as *const u32, code.len() as i32) };
            if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
        }

        pub fn destroy_shader_module(device: &Value, shader: &Value) {
            let d = match device { Value::Ptr(p) => *p, _ => return };
            let s = match shader { Value::Ptr(p) => *p, _ => return };
            unsafe { vk_destroy_shader_module(d, s) }
        }

        pub fn create_pipeline(device: &Value, vert_shader: &Value, frag_shader: &Value, width: i64, height: i64) -> Value {
            let dev = match device { Value::Ptr(p) => *p, _ => return Value::None };
            let vs = match vert_shader { Value::Ptr(p) => *p, _ => return Value::None };
            let fs = match frag_shader { Value::Ptr(p) => *p, _ => return Value::None };
            let pl = unsafe { vk_create_pipeline(dev, vs, fs, width as i32, height as i32) };
            if pl.is_null() { Value::None } else { Value::Ptr(pl) }
        }

        pub fn destroy_pipeline(device: &Value, pipeline: &Value) {
            let d = match device { Value::Ptr(p) => *p, _ => return };
            let pl = match pipeline { Value::Ptr(p) => *p, _ => return };
            unsafe { vk_destroy_pipeline(d, pl) }
        }

        pub fn create_command_buffer(device: &Value) -> Value {
            let dev = match device { Value::Ptr(p) => *p, _ => return Value::None };
            let cmd = unsafe { vk_create_command_buffer(dev) };
            if cmd.is_null() { Value::None } else { Value::Ptr(cmd) }
        }

        pub fn begin_command_buffer(cmd: &Value) {
            if let Value::Ptr(p) = cmd { unsafe { vk_begin_command_buffer(*p) } }
        }

        pub fn cmd_bind_pipeline(cmd: &Value, pipeline: &Value) {
            let c = match cmd { Value::Ptr(p) => *p, _ => return };
            let pl = match pipeline { Value::Ptr(p) => *p, _ => return };
            unsafe { vk_cmd_bind_pipeline(c, pl) }
        }

        pub fn cmd_draw(cmd: &Value, vertex_count: i64, instance_count: i64) {
            if let Value::Ptr(p) = cmd { unsafe { vk_cmd_draw(*p, vertex_count as i32, instance_count as i32) } }
        }

        pub fn end_command_buffer(cmd: &Value) {
            if let Value::Ptr(p) = cmd { unsafe { vk_end_command_buffer(*p) } }
        }

        pub fn queue_submit(device: &Value, cmd: &Value) {
            let d = match device { Value::Ptr(p) => *p, _ => return };
            let c = match cmd { Value::Ptr(p) => *p, _ => return };
            unsafe { vk_queue_submit(d, c) }
        }

        pub fn device_wait_idle(device: &Value) {
            if let Value::Ptr(p) = device { unsafe { vk_device_wait_idle(*p) } }
        }

        pub fn get_physical_device_memory_properties(device: &Value) -> Value {
            let ptr = match device { Value::Ptr(p) => *p, _ => return Value::Tuple(vec![Value::Int(0), Value::List(vec![])]) };
            let mut heap_count: i32 = 0;
            let mut heap_sizes: [i32; 16] = [0; 16];
            unsafe { vk_get_physical_device_memory_properties(ptr, &mut heap_count, heap_sizes.as_mut_ptr(), 16) };
            let heaps = heap_sizes[..heap_count as usize].iter().map(|&s| Value::Int(s as i64)).collect();
            Value::Tuple(vec![Value::Int(heap_count as i64), Value::List(heaps)])
        }
    }

    pub mod cuda {
        use super::super::Value;
        use std::ffi::c_void;
        use std::os::raw::c_char;

        extern "C" {
            fn cuda_init() -> i32;
            fn cuda_device_count() -> i32;
            fn cuda_device_name(device: i32) -> *mut c_char;
            fn cuda_device_props(device: i32) -> *mut c_void;
            fn cuda_get_device_props_major(props: *mut c_void) -> i32;
            fn cuda_get_device_props_minor(props: *mut c_void) -> i32;
            fn cuda_get_device_props_total_mem(props: *mut c_void) -> usize;
            fn cuda_get_device_props_multiprocessors(props: *mut c_void) -> i32;
            fn cuda_free_device_props(props: *mut c_void);
            fn cuda_free_string(s: *mut c_char);
            fn cuda_set_device(device: i32) -> i32;
            fn cuda_malloc(size: usize) -> *mut c_void;
            fn cuda_free(ptr: *mut c_void);
            fn cuda_memcpy_host_to_device(host: *const c_void, dev: *mut c_void, size: usize) -> i32;
            fn cuda_memcpy_device_to_host(dev: *const c_void, host: *mut c_void, size: usize) -> i32;
            fn cuda_memcpy_device_to_device(src: *const c_void, dst: *mut c_void, size: usize) -> i32;
            fn cuda_malloc_host(size: usize) -> *mut c_void;
            fn cuda_free_host(ptr: *mut c_void);
            fn cuda_memset(ptr: *mut c_void, val: i32, size: usize) -> i32;
            fn cuda_synchronize() -> i32;
            fn cuda_get_last_error() -> *const c_char;
            fn cuda_launch_vector_add(a_dev: *mut c_void, b_dev: *mut c_void, c_dev: *mut c_void, n: i32) -> i32;
            fn cuda_launch_kernel(function_ptr: *mut c_void, grid_dim_x: i32, grid_dim_y: i32, grid_dim_z: i32, block_dim_x: i32, block_dim_y: i32, block_dim_z: i32, args: *mut *mut c_void) -> i32;
            fn cublas_create_ffi() -> *mut c_void;
            fn cublas_destroy_ffi(handle: *mut c_void);
            fn cublas_sgemm_ffi(handle: *mut c_void, transa: *const c_char, transb: *const c_char, m: i32, n: i32, k: i32, alpha: f32, A: *const f32, lda: i32, B: *const f32, ldb: i32, beta: f32, C: *mut f32, ldc: i32) -> i32;
            fn cublas_sdot_ffi(handle: *mut c_void, n: i32, x: *const f32, incx: i32, y: *const f32, incy: i32) -> f32;
            fn cuda_compile_ptx(source: *const c_char, func_name: *const c_char) -> *mut c_void;
            fn cuda_get_ptx_function(module: *mut c_void, func_name: *const c_char) -> *mut c_void;
            fn cuda_unload_ptx_module(module: *mut c_void);
            fn cuda_launch_ptx(function: *mut c_void, args: *mut *mut c_void, grid_dim_x: i32, grid_dim_y: i32, grid_dim_z: i32, block_dim_x: i32, block_dim_y: i32, block_dim_z: i32) -> i32;
        }

        pub fn init() -> Value {
            Value::Int(unsafe { cuda_init() } as i64)
        }

        pub fn device_count() -> Value {
            Value::Int(unsafe { cuda_device_count() } as i64)
        }

        pub fn device_name(device: i64) -> Value {
            let ptr = unsafe { cuda_device_name(device as i32) };
            if ptr.is_null() { return Value::Str(String::new()) }
            let s = unsafe { std::ffi::CStr::from_ptr(ptr) }.to_string_lossy().into_owned();
            unsafe { cuda_free_string(ptr) };
            Value::Str(s)
        }

        pub fn device_props(device: i64) -> Value {
            let ptr = unsafe { cuda_device_props(device as i32) };
            if ptr.is_null() { return Value::None }
            let name = {
                let n = unsafe { cuda_get_device_props_major(ptr) };
                let minor = unsafe { cuda_get_device_props_minor(ptr) };
                let mem = unsafe { cuda_get_device_props_total_mem(ptr) };
                let mpc = unsafe { cuda_get_device_props_multiprocessors(ptr) };
                unsafe { cuda_free_device_props(ptr) };
                Value::Tuple(vec![
                    Value::Int(n as i64),
                    Value::Int(minor as i64),
                    Value::Int(mem as i64),
                    Value::Int(mpc as i64),
                ])
            };
            name
        }

        pub fn set_device(device: i64) -> Value {
            Value::Int(unsafe { cuda_set_device(device as i32) } as i64)
        }

        pub fn malloc(size: i64) -> Value {
            let ptr = unsafe { cuda_malloc(size as usize) };
            if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
        }

        pub fn free(ptr: &Value) -> Value {
            if let Value::Ptr(p) = ptr { unsafe { cuda_free(*p) } }
            Value::None
        }

        pub fn memcpy_host_to_device(host: &[u8], dev: &Value) -> Value {
            let d = match dev { Value::Ptr(p) => *p, _ => return Value::Int(-1) };
            Value::Int(unsafe { cuda_memcpy_host_to_device(host.as_ptr() as *const c_void, d, host.len()) } as i64)
        }

        pub fn memcpy_device_to_host(dev: &Value, size: i64) -> Value {
            let d = match dev { Value::Ptr(p) => *p, _ => return Value::Bytes(vec![]) };
            let mut host = vec![0u8; size as usize];
            unsafe { cuda_memcpy_device_to_host(d as *const c_void, host.as_mut_ptr() as *mut c_void, size as usize); }
            Value::Bytes(host)
        }

        pub fn memcpy_device_to_device(src: &Value, dst: &Value, size: i64) -> Value {
            let s = match src { Value::Ptr(p) => *p, _ => return Value::Int(-1) };
            let d = match dst { Value::Ptr(p) => *p, _ => return Value::Int(-1) };
            Value::Int(unsafe { cuda_memcpy_device_to_device(s as *const c_void, d, size as usize) } as i64)
        }

        pub fn malloc_host(size: i64) -> Value {
            let ptr = unsafe { cuda_malloc_host(size as usize) };
            if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
        }

        pub fn free_host(ptr: &Value) -> Value {
            if let Value::Ptr(p) = ptr { unsafe { cuda_free_host(*p) } }
            Value::None
        }

        pub fn memset(ptr: &Value, val: i64, size: i64) -> Value {
            let p = match ptr { Value::Ptr(p) => *p, _ => return Value::Int(-1) };
            Value::Int(unsafe { cuda_memset(p, val as i32, size as usize) } as i64)
        }

        pub fn synchronize() -> Value {
            Value::Int(unsafe { cuda_synchronize() } as i64)
        }

        pub fn get_last_error() -> Value {
            let s = unsafe { std::ffi::CStr::from_ptr(cuda_get_last_error()) }.to_string_lossy().into_owned();
            Value::Str(s)
        }

        pub fn launch_vector_add(a_dev: &Value, b_dev: &Value, c_dev: &Value, n: i64) -> Value {
            let a = match a_dev { Value::Ptr(p) => *p, _ => return Value::Int(-1) };
            let b = match b_dev { Value::Ptr(p) => *p, _ => return Value::Int(-1) };
            let c = match c_dev { Value::Ptr(p) => *p, _ => return Value::Int(-1) };
            Value::Int(unsafe { cuda_launch_vector_add(a, b, c, n as i32) } as i64)
        }

        pub fn launch_kernel(function_ptr: *mut c_void, grid_dim: (i64, i64, i64), block_dim: (i64, i64, i64), args: &mut Vec<*mut c_void>) -> Value {
            Value::Int(unsafe { cuda_launch_kernel(function_ptr, grid_dim.0 as i32, grid_dim.1 as i32, grid_dim.2 as i32, block_dim.0 as i32, block_dim.1 as i32, block_dim.2 as i32, args.as_mut_ptr()) } as i64)
        }

        pub fn cublas_create() -> Value {
            let ptr = unsafe { cublas_create_ffi() };
            if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
        }

        pub fn cublas_destroy(handle: &Value) -> Value {
            if let Value::Ptr(p) = handle { unsafe { cublas_destroy_ffi(*p) } }
            Value::None
        }

        pub fn cublas_sgemm(handle: &Value, transa: &str, transb: &str, m: i64, n: i64, k: i64, alpha: f64, A: &Value, lda: i64, B: &Value, ldb: i64, beta: f64, C: &Value, ldc: i64) -> Value {
            let h = match handle { Value::Ptr(p) => *p, _ => return Value::Int(-1) };
            let a_ptr = match A { Value::Ptr(p) => *p as *const f32, Value::Bytes(b) => b.as_ptr() as *const f32, _ => return Value::Int(-1) };
            let b_ptr = match B { Value::Ptr(p) => *p as *const f32, Value::Bytes(b) => b.as_ptr() as *const f32, _ => return Value::Int(-1) };
            let c_ptr = match C { Value::Ptr(p) => *p as *mut f32, _ => return Value::Int(-1) };
            let ta = std::ffi::CString::new(transa).unwrap_or_default();
            let tb = std::ffi::CString::new(transb).unwrap_or_default();
            Value::Int(unsafe { cublas_sgemm_ffi(h, ta.as_ptr(), tb.as_ptr(), m as i32, n as i32, k as i32, alpha as f32, a_ptr, lda as i32, b_ptr, ldb as i32, beta as f32, c_ptr, ldc as i32) } as i64)
        }

        pub fn cublas_sdot(handle: &Value, n: i64, x: &Value, incx: i64, y: &Value, incy: i64) -> Value {
            let h = match handle { Value::Ptr(p) => *p, _ => return Value::Float(0.0) };
            let x_ptr = match x { Value::Ptr(p) => *p as *const f32, Value::Bytes(b) => b.as_ptr() as *const f32, _ => return Value::Float(0.0) };
            let y_ptr = match y { Value::Ptr(p) => *p as *const f32, Value::Bytes(b) => b.as_ptr() as *const f32, _ => return Value::Float(0.0) };
            Value::Float(unsafe { cublas_sdot_ffi(h, n as i32, x_ptr, incx as i32, y_ptr, incy as i32) } as f64)
        }

        pub fn compile_ptx(source: &str, func_name: &str) -> Value {
            let cs = std::ffi::CString::new(source).unwrap_or_default();
            let cf = std::ffi::CString::new(func_name).unwrap_or_default();
            let module = unsafe { cuda_compile_ptx(cs.as_ptr(), cf.as_ptr()) };
            if module.is_null() { return Value::None }
            let func = unsafe { cuda_get_ptx_function(module, cf.as_ptr()) };
            Value::Tuple(vec![Value::Ptr(module), if func.is_null() { Value::None } else { Value::Ptr(func) }])
        }

        pub fn launch_ptx(func: &Value, args: &mut Vec<*mut c_void>, grid_dim: (i64, i64, i64), block_dim: (i64, i64, i64)) -> Value {
            let f = match func { Value::Ptr(p) => *p, _ => return Value::Int(-1) };
            Value::Int(unsafe { cuda_launch_ptx(f, args.as_mut_ptr(), grid_dim.0 as i32, grid_dim.1 as i32, grid_dim.2 as i32, block_dim.0 as i32, block_dim.1 as i32, block_dim.2 as i32) } as i64)
        }
    }

    pub mod curl {
        use super::super::Value;
        use std::ffi::c_void;
        use std::os::raw::c_char;

        extern "C" {
            fn curl_easy_init_() -> *mut c_void;
            fn curl_easy_cleanup_(h: *mut c_void);
            fn curl_easy_setopt_url(h: *mut c_void, url: *const c_char);
            fn curl_easy_setopt_post(h: *mut c_void, data: *const c_char);
            fn curl_easy_setopt_timeout(h: *mut c_void, s: i64);
            fn curl_easy_setopt_follow_location(h: *mut c_void, e: i64);
            fn curl_easy_setopt_user_agent(h: *mut c_void, ua: *const c_char);
            fn curl_easy_setopt_verbose(h: *mut c_void, e: i64);
            fn curl_easy_setopt_header(h: *mut c_void, e: i64);
            fn curl_easy_setopt_customrequest(h: *mut c_void, m: *const c_char);
            fn curl_easy_perform_(h: *mut c_void, out: *mut *mut c_char) -> i64;
            fn curl_build_slist(headers: *const *const c_char) -> *mut c_void;
            fn curl_free_slist(slist: *mut c_void);
            fn curl_easy_setopt_slist(h: *mut c_void, slist: *mut c_void);
            fn curl_free_string(s: *mut c_char);
        }

        pub fn get(url: &str) -> Value {
            request(url, "GET", None, None)
        }

        pub fn post(url: &str, data: &str) -> Value {
            request(url, "POST", Some(data), None)
        }

        pub fn request(url: &str, method: &str, data: Option<&str>, headers: Option<Vec<(String,String)>>) -> Value {
            let handle = unsafe { curl_easy_init_() };
            if handle.is_null() {
                return Value::Tuple(vec![Value::Int(-1), Value::Str("Failed to init curl".to_string())]);
            }

            let curl = std::ffi::CString::new(url).unwrap_or_default();
            let cmethod = std::ffi::CString::new(method).unwrap_or_default();
            unsafe {
                curl_easy_setopt_url(handle, curl.as_ptr());
                curl_easy_setopt_customrequest(handle, cmethod.as_ptr());
                curl_easy_setopt_follow_location(handle, 1);
                curl_easy_setopt_timeout(handle, 30);
            }

            if let Some(postdata) = data {
                let cdata = std::ffi::CString::new(postdata).unwrap_or_default();
                unsafe { curl_easy_setopt_post(handle, cdata.as_ptr()); }
            }

            let slist = if let Some(hdrs) = headers {
                let raw: Vec<std::ffi::CString> = hdrs.iter().map(|(k,v)| {
                    std::ffi::CString::new(format!("{}: {}", k, v)).unwrap_or_default()
                }).collect();
                let mut ptrs: Vec<*const c_char> = raw.iter().map(|c| c.as_ptr()).collect();
                ptrs.push(std::ptr::null());
                unsafe { curl_build_slist(ptrs.as_ptr()) }
            } else {
                std::ptr::null_mut()
            };

            if !slist.is_null() {
                unsafe { curl_easy_setopt_slist(handle, slist); }
            }

            let mut out_body: *mut c_char = std::ptr::null_mut();
            let status = unsafe { curl_easy_perform_(handle, &mut out_body) };

            let body = if !out_body.is_null() {
                let s = unsafe { std::ffi::CStr::from_ptr(out_body) }.to_string_lossy().into_owned();
                unsafe { curl_free_string(out_body) };
                s
            } else {
                String::new()
            };

            if !slist.is_null() {
                unsafe { curl_free_slist(slist); }
            }

            unsafe { curl_easy_cleanup_(handle) };

            Value::Tuple(vec![Value::Int(status), Value::Str(body)])
        }
    }



    pub mod sqlite3 {
        use super::super::Value;
        use std::ffi::c_void;
        use std::os::raw::c_char;

        extern "C" {
            fn sqlite3_open_db(path: *const c_char) -> *mut c_void;
            fn sqlite3_close_db(db: *mut c_void);
            fn sqlite3_last_error(db: *mut c_void) -> *const c_char;
            fn sqlite3_exec_query(db: *mut c_void, sql: *const c_char) -> *mut c_void;
            fn sqlite3_stmt_step(stmt: *mut c_void) -> i32;
            fn sqlite3_stmt_finalize(stmt: *mut c_void);
            fn sqlite3_stmt_column_count(stmt: *mut c_void) -> i32;
            fn sqlite3_stmt_column_name(stmt: *mut c_void, col: i32) -> *const c_char;
            fn sqlite3_stmt_column_type(stmt: *mut c_void, col: i32) -> i32;
            fn sqlite3_stmt_column_int64(stmt: *mut c_void, col: i32) -> i64;
            fn sqlite3_stmt_column_double(stmt: *mut c_void, col: i32) -> f64;
            fn sqlite3_stmt_column_text(stmt: *mut c_void, col: i32) -> *const c_char;
            fn sqlite3_stmt_bind_int(stmt: *mut c_void, idx: i32, val: i64) -> i32;
            fn sqlite3_stmt_bind_double(stmt: *mut c_void, idx: i32, val: f64) -> i32;
            fn sqlite3_stmt_bind_text(stmt: *mut c_void, idx: i32, val: *const c_char) -> i32;
            fn sqlite3_changes_count(db: *mut c_void) -> i32;
            fn sqlite3_last_insert_rowid_(db: *mut c_void) -> i64;
            fn sqlite3_exec_direct(db: *mut c_void, sql: *const c_char);
        }

        pub fn connect(path: &str) -> Value {
            let c = std::ffi::CString::new(path).unwrap_or_default();
            let ptr = unsafe { sqlite3_open_db(c.as_ptr()) };
            if ptr.is_null() {
                Value::None
            } else {
                Value::Ptr(ptr)
            }
        }

        pub fn close(db: &Value) {
            if let Value::Ptr(p) = db {
                unsafe { sqlite3_close_db(*p) }
            }
        }

        pub fn execute(db: &Value, sql: &str) -> Value {
            let ptr = match db { Value::Ptr(p) => *p, _ => return Value::None };
            let c = std::ffi::CString::new(sql).unwrap_or_default();
            let stmt = unsafe { sqlite3_exec_query(ptr, c.as_ptr()) };
            if stmt.is_null() {
                let err = unsafe { std::ffi::CStr::from_ptr(sqlite3_last_error(ptr)) }.to_string_lossy().into_owned();
                Value::Str(err)
            } else {
                Value::Ptr(stmt)
            }
        }

        pub fn fetch_all(stmt: &Value) -> Value {
            let ptr = match stmt { Value::Ptr(p) => *p, _ => return Value::None };
            let cols = unsafe { sqlite3_stmt_column_count(ptr) };
            let mut rows = Vec::new();
            while unsafe { sqlite3_stmt_step(ptr) } == 100 {
                let mut row = Vec::new();
                for i in 0..cols {
                    let typ = unsafe { sqlite3_stmt_column_type(ptr, i) };
                    row.push(match typ {
                        1 => Value::Int(unsafe { sqlite3_stmt_column_int64(ptr, i) }),
                        2 => Value::Float(unsafe { sqlite3_stmt_column_double(ptr, i) }),
                        3 => {
                            let t = unsafe { sqlite3_stmt_column_text(ptr, i) };
                            if t.is_null() { Value::None } else { Value::Str(unsafe { std::ffi::CStr::from_ptr(t) }.to_string_lossy().into_owned()) }
                        }
                        _ => Value::None,
                    });
                }
                rows.push(Value::Tuple(row));
            }
            unsafe { sqlite3_stmt_finalize(ptr) };
            Value::List(rows)
        }

        pub fn changes(db: &Value) -> Value {
            if let Value::Ptr(p) = db {
                Value::Int(unsafe { sqlite3_changes_count(*p) } as i64)
            } else {
                Value::Int(0)
            }
        }

        pub fn last_row_id(db: &Value) -> Value {
            if let Value::Ptr(p) = db {
                Value::Int(unsafe { sqlite3_last_insert_rowid_(*p) })
            } else {
                Value::Int(0)
            }
        }
    }

    pub mod qt6 {
        use super::super::Value;
        use std::ffi::c_void;
        use std::os::raw::c_char;

        extern "C" {
            fn qt_app_create(argc: i32, argv: *mut *mut c_char) -> *mut c_void;
            fn qt_app_exec(app: *mut c_void) -> i32;
            fn qt_app_set_style_fusion(name: *const c_char);
            fn qt_mainwindow_create() -> *mut c_void;
            fn qt_mainwindow_set_title(win: *mut c_void, title: *const c_char);
            fn qt_mainwindow_set_min_size(win: *mut c_void, w: i32, h: i32);
            fn qt_mainwindow_set_central(win: *mut c_void, widget: *mut c_void);
            fn qt_mainwindow_status_bar(win: *mut c_void) -> *mut c_void;
            fn qt_widget_create() -> *mut c_void;
            fn qt_widget_show(win: *mut c_void);
            fn qt_widget_set_layout(widget: *mut c_void, layout: *mut c_void);
            fn qt_vbox_create(parent: *mut c_void) -> *mut c_void;
            fn qt_hbox_create(parent: *mut c_void) -> *mut c_void;
            fn qt_layout_set_spacing(layout: *mut c_void, spacing: i32);
            fn qt_layout_add_widget(layout: *mut c_void, widget: *mut c_void, stretch: i32);
            fn qt_layout_add_layout(parent: *mut c_void, child: *mut c_void);
            fn qt_layout_add_stretch(layout: *mut c_void);
            fn qt_button_create(text: *const c_char) -> *mut c_void;
            fn qt_button_set_text(btn: *mut c_void, text: *const c_char);
            fn qt_button_set_min_height(btn: *mut c_void, h: i32);
            fn qt_label_create(text: *const c_char) -> *mut c_void;
            fn qt_label_set_text(label: *mut c_void, text: *const c_char);
            fn qt_lineedit_create() -> *mut c_void;
            fn qt_lineedit_set_placeholder(edit: *mut c_void, text: *const c_char);
            fn qt_lineedit_set_min_height(edit: *mut c_void, h: i32);
            fn qt_lineedit_set_text(edit: *mut c_void, text: *const c_char);
            fn qt_lineedit_text(edit: *mut c_void) -> *const c_char;
            fn qt_listwidget_create() -> *mut c_void;
            fn qt_listwidget_add_item(list: *mut c_void, text: *const c_char);
            fn qt_checkbox_create(text: *const c_char) -> *mut c_void;
            fn qt_checkbox_set_checked(cb: *mut c_void, checked: i32);
            fn qt_checkbox_is_checked(cb: *mut c_void) -> i32;
            fn qt_font_create(family: *const c_char, size: i32) -> *mut c_void;
            fn qt_color_create(r: i32, g: i32, b: i32) -> *mut c_void;
            fn qt_palette_create() -> *mut c_void;
            fn qt_palette_set_color(pal: *mut c_void, role: i32, color: *mut c_void);
            fn qt_widget_set_palette(widget: *mut c_void, pal: *mut c_void);
            fn qt_filedialog_get_dir(title: *const c_char, dir: *const c_char) -> *const c_char;
            fn qt_statusbar_message(sb: *mut c_void, msg: *const c_char);
            fn qt_lineedit_set_font(edit: *mut c_void, font: *mut c_void);
            fn qt_button_set_enabled(btn: *mut c_void, enabled: i32);
            fn qt_thread_start(t: *mut c_void);
            fn qt_thread_is_running(t: *mut c_void) -> i32;
            fn qt_thread_quit(t: *mut c_void);
            fn qt_thread_wait(t: *mut c_void);
            fn qt_register_callback(cb: extern "C" fn()) -> i32;
            fn qt_connect_clicked(btn: *mut c_void, id: i32) -> *mut c_void;
            fn qt_connect_return_pressed(edit: *mut c_void, id: i32) -> *mut c_void;
            fn qt_connect_item_double_clicked(list: *mut c_void, id: i32) -> *mut c_void;
        }

        pub fn construct(class: &str, args: Vec<Value>) -> Value {
            match class {
                "QApplication" => Value::Ptr(unsafe { qt_app_create(0, std::ptr::null_mut()) }),
                "QMainWindow" => Value::Ptr(unsafe { qt_mainwindow_create() }),
                "QWidget" => Value::Ptr(unsafe { qt_widget_create() }),
                "QVBoxLayout" | "QVBoxLayout_" => {
                    let p = args.first().and_then(|v| match v { Value::Ptr(p) => Some(*p), _ => None }).unwrap_or(std::ptr::null_mut());
                    Value::Ptr(unsafe { qt_vbox_create(p) })
                }
                "QHBoxLayout" | "QHBoxLayout_" => {
                    let p = args.first().and_then(|v| match v { Value::Ptr(p) => Some(*p), _ => None }).unwrap_or(std::ptr::null_mut());
                    Value::Ptr(unsafe { qt_hbox_create(p) })
                }
                "QPushButton" => {
                    let t = args.first().map(|v| v.to_string()).unwrap_or_default();
                    let c = std::ffi::CString::new(t).unwrap_or_default();
                    Value::Ptr(unsafe { qt_button_create(c.as_ptr()) })
                }
                "QLabel" => {
                    let t = args.first().map(|v| v.to_string()).unwrap_or_default();
                    let c = std::ffi::CString::new(t).unwrap_or_default();
                    Value::Ptr(unsafe { qt_label_create(c.as_ptr()) })
                }
                "QLineEdit" => Value::Ptr(unsafe { qt_lineedit_create() }),
                "QListWidget" => Value::Ptr(unsafe { qt_listwidget_create() }),
                "QCheckBox" => {
                    let t = args.first().map(|v| v.to_string()).unwrap_or_default();
                    let c = std::ffi::CString::new(t).unwrap_or_default();
                    Value::Ptr(unsafe { qt_checkbox_create(c.as_ptr()) })
                }
                "QStatusBar" | "QListWidgetItem" | "SearchThread" | "QThread" => Value::None,
                "QFont" => {
                    let f = args.get(0).map(|v| v.to_string()).unwrap_or_default();
                    let s = args.get(1).map(|v| v.to_int()).unwrap_or(12);
                    let c = std::ffi::CString::new(f).unwrap_or_default();
                    Value::Ptr(unsafe { qt_font_create(c.as_ptr(), s as i32) })
                }
                "QColor" => {
                    let r = args.get(0).map(|v| v.to_int()).unwrap_or(0) as i32;
                    let g = args.get(1).map(|v| v.to_int()).unwrap_or(0) as i32;
                    let b = args.get(2).map(|v| v.to_int()).unwrap_or(0) as i32;
                    Value::Ptr(unsafe { qt_color_create(r, g, b) })
                }
                "QPalette" => Value::Ptr(unsafe { qt_palette_create() }),
                _ => Value::None,
            }
        }

        pub fn method(obj: &Value, name: &str, args: Vec<Value>) -> Value {
            let ptr = match obj { Value::Ptr(p) => *p, _ => return Value::None };
            match name {
                "setWindowTitle" | "setWindowTitle_" => {
                    let t = args.first().map(|v| v.to_string()).unwrap_or_default();
                    let c = std::ffi::CString::new(t).unwrap_or_default();
                    unsafe { qt_mainwindow_set_title(ptr, c.as_ptr()) }
                }
                "setMinimumSize" | "setMinimumSize_" => {
                    let w = args.get(0).map(|v| v.to_int()).unwrap_or(0) as i32;
                    let h = args.get(1).map(|v| v.to_int()).unwrap_or(0) as i32;
                    unsafe { qt_mainwindow_set_min_size(ptr, w, h) }
                }
                "setCentralWidget" | "setCentralWidget_" => {
                    if let Some(Value::Ptr(c)) = args.first() { unsafe { qt_mainwindow_set_central(ptr, *c) } }
                }
                "show" | "show_" => unsafe { qt_widget_show(ptr) },
                "setLayout" | "setLayout_" => {
                    if let Some(Value::Ptr(l)) = args.first() { unsafe { qt_widget_set_layout(ptr, *l) } }
                }
                "setSpacing" | "setSpacing_" => {
                    unsafe { qt_layout_set_spacing(ptr, args.first().map(|v| v.to_int()).unwrap_or(0) as i32) }
                }
                "addWidget" | "addWidget_" => {
                    let s = args.get(1).map(|v| v.to_int()).unwrap_or(0) as i32;
                    if let Some(Value::Ptr(w)) = args.first() { unsafe { qt_layout_add_widget(ptr, *w, s) } }
                }
                "addLayout" | "addLayout_" => {
                    if let Some(Value::Ptr(c)) = args.first() { unsafe { qt_layout_add_layout(ptr, *c) } }
                }
                "addStretch" | "addStretch_" => unsafe { qt_layout_add_stretch(ptr) },
                "setText" | "setText_" => {
                    let t = args.first().map(|v| v.to_string()).unwrap_or_default();
                    let c = std::ffi::CString::new(t).unwrap_or_default();
                    unsafe { qt_label_set_text(ptr, c.as_ptr()) }
                }
                "setPlaceholderText" | "setPlaceholderText_" => {
                    let t = args.first().map(|v| v.to_string()).unwrap_or_default();
                    let c = std::ffi::CString::new(t).unwrap_or_default();
                    unsafe { qt_lineedit_set_placeholder(ptr, c.as_ptr()) }
                }
                "setMinimumHeight" | "setMinimumHeight_" => {
                    unsafe { qt_lineedit_set_min_height(ptr, args.first().map(|v| v.to_int()).unwrap_or(0) as i32) }
                }
                "setFont" | "setFont_" => {
                    if let Some(Value::Ptr(f)) = args.first() { unsafe { qt_lineedit_set_font(ptr, *f) } }
                }
                "text" | "text_" => {
                    let c = unsafe { qt_lineedit_text(ptr) };
                    if c.is_null() { return Value::Str(String::new()); }
                    return Value::Str(unsafe { std::ffi::CStr::from_ptr(c) }.to_string_lossy().into_owned());
                }
                "clear" | "clear_" => unsafe { qt_listwidget_add_item(ptr, std::ptr::null()) },
                "addItem" | "addItem_" => {
                    let t = args.first().map(|v| v.to_string()).unwrap_or_default();
                    let c = std::ffi::CString::new(t).unwrap_or_default();
                    unsafe { qt_listwidget_add_item(ptr, c.as_ptr()) }
                }
                "setChecked" | "setChecked_" => {
                    let v = if args.first().map(|v| v.to_bool()).unwrap_or(false) { 1 } else { 0 };
                    unsafe { qt_checkbox_set_checked(ptr, v) }
                }
                "isChecked" | "isChecked_" => {
                    return Value::Bool(unsafe { qt_checkbox_is_checked(ptr) } != 0);
                }
                "setPalette" | "setPalette_" => {
                    if let Some(Value::Ptr(p)) = args.first() { unsafe { qt_widget_set_palette(ptr, *p) } }
                }
                "setColor" | "setColor_" => {
                    let r = args.get(0).map(|v| v.to_int()).unwrap_or(0) as i32;
                    if let Some(Value::Ptr(c)) = args.get(1) { unsafe { qt_palette_set_color(ptr, r, *c) } }
                }
                "showMessage" | "showMessage_" => {
                    let t = args.first().map(|v| v.to_string()).unwrap_or_default();
                    let c = std::ffi::CString::new(t).unwrap_or_default();
                    unsafe { qt_statusbar_message(ptr, c.as_ptr()) }
                }
                "setStyle" | "setStyle_" => {
                    let t = args.first().map(|v| v.to_string()).unwrap_or_default();
                    let c = std::ffi::CString::new(t).unwrap_or_default();
                    unsafe { qt_app_set_style_fusion(c.as_ptr()) }
                }
                "exec" | "exec_" => return Value::Int(unsafe { qt_app_exec(ptr) } as i64),
                "statusBar" | "statusBar_" => return Value::Ptr(unsafe { qt_mainwindow_status_bar(ptr) }),
                "connect" | "connect_" | "setEnabled" | "setEnabled_" | "start" | "start_" |
                "wait" | "wait_" | "stop" | "stop_" | "quit" | "quit_" | "emit" | "emit_" => {}
                "isRunning" | "isRunning_" => return Value::Bool(false),
                _ => {}
            }
            Value::None
        }
    }

    pub mod lvgl {
        use super::super::Value;
        use std::ffi::c_void;
        use std::os::raw::c_char;

        type LvCallback = extern "C" fn(*mut c_void);

        extern "C" {
            fn lvgl_init() -> i32;
            fn lvgl_create_display(width: i32, height: i32, buf1: *mut c_void, buf2: *mut c_void) -> *mut c_void;
            fn lvgl_tick_inc(ms: i32);
            fn lvgl_task_handler();
            fn lvgl_create_obj(parent: *mut c_void) -> *mut c_void;
            fn lvgl_create_btn(parent: *mut c_void) -> *mut c_void;
            fn lvgl_create_label(parent: *mut c_void) -> *mut c_void;
            fn lvgl_create_slider(parent: *mut c_void) -> *mut c_void;
            fn lvgl_create_arc(parent: *mut c_void) -> *mut c_void;
            fn lvgl_create_bar(parent: *mut c_void) -> *mut c_void;
            fn lvgl_create_dropdown(parent: *mut c_void) -> *mut c_void;
            fn lvgl_create_textarea(parent: *mut c_void) -> *mut c_void;
            fn lvgl_create_checkbox(parent: *mut c_void) -> *mut c_void;
            fn lvgl_create_switch(parent: *mut c_void) -> *mut c_void;
            fn lvgl_create_chart(parent: *mut c_void) -> *mut c_void;
            fn lvgl_create_image(parent: *mut c_void) -> *mut c_void;
            fn lvgl_obj_set_pos(obj: *mut c_void, x: i32, y: i32);
            fn lvgl_obj_set_size(obj: *mut c_void, w: i32, h: i32);
            fn lvgl_obj_set_align(obj: *mut c_void, align: i32);
            fn lvgl_obj_center(obj: *mut c_void);
            fn lvgl_obj_add_flag(obj: *mut c_void, flag: i32);
            fn lvgl_obj_clear_flag(obj: *mut c_void, flag: i32);
            fn lvgl_label_set_text(obj: *mut c_void, text: *const c_char);
            fn lvgl_btn_set_text(obj: *mut c_void, text: *const c_char);
            fn lvgl_slider_set_value(obj: *mut c_void, value: i32, anim: i32);
            fn lvgl_slider_get_value(obj: *mut c_void) -> i32;
            fn lvgl_textarea_set_text(obj: *mut c_void, text: *const c_char);
            fn lvgl_textarea_get_text(obj: *mut c_void) -> *const c_char;
            fn lvgl_dropdown_set_options(obj: *mut c_void, options: *const c_char);
            fn lvgl_dropdown_get_selected(obj: *mut c_void) -> i32;
            fn lvgl_arc_set_value(obj: *mut c_void, value: i32);
            fn lvgl_arc_set_range(obj: *mut c_void, min: i32, max: i32);
            fn lvgl_bar_set_value(obj: *mut c_void, value: i32, anim: i32);
            fn lvgl_bar_set_range(obj: *mut c_void, min: i32, max: i32);
            fn lvgl_set_style_bg_color(obj: *mut c_void, r: i32, g: i32, b: i32, sel: i32);
            fn lvgl_set_style_border_color(obj: *mut c_void, r: i32, g: i32, b: i32, sel: i32);
            fn lvgl_set_style_text_color(obj: *mut c_void, r: i32, g: i32, b: i32, sel: i32);
            fn lvgl_set_style_radius(obj: *mut c_void, r: i32, sel: i32);
            fn lvgl_set_style_pad(obj: *mut c_void, pad: i32, sel: i32);
            fn lvgl_obj_add_event_cb(obj: *mut c_void, cb_id: i32, event_code: i32);
            fn lvgl_scr_act() -> *mut c_void;
            fn lvgl_disp_get_default() -> *mut c_void;
            fn lv_register_callback(cb: LvCallback) -> i32;
        }

        pub fn construct(class: &str, args: Vec<Value>) -> Value {
            match class {
                "Obj" | "Widget" => {
                    let p = args.first().and_then(|v| match v { Value::Ptr(p) => Some(*p), _ => None }).unwrap_or(std::ptr::null_mut());
                    Value::Ptr(unsafe { lvgl_create_obj(p) })
                }
                "Btn" | "Button" => {
                    let p = args.first().and_then(|v| match v { Value::Ptr(p) => Some(*p), _ => None }).unwrap_or(std::ptr::null_mut());
                    Value::Ptr(unsafe { lvgl_create_btn(p) })
                }
                "Label" => {
                    let p = args.first().and_then(|v| match v { Value::Ptr(p) => Some(*p), _ => None }).unwrap_or(std::ptr::null_mut());
                    Value::Ptr(unsafe { lvgl_create_label(p) })
                }
                "Slider" => {
                    let p = args.first().and_then(|v| match v { Value::Ptr(p) => Some(*p), _ => None }).unwrap_or(std::ptr::null_mut());
                    Value::Ptr(unsafe { lvgl_create_slider(p) })
                }
                "Arc" => {
                    let p = args.first().and_then(|v| match v { Value::Ptr(p) => Some(*p), _ => None }).unwrap_or(std::ptr::null_mut());
                    Value::Ptr(unsafe { lvgl_create_arc(p) })
                }
                "Bar" => {
                    let p = args.first().and_then(|v| match v { Value::Ptr(p) => Some(*p), _ => None }).unwrap_or(std::ptr::null_mut());
                    Value::Ptr(unsafe { lvgl_create_bar(p) })
                }
                "Dropdown" => {
                    let p = args.first().and_then(|v| match v { Value::Ptr(p) => Some(*p), _ => None }).unwrap_or(std::ptr::null_mut());
                    Value::Ptr(unsafe { lvgl_create_dropdown(p) })
                }
                "TextArea" => {
                    let p = args.first().and_then(|v| match v { Value::Ptr(p) => Some(*p), _ => None }).unwrap_or(std::ptr::null_mut());
                    Value::Ptr(unsafe { lvgl_create_textarea(p) })
                }
                "CheckBox" | "Checkbox" => {
                    let p = args.first().and_then(|v| match v { Value::Ptr(p) => Some(*p), _ => None }).unwrap_or(std::ptr::null_mut());
                    Value::Ptr(unsafe { lvgl_create_checkbox(p) })
                }
                "Switch" => {
                    let p = args.first().and_then(|v| match v { Value::Ptr(p) => Some(*p), _ => None }).unwrap_or(std::ptr::null_mut());
                    Value::Ptr(unsafe { lvgl_create_switch(p) })
                }
                "Chart" => {
                    let p = args.first().and_then(|v| match v { Value::Ptr(p) => Some(*p), _ => None }).unwrap_or(std::ptr::null_mut());
                    Value::Ptr(unsafe { lvgl_create_chart(p) })
                }
                "Img" | "Image" => {
                    let p = args.first().and_then(|v| match v { Value::Ptr(p) => Some(*p), _ => None }).unwrap_or(std::ptr::null_mut());
                    Value::Ptr(unsafe { lvgl_create_image(p) })
                }
                _ => Value::None,
            }
        }

        pub fn method(obj: &Value, name: &str, args: Vec<Value>) -> Value {
            let ptr = match obj { Value::Ptr(p) => *p, _ => return Value::None };
            match name {
                "set_pos" | "setPos" | "set_pos_" => {
                    let x = args.get(0).map(|v| v.to_int()).unwrap_or(0) as i32;
                    let y = args.get(1).map(|v| v.to_int()).unwrap_or(0) as i32;
                    unsafe { lvgl_obj_set_pos(ptr, x, y) }
                }
                "set_size" | "setSize" | "set_size_" => {
                    let w = args.get(0).map(|v| v.to_int()).unwrap_or(0) as i32;
                    let h = args.get(1).map(|v| v.to_int()).unwrap_or(0) as i32;
                    unsafe { lvgl_obj_set_size(ptr, w, h) }
                }
                "set_align" | "setAlign" | "set_align_" => {
                    let a = args.first().map(|v| v.to_int()).unwrap_or(0) as i32;
                    unsafe { lvgl_obj_set_align(ptr, a) }
                }
                "center" | "center_" => unsafe { lvgl_obj_center(ptr) },
                "add_flag" | "addFlag" | "add_flag_" => {
                    let f = args.first().map(|v| v.to_int()).unwrap_or(0) as i32;
                    unsafe { lvgl_obj_add_flag(ptr, f) }
                }
                "clear_flag" | "clearFlag" | "clear_flag_" => {
                    let f = args.first().map(|v| v.to_int()).unwrap_or(0) as i32;
                    unsafe { lvgl_obj_clear_flag(ptr, f) }
                }
                "set_text" | "setText" | "set_text_" => {
                    let t = args.first().map(|v| v.to_string()).unwrap_or_default();
                    let c = std::ffi::CString::new(t).unwrap_or_default();
                    unsafe { lvgl_label_set_text(ptr, c.as_ptr()) }
                }
                "set_value" | "setValue" | "set_value_" => {
                    let v = args.get(0).map(|v| v.to_int()).unwrap_or(0) as i32;
                    let a = args.get(1).map(|v| v.to_int()).unwrap_or(0) as i32;
                    unsafe { lvgl_slider_set_value(ptr, v, a) }
                }
                "get_value" | "getValue" | "get_value_" => {
                    return Value::Int(unsafe { lvgl_slider_get_value(ptr) } as i64);
                }
                "get_text" | "getText" | "get_text_" => {
                    let t = unsafe { lvgl_textarea_get_text(ptr) };
                    if t.is_null() { return Value::Str(String::new()); }
                    return Value::Str(unsafe { std::ffi::CStr::from_ptr(t) }.to_string_lossy().into_owned());
                }
                "set_options" | "setOptions" | "set_options_" => {
                    let o = args.first().map(|v| v.to_string()).unwrap_or_default();
                    let c = std::ffi::CString::new(o).unwrap_or_default();
                    unsafe { lvgl_dropdown_set_options(ptr, c.as_ptr()) }
                }
                "get_selected" | "getSelected" | "get_selected_" => {
                    return Value::Int(unsafe { lvgl_dropdown_get_selected(ptr) } as i64);
                }
                "set_range" | "setRange" | "set_range_" => {
                    let min = args.get(0).map(|v| v.to_int()).unwrap_or(0) as i32;
                    let max = args.get(1).map(|v| v.to_int()).unwrap_or(100) as i32;
                    unsafe { lvgl_arc_set_range(ptr, min, max) }
                }
                "set_bg_color" | "setBgColor" | "set_bg_color_" => {
                    let r = args.get(0).map(|v| v.to_int()).unwrap_or(0) as i32;
                    let g = args.get(1).map(|v| v.to_int()).unwrap_or(0) as i32;
                    let b = args.get(2).map(|v| v.to_int()).unwrap_or(0) as i32;
                    let sel = args.get(3).map(|v| v.to_int()).unwrap_or(0) as i32;
                    unsafe { lvgl_set_style_bg_color(ptr, r, g, b, sel) }
                }
                "set_border_color" | "setBorderColor" | "set_border_color_" => {
                    let r = args.get(0).map(|v| v.to_int()).unwrap_or(0) as i32;
                    let g = args.get(1).map(|v| v.to_int()).unwrap_or(0) as i32;
                    let b = args.get(2).map(|v| v.to_int()).unwrap_or(0) as i32;
                    let sel = args.get(3).map(|v| v.to_int()).unwrap_or(0) as i32;
                    unsafe { lvgl_set_style_border_color(ptr, r, g, b, sel) }
                }
                "set_text_color" | "setTextColor" | "set_text_color_" => {
                    let r = args.get(0).map(|v| v.to_int()).unwrap_or(0) as i32;
                    let g = args.get(1).map(|v| v.to_int()).unwrap_or(0) as i32;
                    let b = args.get(2).map(|v| v.to_int()).unwrap_or(0) as i32;
                    let sel = args.get(3).map(|v| v.to_int()).unwrap_or(0) as i32;
                    unsafe { lvgl_set_style_text_color(ptr, r, g, b, sel) }
                }
                "set_radius" | "setRadius" | "set_radius_" => {
                    let r = args.first().map(|v| v.to_int()).unwrap_or(0) as i32;
                    let sel = args.get(1).map(|v| v.to_int()).unwrap_or(0) as i32;
                    unsafe { lvgl_set_style_radius(ptr, r, sel) }
                }
                "set_pad" | "setPad" | "set_pad_" => {
                    let p = args.first().map(|v| v.to_int()).unwrap_or(0) as i32;
                    let sel = args.get(1).map(|v| v.to_int()).unwrap_or(0) as i32;
                    unsafe { lvgl_set_style_pad(ptr, p, sel) }
                }
                "add_event_cb" | "addEventCb" | "add_event_cb_" => {
                    let cb_id = args.get(0).map(|v| v.to_int()).unwrap_or(0) as i32;
                    let ev = args.get(1).map(|v| v.to_int()).unwrap_or(0) as i32;
                    unsafe { lvgl_obj_add_event_cb(ptr, cb_id, ev) }
                }
                _ => {}
            }
            Value::None
        }

        pub fn init() -> Value {
            Value::Int(unsafe { lvgl_init() } as i64)
        }

        pub fn create_display(width: i64, height: i64, buf1: *mut c_void, buf2: *mut c_void) -> Value {
            let ptr = unsafe { lvgl_create_display(width as i32, height as i32, buf1, buf2) };
            if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
        }

        pub fn tick_inc(ms: i64) {
            unsafe { lvgl_tick_inc(ms as i32) }
        }

        pub fn task_handler() {
            unsafe { lvgl_task_handler() }
        }

        pub fn scr_act() -> Value {
            let ptr = unsafe { lvgl_scr_act() };
            if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
        }

        pub fn disp_get_default() -> Value {
            let ptr = unsafe { lvgl_disp_get_default() };
            if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
        }

        pub fn register_callback(cb: LvCallback) -> i32 {
            unsafe { lv_register_callback(cb) }
        }
    }

    pub mod image {
        use super::super::Value;
        use std::os::raw::c_char;

        extern "C" {
            fn png_load(path: *const c_char, out_w: *mut i32, out_h: *mut i32, out_channels: *mut i32, out_data: *mut *mut u8);
            fn png_save(path: *const c_char, w: i32, h: i32, channels: i32, data: *const u8);
            fn jpeg_load(path: *const c_char, out_w: *mut i32, out_h: *mut i32, out_channels: *mut i32, out_data: *mut *mut u8);
            fn jpeg_save(path: *const c_char, w: i32, h: i32, channels: i32, data: *const u8, quality: i32);
            fn webp_load(path: *const c_char, out_w: *mut i32, out_h: *mut i32, out_channels: *mut i32, out_data: *mut *mut u8);
            fn webp_save(path: *const c_char, w: i32, h: i32, channels: i32, data: *const u8, quality: i32);
            fn image_resize(src: *const u8, src_w: i32, src_h: i32, channels: i32, dst_w: i32, dst_h: i32, out_data: *mut *mut u8);
            fn image_data_free(data: *mut u8);
        }

        pub fn load(path: &str) -> Value {
            let cpath = std::ffi::CString::new(path).unwrap_or_default();
            let mut w: i32 = 0;
            let mut h: i32 = 0;
            let mut ch: i32 = 0;
            let mut data: *mut u8 = std::ptr::null_mut();
            let lower = path.to_lowercase();
            if lower.ends_with(".png") {
                unsafe { png_load(cpath.as_ptr(), &mut w, &mut h, &mut ch, &mut data) }
            } else if lower.ends_with(".jpg") || lower.ends_with(".jpeg") {
                unsafe { jpeg_load(cpath.as_ptr(), &mut w, &mut h, &mut ch, &mut data) }
            } else if lower.ends_with(".webp") {
                unsafe { webp_load(cpath.as_ptr(), &mut w, &mut h, &mut ch, &mut data) }
            } else {
                return Value::Tuple(vec![Value::Int(0), Value::Int(0), Value::Int(0), Value::Bytes(vec![])]);
            }
            if data.is_null() || w == 0 || h == 0 {
                return Value::Tuple(vec![Value::Int(0), Value::Int(0), Value::Int(0), Value::Bytes(vec![])]);
            }
            let len = (w * h * ch) as usize;
            let bytes = unsafe { std::slice::from_raw_parts(data, len) }.to_vec();
            unsafe { image_data_free(data) };
            Value::Tuple(vec![Value::Int(w as i64), Value::Int(h as i64), Value::Int(ch as i64), Value::Bytes(bytes)])
        }

        pub fn save(path: &str, width: i64, height: i64, channels: i64, data: &[u8]) -> Value {
            let cpath = std::ffi::CString::new(path).unwrap_or_default();
            let lower = path.to_lowercase();
            if lower.ends_with(".png") {
                unsafe { png_save(cpath.as_ptr(), width as i32, height as i32, channels as i32, data.as_ptr()) }
            } else if lower.ends_with(".jpg") || lower.ends_with(".jpeg") {
                unsafe { jpeg_save(cpath.as_ptr(), width as i32, height as i32, channels as i32, data.as_ptr(), 90) }
            } else if lower.ends_with(".webp") {
                unsafe { webp_save(cpath.as_ptr(), width as i32, height as i32, channels as i32, data.as_ptr(), 80) }
            }
            Value::None
        }

        pub fn resize(data: &[u8], src_w: i64, src_h: i64, channels: i64, dst_w: i64, dst_h: i64) -> Value {
            let mut out: *mut u8 = std::ptr::null_mut();
            unsafe {
                image_resize(data.as_ptr(), src_w as i32, src_h as i32, channels as i32, dst_w as i32, dst_h as i32, &mut out);
            }
            if out.is_null() {
                return Value::Bytes(vec![]);
            }
            let len = (dst_w * dst_h * channels) as usize;
            let bytes = unsafe { std::slice::from_raw_parts(out, len) }.to_vec();
            unsafe { image_data_free(out) };
            Value::Bytes(bytes)
        }
    }

    pub mod torch {
        use super::super::Value;
        use std::ffi::c_void;

        extern "C" {
            fn torch_init() -> i32;
            fn torch_tensor_create(data: *const f32, dims: *const i64, ndim: i32, dtype: i32) -> *mut c_void;
            fn torch_tensor_from_numpy(data: *const f32, shape: *const i64, ndim: i32) -> *mut c_void;
            fn torch_tensor_fill(tensor: *mut c_void, value: f32);
            fn torch_tensor_zeros(shape: *const i64, ndim: i32) -> *mut c_void;
            fn torch_tensor_ones(shape: *const i64, ndim: i32) -> *mut c_void;
            fn torch_tensor_rand(shape: *const i64, ndim: i32) -> *mut c_void;
            fn torch_tensor_clone(tensor: *mut c_void) -> *mut c_void;
            fn torch_tensor_add(a: *mut c_void, b: *mut c_void) -> *mut c_void;
            fn torch_tensor_sub(a: *mut c_void, b: *mut c_void) -> *mut c_void;
            fn torch_tensor_mul(a: *mut c_void, b: *mut c_void) -> *mut c_void;
            fn torch_tensor_div(a: *mut c_void, b: *mut c_void) -> *mut c_void;
            fn torch_tensor_matmul(a: *mut c_void, b: *mut c_void) -> *mut c_void;
            fn torch_tensor_relu(tensor: *mut c_void) -> *mut c_void;
            fn torch_tensor_sigmoid(tensor: *mut c_void) -> *mut c_void;
            fn torch_tensor_tanh(tensor: *mut c_void) -> *mut c_void;
            fn torch_tensor_softmax(tensor: *mut c_void, dim: i32) -> *mut c_void;
            fn torch_tensor_sum(tensor: *mut c_void, dim: i32) -> *mut c_void;
            fn torch_tensor_mean(tensor: *mut c_void, dim: i32) -> *mut c_void;
            fn torch_tensor_reshape(tensor: *mut c_void, shape: *const i64, ndim: i32) -> *mut c_void;
            fn torch_tensor_view(tensor: *mut c_void, shape: *const i64, ndim: i32) -> *mut c_void;
            fn torch_tensor_to_string(tensor: *mut c_void) -> *mut i8;
            fn torch_tensor_dim(tensor: *mut c_void) -> i32;
            fn torch_tensor_sizes(tensor: *mut c_void, out_dims: *mut i64);
            fn torch_tensor_item(tensor: *mut c_void) -> f64;
            fn torch_tensor_to_float_array(tensor: *mut c_void, out_data: *mut f32, out_len: *mut i64);
            fn torch_tensor_free(tensor: *mut c_void);
            fn torch_tensor_requires_grad(tensor: *mut c_void, req: i32);
            fn torch_tensor_backward(tensor: *mut c_void);
            fn torch_tensor_grad(tensor: *mut c_void) -> *mut c_void;
            fn torch_linear_create(in_features: i32, out_features: i32, bias: i32) -> *mut c_void;
            fn torch_linear_forward(module: *mut c_void, input: *mut c_void) -> *mut c_void;
            fn torch_module_free(module: *mut c_void);
            fn torch_free_string(s: *mut i8);
        }

        pub fn init() -> Value {
            Value::Int(unsafe { torch_init() } as i64)
        }

        pub fn tensor(data: &[f32], dims: &[i64], dtype: i32) -> Value {
            let ptr = unsafe { torch_tensor_create(data.as_ptr(), dims.as_ptr(), dims.len() as i32, dtype) };
            if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
        }

        pub fn from_numpy(data: &[f32], shape: &[i64]) -> Value {
            let ptr = unsafe { torch_tensor_from_numpy(data.as_ptr(), shape.as_ptr(), shape.len() as i32) };
            if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
        }

        pub fn fill(tensor: &Value, value: f64) {
            if let Value::Ptr(p) = tensor { unsafe { torch_tensor_fill(*p, value as f32) } }
        }

        pub fn zeros(shape: &[i64]) -> Value {
            let ptr = unsafe { torch_tensor_zeros(shape.as_ptr(), shape.len() as i32) };
            if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
        }

        pub fn ones(shape: &[i64]) -> Value {
            let ptr = unsafe { torch_tensor_ones(shape.as_ptr(), shape.len() as i32) };
            if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
        }

        pub fn rand(shape: &[i64]) -> Value {
            let ptr = unsafe { torch_tensor_rand(shape.as_ptr(), shape.len() as i32) };
            if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
        }

        pub fn clone(tensor: &Value) -> Value {
            if let Value::Ptr(p) = tensor {
                let ptr = unsafe { torch_tensor_clone(*p) };
                if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
            } else { Value::None }
        }

        pub fn add(a: &Value, b: &Value) -> Value {
            match (a, b) {
                (Value::Ptr(pa), Value::Ptr(pb)) => {
                    let ptr = unsafe { torch_tensor_add(*pa, *pb) };
                    if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
                }
                _ => Value::None,
            }
        }

        pub fn sub(a: &Value, b: &Value) -> Value {
            match (a, b) {
                (Value::Ptr(pa), Value::Ptr(pb)) => {
                    let ptr = unsafe { torch_tensor_sub(*pa, *pb) };
                    if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
                }
                _ => Value::None,
            }
        }

        pub fn mul(a: &Value, b: &Value) -> Value {
            match (a, b) {
                (Value::Ptr(pa), Value::Ptr(pb)) => {
                    let ptr = unsafe { torch_tensor_mul(*pa, *pb) };
                    if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
                }
                _ => Value::None,
            }
        }

        pub fn div(a: &Value, b: &Value) -> Value {
            match (a, b) {
                (Value::Ptr(pa), Value::Ptr(pb)) => {
                    let ptr = unsafe { torch_tensor_div(*pa, *pb) };
                    if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
                }
                _ => Value::None,
            }
        }

        pub fn matmul(a: &Value, b: &Value) -> Value {
            match (a, b) {
                (Value::Ptr(pa), Value::Ptr(pb)) => {
                    let ptr = unsafe { torch_tensor_matmul(*pa, *pb) };
                    if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
                }
                _ => Value::None,
            }
        }

        pub fn relu(tensor: &Value) -> Value {
            if let Value::Ptr(p) = tensor {
                let ptr = unsafe { torch_tensor_relu(*p) };
                if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
            } else { Value::None }
        }

        pub fn sigmoid(tensor: &Value) -> Value {
            if let Value::Ptr(p) = tensor {
                let ptr = unsafe { torch_tensor_sigmoid(*p) };
                if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
            } else { Value::None }
        }

        pub fn tanh(tensor: &Value) -> Value {
            if let Value::Ptr(p) = tensor {
                let ptr = unsafe { torch_tensor_tanh(*p) };
                if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
            } else { Value::None }
        }

        pub fn softmax(tensor: &Value, dim: i32) -> Value {
            if let Value::Ptr(p) = tensor {
                let ptr = unsafe { torch_tensor_softmax(*p, dim) };
                if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
            } else { Value::None }
        }

        pub fn sum(tensor: &Value, dim: i32) -> Value {
            if let Value::Ptr(p) = tensor {
                let ptr = unsafe { torch_tensor_sum(*p, dim) };
                if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
            } else { Value::None }
        }

        pub fn mean(tensor: &Value, dim: i32) -> Value {
            if let Value::Ptr(p) = tensor {
                let ptr = unsafe { torch_tensor_mean(*p, dim) };
                if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
            } else { Value::None }
        }

        pub fn reshape(tensor: &Value, shape: &[i64]) -> Value {
            if let Value::Ptr(p) = tensor {
                let ptr = unsafe { torch_tensor_reshape(*p, shape.as_ptr(), shape.len() as i32) };
                if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
            } else { Value::None }
        }

        pub fn view(tensor: &Value, shape: &[i64]) -> Value {
            if let Value::Ptr(p) = tensor {
                let ptr = unsafe { torch_tensor_view(*p, shape.as_ptr(), shape.len() as i32) };
                if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
            } else { Value::None }
        }

        pub fn to_string(tensor: &Value) -> Value {
            if let Value::Ptr(p) = tensor {
                let s = unsafe { torch_tensor_to_string(*p) };
                if s.is_null() { return Value::Str(String::new()) }
                let result = Value::Str(unsafe { std::ffi::CStr::from_ptr(s as *const i8) }.to_string_lossy().into_owned());
                unsafe { torch_free_string(s) };
                result
            } else { Value::Str(String::new()) }
        }

        pub fn dim(tensor: &Value) -> Value {
            if let Value::Ptr(p) = tensor {
                Value::Int(unsafe { torch_tensor_dim(*p) } as i64)
            } else { Value::Int(0) }
        }

        pub fn sizes(tensor: &Value) -> Value {
            if let Value::Ptr(p) = tensor {
                let ndim = unsafe { torch_tensor_dim(*p) };
                let mut dims: Vec<i64> = vec![0i64; ndim as usize];
                unsafe { torch_tensor_sizes(*p, dims.as_mut_ptr()) };
                Value::Tuple(dims.into_iter().map(|d| Value::Int(d)).collect())
            } else { Value::Tuple(vec![]) }
        }

        pub fn item(tensor: &Value) -> Value {
            if let Value::Ptr(p) = tensor {
                Value::Float(unsafe { torch_tensor_item(*p) })
            } else { Value::None }
        }

        pub fn to_float_array(tensor: &Value) -> Value {
            if let Value::Ptr(p) = tensor {
                let mut out_len: i64 = 0;
                let ndim = unsafe { torch_tensor_dim(*p) };
                let mut sizes: Vec<i64> = vec![0i64; ndim as usize];
                unsafe { torch_tensor_sizes(*p, sizes.as_mut_ptr()) };
                let total: i64 = sizes.iter().product();
                let mut data: Vec<f32> = vec![0.0f32; total as usize];
                unsafe { torch_tensor_to_float_array(*p, data.as_mut_ptr(), &mut out_len) };
                data.truncate(out_len as usize);
                Value::List(data.into_iter().map(|x| Value::Float(x as f64)).collect())
            } else { Value::List(vec![]) }
        }

        pub fn free(tensor: &Value) {
            if let Value::Ptr(p) = tensor { unsafe { torch_tensor_free(*p) } }
        }

        pub fn requires_grad(tensor: &Value, req: bool) {
            if let Value::Ptr(p) = tensor { unsafe { torch_tensor_requires_grad(*p, if req { 1 } else { 0 }) } }
        }

        pub fn backward(tensor: &Value) {
            if let Value::Ptr(p) = tensor { unsafe { torch_tensor_backward(*p) } }
        }

        pub fn grad(tensor: &Value) -> Value {
            if let Value::Ptr(p) = tensor {
                let ptr = unsafe { torch_tensor_grad(*p) };
                if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
            } else { Value::None }
        }

        pub fn linear(in_features: i32, out_features: i32, bias: bool) -> Value {
            let ptr = unsafe { torch_linear_create(in_features, out_features, if bias { 1 } else { 0 }) };
            if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
        }

        pub fn linear_forward(module: &Value, input: &Value) -> Value {
            match (module, input) {
                (Value::Ptr(pm), Value::Ptr(pi)) => {
                    let ptr = unsafe { torch_linear_forward(*pm, *pi) };
                    if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
                }
                _ => Value::None,
            }
        }

        pub fn module_free(module: &Value) {
            if let Value::Ptr(p) = module { unsafe { torch_module_free(*p) } }
        }
    }

    pub mod opengl {
        use super::super::Value;
        use std::ffi::c_void;
        use std::os::raw::c_char;

        extern "C" {
            fn glfw_init() -> i32;
            fn glfw_create_window(width: i32, height: i32, title: *const c_char) -> *mut c_void;
            fn glfw_make_context_current(window: *mut c_void);
            fn glfw_swap_buffers(window: *mut c_void);
            fn glfw_poll_events();
            fn glfw_window_should_close(window: *mut c_void) -> i32;
            fn glfw_destroy_window(window: *mut c_void);
            fn glfw_terminate();
            fn glfw_get_time() -> f64;
            fn gl_clear_color(r: f32, g: f32, b: f32, a: f32);
            fn gl_clear(mask: i32);
            fn gl_viewport(x: i32, y: i32, w: i32, h: i32);
            fn gl_begin(mode: i32);
            fn gl_end();
            fn gl_vertex2f(x: f32, y: f32);
            fn gl_vertex3f(x: f32, y: f32, z: f32);
            fn gl_color3f(r: f32, g: f32, b: f32);
            fn gl_color4f(r: f32, g: f32, b: f32, a: f32);
            fn gl_load_identity();
            fn gl_translatef(x: f32, y: f32, z: f32);
            fn gl_rotatef(angle: f32, x: f32, y: f32, z: f32);
            fn gl_ortho(left: f32, right: f32, bottom: f32, top: f32, near_val: f32, far_val: f32);
            fn gl_matrix_mode(mode: i32);
            fn gl_enable(cap: i32);
            fn gl_disable(cap: i32);
            fn gl_flush();
            fn gl_get_error() -> i32;
            fn gl_create_shader(shader_type: i32) -> u32;
            fn gl_shader_source(shader: u32, source: *const c_char);
            fn gl_compile_shader(shader: u32);
            fn gl_create_program() -> u32;
            fn gl_attach_shader(program: u32, shader: u32);
            fn gl_link_program(program: u32);
            fn gl_use_program(program: u32);
        }

        pub fn construct(class: &str, args: Vec<Value>) -> Value {
            match class {
                "GLFWwindow" | "Window" => {
                    let w = args.get(0).map(|v| v.to_int()).unwrap_or(640) as i32;
                    let h = args.get(1).map(|v| v.to_int()).unwrap_or(480) as i32;
                    let title = args.get(2).map(|v| v.to_string()).unwrap_or_default();
                    let c = std::ffi::CString::new(title).unwrap_or_default();
                    Value::Ptr(unsafe { glfw_create_window(w, h, c.as_ptr()) })
                }
                _ => Value::None,
            }
        }

        pub fn method(obj: &Value, name: &str, args: Vec<Value>) -> Value {
            let ptr = match obj { Value::Ptr(p) => *p, _ => return Value::None };
            match name {
                "make_context_current" | "makeContextCurrent" | "make_context_current_" => {
                    unsafe { glfw_make_context_current(ptr) }
                }
                "swap_buffers" | "swapBuffers" | "swap_buffers_" => {
                    unsafe { glfw_swap_buffers(ptr) }
                }
                "should_close" | "shouldClose" | "should_close_" => {
                    return Value::Bool(unsafe { glfw_window_should_close(ptr) } != 0);
                }
                "destroy" | "destroy_" => {
                    unsafe { glfw_destroy_window(ptr) }
                }
                _ => {}
            }
            Value::None
        }

        pub fn init() -> Value {
            Value::Int(unsafe { glfw_init() } as i64)
        }

        pub fn poll_events() -> Value {
            unsafe { glfw_poll_events() }
            Value::None
        }

        pub fn terminate() -> Value {
            unsafe { glfw_terminate() }
            Value::None
        }

        pub fn get_time() -> Value {
            Value::Float(unsafe { glfw_get_time() })
        }

        pub fn clear_color(r: f32, g: f32, b: f32, a: f32) -> Value {
            unsafe { gl_clear_color(r, g, b, a) }
            Value::None
        }

        pub fn clear(mask: i32) -> Value {
            unsafe { gl_clear(mask) }
            Value::None
        }

        pub fn viewport(x: i32, y: i32, w: i32, h: i32) -> Value {
            unsafe { gl_viewport(x, y, w, h) }
            Value::None
        }

        pub fn begin(mode: i32) -> Value {
            unsafe { gl_begin(mode) }
            Value::None
        }

        pub fn end_() -> Value {
            unsafe { gl_end() }
            Value::None
        }

        pub fn vertex2f(x: f32, y: f32) -> Value {
            unsafe { gl_vertex2f(x, y) }
            Value::None
        }

        pub fn vertex3f(x: f32, y: f32, z: f32) -> Value {
            unsafe { gl_vertex3f(x, y, z) }
            Value::None
        }

        pub fn color3f(r: f32, g: f32, b: f32) -> Value {
            unsafe { gl_color3f(r, g, b) }
            Value::None
        }

        pub fn color4f(r: f32, g: f32, b: f32, a: f32) -> Value {
            unsafe { gl_color4f(r, g, b, a) }
            Value::None
        }

        pub fn load_identity() -> Value {
            unsafe { gl_load_identity() }
            Value::None
        }

        pub fn translatef(x: f32, y: f32, z: f32) -> Value {
            unsafe { gl_translatef(x, y, z) }
            Value::None
        }

        pub fn rotatef(angle: f32, x: f32, y: f32, z: f32) -> Value {
            unsafe { gl_rotatef(angle, x, y, z) }
            Value::None
        }

        pub fn ortho(left: f32, right: f32, bottom: f32, top: f32, near_val: f32, far_val: f32) -> Value {
            unsafe { gl_ortho(left, right, bottom, top, near_val, far_val) }
            Value::None
        }

        pub fn matrix_mode(mode: i32) -> Value {
            unsafe { gl_matrix_mode(mode) }
            Value::None
        }

        pub fn enable(cap: i32) -> Value {
            unsafe { gl_enable(cap) }
            Value::None
        }

        pub fn disable(cap: i32) -> Value {
            unsafe { gl_disable(cap) }
            Value::None
        }

        pub fn flush() -> Value {
            unsafe { gl_flush() }
            Value::None
        }

        pub fn get_error() -> Value {
            Value::Int(unsafe { gl_get_error() } as i64)
        }

        pub fn create_shader(shader_type: i32) -> Value {
            Value::Int(unsafe { gl_create_shader(shader_type) } as i64)
        }

        pub fn shader_source(shader: u32, source: &str) -> Value {
            let c = std::ffi::CString::new(source).unwrap_or_default();
            unsafe { gl_shader_source(shader, c.as_ptr()) }
            Value::None
        }

        pub fn compile_shader(shader: u32) -> Value {
            unsafe { gl_compile_shader(shader) }
            Value::None
        }

        pub fn create_program() -> Value {
            Value::Int(unsafe { gl_create_program() } as i64)
        }

        pub fn attach_shader(program: u32, shader: u32) -> Value {
            unsafe { gl_attach_shader(program, shader) }
            Value::None
        }

        pub fn link_program(program: u32) -> Value {
            unsafe { gl_link_program(program) }
            Value::None
        }

        pub fn use_program(program: u32) -> Value {
            unsafe { gl_use_program(program) }
            Value::None
        }
    }

    pub mod sdl2 {
        use super::super::Value;
        use std::ffi::c_void;
        use std::os::raw::c_char;

        extern "C" {
            fn sdl_init(flags: u32) -> i32;
            fn sdl_create_window(title: *const c_char, x: i32, y: i32, w: i32, h: i32, flags: u32) -> *mut c_void;
            fn sdl_destroy_window(win: *mut c_void);
            fn sdl_create_renderer(window: *mut c_void, index: i32, flags: u32) -> *mut c_void;
            fn sdl_destroy_renderer(renderer: *mut c_void);
            fn sdl_render_clear(renderer: *mut c_void);
            fn sdl_render_present(renderer: *mut c_void);
            fn sdl_set_render_draw_color(renderer: *mut c_void, r: u8, g: u8, b: u8, a: u8);
            fn sdl_render_fill_rect(renderer: *mut c_void, x: i32, y: i32, w: i32, h: i32);
            fn sdl_poll_event(type_out: *mut u32, keycode_out: *mut u32, scancode_out: *mut u32, keymod_out: *mut u8, repeat_out: *mut u8) -> i32;
            fn sdl_delay(ms: u32);
            fn sdl_get_ticks() -> u32;
            fn sdl_create_texture_from_surface(renderer: *mut c_void, surface: *mut c_void) -> *mut c_void;
            fn sdl_render_copy(renderer: *mut c_void, texture: *mut c_void, sx: i32, sy: i32, sw: i32, sh: i32, dx: i32, dy: i32, dw: i32, dh: i32);
            fn sdl_render_copy_full(renderer: *mut c_void, texture: *mut c_void);
            fn sdl_destroy_texture(texture: *mut c_void);
            fn sdl_image_load(file: *const c_char) -> *mut c_void;
            fn sdl_free_surface(surface: *mut c_void);
            fn sdl_get_keyboard_state(numkeys: *mut i32) -> *const u8;
            fn sdl_ttf_init() -> i32;
            fn sdl_ttf_quit();
            fn sdl_ttf_open_font(path: *const c_char, size: i32) -> *mut c_void;
            fn sdl_ttf_render_text_solid(font: *mut c_void, text: *const c_char, r: u8, g: u8, b: u8) -> *mut c_void;
            fn sdl_mixer_init(freq: i32, format: u16, channels: i32, chunksize: i32) -> i32;
            fn sdl_mixer_load_music(file: *const c_char) -> *mut c_void;
            fn sdl_mixer_play_music(music: *mut c_void, loops: i32);
            fn sdl_mixer_load_chunk(file: *const c_char) -> *mut c_void;
            fn sdl_mixer_play_channel(channel: i32, chunk: *mut c_void, loops: i32) -> i32;
            fn sdl_quit();
        }

        pub fn init(flags: u32) -> Value {
            Value::Int(unsafe { sdl_init(flags) } as i64)
        }

        pub fn create_window(title: &str, x: i32, y: i32, w: i32, h: i32, flags: u32) -> Value {
            let c = std::ffi::CString::new(title).unwrap_or_default();
            let ptr = unsafe { sdl_create_window(c.as_ptr(), x, y, w, h, flags) };
            if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
        }

        pub fn destroy_window(win: &Value) {
            if let Value::Ptr(p) = win { unsafe { sdl_destroy_window(*p) } }
        }

        pub fn create_renderer(window: &Value, index: i32, flags: u32) -> Value {
            let ptr = match window { Value::Ptr(p) => *p, _ => return Value::None };
            let r = unsafe { sdl_create_renderer(ptr, index, flags) };
            if r.is_null() { Value::None } else { Value::Ptr(r) }
        }

        pub fn destroy_renderer(renderer: &Value) {
            if let Value::Ptr(p) = renderer { unsafe { sdl_destroy_renderer(*p) } }
        }

        pub fn render_clear(renderer: &Value) {
            if let Value::Ptr(p) = renderer { unsafe { sdl_render_clear(*p) } }
        }

        pub fn render_present(renderer: &Value) {
            if let Value::Ptr(p) = renderer { unsafe { sdl_render_present(*p) } }
        }

        pub fn set_render_draw_color(renderer: &Value, r: u8, g: u8, b: u8, a: u8) {
            if let Value::Ptr(p) = renderer { unsafe { sdl_set_render_draw_color(*p, r, g, b, a) } }
        }

        pub fn render_fill_rect(renderer: &Value, x: i32, y: i32, w: i32, h: i32) {
            if let Value::Ptr(p) = renderer { unsafe { sdl_render_fill_rect(*p, x, y, w, h) } }
        }

        pub fn poll_event() -> Value {
            let mut type_out: u32 = 0;
            let mut keycode_out: u32 = 0;
            let mut scancode_out: u32 = 0;
            let mut keymod_out: u8 = 0;
            let mut repeat_out: u8 = 0;
            let ret = unsafe { sdl_poll_event(&mut type_out, &mut keycode_out, &mut scancode_out, &mut keymod_out, &mut repeat_out) };
            if ret == 0 {
                Value::None
            } else {
                Value::Tuple(vec![
                    Value::Int(type_out as i64),
                    Value::Int(keycode_out as i64),
                    Value::Int(scancode_out as i64),
                    Value::Int(keymod_out as i64),
                    Value::Int(repeat_out as i64),
                ])
            }
        }

        pub fn delay(ms: u32) {
            unsafe { sdl_delay(ms) }
        }

        pub fn get_ticks() -> Value {
            Value::Int(unsafe { sdl_get_ticks() } as i64)
        }

        pub fn create_texture_from_surface(renderer: &Value, surface: &Value) -> Value {
            let rp = match renderer { Value::Ptr(p) => *p, _ => return Value::None };
            let sp = match surface { Value::Ptr(p) => *p, _ => return Value::None };
            let tex = unsafe { sdl_create_texture_from_surface(rp, sp) };
            if tex.is_null() { Value::None } else { Value::Ptr(tex) }
        }

        pub fn render_copy(renderer: &Value, texture: &Value, sx: i32, sy: i32, sw: i32, sh: i32, dx: i32, dy: i32, dw: i32, dh: i32) {
            let rp = match renderer { Value::Ptr(p) => *p, _ => return };
            let tp = match texture { Value::Ptr(p) => *p, _ => return };
            unsafe { sdl_render_copy(rp, tp, sx, sy, sw, sh, dx, dy, dw, dh) }
        }

        pub fn render_copy_full(renderer: &Value, texture: &Value) {
            let rp = match renderer { Value::Ptr(p) => *p, _ => return };
            let tp = match texture { Value::Ptr(p) => *p, _ => return };
            unsafe { sdl_render_copy_full(rp, tp) }
        }

        pub fn destroy_texture(texture: &Value) {
            if let Value::Ptr(p) = texture { unsafe { sdl_destroy_texture(*p) } }
        }

        pub fn image_load(file: &str) -> Value {
            let c = std::ffi::CString::new(file).unwrap_or_default();
            let ptr = unsafe { sdl_image_load(c.as_ptr()) };
            if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
        }

        pub fn free_surface(surface: &Value) {
            if let Value::Ptr(p) = surface { unsafe { sdl_free_surface(*p) } }
        }

        pub fn get_keyboard_state() -> Value {
            let mut numkeys: i32 = 0;
            let ptr = unsafe { sdl_get_keyboard_state(&mut numkeys) };
            if ptr.is_null() { return Value::List(vec![]) }
            let mut keys = Vec::with_capacity(numkeys as usize);
            for i in 0..numkeys {
                keys.push(Value::Int(unsafe { *ptr.offset(i as isize) } as i64));
            }
            Value::List(keys)
        }

        pub fn ttf_init() -> Value {
            Value::Int(unsafe { sdl_ttf_init() } as i64)
        }

        pub fn ttf_quit() {
            unsafe { sdl_ttf_quit() }
        }

        pub fn ttf_open_font(path: &str, size: i32) -> Value {
            let c = std::ffi::CString::new(path).unwrap_or_default();
            let ptr = unsafe { sdl_ttf_open_font(c.as_ptr(), size) };
            if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
        }

        pub fn ttf_render_text_solid(font: &Value, text: &str, r: u8, g: u8, b: u8) -> Value {
            let fp = match font { Value::Ptr(p) => *p, _ => return Value::None };
            let c = std::ffi::CString::new(text).unwrap_or_default();
            let ptr = unsafe { sdl_ttf_render_text_solid(fp, c.as_ptr(), r, g, b) };
            if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
        }

        pub fn mixer_init(freq: i32, format: u16, channels: i32, chunksize: i32) -> Value {
            Value::Int(unsafe { sdl_mixer_init(freq, format, channels, chunksize) } as i64)
        }

        pub fn mixer_load_music(file: &str) -> Value {
            let c = std::ffi::CString::new(file).unwrap_or_default();
            let ptr = unsafe { sdl_mixer_load_music(c.as_ptr()) };
            if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
        }

        pub fn mixer_play_music(music: &Value, loops: i32) {
            if let Value::Ptr(p) = music { unsafe { sdl_mixer_play_music(*p, loops) } }
        }

        pub fn mixer_load_chunk(file: &str) -> Value {
            let c = std::ffi::CString::new(file).unwrap_or_default();
            let ptr = unsafe { sdl_mixer_load_chunk(c.as_ptr()) };
            if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
        }

        pub fn mixer_play_channel(channel: i32, chunk: &Value, loops: i32) -> Value {
            let cp = match chunk { Value::Ptr(p) => *p, _ => return Value::Int(-1) };
            Value::Int(unsafe { sdl_mixer_play_channel(channel, cp, loops) } as i64)
        }

        pub fn quit() {
            unsafe { sdl_quit() }
        }

        pub fn construct(class: &str, args: Vec<Value>) -> Value {
            match class {
                "SDL_Window" | "Window" => {
                    let title = args.get(0).map(|v| v.to_string()).unwrap_or_default();
                    let x = args.get(1).map(|v| v.to_int()).unwrap_or(0x1FFF0000) as i32;
                    let y = args.get(2).map(|v| v.to_int()).unwrap_or(0x1FFF0000) as i32;
                    let w = args.get(3).map(|v| v.to_int()).unwrap_or(640) as i32;
                    let h = args.get(4).map(|v| v.to_int()).unwrap_or(480) as i32;
                    let flags = args.get(5).map(|v| v.to_int()).unwrap_or(0) as u32;
                    create_window(&title, x, y, w, h, flags)
                }
                "SDL_Renderer" | "Renderer" => {
                    let win = args.get(0).unwrap_or(&Value::None);
                    let idx = args.get(1).map(|v| v.to_int()).unwrap_or(-1) as i32;
                    let flags = args.get(2).map(|v| v.to_int()).unwrap_or(0) as u32;
                    create_renderer(win, idx, flags)
                }
                "SDL_Texture" | "Texture" => Value::None,
                "SDL_Surface" | "Surface" => Value::None,
                "SDL_Font" | "Font" => {
                    let path = args.get(0).map(|v| v.to_string()).unwrap_or_default();
                    let size = args.get(1).map(|v| v.to_int()).unwrap_or(16) as i32;
                    ttf_open_font(&path, size)
                }
                "SDL_Music" | "Music" => {
                    let path = args.get(0).map(|v| v.to_string()).unwrap_or_default();
                    mixer_load_music(&path)
                }
                "SDL_Chunk" | "Chunk" => {
                    let path = args.get(0).map(|v| v.to_string()).unwrap_or_default();
                    mixer_load_chunk(&path)
                }
                _ => Value::None,
            }
        }

        pub fn method(obj: &Value, name: &str, args: Vec<Value>) -> Value {
            let ptr = match obj { Value::Ptr(p) => *p, _ => return Value::None };
            match name {
                "destroy" | "destroy_" => {
                    unsafe { sdl_destroy_window(ptr) }
                }
                "clear" | "clear_" => {
                    unsafe { sdl_render_clear(ptr) }
                }
                "present" | "present_" => {
                    unsafe { sdl_render_present(ptr) }
                }
                "set_draw_color" | "set_draw_color_" | "setDrawColor" | "setDrawColor_" => {
                    let r = args.get(0).map(|v| v.to_int()).unwrap_or(0) as u8;
                    let g = args.get(1).map(|v| v.to_int()).unwrap_or(0) as u8;
                    let b = args.get(2).map(|v| v.to_int()).unwrap_or(0) as u8;
                    let a = args.get(3).map(|v| v.to_int()).unwrap_or(255) as u8;
                    unsafe { sdl_set_render_draw_color(ptr, r, g, b, a) }
                }
                "fill_rect" | "fill_rect_" | "fillRect" | "fillRect_" => {
                    let x = args.get(0).map(|v| v.to_int()).unwrap_or(0) as i32;
                    let y = args.get(1).map(|v| v.to_int()).unwrap_or(0) as i32;
                    let w = args.get(2).map(|v| v.to_int()).unwrap_or(10) as i32;
                    let h = args.get(3).map(|v| v.to_int()).unwrap_or(10) as i32;
                    unsafe { sdl_render_fill_rect(ptr, x, y, w, h) }
                }
                "copy" | "copy_" | "renderCopy" | "renderCopy_" => {
                    let tex = args.get(0).unwrap_or(&Value::None);
                    let tp = match tex { Value::Ptr(p) => *p, _ => return Value::None };
                    if args.len() >= 9 {
                        let sx = args.get(1).map(|v| v.to_int()).unwrap_or(0) as i32;
                        let sy = args.get(2).map(|v| v.to_int()).unwrap_or(0) as i32;
                        let sw = args.get(3).map(|v| v.to_int()).unwrap_or(0) as i32;
                        let sh = args.get(4).map(|v| v.to_int()).unwrap_or(0) as i32;
                        let dx = args.get(5).map(|v| v.to_int()).unwrap_or(0) as i32;
                        let dy = args.get(6).map(|v| v.to_int()).unwrap_or(0) as i32;
                        let dw = args.get(7).map(|v| v.to_int()).unwrap_or(0) as i32;
                        let dh = args.get(8).map(|v| v.to_int()).unwrap_or(0) as i32;
                        unsafe { sdl_render_copy(ptr, tp, sx, sy, sw, sh, dx, dy, dw, dh) }
                    } else {
                        unsafe { sdl_render_copy_full(ptr, tp) }
                    }
                }
                "load" | "load_" => {
                    let file = args.get(0).map(|v| v.to_string()).unwrap_or_default();
                    let c = std::ffi::CString::new(file).unwrap_or_default();
                    let tex = unsafe { sdl_image_load(c.as_ptr()) };
                    return if tex.is_null() { Value::None } else { Value::Ptr(tex) };
                }
                "free" | "free_" => {
                    unsafe { sdl_free_surface(ptr) }
                }
                "close" | "close_" => {
                    unsafe { sdl_destroy_renderer(ptr) }
                }
                "destroy_texture" | "destroy_texture_" => {
                    unsafe { sdl_destroy_texture(ptr) }
                }
                "render_text" | "render_text_" | "renderText" | "renderText_" => {
                    let text = args.get(0).map(|v| v.to_string()).unwrap_or_default();
                    let r = args.get(1).map(|v| v.to_int()).unwrap_or(255) as u8;
                    let g = args.get(2).map(|v| v.to_int()).unwrap_or(255) as u8;
                    let b = args.get(3).map(|v| v.to_int()).unwrap_or(255) as u8;
                    let c = std::ffi::CString::new(text).unwrap_or_default();
                    let surf = unsafe { sdl_ttf_render_text_solid(ptr, c.as_ptr(), r, g, b) };
                    return if surf.is_null() { Value::None } else { Value::Ptr(surf) };
                }
                "play" | "play_" => {
                    let loops = args.get(0).map(|v| v.to_int()).unwrap_or(0) as i32;
                    unsafe { sdl_mixer_play_music(ptr, loops) }
                }
                "play_channel" | "play_channel_" => {
                    let channel = args.get(0).map(|v| v.to_int()).unwrap_or(-1) as i32;
                    let loops = args.get(1).map(|v| v.to_int()).unwrap_or(0) as i32;
                    return Value::Int(unsafe { sdl_mixer_play_channel(channel, ptr, loops) } as i64);
                }
                _ => {}
            }
            Value::None
        }
    }

    pub mod gtk4 {
        use super::super::Value;
        use std::ffi::c_void;
        use std::os::raw::c_char;

        extern "C" {
            fn gtk4_app_create() -> *mut c_void;
            fn gtk4_app_run(app: *mut c_void, argc: i32, argv: *mut *mut c_char) -> i32;
            fn gtk4_app_quit(app: *mut c_void);
            fn gtk4_window_new(app: *mut c_void) -> *mut c_void;
            fn gtk4_window_set_default_size(win: *mut c_void, w: i32, h: i32);
            fn gtk4_window_set_child(win: *mut c_void, child: *mut c_void);
            fn gtk4_window_set_titlebar(win: *mut c_void, bar: *mut c_void);
            fn gtk4_widget_set_visible(widget: *mut c_void, visible: i32);
            fn gtk4_button_new_with_label(label: *const c_char) -> *mut c_void;
            fn gtk4_button_set_label(btn: *mut c_void, label: *const c_char);
            fn gtk4_label_new(text: *const c_char) -> *mut c_void;
            fn gtk4_entry_new() -> *mut c_void;
            fn gtk4_entry_set_placeholder_text(entry: *mut c_void, text: *const c_char);
            fn gtk4_entry_get_text(entry: *mut c_void) -> *const c_char;
            fn gtk4_box_new(orientation: i32, spacing: i32) -> *mut c_void;
            fn gtk4_box_append(box_: *mut c_void, child: *mut c_void);
            fn gtk4_scrolled_window_new() -> *mut c_void;
            fn gtk4_text_view_new() -> *mut c_void;
            fn gtk4_text_view_get_buffer(view: *mut c_void) -> *mut c_void;
            fn gtk4_text_buffer_get_text(buf: *mut c_void, start: i32, end: i32) -> *const c_char;
            fn gtk4_text_buffer_set_text(buf: *mut c_void, text: *const c_char);
            fn gtk4_header_bar_new() -> *mut c_void;
            fn gtk4_signal_connect(instance: *mut c_void, signal: *const c_char, cb_id: i32) -> i64;
            fn gtk4_register_callback(cb: extern "C" fn()) -> i32;
            fn gtk4_set_title(widget: *mut c_void, title: *const c_char);
            fn gtk4_set_text(widget: *mut c_void, text: *const c_char);
            fn gtk4_get_text(widget: *mut c_void) -> *const c_char;
        }

        pub fn construct(class: &str, args: Vec<Value>) -> Value {
            match class {
                "GtkApplication" => Value::Ptr(unsafe { gtk4_app_create() }),
                "GtkWindow" => {
                    let app = args.first().and_then(|v| match v { Value::Ptr(p) => Some(*p), _ => None }).unwrap_or(std::ptr::null_mut());
                    Value::Ptr(unsafe { gtk4_window_new(app) })
                }
                "GtkBox" => {
                    let orientation = args.get(0).map(|v| v.to_int()).unwrap_or(1) as i32;
                    let spacing = args.get(1).map(|v| v.to_int()).unwrap_or(0) as i32;
                    Value::Ptr(unsafe { gtk4_box_new(orientation, spacing) })
                }
                "GtkButton" => {
                    let t = args.first().map(|v| v.to_string()).unwrap_or_default();
                    let c = std::ffi::CString::new(t).unwrap_or_default();
                    Value::Ptr(unsafe { gtk4_button_new_with_label(c.as_ptr()) })
                }
                "GtkLabel" => {
                    let t = args.first().map(|v| v.to_string()).unwrap_or_default();
                    let c = std::ffi::CString::new(t).unwrap_or_default();
                    Value::Ptr(unsafe { gtk4_label_new(c.as_ptr()) })
                }
                "GtkEntry" => Value::Ptr(unsafe { gtk4_entry_new() }),
                "GtkScrolledWindow" => Value::Ptr(unsafe { gtk4_scrolled_window_new() }),
                "GtkTextView" => Value::Ptr(unsafe { gtk4_text_view_new() }),
                "GtkHeaderBar" => Value::Ptr(unsafe { gtk4_header_bar_new() }),
                _ => Value::None,
            }
        }

        pub fn method(obj: &Value, name: &str, args: Vec<Value>) -> Value {
            let ptr = match obj { Value::Ptr(p) => *p, _ => return Value::None };
            match name {
                "run" | "run_" => {
                    return Value::Int(unsafe { gtk4_app_run(ptr, 0, std::ptr::null_mut()) } as i64);
                }
                "quit" | "quit_" => {
                    unsafe { gtk4_app_quit(ptr) }
                }
                "set_default_size" | "set_default_size_" => {
                    let w = args.get(0).map(|v| v.to_int()).unwrap_or(800) as i32;
                    let h = args.get(1).map(|v| v.to_int()).unwrap_or(600) as i32;
                    unsafe { gtk4_window_set_default_size(ptr, w, h) }
                }
                "set_child" | "set_child_" => {
                    if let Some(Value::Ptr(c)) = args.first() { unsafe { gtk4_window_set_child(ptr, *c) } }
                }
                "set_titlebar" | "set_titlebar_" => {
                    if let Some(Value::Ptr(b)) = args.first() { unsafe { gtk4_window_set_titlebar(ptr, *b) } }
                }
                "show" | "show_" => unsafe { gtk4_widget_set_visible(ptr, 1) },
                "set_visible" | "set_visible_" => {
                    let v = if args.first().map(|a| a.to_bool()).unwrap_or(false) { 1 } else { 0 };
                    unsafe { gtk4_widget_set_visible(ptr, v) }
                }
                "set_label" | "set_label_" => {
                    let t = args.first().map(|v| v.to_string()).unwrap_or_default();
                    let c = std::ffi::CString::new(t).unwrap_or_default();
                    unsafe { gtk4_button_set_label(ptr, c.as_ptr()) }
                }
                "set_title" | "set_title_" => {
                    let t = args.first().map(|v| v.to_string()).unwrap_or_default();
                    let c = std::ffi::CString::new(t).unwrap_or_default();
                    unsafe { gtk4_set_title(ptr, c.as_ptr()) }
                }
                "set_text" | "set_text_" => {
                    let t = args.first().map(|v| v.to_string()).unwrap_or_default();
                    let c = std::ffi::CString::new(t).unwrap_or_default();
                    unsafe { gtk4_set_text(ptr, c.as_ptr()) }
                }
                "set_placeholder_text" | "set_placeholder_text_" => {
                    let t = args.first().map(|v| v.to_string()).unwrap_or_default();
                    let c = std::ffi::CString::new(t).unwrap_or_default();
                    unsafe { gtk4_entry_set_placeholder_text(ptr, c.as_ptr()) }
                }
                "get_text" | "get_text_" => {
                    let s = unsafe { gtk4_get_text(ptr) };
                    if s.is_null() { return Value::Str(String::new()); }
                    return Value::Str(unsafe { std::ffi::CStr::from_ptr(s) }.to_string_lossy().into_owned());
                }
                "append" | "append_" => {
                    if let Some(Value::Ptr(c)) = args.first() { unsafe { gtk4_box_append(ptr, *c) } }
                }
                "get_buffer" | "get_buffer_" => {
                    let buf = unsafe { gtk4_text_view_get_buffer(ptr) };
                    return if buf.is_null() { Value::None } else { Value::Ptr(buf) };
                }
                "connect" | "connect_" => {
                    // connect(signal_name, callback): register callback and connect signal
                }
                _ => {}
            }
            Value::None
        }
    }

    pub mod ffmpeg {
        use super::super::Value;
        use std::ffi::c_void;
        use std::os::raw::c_char;

        extern "C" {
            fn ffmpeg_init() -> i32;
            fn ffmpeg_open_input(path: *const c_char) -> *mut c_void;
            fn ffmpeg_close_input(ctx: *mut c_void);
            fn ffmpeg_find_stream(ctx: *mut c_void, type_: i32) -> i32;
            fn ffmpeg_get_codec_params(ctx: *mut c_void, stream_idx: i32, out_w: *mut i32, out_h: *mut i32, out_codec_name: *mut *mut c_char, out_pix_fmt: *mut i32);
            fn ffmpeg_read_frame(ctx: *mut c_void, out_w: *mut i32, out_h: *mut i32, out_channels: *mut i32, out_data: *mut *mut u8) -> i32;
            fn ffmpeg_seek(ctx: *mut c_void, timestamp: f64) -> i32;
            fn ffmpeg_get_duration(ctx: *mut c_void) -> f64;
            fn ffmpeg_extract_thumbnail(ctx: *mut c_void, time_sec: f64, out_w: *mut i32, out_h: *mut i32, out_data: *mut *mut u8) -> i32;
            fn ffmpeg_get_metadata(ctx: *mut c_void, key: *const c_char) -> *mut c_char;
            fn ffmpeg_get_all_metadata(ctx: *mut c_void) -> *mut c_char;
            fn ffmpeg_free_string(s: *mut c_char);
            fn ffmpeg_free_data(data: *mut u8);
        }

        pub fn init() -> Value {
            Value::Int(unsafe { ffmpeg_init() } as i64)
        }

        pub fn open(path: &str) -> Value {
            let cpath = std::ffi::CString::new(path).unwrap_or_default();
            let ptr = unsafe { ffmpeg_open_input(cpath.as_ptr()) };
            if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
        }

        pub fn close(ctx: &Value) {
            if let Value::Ptr(p) = ctx {
                unsafe { ffmpeg_close_input(*p) }
            }
        }

        pub fn find_stream(ctx: &Value, type_: i64) -> Value {
            if let Value::Ptr(p) = ctx {
                Value::Int(unsafe { ffmpeg_find_stream(*p, type_ as i32) } as i64)
            } else {
                Value::Int(-1)
            }
        }

        pub fn get_codec_params(ctx: &Value, stream_idx: i64) -> Value {
            let ptr = match ctx { Value::Ptr(p) => *p, _ => return Value::None };
            let mut w: i32 = 0;
            let mut h: i32 = 0;
            let mut codec_name: *mut c_char = std::ptr::null_mut();
            let mut pix_fmt: i32 = 0;
            unsafe {
                ffmpeg_get_codec_params(ptr, stream_idx as i32, &mut w, &mut h, &mut codec_name, &mut pix_fmt);
            }
            let name = if !codec_name.is_null() {
                let s = unsafe { std::ffi::CStr::from_ptr(codec_name) }.to_string_lossy().into_owned();
                unsafe { ffmpeg_free_string(codec_name) };
                s
            } else {
                String::new()
            };
            Value::Tuple(vec![Value::Int(w as i64), Value::Int(h as i64), Value::Str(name), Value::Int(pix_fmt as i64)])
        }

        pub fn read_frame(ctx: &Value) -> Value {
            let ptr = match ctx { Value::Ptr(p) => *p, _ => return Value::Int(-1) };
            let mut w: i32 = 0;
            let mut h: i32 = 0;
            let mut ch: i32 = 0;
            let mut data: *mut u8 = std::ptr::null_mut();
            let ret = unsafe { ffmpeg_read_frame(ptr, &mut w, &mut h, &mut ch, &mut data) };
            if ret <= 0 || data.is_null() {
                return Value::Int(-1);
            }
            let len = (w * h * ch) as usize;
            let bytes = unsafe { std::slice::from_raw_parts(data, len) }.to_vec();
            unsafe { ffmpeg_free_data(data) };
            Value::Tuple(vec![Value::Int(w as i64), Value::Int(h as i64), Value::Int(ch as i64), Value::Bytes(bytes)])
        }

        pub fn seek(ctx: &Value, timestamp: f64) -> Value {
            if let Value::Ptr(p) = ctx {
                Value::Int(unsafe { ffmpeg_seek(*p, timestamp) } as i64)
            } else {
                Value::Int(-1)
            }
        }

        pub fn get_duration(ctx: &Value) -> Value {
            if let Value::Ptr(p) = ctx {
                Value::Float(unsafe { ffmpeg_get_duration(*p) })
            } else {
                Value::Float(0.0)
            }
        }

        pub fn extract_thumbnail(ctx: &Value, time_sec: f64) -> Value {
            let ptr = match ctx { Value::Ptr(p) => *p, _ => return Value::Tuple(vec![Value::Int(0), Value::Int(0), Value::Bytes(vec![])]) };
            let mut w: i32 = 0;
            let mut h: i32 = 0;
            let mut data: *mut u8 = std::ptr::null_mut();
            let ret = unsafe { ffmpeg_extract_thumbnail(ptr, time_sec, &mut w, &mut h, &mut data) };
            if ret != 0 || data.is_null() {
                return Value::Tuple(vec![Value::Int(0), Value::Int(0), Value::Bytes(vec![])]);
            }
            let len = (w * h * 4) as usize;
            let bytes = unsafe { std::slice::from_raw_parts(data, len) }.to_vec();
            unsafe { ffmpeg_free_data(data) };
            Value::Tuple(vec![Value::Int(w as i64), Value::Int(h as i64), Value::Bytes(bytes)])
        }

        pub fn get_metadata(ctx: &Value, key: &str) -> Value {
            let ptr = match ctx { Value::Ptr(p) => *p, _ => return Value::None };
            let ckey = std::ffi::CString::new(key).unwrap_or_default();
            let s = unsafe { ffmpeg_get_metadata(ptr, ckey.as_ptr()) };
            if s.is_null() { return Value::None }
            let result = Value::Str(unsafe { std::ffi::CStr::from_ptr(s) }.to_string_lossy().into_owned());
            unsafe { ffmpeg_free_string(s) };
            result
        }

        pub fn get_all_metadata(ctx: &Value) -> Value {
            let ptr = match ctx { Value::Ptr(p) => *p, _ => return Value::Str(String::new()) };
            let s = unsafe { ffmpeg_get_all_metadata(ptr) };
            if s.is_null() { return Value::Str(String::new()) }
            let result = Value::Str(unsafe { std::ffi::CStr::from_ptr(s) }.to_string_lossy().into_owned());
            unsafe { ffmpeg_free_string(s) };
            result
        }
    }

    pub mod git {
        use super::super::Value;
        use std::ffi::c_void;
        use std::os::raw::c_char;

        extern "C" {
            fn git_init_() -> i32;
            fn git_shutdown_();
            fn git_clone_(url: *const c_char, path: *const c_char) -> i32;
            fn git_open_(path: *const c_char) -> *mut c_void;
            fn git_free_(repo: *mut c_void);
            fn git_commit_id_(repo: *mut c_void, branch: *const c_char) -> *mut c_char;
            fn git_branch_list_(repo: *mut c_void, out_count: *mut i32) -> *mut *mut c_char;
            fn git_status_(repo: *mut c_void, filepath: *const c_char) -> i32;
            fn git_add_(repo: *mut c_void, filepath: *const c_char) -> i32;
            fn git_commit_(repo: *mut c_void, message: *const c_char, name: *const c_char, email: *const c_char) -> i32;
            fn git_push_(repo: *mut c_void, remote_name: *const c_char, refspec: *const c_char) -> i32;
            fn git_pull_(repo: *mut c_void, remote_name: *const c_char, merge_branch: *const c_char) -> i32;
            fn git_diff_stats_(repo: *mut c_void) -> *mut c_char;
            fn git_log_(repo: *mut c_void, max_count: i32, out_count: *mut i32) -> *mut *mut c_char;
            fn git_free_str(s: *mut c_char);
            fn git_free_str_array(arr: *mut *mut c_char, count: i32);
        }

        pub fn init() -> Value {
            Value::Int(unsafe { git_init_() } as i64)
        }

        pub fn shutdown() -> Value {
            unsafe { git_shutdown_() }
            Value::None
        }

        pub fn clone_(url: &str, path: &str) -> Value {
            let curl = std::ffi::CString::new(url).unwrap_or_default();
            let cpath = std::ffi::CString::new(path).unwrap_or_default();
            Value::Int(unsafe { git_clone_(curl.as_ptr(), cpath.as_ptr()) } as i64)
        }

        pub fn open(path: &str) -> Value {
            let cpath = std::ffi::CString::new(path).unwrap_or_default();
            let ptr = unsafe { git_open_(cpath.as_ptr()) };
            if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
        }

        pub fn free(repo: &Value) {
            if let Value::Ptr(p) = repo {
                unsafe { git_free_(*p) }
            }
        }

        pub fn commit_id(repo: &Value, branch: Option<&str>) -> Value {
            let ptr = match repo { Value::Ptr(p) => *p, _ => return Value::None };
            let cbranch = branch.and_then(|b| std::ffi::CString::new(b).ok());
            let s = unsafe { git_commit_id_(ptr, cbranch.as_ref().map(|c| c.as_ptr()).unwrap_or(std::ptr::null())) };
            if s.is_null() { return Value::None }
            let result = Value::Str(unsafe { std::ffi::CStr::from_ptr(s) }.to_string_lossy().into_owned());
            unsafe { git_free_str(s) };
            result
        }

        pub fn branch_list(repo: &Value) -> Value {
            let ptr = match repo { Value::Ptr(p) => *p, _ => return Value::List(vec![]) };
            let mut count: i32 = 0;
            let arr = unsafe { git_branch_list_(ptr, &mut count) };
            if arr.is_null() { return Value::List(vec![]) }
            let mut branches = Vec::with_capacity(count as usize);
            for i in 0..count {
                let s = unsafe { *arr.offset(i as isize) };
                if !s.is_null() {
                    branches.push(Value::Str(unsafe { std::ffi::CStr::from_ptr(s) }.to_string_lossy().into_owned()));
                }
            }
            unsafe { git_free_str_array(arr, count) };
            Value::List(branches)
        }

        pub fn status(repo: &Value, filepath: &str) -> Value {
            let ptr = match repo { Value::Ptr(p) => *p, _ => return Value::Int(-1) };
            let cpath = std::ffi::CString::new(filepath).unwrap_or_default();
            Value::Int(unsafe { git_status_(ptr, cpath.as_ptr()) } as i64)
        }

        pub fn add(repo: &Value, filepath: &str) -> Value {
            let ptr = match repo { Value::Ptr(p) => *p, _ => return Value::Int(-1) };
            let cpath = std::ffi::CString::new(filepath).unwrap_or_default();
            Value::Int(unsafe { git_add_(ptr, cpath.as_ptr()) } as i64)
        }

        pub fn commit(repo: &Value, message: &str, name: &str, email: &str) -> Value {
            let ptr = match repo { Value::Ptr(p) => *p, _ => return Value::Int(-1) };
            let cmsg = std::ffi::CString::new(message).unwrap_or_default();
            let cname = std::ffi::CString::new(name).unwrap_or_default();
            let cemail = std::ffi::CString::new(email).unwrap_or_default();
            Value::Int(unsafe { git_commit_(ptr, cmsg.as_ptr(), cname.as_ptr(), cemail.as_ptr()) } as i64)
        }

        pub fn push(repo: &Value, remote_name: &str, refspec: &str) -> Value {
            let ptr = match repo { Value::Ptr(p) => *p, _ => return Value::Int(-1) };
            let cr = std::ffi::CString::new(remote_name).unwrap_or_default();
            let cs = std::ffi::CString::new(refspec).unwrap_or_default();
            Value::Int(unsafe { git_push_(ptr, cr.as_ptr(), cs.as_ptr()) } as i64)
        }

        pub fn pull(repo: &Value, remote_name: &str, merge_branch: &str) -> Value {
            let ptr = match repo { Value::Ptr(p) => *p, _ => return Value::Int(-1) };
            let cr = std::ffi::CString::new(remote_name).unwrap_or_default();
            let cb = std::ffi::CString::new(merge_branch).unwrap_or_default();
            Value::Int(unsafe { git_pull_(ptr, cr.as_ptr(), cb.as_ptr()) } as i64)
        }

        pub fn diff_stats(repo: &Value) -> Value {
            let ptr = match repo { Value::Ptr(p) => *p, _ => return Value::None };
            let s = unsafe { git_diff_stats_(ptr) };
            if s.is_null() { return Value::None }
            let result = Value::Str(unsafe { std::ffi::CStr::from_ptr(s) }.to_string_lossy().into_owned());
            unsafe { git_free_str(s) };
            result
        }

        pub fn log(repo: &Value, max_count: i64) -> Value {
            let ptr = match repo { Value::Ptr(p) => *p, _ => return Value::List(vec![]) };
            let mut count: i32 = 0;
            let arr = unsafe { git_log_(ptr, max_count as i32, &mut count) };
            if arr.is_null() { return Value::List(vec![]) }
            let mut commits = Vec::with_capacity(count as usize);
            for i in 0..count {
                let s = unsafe { *arr.offset(i as isize) };
                if !s.is_null() {
                    commits.push(Value::Str(unsafe { std::ffi::CStr::from_ptr(s) }.to_string_lossy().into_owned()));
                }
            }
            unsafe { git_free_str_array(arr, count) };
            Value::List(commits)
        }
    }

    pub mod font {
        use super::super::Value;
        use std::ffi::c_void;
        use std::os::raw::c_char;

        extern "C" {
            fn font_init() -> i32;
            fn font_load_face(path: *const c_char, index: i32) -> *mut c_void;
            fn font_done_face(face: *mut c_void);
            fn font_set_size(face: *mut c_void, size: i32, dpi: i32) -> i32;
            fn font_get_glyph(
                face: *mut c_void, charcode: u32,
                width: *mut i32, height: *mut i32,
                bearing_x: *mut i32, bearing_y: *mut i32, advance: *mut i32,
                bitmap: *mut *mut u8, bitmap_size: *mut i32,
            ) -> i32;
            fn font_get_kerning(face: *mut c_void, left: u32, right: u32, x: *mut i32, y: *mut i32) -> i32;
            fn font_get_name(face: *mut c_void) -> *const c_char;
            fn font_get_num_glyphs(face: *mut c_void) -> i32;
            fn ry_hb_create_font(face: *mut c_void) -> *mut c_void;
            fn ry_hb_destroy_font(font: *mut c_void);
            fn ry_hb_buffer_create() -> *mut c_void;
            fn ry_hb_buffer_destroy(buf: *mut c_void);
            fn ry_hb_buffer_add_utf8(buf: *mut c_void, text: *const c_char);
            fn ry_hb_buffer_set_script(buf: *mut c_void, script: i32);
            fn ry_hb_buffer_set_language(buf: *mut c_void, lang: *const c_char);
            fn ry_hb_buffer_set_direction(buf: *mut c_void, dir: i32);
            fn ry_hb_shape_text(font: *mut c_void, buf: *mut c_void);
            fn ry_hb_buffer_get_glyph_infos(buf: *mut c_void, out_count: *mut i32) -> *mut GlyphInfo;
            fn font_free_bitmap(bitmap: *mut u8);
            fn font_free_glyph_infos(infos: *mut GlyphInfo);
        }

        #[repr(C)]
        #[derive(Clone, Copy)]
        struct GlyphInfo {
            glyph_id: u32,
            cluster: u32,
            x_advance: i32,
            y_advance: i32,
            x_offset: i32,
            y_offset: i32,
        }

        pub fn ft_init() -> Value {
            Value::Int(unsafe { font_init() } as i64)
        }

        pub fn load_face(path: &str, index: i32) -> Value {
            let cpath = std::ffi::CString::new(path).unwrap_or_default();
            let ptr = unsafe { font_load_face(cpath.as_ptr(), index) };
            if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
        }

        pub fn done_face(face: &Value) -> Value {
            if let Value::Ptr(p) = face {
                unsafe { font_done_face(*p) }
            }
            Value::None
        }

        pub fn set_size(face: &Value, size: i32, dpi: i32) -> Value {
            if let Value::Ptr(p) = face {
                Value::Int(unsafe { font_set_size(*p, size, dpi) } as i64)
            } else {
                Value::Int(-1)
            }
        }

        pub fn get_glyph(face: &Value, charcode: u32) -> Value {
            let ptr = match face { Value::Ptr(p) => *p, _ => return Value::None };
            let mut width: i32 = 0;
            let mut height: i32 = 0;
            let mut bearing_x: i32 = 0;
            let mut bearing_y: i32 = 0;
            let mut advance: i32 = 0;
            let mut bitmap: *mut u8 = std::ptr::null_mut();
            let mut bitmap_size: i32 = 0;
            let ret = unsafe {
                font_get_glyph(ptr, charcode, &mut width, &mut height, &mut bearing_x, &mut bearing_y, &mut advance, &mut bitmap, &mut bitmap_size)
            };
            if ret != 0 {
                return Value::None;
            }
            let bytes = if !bitmap.is_null() && bitmap_size > 0 {
                let slice = unsafe { std::slice::from_raw_parts(bitmap, bitmap_size as usize) };
                let v = slice.to_vec();
                unsafe { font_free_bitmap(bitmap) };
                v
            } else {
                vec![]
            };
            Value::Tuple(vec![
                Value::Int(width as i64),
                Value::Int(height as i64),
                Value::Int(bearing_x as i64),
                Value::Int(bearing_y as i64),
                Value::Int(advance as i64),
                Value::Bytes(bytes),
            ])
        }

        pub fn get_kerning(face: &Value, left: u32, right: u32) -> Value {
            let ptr = match face { Value::Ptr(p) => *p, _ => return Value::Tuple(vec![Value::Int(0), Value::Int(0)]) };
            let mut x: i32 = 0;
            let mut y: i32 = 0;
            let ret = unsafe { font_get_kerning(ptr, left, right, &mut x, &mut y) };
            if ret != 0 {
                return Value::Tuple(vec![Value::Int(0), Value::Int(0)]);
            }
            Value::Tuple(vec![Value::Int(x as i64), Value::Int(y as i64)])
        }

        pub fn get_name(face: &Value) -> Value {
            if let Value::Ptr(p) = face {
                let s = unsafe { font_get_name(*p) };
                if s.is_null() {
                    Value::Str(String::new())
                } else {
                    Value::Str(unsafe { std::ffi::CStr::from_ptr(s) }.to_string_lossy().into_owned())
                }
            } else {
                Value::Str(String::new())
            }
        }

        pub fn get_num_glyphs(face: &Value) -> Value {
            if let Value::Ptr(p) = face {
                Value::Int(unsafe { font_get_num_glyphs(*p) } as i64)
            } else {
                Value::Int(0)
            }
        }

        pub fn hb_create_font_face(face: &Value) -> Value {
            let ptr = match face { Value::Ptr(p) => *p, _ => return Value::None };
            let hb = unsafe { ry_hb_create_font(ptr) };
            if hb.is_null() { Value::None } else { Value::Ptr(hb) }
        }

        pub fn hb_destroy_font_face(font: &Value) -> Value {
            if let Value::Ptr(p) = font {
                unsafe { ry_hb_destroy_font(*p) }
            }
            Value::None
        }

        pub fn hb_buffer_create_() -> Value {
            let buf = unsafe { ry_hb_buffer_create() };
            if buf.is_null() { Value::None } else { Value::Ptr(buf) }
        }

        pub fn hb_buffer_destroy_(buf: &Value) -> Value {
            if let Value::Ptr(p) = buf {
                unsafe { ry_hb_buffer_destroy(*p) }
            }
            Value::None
        }

        pub fn hb_buffer_add_utf8_(buf: &Value, text: &str) -> Value {
            if let Value::Ptr(p) = buf {
                let ctext = std::ffi::CString::new(text).unwrap_or_default();
                unsafe { ry_hb_buffer_add_utf8(*p, ctext.as_ptr()) }
            }
            Value::None
        }

        pub fn hb_buffer_set_script_(buf: &Value, script: i32) -> Value {
            if let Value::Ptr(p) = buf {
                unsafe { ry_hb_buffer_set_script(*p, script) }
            }
            Value::None
        }

        pub fn hb_buffer_set_language_(buf: &Value, lang: &str) -> Value {
            if let Value::Ptr(p) = buf {
                let clang = std::ffi::CString::new(lang).unwrap_or_default();
                unsafe { ry_hb_buffer_set_language(*p, clang.as_ptr()) }
            }
            Value::None
        }

        pub fn hb_buffer_set_direction_(buf: &Value, dir: i32) -> Value {
            if let Value::Ptr(p) = buf {
                unsafe { ry_hb_buffer_set_direction(*p, dir) }
            }
            Value::None
        }

        pub fn hb_shape_(font: &Value, buf: &Value) -> Value {
            let fp = match font { Value::Ptr(p) => *p, _ => return Value::None };
            let bp = match buf { Value::Ptr(p) => *p, _ => return Value::None };
            unsafe { ry_hb_shape_text(fp, bp) }
            Value::None
        }

        pub fn hb_buffer_get_glyph_infos_(buf: &Value) -> Value {
            let ptr = match buf { Value::Ptr(p) => *p, _ => return Value::None };
            let mut count: i32 = 0;
            let infos = unsafe { ry_hb_buffer_get_glyph_infos(ptr, &mut count) };
            if infos.is_null() || count == 0 {
                return Value::List(vec![]);
            }
            let mut result = Vec::with_capacity(count as usize);
            for i in 0..count {
                let info = unsafe { *infos.offset(i as isize) };
                result.push(Value::Tuple(vec![
                    Value::Int(info.glyph_id as i64),
                    Value::Int(info.cluster as i64),
                    Value::Int(info.x_advance as i64),
                    Value::Int(info.y_advance as i64),
                    Value::Int(info.x_offset as i64),
                    Value::Int(info.y_offset as i64),
                ]));
            }
            unsafe { font_free_glyph_infos(infos) };
            Value::List(result)
        }
    }

    pub mod numpy {
        use super::super::Value;
        use std::ffi::c_void;

        extern "C" {
            fn numpy_zeros(shape: *const i64, ndim: i32) -> *mut c_void;
            fn numpy_ones(shape: *const i64, ndim: i32) -> *mut c_void;
            fn numpy_eye(n: i32) -> *mut c_void;
            fn numpy_arange(start: f64, stop: f64, step: f64) -> *mut c_void;
            fn numpy_linspace(start: f64, stop: f64, num: i64) -> *mut c_void;
            fn numpy_full(shape: *const i64, ndim: i32, value: f64) -> *mut c_void;
            fn numpy_copy(arr: *mut c_void) -> *mut c_void;
            fn numpy_reshape(arr: *mut c_void, new_shape: *const i64, new_ndim: i32) -> *mut c_void;
            fn numpy_transpose(arr: *mut c_void) -> *mut c_void;
            fn numpy_concatenate(arrs: *mut *mut c_void, num_arrs: i32, axis: i32) -> *mut c_void;
            fn numpy_stack(arrs: *mut *mut c_void, num_arrs: i32, axis: i32) -> *mut c_void;
            fn numpy_slice(arr: *mut c_void, start: i64, stop: i64, step: i64, axis: i32) -> *mut c_void;
            fn numpy_add(a: *mut c_void, b: *mut c_void) -> *mut c_void;
            fn numpy_sub(a: *mut c_void, b: *mut c_void) -> *mut c_void;
            fn numpy_mul(a: *mut c_void, b: *mut c_void) -> *mut c_void;
            fn numpy_div(a: *mut c_void, b: *mut c_void) -> *mut c_void;
            fn numpy_dot(a: *mut c_void, b: *mut c_void) -> *mut c_void;
            fn numpy_matmul(a: *mut c_void, b: *mut c_void) -> *mut c_void;
            fn numpy_sum(arr: *mut c_void, axis: i32) -> *mut c_void;
            fn numpy_mean(arr: *mut c_void, axis: i32) -> *mut c_void;
            fn numpy_std(arr: *mut c_void, axis: i32) -> *mut c_void;
            fn numpy_min(arr: *mut c_void, axis: i32) -> *mut c_void;
            fn numpy_max(arr: *mut c_void, axis: i32) -> *mut c_void;
            fn numpy_argmin(arr: *mut c_void) -> i64;
            fn numpy_argmax(arr: *mut c_void) -> i64;
            fn numpy_exp(arr: *mut c_void) -> *mut c_void;
            fn numpy_log(arr: *mut c_void) -> *mut c_void;
            fn numpy_sqrt(arr: *mut c_void) -> *mut c_void;
            fn numpy_sin(arr: *mut c_void) -> *mut c_void;
            fn numpy_cos(arr: *mut c_void) -> *mut c_void;
            fn numpy_tan(arr: *mut c_void) -> *mut c_void;
            fn numpy_abs(arr: *mut c_void) -> *mut c_void;
            fn numpy_floor(arr: *mut c_void) -> *mut c_void;
            fn numpy_ceil(arr: *mut c_void) -> *mut c_void;
            fn numpy_clip(arr: *mut c_void, min: f64, max: f64) -> *mut c_void;
            fn numpy_where(cond: *mut c_void, x: *mut c_void, y: *mut c_void) -> *mut c_void;
            fn numpy_sort(arr: *mut c_void, axis: i32) -> *mut c_void;
            fn numpy_unique(arr: *mut c_void) -> *mut c_void;
            fn numpy_to_string(arr: *mut c_void) -> *mut i8;
            fn numpy_shape(arr: *mut c_void, out_dims: *mut i64);
            fn numpy_ndim(arr: *mut c_void) -> i32;
            fn numpy_size(arr: *mut c_void) -> i64;
            fn numpy_item(arr: *mut c_void, indices: *const i64, num_indices: i32) -> f64;
            fn numpy_set_item(arr: *mut c_void, indices: *const i64, num_indices: i32, value: f64);
            fn numpy_to_float_array(arr: *mut c_void, out_data: *mut f64);
            fn numpy_from_float_array(data: *const f64, shape: *const i64, ndim: i32) -> *mut c_void;
            fn numpy_free(arr: *mut c_void);
            fn numpy_free_string(s: *mut i8);
        }

        pub fn zeros(shape: &[i64]) -> Value {
            let ptr = unsafe { numpy_zeros(shape.as_ptr(), shape.len() as i32) };
            if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
        }

        pub fn ones(shape: &[i64]) -> Value {
            let ptr = unsafe { numpy_ones(shape.as_ptr(), shape.len() as i32) };
            if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
        }

        pub fn eye(n: i64) -> Value {
            let ptr = unsafe { numpy_eye(n as i32) };
            if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
        }

        pub fn arange(start: f64, stop: f64, step: f64) -> Value {
            let ptr = unsafe { numpy_arange(start, stop, step) };
            if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
        }

        pub fn linspace(start: f64, stop: f64, num: i64) -> Value {
            let ptr = unsafe { numpy_linspace(start, stop, num) };
            if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
        }

        pub fn full(shape: &[i64], value: f64) -> Value {
            let ptr = unsafe { numpy_full(shape.as_ptr(), shape.len() as i32, value) };
            if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
        }

        pub fn copy(arr: &Value) -> Value {
            if let Value::Ptr(p) = arr {
                let ptr = unsafe { numpy_copy(*p) };
                if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
            } else { Value::None }
        }

        pub fn reshape(arr: &Value, new_shape: &[i64]) -> Value {
            if let Value::Ptr(p) = arr {
                let ptr = unsafe { numpy_reshape(*p, new_shape.as_ptr(), new_shape.len() as i32) };
                if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
            } else { Value::None }
        }

        pub fn transpose(arr: &Value) -> Value {
            if let Value::Ptr(p) = arr {
                let ptr = unsafe { numpy_transpose(*p) };
                if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
            } else { Value::None }
        }

        pub fn concatenate(arrs: &[Value], axis: i64) -> Value {
            let mut ptrs: Vec<*mut c_void> = arrs.iter().filter_map(|v| match v { Value::Ptr(p) => Some(*p), _ => None }).collect();
            let ptr = unsafe { numpy_concatenate(ptrs.as_mut_ptr(), ptrs.len() as i32, axis as i32) };
            if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
        }

        pub fn stack(arrs: &[Value], axis: i64) -> Value {
            let mut ptrs: Vec<*mut c_void> = arrs.iter().filter_map(|v| match v { Value::Ptr(p) => Some(*p), _ => None }).collect();
            let ptr = unsafe { numpy_stack(ptrs.as_mut_ptr(), ptrs.len() as i32, axis as i32) };
            if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
        }

        pub fn slice(arr: &Value, start: i64, stop: i64, step: i64, axis: i64) -> Value {
            if let Value::Ptr(p) = arr {
                let ptr = unsafe { numpy_slice(*p, start, stop, step, axis as i32) };
                if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
            } else { Value::None }
        }

        pub fn add(a: &Value, b: &Value) -> Value {
            match (a, b) {
                (Value::Ptr(pa), Value::Ptr(pb)) => {
                    let ptr = unsafe { numpy_add(*pa, *pb) };
                    if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
                }
                _ => Value::None,
            }
        }

        pub fn sub(a: &Value, b: &Value) -> Value {
            match (a, b) {
                (Value::Ptr(pa), Value::Ptr(pb)) => {
                    let ptr = unsafe { numpy_sub(*pa, *pb) };
                    if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
                }
                _ => Value::None,
            }
        }

        pub fn mul(a: &Value, b: &Value) -> Value {
            match (a, b) {
                (Value::Ptr(pa), Value::Ptr(pb)) => {
                    let ptr = unsafe { numpy_mul(*pa, *pb) };
                    if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
                }
                _ => Value::None,
            }
        }

        pub fn div(a: &Value, b: &Value) -> Value {
            match (a, b) {
                (Value::Ptr(pa), Value::Ptr(pb)) => {
                    let ptr = unsafe { numpy_div(*pa, *pb) };
                    if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
                }
                _ => Value::None,
            }
        }

        pub fn dot(a: &Value, b: &Value) -> Value {
            match (a, b) {
                (Value::Ptr(pa), Value::Ptr(pb)) => {
                    let ptr = unsafe { numpy_dot(*pa, *pb) };
                    if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
                }
                _ => Value::None,
            }
        }

        pub fn matmul(a: &Value, b: &Value) -> Value {
            match (a, b) {
                (Value::Ptr(pa), Value::Ptr(pb)) => {
                    let ptr = unsafe { numpy_matmul(*pa, *pb) };
                    if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
                }
                _ => Value::None,
            }
        }

        pub fn sum(arr: &Value, axis: i64) -> Value {
            if let Value::Ptr(p) = arr {
                let ptr = unsafe { numpy_sum(*p, axis as i32) };
                if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
            } else { Value::None }
        }

        pub fn mean(arr: &Value, axis: i64) -> Value {
            if let Value::Ptr(p) = arr {
                let ptr = unsafe { numpy_mean(*p, axis as i32) };
                if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
            } else { Value::None }
        }

        pub fn std(arr: &Value, axis: i64) -> Value {
            if let Value::Ptr(p) = arr {
                let ptr = unsafe { numpy_std(*p, axis as i32) };
                if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
            } else { Value::None }
        }

        pub fn min(arr: &Value, axis: i64) -> Value {
            if let Value::Ptr(p) = arr {
                let ptr = unsafe { numpy_min(*p, axis as i32) };
                if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
            } else { Value::None }
        }

        pub fn max(arr: &Value, axis: i64) -> Value {
            if let Value::Ptr(p) = arr {
                let ptr = unsafe { numpy_max(*p, axis as i32) };
                if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
            } else { Value::None }
        }

        pub fn argmin(arr: &Value) -> Value {
            if let Value::Ptr(p) = arr {
                Value::Int(unsafe { numpy_argmin(*p) })
            } else { Value::Int(-1) }
        }

        pub fn argmax(arr: &Value) -> Value {
            if let Value::Ptr(p) = arr {
                Value::Int(unsafe { numpy_argmax(*p) })
            } else { Value::Int(-1) }
        }

        pub fn exp(arr: &Value) -> Value {
            if let Value::Ptr(p) = arr {
                let ptr = unsafe { numpy_exp(*p) };
                if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
            } else { Value::None }
        }

        pub fn log(arr: &Value) -> Value {
            if let Value::Ptr(p) = arr {
                let ptr = unsafe { numpy_log(*p) };
                if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
            } else { Value::None }
        }

        pub fn sqrt(arr: &Value) -> Value {
            if let Value::Ptr(p) = arr {
                let ptr = unsafe { numpy_sqrt(*p) };
                if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
            } else { Value::None }
        }

        pub fn sin(arr: &Value) -> Value {
            if let Value::Ptr(p) = arr {
                let ptr = unsafe { numpy_sin(*p) };
                if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
            } else { Value::None }
        }

        pub fn cos(arr: &Value) -> Value {
            if let Value::Ptr(p) = arr {
                let ptr = unsafe { numpy_cos(*p) };
                if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
            } else { Value::None }
        }

        pub fn tan(arr: &Value) -> Value {
            if let Value::Ptr(p) = arr {
                let ptr = unsafe { numpy_tan(*p) };
                if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
            } else { Value::None }
        }

        pub fn abs(arr: &Value) -> Value {
            if let Value::Ptr(p) = arr {
                let ptr = unsafe { numpy_abs(*p) };
                if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
            } else { Value::None }
        }

        pub fn floor(arr: &Value) -> Value {
            if let Value::Ptr(p) = arr {
                let ptr = unsafe { numpy_floor(*p) };
                if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
            } else { Value::None }
        }

        pub fn ceil(arr: &Value) -> Value {
            if let Value::Ptr(p) = arr {
                let ptr = unsafe { numpy_ceil(*p) };
                if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
            } else { Value::None }
        }

        pub fn clip(arr: &Value, min: f64, max: f64) -> Value {
            if let Value::Ptr(p) = arr {
                let ptr = unsafe { numpy_clip(*p, min, max) };
                if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
            } else { Value::None }
        }

        pub fn where_(cond: &Value, x: &Value, y: &Value) -> Value {
            match (cond, x, y) {
                (Value::Ptr(pc), Value::Ptr(px), Value::Ptr(py)) => {
                    let ptr = unsafe { numpy_where(*pc, *px, *py) };
                    if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
                }
                _ => Value::None,
            }
        }

        pub fn sort(arr: &Value, axis: i64) -> Value {
            if let Value::Ptr(p) = arr {
                let ptr = unsafe { numpy_sort(*p, axis as i32) };
                if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
            } else { Value::None }
        }

        pub fn unique(arr: &Value) -> Value {
            if let Value::Ptr(p) = arr {
                let ptr = unsafe { numpy_unique(*p) };
                if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
            } else { Value::None }
        }

        pub fn to_string(arr: &Value) -> Value {
            if let Value::Ptr(p) = arr {
                let s = unsafe { numpy_to_string(*p) };
                if s.is_null() { return Value::Str(String::new()) }
                let result = Value::Str(unsafe { std::ffi::CStr::from_ptr(s as *const i8) }.to_string_lossy().into_owned());
                unsafe { numpy_free_string(s) };
                result
            } else { Value::Str(String::new()) }
        }

        pub fn shape(arr: &Value) -> Value {
            if let Value::Ptr(p) = arr {
                let ndim = unsafe { numpy_ndim(*p) };
                let mut dims: Vec<i64> = vec![0i64; ndim as usize];
                unsafe { numpy_shape(*p, dims.as_mut_ptr()) };
                Value::Tuple(dims.into_iter().map(|d| Value::Int(d)).collect())
            } else { Value::Tuple(vec![]) }
        }

        pub fn ndim(arr: &Value) -> Value {
            if let Value::Ptr(p) = arr {
                Value::Int(unsafe { numpy_ndim(*p) } as i64)
            } else { Value::Int(0) }
        }

        pub fn size(arr: &Value) -> Value {
            if let Value::Ptr(p) = arr {
                Value::Int(unsafe { numpy_size(*p) })
            } else { Value::Int(0) }
        }

        pub fn item(arr: &Value, indices: &[i64]) -> Value {
            if let Value::Ptr(p) = arr {
                Value::Float(unsafe { numpy_item(*p, indices.as_ptr(), indices.len() as i32) })
            } else { Value::None }
        }

        pub fn set_item(arr: &Value, indices: &[i64], value: f64) {
            if let Value::Ptr(p) = arr {
                unsafe { numpy_set_item(*p, indices.as_ptr(), indices.len() as i32, value) }
            }
        }

        pub fn to_float_array(arr: &Value) -> Value {
            if let Value::Ptr(p) = arr {
                let n = unsafe { numpy_size(*p) } as usize;
                let mut data: Vec<f64> = vec![0.0f64; n];
                unsafe { numpy_to_float_array(*p, data.as_mut_ptr()) };
                Value::List(data.into_iter().map(|x| Value::Float(x)).collect())
            } else { Value::List(vec![]) }
        }

        pub fn from_float_array(data: &[f64], shape: &[i64]) -> Value {
            let ptr = unsafe { numpy_from_float_array(data.as_ptr(), shape.as_ptr(), shape.len() as i32) };
            if ptr.is_null() { Value::None } else { Value::Ptr(ptr) }
        }

        pub fn free(arr: &Value) {
            if let Value::Ptr(p) = arr { unsafe { numpy_free(*p) } }
        }
    }
}
