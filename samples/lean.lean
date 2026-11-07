import Std.Data.HashMap
open Std (HashMap)

/-!
# Binary trees

A small module showing inductive types, structures, type classes,
pattern matching, `do` notation and a proof by induction.
-/

namespace Tree

/-- A binary tree with values at the nodes. -/
inductive Tree (α : Type u) where
  | leaf : Tree α
  | node (left : Tree α) (value : α) (right : Tree α) : Tree α
  deriving Repr, BEq

variable {α : Type u}

/-- Number of nodes in the tree. -/
def size : Tree α → Nat
  | .leaf => 0
  | .node l _ r => size l + 1 + size r

def insert [Ord α] (t : Tree α) (x : α) : Tree α :=
  match t with
  | .leaf => .node .leaf x .leaf
  | .node l v r =>
    match compare x v with
    | .lt => .node (insert l x) v r
    | .gt => .node l v (insert r x)
    | .eq => t

def toList : Tree α → List α
  | .leaf => []
  | .node l v r => toList l ++ [v] ++ toList r

theorem size_toList (t : Tree α) : (toList t).length = size t := by
  induction t with
  | leaf => rfl
  | node l v r ihl ihr =>
    simp [toList, size, ihl, ihr]
    omega

end Tree

structure Point where
  x : Float
  y : Float
  deriving Repr

instance : Add Point where
  add p q := { x := p.x + q.x, y := p.y + q.y }

class Shape (σ : Type) where
  area : σ → Float

instance : Shape Point where
  area _ := 0.0

def wordCounts (words : List String) : HashMap String Nat := Id.run do
  let mut counts : HashMap String Nat := {}
  for w in words do
    counts := counts.insert w (counts.getD w 0 + 1)
  return counts

def main : IO Unit := do
  let t := [5, 3, 8, 1, 4].foldl Tree.insert Tree.Tree.leaf
  IO.println s!"sorted: {Tree.toList t}, size: {Tree.size t}"
  let p : Point := ⟨1.5, -2.0⟩ + ⟨0.5, 4.0⟩
  IO.println s!"point: {repr p}"
  if (wordCounts ["a", "b", "a"]).getD "a" 0 == 2 then
    IO.println "ok"
  else
    throw <| IO.userError "unexpected count"

#eval Tree.size (Tree.insert .leaf 'c')
#check @Tree.size_toList
