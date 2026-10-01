use librustdesk::*;

#[cfg(not(target_os = "macos"))]
fn main() {}

#[cfg(target_os = "macos")]
fn main() {
    // 与 Windows/Linux 的服务进程保持一致：服务路径也必须完成 tiny 初始化，
    // 否则本进程 APP_NAME 停留在默认值 "RustDesk"，ipc_service socket
    // (/tmp/RustDesk-service) 与配置目录 (com.carriez.RustDesk) 会和原生
    // RustDesk 冲突，且 `--write-plists` 会生成错误名字的 launchd plist，
    // 用户会话的 --server 也永远连不上 daemon 的 ipc_service。
    // 必须在任何读 APP_NAME 的调用（write_plists/load_custom_client/
    // init_log/start_os_service）之前执行。
    #[cfg(feature = "rustdesk-tiny")]
    crate::tiny::initialize();
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "--write-plists" {
        if let Err(e) = librustdesk::platform::write_plists() {
            eprintln!("Failed to write plists: {}", e);
            std::process::exit(1);
        }
        std::process::exit(0);
    }
    crate::common::load_custom_client();
    hbb_common::init_log(false, "service");
    crate::start_os_service();
}
