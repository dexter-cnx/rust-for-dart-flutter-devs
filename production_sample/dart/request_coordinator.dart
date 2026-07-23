import 'native_engine.dart';

/// Adds latest-request-wins on top of cooperative native cancellation.
final class RequestCoordinator {
  RequestCoordinator(this._engine);

  final NativeEngine _engine;
  int _generation = 0;
  String? _activeRequestId;

  Future<EngineResult?> process(List<int> bytes) async {
    final generation = ++_generation;
    final previous = _activeRequestId;
    if (previous != null) {
      await _engine.cancel(previous);
    }

    final requestId = 'request-$generation';
    _activeRequestId = requestId;
    final result = await _engine.process(
      ProcessRequest(requestId: requestId, bytes: bytes),
    );

    if (generation != _generation) return null;
    return result;
  }

  Future<void> dispose() async {
    _generation++;
    final active = _activeRequestId;
    _activeRequestId = null;
    if (active != null) await _engine.cancel(active);
  }
}
