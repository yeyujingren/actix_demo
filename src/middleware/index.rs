/**
 * actix web 的中间件系统允许我们为 请求和响应程序提供额外的行为
 * 中间件可以捕获进入的请求程序，能够让我们可以修改甚至与提前返回一个对应的请求体
 * 
 * 中间件常常被用在以下几种场景中
 * 预处理请求
 * 后处理响应
 * 修改应用状态
 * 接入额外的服务（redis、日至系统、sessions）
*/

// 创建一个简单的中间件
use std::{future::{ready, Ready}};

use actix_web::{
  dev::{forward_ready,Server, ServiceRequest, ServiceResponse, Transform, Service},
  Error, Result
};

use futures_util::future::LocalBoxFuture;

// 中间件程序包括两个步骤
// 1、初始化，中间件构造器被调用， 下一个中间件作为参数传入中间件组成的链中
// 2、初始化完成后，中间件在正常请求中被调用

pub struct SayHi;

// 中间件的构造器实现了 Transform 特征
// S： 下一个服务对应的类型
// B： 响应体对应的类型
impl<S, B> Transform<S, ServiceRequest> for SayHi
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = SayHiMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(SayHiMiddleware { service }))
    }
}

pub struct SayHiMiddleware<S> {
  service: S,
}


impl<S, B> Service<ServiceRequest> for SayHiMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        println!("Hi from start. You requested: {}", req.path());

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;

            println!("Hi from response");
            Ok(res)
        })
    }
}
