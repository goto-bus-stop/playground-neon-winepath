use neon::prelude::*;
use winepath::{WinePathError, WineConfig};

declare_types! {
    pub class JsWineConfig for WineConfig {
        init(mut cx) {
            if cx.len() > 0 {
                let path = cx.argument::<JsString>(0)?;
                return Ok(WineConfig::from_prefix(path.value()));
            }

            match WineConfig::from_env() {
                Err(err) => panic!("{}", err),
                Ok(config) => Ok(config),
            }
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
            let wine_path = {
                let guard = cx.lock();
                let wine_path = this.borrow(&guard).to_wine_path(input).unwrap();
                wine_path
            };
            Ok(cx.string(wine_path).upcast())
        }

        method toNativePath(mut cx) {
            let this = cx.this();
            let input = cx.argument::<JsString>(0)?.value();
            let native_path = {
                let guard = cx.lock();
                let native_path = this.borrow(&guard).to_native_path(input).unwrap();
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
