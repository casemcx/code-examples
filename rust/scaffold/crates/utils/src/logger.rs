//! 日志工具模块
//!
//! 提供统一的日志接口，基于 tracing 实现
//!
//! # 示例
//!
//! ```no_run
//! use utils::log;
//!
//! log::info!("应用启动");
//! log::warn!("配置文件不存在");
//! log::error!("连接失败: {}", err);
//! ```

/// 重新导出 tracing 的日志宏
pub use tracing::{info, debug, warn, error, trace};

/// 日志工具类
///
/// 提供链式调用的日志接口
pub struct Log;

impl Log {
    /// 记录 INFO 级别日志
    #[inline]
    pub fn info(msg: &str) {
        info!("{}", msg);
    }

    /// 记录 DEBUG 级别日志
    #[inline]
    pub fn debug(msg: &str) {
        debug!("{}", msg);
    }

    /// 记录 WARN 级别日志
    #[inline]
    pub fn warn(msg: &str) {
        warn!("{}", msg);
    }

    /// 记录 ERROR 级别日志
    #[inline]
    pub fn error(msg: &str) {
        error!("{}", msg);
    }

    /// 记录 TRACE 级别日志
    #[inline]
    pub fn trace(msg: &str) {
        trace!("{}", msg);
    }
}

