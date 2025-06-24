// Learn Dart in Y Minutes
// Dart is a language that can run in any platform such as Web, CLI, Desktop, Mobile and IoT devices.

import "dart:collection";
import "dart:math" as math;

// Learn about Dart by running
// dart dart.dart

// Constants
const CONSTANT_VALUE = "I CANNOT CHANGE";
// final value cannot be changed once instantiated.
final finalValue = "value cannot be changed once instantiated";
var mutableValue = "Variable string";
dynamic dynamicValue = "I'm a string";

// Comments: Single line comments start with //

/*
 * Multi-line comments like this
 */

/// Code doc comments
/// Use triple-slash for code documentation

// Example 1: Functions and scopes
example1() {
  nested1() {
    nested2() => print("Example1 nested 1 nested 2");
    nested2();
  }
  nested1();
}

// Example 2: Higher-order functions
example2() {
  nested1(void Function() fn) {
    fn();
  }
  nested1(() => print("Example2 nested 1"));
}

// Example 3: Optional parameters
example3({String? optStr, int? optInt}) {
  print("Example3 optStr:'$optStr' optInt:$optInt");
}

// Example 4: Optional positional parameters
example4([String? optStr, int? optInt]) {
  print("Example4 optStr:'$optStr' optInt:$optInt");
}

// Example 5: Classes
class Example5Class {
  var _private = "Example5 private value";
  
  sayIt() {
    print("Example5 sayIt");
  }
  
  get getter => _private;
  set setter(val) => _private = val;
}

// Example 6: Class inheritance
class Example6Class extends Example5Class {
  @override
  sayIt() {
    super.sayIt();
    print("Example6 sayIt");
  }
}

// Example 7: Abstract classes and interfaces
abstract class Example7Class {
  var _private = "Example7 private value";
  sayIt() => print("Example7 sayIt");
  doElse();
}

class Example7ActualClass extends Example7Class {
  doElse() => print("Example7 doElse");
}

// Example 8: Collections - Lists and Maps
example8() {
  var example8List = const ["Example8 const array"];
  var example8Map = const {"someKey": "Example8 const map"};
  print("Example8 list:'${example8List}' map:'${example8Map}'");
  
  // Explicit type declarations
  List<String> explicitList = <String>[];
  Map<String, dynamic> explicitMaps = <String, dynamic>{};
  explicitList.add("SomeString");
  explicitMaps["someKey"] = "someValue";
  print("Example8 explicitList:'${explicitList}' explicitMaps:'${explicitMaps}'");
}

// Example 9: Loops
example9() {
  var example9Array = const ["a", "b"];
  for (int i = 0; i < example9Array.length; i++) {
    print("Example9 for loop '${example9Array[i]}'");
  }
  
  for (var item in example9Array) {
    print("Example9 for-in loop '$item'");
  }
  
  example9Array.forEach((e) => print("Example9 forEach loop '$e'"));
  
  var example9Map = const {"k1": "v1", "k2": "v2"};
  for (var k in example9Map.keys) {
    print("Example9 Map loop key:'$k' value:'${example9Map[k]}'");
  }
}

// Example 10: Switch statement
example10() {
  var value = 2;
  switch (value) {
    case 1:
      print("Example10 value is 1");
      break;
    case 2:
      print("Example10 value is 2");
      break;
    default:
      print("Example10 value is something else");
  }
}

// Example 11: Exceptions
example11() {
  try {
    throw StateError("Example11 StateError");
  } catch (e) {
    print("Example11 caught error: $e");
  } finally {
    print("Example11 in finally block");
  }
}

// Example 12: Null safety
example12() {
  String? nullable = null;
  String nonNullable = "not null";
  
  // Null-aware operators
  print("Example12 nullable?.length: ${nullable?.length}");
  print("Example12 nullable ?? 'default': ${nullable ?? 'default'}");
  
  // Null assertion
  nullable = "now has value";
  print("Example12 nullable!.length: ${nullable!.length}");
}

// Example 13: Futures and async/await
Future<String> example13Future() async {
  await Future.delayed(Duration(seconds: 1));
  return "Example13 future result";
}

example13() async {
  print("Example13 before await");
  var result = await example13Future();
  print("Example13 after await: $result");
}

// Example 14: Streams
example14() async {
  Stream<int> countStream(int to) async* {
    for (int i = 1; i <= to; i++) {
      yield i;
    }
  }
  
  await for (var value in countStream(5)) {
    print("Example14 stream value: $value");
  }
}

// Example 15: Generics
class Example15<T> {
  void printType() {
    print("Example15 Type: $T");
  }
  
  genericMethod<M>() {
    print("Example15 class: $T, method: $M");
  }
}

// Example 16: Mixins
mixin Example16Mixin {
  void mixinMethod() {
    print("Example16 mixin method");
  }
}

class Example16Class with Example16Mixin {
  void classMethod() {
    print("Example16 class method");
  }
}

// Example 17: Extensions
extension Example17Extension on String {
  bool get isEmail => this.contains("@");
  String reverse() => this.split('').reversed.join('');
}

// Example 18: Factory constructors
class Example18 {
  final String name;
  static final Map<String, Example18> _cache = {};
  
  Example18._internal(this.name);
  
  factory Example18(String name) {
    return _cache.putIfAbsent(name, () => Example18._internal(name));
  }
}

// Example 19: Named constructors
class Example19 {
  final int x, y;
  
  Example19(this.x, this.y);
  Example19.origin() : x = 0, y = 0;
  Example19.fromJson(Map<String, dynamic> json) 
      : x = json['x'], y = json['y'];
}

