import 'native_engine.dart';

/// Production adapter boundary.
///
/// Replace the TODO calls with your generated flutter_rust_bridge API.
/// Keep generated bridge imports in this file (or a sibling facade), not in UI.
final class RustNativeEngine implements NativeEngine {
  @override
  Future<EngineResult> process(ProcessRequest request) async {
    try {
      // final output = await rustApi.process(
      //   requestId: request.requestId,
      //   bytes: Uint8List.fromList(request.bytes),
      // );
      throw UnimplementedError('Connect generated bridge API here');
    } catch (error, stackTrace) {
      return EngineFailure(
        'internal',
        diagnostic: '$error
$stackTrace',
      );
    }
  }

  @override
  Future<void> cancel(String requestId) async {
    // await rustApi.cancel(requestId: requestId);
  }
}
