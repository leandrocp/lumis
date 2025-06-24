;; Comments
; Comments start with semicolons
; Clojure is written in "forms", which are lists of things inside parentheses

;; Namespace
(ns clojure-sample
  (:require [clojure.string :as str]))

;; You call java methods using the . shorthand
(java.util.Date.)

;; Literals
; Numbers
42      ; integer
6.022e23 ; float
22/7    ; rational
0xff    ; hex
0b1011  ; binary

; Strings are always double-quoted
"Hello world"

; Characters are preceded by backslashes
\g \r \a \c \e

; Keywords start with colons
:a :b :c

; Symbols are used to name variables and functions
'a 'b 'c

; Vectors are linear collections
[1 2 3 4]

; Lists are linked-list data structures
'(1 2 3 4)

; Maps are associative data structures
{:name "John" :age 30}

; Sets are collections of unique values
#{1 2 3}

;; Functions
; Functions are called like this:
(str "Hello" " " "World") ; => "Hello World"

; Math is straightforward
(+ 1 1) ; => 2
(- 2 1) ; => 1
(* 1 2) ; => 2
(/ 2 1) ; => 2

; Equality is =
(= 1 1) ; => true
(= 2 1) ; => false

; You need not for logic, too
(not true) ; => false

; Nesting forms works as you expect
(+ 1 (- 3 2)) ; = 1 + (3 - 2) = 2

