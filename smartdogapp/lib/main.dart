import 'package:firebase_messaging/firebase_messaging.dart';
import 'package:flutter/material.dart';
import 'package:overlay_support/overlay_support.dart';
import 'package:smartdogapp/notificationbadge.dart';
import 'package:smartdogapp/pushnotification.dart';
//import 'package:firebase_analytics/firebase_analytics.dart';

void main() {
  runApp(MyApp());
}

class MyApp extends StatelessWidget {



  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return OverlaySupport.global(
      child: MaterialApp(
        title: 'Flutter Demo',
        theme: ThemeData(

          primarySwatch: Colors.blue,
        ),
        home: MyHomePage(title: 'Flutter Demo Home Page'),
      ),
    );
  }
}

class MyHomePage extends StatefulWidget {
  MyHomePage({Key? key, required this.title}) : super(key: key);

  // This widget is the home page of your application. It is stateful, meaning
  // that it has a State object (defined below) that contains fields that affect
  // how it looks.

  // This class is the configuration for the state. It holds the values (in this
  // case the title) provided by the parent (in this case the App widget) and
  // used by the build method of the State. Fields in a Widget subclass are
  // always marked "final".

  final String title;

  @override
  _MyHomePageState createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  late final FirebaseMessaging _messaging;
  PushNotification? _notificationInfo;



  int _counter = 0;

  void onClick() {
    // popup a toast.
    toast('Hello world!');
    _incrementCounter();
    // show a notification at top of screen.
    showSimpleNotification(
        Text("this is a message from simple notification"),
        background: Colors.green);
  }

  void registerNotification() async {
    _messaging = FirebaseMessaging.instance;
    NotificationSettings settings = await _messaging.requestPermission(
      alert: true,
      badge: true,
      provisional: true,
      sound: true,
    );

    if(settings.authorizationStatus == AuthorizationStatus.authorized) {
      // handle notifications
      FirebaseMessaging.onMessage.listen((RemoteMessage msg) {
        print("Fetched a message");
        PushNotification notification = PushNotification(
          title: msg.notification?.title,
          body: msg.notification?.body,
        );
        setState(() {
          _notificationInfo = notification;
          _counter++;
        });
      });

      if (_notificationInfo != null) {
        showSimpleNotification(
          Text(_notificationInfo!.title!),
          leading: NotificationBage(totalNotifications: _counter),
          subtitle: Text(_notificationInfo!.body!),
          background: Colors.green,
          duration: Duration(seconds: 2),
        );
      }

    } else {
      print("Enable notifications");
    }
  }

  void _incrementCounter() {
    setState(() {

      _counter++;
    });
  }

  @override
  Widget build(BuildContext context) {

    return Scaffold(
      appBar: AppBar(

        title: Text(widget.title),
      ),
      body: Center(

        child: Column(

          mainAxisAlignment: MainAxisAlignment.center,
          children: <Widget>[
            Text(
              'You have pushed the button this many times:',
            ),
            Text(
              '$_counter',
              style: Theme.of(context).textTheme.headline4,
            ),
          ],
        ),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: onClick,
        tooltip: 'Increment',
        child: Icon(Icons.add),
      ), // This trailing comma makes auto-formatting nicer for build methods.
    );
  }
}
