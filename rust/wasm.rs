mod ultimate_tic_tac_toe;

#[cfg(target_arch = "wasm32")]
pub mod wasm {
    use std::time::Duration;
    use wasm_bindgen::prelude::*;

    #[cfg(feature = "wee_alloc")]
    #[global_allocator]
    static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

    pub fn set_panic_hook() {
        #[cfg(feature = "console_error_panic_hook")]
        console_error_panic_hook::set_once();
    }

    #[wasm_bindgen]
    extern {
        #[wasm_bindgen(js_namespace = console)]
        pub fn log(s: &str);

        #[no_mangle]
        static performance: web_sys::Performance;
    }

    pub struct Timer(f64);

    impl Timer {
        pub fn new(time_limit: &Duration) -> Self {
            unsafe { Timer(performance.now() + time_limit.as_secs_f64() * 1000.) }
        }

        pub fn elapsed(&self) -> bool {
            unsafe { performance.now() > self.0 }
        }
    }

    #[macro_export]
    macro_rules! log {
        ($($arg:tt)*) => ($crate::wasm::log(&format_args!($($arg)*).to_string()))
    }
}
