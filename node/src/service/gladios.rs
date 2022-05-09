pub struct ExecutorDispatch;
pub use gladios_runtime::RuntimeApi;

impl sc_executor::NativeExecutionDispatch for ExecutorDispatch {
	/// Only enable the benchmarking host functions when we actually want to benchmark.
	#[cfg(feature = "runtime-benchmarks")]
	type ExtendHostFunctions = frame_benchmarking::benchmarking::HostFunctions;
	/// Otherwise we only use the default Substrate host functions.
	#[cfg(not(feature = "runtime-benchmarks"))]
	type ExtendHostFunctions = ();

	fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
		gladios_runtime::api::dispatch(method, data)
	}

	fn native_version() -> sc_executor::NativeVersion {
		gladios_runtime::native_version()
	}
}