abstract interface class NativeEngine {
  Future<EngineResult> process(ProcessRequest request);
  Future<void> cancel(String requestId);
}

final class ProcessRequest {
  const ProcessRequest({required this.requestId, required this.bytes});
  final String requestId;
  final List<int> bytes;
}

sealed class EngineResult {
  const EngineResult();
}

final class EngineSuccess extends EngineResult {
  const EngineSuccess(this.bytes);
  final List<int> bytes;
}

final class EngineCancelled extends EngineResult {
  const EngineCancelled();
}

final class EngineFailure extends EngineResult {
  const EngineFailure(this.code, {this.diagnostic});
  final String code;
  final String? diagnostic;
}
