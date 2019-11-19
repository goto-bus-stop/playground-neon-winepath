use neon::prelude::*;
use winepath::{WinePathError, WineConfig};

trait OrPanic<R> {
    fn or_panic(self) -> R;
}

impl<R, E> OrPanic<R> for Result<R, E> where E: std::error::Error {
    fn or_panic(self) -> R {
        match self {
            Ok(r) => r,
            Err(e) => panic!("{}", e),
        }
    }
}

declare_types! {
    pub class JsWineConfig for WineConfig {
        init(mut cx) {
            if cx.len() > 0 {
                let path = cx.argument::<JsString>(0)?;
                return Ok(WineConfig::from_prefix(path.value()));
            }

            Ok(WineConfig::from_env().or_panic())
        }

        method prefix(mut cx) {
            let this = cx.this();
            let prefix = {
                let guard = cx.lock();
                let borrow = this.borrow(&guard);
                let native_path = borrow.prefix();
                native_path.to_string_lossy().to_string()
            };
            Ok(cx.string(prefix).upcast())
        }

        method toWinePath(mut cx) {
            let this = cx.this();
            let input = cx.argument::<JsString>(0)?.value();
            let input = std::fs::canonicalize(input).or_panic();
            let wine_path = {
                let guard = cx.lock();
                let wine_path = this.borrow(&guard).to_wine_path(input).or_panic();
                wine_path
            };
            Ok(cx.string(wine_path).upcast())
        }

        method toNativePath(mut cx) {
            let this = cx.this();
            let input = cx.argument::<JsString>(0)?.value();
            let native_path = {
                let guard = cx.lock();
                let native_path = this.borrow(&guard).to_native_path(input).or_panic();
                native_path.to_string_lossy().to_string()
            };
            Ok(cx.string(native_path).upcast())
        }
    }
}

register_module!(mut cx, {
    cx.export_class::<JsWineConfig>("WineConfig")?;
    Ok(())
});
