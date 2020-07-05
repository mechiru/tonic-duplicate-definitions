# tonic-duplicate-definitions

## error message
```
$ cargo build
error[E0592]: duplicate definitions with name `connect`
  --> src/connect.rs:14:9
   |
14 | /         pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
15 | |         where
16 | |             D: std::convert::TryInto<tonic::transport::Endpoint>,
17 | |             D::Error: Into<StdError>,
...  |
20 | |             Ok(Self::new(conn))
21 | |         }
   | |_________^ duplicate definitions for `connect`
...
38 | /         pub async fn connect(
39 | |             &mut self,
40 | |             request: impl tonic::IntoRequest<super::ConnectRequest>,
41 | |         ) -> Result<tonic::Response<super::ConnectResponse>, tonic::Status> {
...  |
50 | |             self.inner.unary(request.into_request(), path, codec).await
51 | |         }
   | |_________- other definition for `connect`
   |
   = note: upstream crates may add a new impl of trait `tower_service::Service<http::request::Request<tonic::body::BoxBody>>` for type `tonic::transport::channel::Channel` in future versions

error: aborting due to previous error

For more information about this error, try `rustc --explain E0592`.
error: could not compile `tonic-duplicate-definitions`.

To learn more, run the command again with --verbose.
```