// Example 20: Enums
enum Example20Color { red, green, blue }

example20() {
  var color = Example20Color.red;
  switch (color) {
    case Example20Color.red:
      print("Example20 color is red");
      break;
    case Example20Color.green:
      print("Example20 color is green");
      break;
    case Example20Color.blue:
      print("Example20 color is blue");
      break;
  }
}

// Example 21: Type aliases
typedef Example21Compare<T> = int Function(T a, T b);

example21() {
  Example21Compare<String> stringCompare = (a, b) => a.compareTo(b);
  print("Example21 compare result: ${stringCompare('apple', 'banana')}");
}

// Example 22: Cascade notation
class Example22Person {
  String? name;
  int? age;
  
  void greet() => print("Hello, I'm $name, $age years old");
}

example22() {
  var person = Example22Person()
    ..name = "John"
    ..age = 30
    ..greet();
}

// Example 23: Records (Dart 3.0+)
example23() {
  var record = (42, "hello", true);
  print("Example23 record: $record");
  print("Example23 first: ${record.$1}");
  
  var namedRecord = (name: "John", age: 30);
  print("Example23 named record: ${namedRecord.name}, ${namedRecord.age}");
}

// Example 24: Pattern matching (Dart 3.0+)
example24() {
  var data = {"type": "user", "name": "John", "age": 30};
  
  switch (data) {
    case {"type": "user", "name": String name}:
      print("Example24 user: $name");
    case {"type": "admin"}:
      print("Example24 admin user");
    default:
      print("Example24 unknown type");
  }
}

// Example 25: Collection if and for
example25() {
  var includeZero = true;
  var numbers = [
    if (includeZero) 0,
    for (var i = 1; i <= 3; i++) i,
  ];
  print("Example25 numbers: $numbers");
}

// Example 26: Spread operator
example26() {
  var list1 = [1, 2, 3];
  var list2 = [4, 5, 6];
  var combined = [...list1, ...list2];
  print("Example26 combined: $combined");
  
  var map1 = {"a": 1, "b": 2};
  var map2 = {"c": 3, "d": 4};
  var combinedMap = {...map1, ...map2};
  print("Example26 combined map: $combinedMap");
}

// Example 27: Late variables
late String example27LateVariable;

example27() {
  example27LateVariable = "Initialized when first used";
  print("Example27 late variable: $example27LateVariable");
}

// Example 28: Required parameters
class Example28 {
  final String name;
  final int age;
  
  Example28({required this.name, required this.age});
}

// Example 29: Static methods and variables
class Example29 {
  static int count = 0;
  
  static void increment() {
    count++;
  }
  
  static void printCount() {
    print("Example29 count: $count");
  }
}

// Example 30: Function types
typedef Example30Handler = void Function(String message);

example30() {
  Example30Handler handler = (message) => print("Example30 handler: $message");
  handler("Hello World");
  
  // Function as parameter
  void processMessage(String msg, Example30Handler callback) {
    var processed = msg.toUpperCase();
    callback(processed);
  }
  
  processMessage("hello", handler);
}

// Main function
void main() {
  print("Learn Dart in Y Minutes!");
  
  // Variable examples
  print("CONSTANT_VALUE: $CONSTANT_VALUE");
  print("finalValue: $finalValue");
  print("mutableValue: $mutableValue");
  print("dynamicValue: $dynamicValue");
  
  // Function examples
  example1();
  example2();
  example3(optStr: "Hello", optInt: 42);
  example4("World", 123);
  
  // Class examples
  var example5 = Example5Class();
  example5.sayIt();
  print("Example5 getter: ${example5.getter}");
  
  var example6 = Example6Class();
  example6.sayIt();
  
  var example7 = Example7ActualClass();
  example7.sayIt();
  example7.doElse();
  
  // Collection examples
  example8();
  example9();
  
  // Control flow examples
  example10();
  example11();
  example12();
  
  // Async examples
  example13();
  example14();
  
  // Generic examples
  var example15 = Example15<String>();
  example15.printType();
  example15.genericMethod<int>();
  
  // Mixin example
  var example16 = Example16Class();
  example16.classMethod();
  example16.mixinMethod();
  
  // Extension example
  print("Example17 'test@example.com'.isEmail: ${'test@example.com'.isEmail}");
  print("Example17 'hello'.reverse(): ${'hello'.reverse()}");
  
  // Factory constructor example
  var example18a = Example18("test");
  var example18b = Example18("test");
  print("Example18 same instance: ${identical(example18a, example18b)}");
  
  // Named constructor example
  var example19a = Example19(10, 20);
  var example19b = Example19.origin();
  var example19c = Example19.fromJson({"x": 5, "y": 15});
  print("Example19 a: (${example19a.x}, ${example19a.y})");
  print("Example19 b: (${example19b.x}, ${example19b.y})");
  print("Example19 c: (${example19c.x}, ${example19c.y})");
  
  // Enum example
  example20();
  
  // Type alias example
  example21();
  
  // Cascade notation example
  example22();
  
  // Records example (Dart 3.0+)
  example23();
  
  // Pattern matching example (Dart 3.0+)
  example24();
  
  // Collection if/for example
  example25();
  
  // Spread operator example
  example26();
  
  // Late variable example
  example27();
  
  // Required parameters example
  var example28 = Example28(name: "John", age: 30);
  print("Example28 person: ${example28.name}, ${example28.age}");
  
  // Static methods example
  Example29.increment();
  Example29.increment();
  Example29.printCount();
  
  // Function types example
  example30();
  
  print("Dart tutorial completed!");
}