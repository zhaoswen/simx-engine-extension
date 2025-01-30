use crate::exec::interface::handler_func;
use crate::service::interface::handler_service;
use engine_share::entity::exception::node::NodeError;
use engine_share::entity::flow::flow::FlowData;
use engine_share::entity::flow::node::Node;
use engine_share::entity::services::Service;

mod exec;
mod service;
pub mod entity;
pub mod common;
mod agent;

#[no_mangle]
#[allow(improper_ctypes_definitions)]
// 测试方法，引擎启用扩展后，会自动调用此方法测试
pub extern "C" fn test() -> bool { true }

// 服务调用入口
#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn serve(service: Service) -> Result<(), NodeError> {
    let future = async {
        handler_service(service).await;
    };
    tokio::runtime::Runtime::new().unwrap().block_on(future);
    Ok(())
}

// 函数调用入口（处理器）
#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn exec(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    let mut res: Result<(), NodeError> = Ok(());
    let future = async {
        res = handler_func(node, flow_data).await
    };
    tokio::runtime::Runtime::new().unwrap().block_on(future);
    res
}

// 初始化调用入口
#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn init() -> bool {
    println!("init");
    true
}

// 销毁调用入口
#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn destroy() -> bool {
    true
}