; Collections
; Lists are linked-list data structures
'(+ 1 2) ; => (+ 1 2)
; (you'd get back the unevaluated list)

; Vectors and lists are java classes too!
(class [1 2 3]) ; => clojure.lang.PersistentVector
(class '(1 2 3)) ; => clojure.lang.PersistentList

; A list would be written as just (1 2 3), but we have to quote it
; to stop the reader thinking it's a function.
; Also, (list 1 2 3) is the same as '(1 2 3)

; You can have a mixed-type collection
[1 "hello" 3.14]

; Use conj to add items to the beginning of lists or the end of vectors
(conj '(1 2 3) 4) ; => (4 1 2 3)
(conj [1 2 3] 4) ; => [1 2 3 4]

; Use concat to add lists or vectors together
(concat [1 2] '(3 4)) ; => (1 2 3 4)

; Use filter, map to interact with collections
(map inc [1 2 3]) ; => (2 3 4)
(filter even? [1 2 3]) ; => (2)

; Use reduce to reduce collections
(reduce + [1 2 3 4])
; = (+ (+ (+ 1 2) 3) 4)
; => 10

; Reduce can take an initial-value argument too
(reduce conj [] '(3 2 1))
; = (conj (conj (conj [] 3) 2) 1)
; => [3 2 1]

;; Functions
; Use fn to create new functions. A function always returns its last statement.
(fn [] "Hello World") ; => fn

; (You need extra parens to call it)
((fn [] "Hello World")) ; => "Hello World"

; You can create a var using def
(def x 1)
x ; => 1

; Assign a function to a var
(def hello-world (fn [] "Hello World"))
(hello-world) ; => "Hello World"

; You can shorten this process by using defn
(defn hello-world [] "Hello World")

; The [] is the list of arguments for the function.
(defn hello [name]
  (str "Hello " name))
(hello "Steve") ; => "Hello Steve"

; You can also use this shorthand to create functions:
(def hello2 #(str "Hello " %1))
(hello2 "Fanny") ; => "Hello Fanny"

; You can have multi-variadic functions, too
(defn hello3
  ([] "Hello World")
  ([name] (str "Hello " name)))
(hello3 "Jake") ; => "Hello Jake"
(hello3) ; => "Hello World"

; Functions can pack extra arguments up in a seq for you
(defn count-args [& args]
  (str "You passed " (count args) " args: " args))
(count-args 1 2 3) ; => "You passed 3 args: (1 2 3)"

; You can mix regular and packed arguments
(defn hello-count [name & args]
  (str "Hello " name ", you passed " (count args) " extra args"))
(hello-count "Finn" 1 2 3)
; => "Hello Finn, you passed 3 extra args"

;; Maps
; Hash maps and array maps share an interface. Hash maps have faster lookups
; but don't retain key order.
(class {:a 1 :b 2 :c 3}) ; => clojure.lang.PersistentArrayMap
(class (hash-map :a 1 :b 2 :c 3)) ; => clojure.lang.PersistentHashMap

; Array maps will automatically become hash maps through most operations
; if they get big enough, so you don't need to worry.

; Maps can use any hashable type as a key, but usually keywords are best
; Keywords are like strings but more efficient
(class :a) ; => clojure.lang.Keyword

(def stringmap {"a" 1, "b" 2, "c" 3})
stringmap  ; => {"a" 1, "b" 2, "c" 3}

(def keymap {:a 1, :b 2, :c 3})
keymap ; => {:a 1, :c 3, :b 2}

; By the way, commas are always treated as whitespace and do nothing.

; Retrieve a value from a map by calling it as a function
(stringmap "a") ; => 1
(keymap :a) ; => 1

; Keywords can be used to retrieve their value from a map, too!
(:b keymap) ; => 2

; Don't try this with strings.
;("a" stringmap)
; => Exception: java.lang.String cannot be cast to clojure.lang.IFn

; Retrieving a non-present key returns nil
(stringmap "d") ; => nil

; Use assoc to add new keys to hash-maps
(def newkeymap (assoc keymap :d 4))
newkeymap ; => {:a 1, :b 2, :c 3, :d 4}

; But remember, clojure types are immutable!
keymap ; => {:a 1, :b 2, :c 3}

; Use dissoc to remove keys
(dissoc keymap :a :b) ; => {:c 3}

;; Sets
#{1 2 3} ; => #{1 2 3}

; Add a member with conj
(conj #{1 2 3} 4) ; => #{1 2 3 4}

; Remove one with disj
(disj #{1 2 3} 1) ; => #{2 3}

; Test for existence by using the set as a function:
(#{1 2 3} 1) ; => 1
(#{1 2 3} 4) ; => nil

; There are more functions in the clojure.sets namespace.

;; Control Flow
; Use if for conditional logic
(if false "a" "b") ; => "b"
(if false "a") ; => nil

; Use let to create temporary bindings
(let [a 1 b 2]
  (> a b)) ; => false

; Group statements together with do
(do
  (print "Hello")
  "World") ; => "World" (prints "Hello")

; Functions have an implicit do
(defn print-and-say-hello [name]
  (print "Saying hello to " name)
  (str "Hello " name))
(print-and-say-hello "Jeff") ; => "Hello Jeff" (prints "Saying hello to Jeff")

; So does let
(let [name "Urkel"]
  (print "Saying hello to " name)
  (str "Hello " name)) ; => "Hello Urkel" (prints "Saying hello to Urkel")

;; Threading Macros
; The -> macro threads values through forms by inserting each into the second position
(->  {:a 1 :b 2} 
     (assoc :c 3) 
     (dissoc :b)) ; => {:a 1, :c 3}

; The ->> macro threads values through forms by inserting each into the last position
(->> [1 2 3 4]
     (map inc)
     (filter even?)) ; => (2 4)

;; Higher-order functions
; Use partial to create new functions from existing ones
(def add-to-hundred (partial + 100))
(add-to-hundred 50) ; => 150

; Use comp to compose functions
(def neg-square-sum (comp - (partial apply +) (partial map #(* % %))))
(neg-square-sum [1 2 3]) ; => -14

;; Atoms
; Create an atom using atom and update it with swap!
(def my-atom (atom {}))

; Update an atom with swap!
(swap! my-atom assoc :a 1) ; => {:a 1}

; Access the value of an atom with deref or @
@my-atom ; => {:a 1}
(deref my-atom) ; => {:a 1}

;; Exception handling
(try
  (/ 2 0)
  (catch ArithmeticException e
    "Cannot divide by zero!"))
; => "Cannot divide by zero!"

;; Sequences
; Sequences are a logical list interface that many data structures implement
(seq [1 2 3]) ; => (1 2 3)
(seq "abc") ; => (\a \b \c)

; first and rest work on sequences
(first [1 2 3]) ; => 1
(rest [1 2 3]) ; => (2 3)

; cons adds an item to the beginning of a list or vector
(cons 0 [1 2 3]) ; => (0 1 2 3)

;; Lazy sequences
; Clojure supports lazy sequences
(range 5) ; => (0 1 2 3 4)
(range) ; => (0 1 2 3 4 5 6 7 8 9 ...)  ; infinite sequence

; Take and drop work with lazy sequences
(take 5 (range)) ; => (0 1 2 3 4)
(drop 5 (range 10)) ; => (5 6 7 8 9)

;; Java Interop
; You can call static methods
(System/currentTimeMillis)

; Use / to call static methods
(/ (System/currentTimeMillis) 1000)

; Use . to call instance methods. Or, use the .method shortcut
(. (java.util.Date.) getTime)
(.getTime (java.util.Date.))

; Use doto to make dealing with (mutable) classes more tolerable
(doto (new java.util.HashMap) (.put "a" 1) (.put "b" 2))

;; STM (Software Transactional Memory)
; Create a ref
(def my-ref (ref 0))

; Update it transactionally
(dosync
  (alter my-ref inc)
  (alter my-ref inc))

@my-ref ; => 2

;; Destructuring
; You can destructure arrays and lists
(let [[a b c] [1 2 3]]
  (str a b c)) ; => "123"

; And maps
(let [{:keys [name age]} {:name "John" :age 30}]
  (str name " is " age " years old")) ; => "John is 30 years old"

;; Macros
; Define a macro using defmacro
(defmacro infix [infixed]
  (list (second infixed) (first infixed) (last infixed)))

; Macros can use syntax-quoting and unquoting
(defmacro unless [pred a b]
  `(if (not ~pred) ~a ~b))

(unless false 1 2) ; => 1

;; Protocols and Records
; Define a protocol
(defprotocol Drawable
  (draw [shape] "Draw the shape"))

; Define a record
(defrecord Circle [radius])

; Extend the protocol to the record
(extend-type Circle
  Drawable
  (draw [circle]
    (str "Drawing a circle with radius " (:radius circle))))

(draw (Circle. 5)) ; => "Drawing a circle with radius 5"

;; Multi-methods
; Define a multi-method based on the class of the first argument
(defmulti area :shape)

(defmethod area :circle [shape]
  (* Math/PI (* (:radius shape) (:radius shape))))

(defmethod area :rectangle [shape]
  (* (:width shape) (:height shape)))

(area {:shape :circle :radius 5}) ; => 78.53981633974483
(area {:shape :rectangle :width 4 :height 3}) ; => 12

;; Regular expressions
(re-find #"hello" "hello world") ; => "hello"
(re-matches #"hello" "hello") ; => "hello"
(re-seq #"\d+" "a1b2c3") ; => ("1" "2" "3")

;; String manipulation
(str/upper-case "hello") ; => "HELLO"
(str/lower-case "WORLD") ; => "world"
(str/split "a,b,c" #",") ; => ["a" "b" "c"]
(str/join ", " ["a" "b" "c"]) ; => "a, b, c"

;; File I/O
(spit "test.txt" "Hello World")
(slurp "test.txt") ; => "Hello World"

;; Working with time
(def now (java.time.LocalDateTime/now))
(.toString now)

;; Error handling with try-catch
(try
  (Integer. "not-a-number")
  (catch NumberFormatException e
    (str "Could not parse number: " (.getMessage e))))
; => "Could not parse number: For input string: \"not-a-number\""

;; Transducers
(def xform (comp (map inc) (filter even?)))
(transduce xform + [1 2 3 4 5]) ; => 12

;; Core.async (if available)
; (require '[clojure.core.async :as async])
; (def ch (async/chan))
; (async/go (async/>! ch "hello"))
; (async/<!! ch) ; => "hello"