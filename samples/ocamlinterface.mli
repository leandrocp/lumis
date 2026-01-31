(** OCaml Interface File (.mli) *)

(** {1 Type Definitions} *)

(** The type of users in the system *)
type user = {
  id : int;
  name : string;
  email : string;
  created_at : float;
}

(** Result type for operations that may fail *)
type 'a result =
  | Ok of 'a
  | Error of string

(** Abstract type for database connections *)
type connection

(** {1 Module Types} *)

(** Signature for comparable types *)
module type Comparable = sig
  type t
  val compare : t -> t -> int
  val equal : t -> t -> bool
end

(** Signature for a key-value store *)
module type KV_STORE = sig
  type key
  type value
  type t

  val create : unit -> t
  val get : t -> key -> value option
  val set : t -> key -> value -> unit
  val delete : t -> key -> bool
  val keys : t -> key list
end

(** {1 Functions} *)

(** [create_user ~name ~email] creates a new user with the given name and email.
    @param name The user's display name
    @param email The user's email address
    @return A new user record *)
val create_user : name:string -> email:string -> user

(** [find_user_by_id conn id] looks up a user by their ID.
    @param conn Database connection
    @param id User ID to search for
    @return [Some user] if found, [None] otherwise *)
val find_user_by_id : connection -> int -> user option

(** [update_user conn user] updates an existing user in the database.
    @raise Invalid_argument if the user doesn't exist *)
val update_user : connection -> user -> unit

(** [delete_user conn id] removes a user from the database.
    @return [Ok ()] on success, [Error msg] on failure *)
val delete_user : connection -> int -> unit result

(** [list_users conn ?limit ?offset ()] retrieves users from the database.
    @param limit Maximum number of users to return (default: 100)
    @param offset Number of users to skip (default: 0) *)
val list_users : connection -> ?limit:int -> ?offset:int -> unit -> user list

(** {1 Functors} *)

(** Create a set module for any comparable type *)
module MakeSet (Elt : Comparable) : sig
  type elt = Elt.t
  type t

  val empty : t
  val add : elt -> t -> t
  val remove : elt -> t -> t
  val mem : elt -> t -> bool
  val cardinal : t -> int
end

(** {1 Exceptions} *)

exception Connection_error of string
exception Not_found of int
