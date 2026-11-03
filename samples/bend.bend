from ./lib/math import sqrt

# A binary tree with values on its leaves
type MyTree:
  Node { ~left, ~right }
  Leaf { val }

object Point { x, y }

#{
  Sums every leaf in parallel: both recursive calls
  are independent, so HVM evaluates them at the same time.
#}
def sum(depth, x):
  switch depth:
    case 0:
      return x
    case _:
      fst = sum(depth-1, x*2+0)
      snd = sum(depth-1, x*2+1)
      return fst + snd

def tree_sum(tree):
  fold tree:
    case MyTree/Node:
      return tree.left + tree.right
    case MyTree/Leaf:
      return tree.val

def gen(n):
  bend x = 0:
    when x < n:
      tree = MyTree/Node { left: fork(x + 1), right: fork(x + 1) }
    else:
      tree = MyTree/Leaf { val: 7 }
  return tree

def classify(n):
  if n == 0:
    return "zero"
  elif n < 0:
    return "negative"
  else:
    return "positive"

def norm(p):
  open Point: p
  with IO:
    * <- IO/print("norm\n")
  return sqrt(p.x ** 2.0 + p.y ** 2.0)

# Pattern-matching (fun) syntax
type Bool = True | False

Not True  = False
Not False = True

Len [] = 0
Len (List/Cons x xs) = (+ 1 (Len xs))

Map f list = match list {
  List/Cons: (List/Cons (f list.head) (Map f list.tail))
  List/Nil: List/Nil
}

hvm prim_add:
  (a (b c)) & a ~ $(b c)

def main():
  xs = [1, 2, 3]
  inc = lambda x: x + 1
  chars = ['a', '\n', 'λ']
  return (sum(20, 0), tree_sum(gen(4)), Len(Map(inc, xs)), classify(-3), 3.14)
