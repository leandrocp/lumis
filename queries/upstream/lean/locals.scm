; Scope-aware queries for tree-sitter-lean.

; ---------- scopes -------------------------------------------------------

[
  (def)
  (theorem)
  (example)
  (abbrev)
  (instance)
  (axiom)
  (opaque)
  (constant)
  (structure)
  (inductive)
  (fun)
  (let)
  (have)
  (forall)
  (exists)
  (match)
  (match_alt)
  (by)
  (do_block)
  (namespace)
  (section)
] @local.scope

; ---------- definitions --------------------------------------------------

(def       name: (identifier) @local.definition.function)
(theorem   name: (identifier) @local.definition.function)
(abbrev    name: (identifier) @local.definition.function)
(opaque    name: (identifier) @local.definition.function)
(constant  name: (identifier) @local.definition.function)
(axiom     name: (identifier) @local.definition.function)
(instance  name: (identifier) @local.definition.function)
(structure name: (identifier) @local.definition.type)
(inductive name: (identifier) @local.definition.type)

(field     name: (identifier) @local.definition.field)
(ctor      name: (identifier) @local.definition.constructor)
(ctor_alt  name: (identifier) @local.definition.constructor)

(namespace name: (identifier) @local.definition.namespace)

(let     name: (identifier) @local.definition.var)
(have    name: (identifier) @local.definition.var)
(named_hole (identifier) @local.definition.var)

(explicit_binder        name: (identifier) @local.definition.parameter)
(implicit_binder        name: (identifier) @local.definition.parameter)
(strict_implicit_binder name: (identifier) @local.definition.parameter)
(instance_binder        name: (identifier) @local.definition.parameter)

; ---------- references ---------------------------------------------------

(identifier) @local.reference
