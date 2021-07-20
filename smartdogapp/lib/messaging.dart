import 'dart:async';

import 'package:firebase/firebase.dart' as firebase;



class Messaging {
  Messaging._();
  static Messaging _instance = Messaging._();
  static Messaging get instance => _instance;
  firebase.Messaging? _mc;

  String? _token;

  final _streamController = StreamController<Map<String, dynamic>>.broadcast();
  Stream<Map<String, dynamic>> get stream => _streamController.stream;

  void close() {
    _streamController.close();
  }

  Future<void> init() async {
    _mc = firebase.messaging();
    _mc!.usePublicVapidKey('key');
    _mc!.onMessage.listen((event) {_streamController.add(event.data);});
  }

  Future requestPermission() {
    return _mc!.requestPermission();
  }

  Future<String> getToken([bool force = false]) async {
    if(force || _token == null) {
      await requestPermission();
      _token = await _mc!.getToken();
    }
    return _token!;
  }
}