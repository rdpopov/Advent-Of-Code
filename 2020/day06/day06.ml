let input = "./input"
let test = "./test"

let parseToList fname =
  let ic = open_in fname in
  let try_read () =
    try Some (input_line ic) with
    | End_of_file -> None
  in
  let rec loop acc =
    match try_read (), acc with
    | None, _ -> List.rev acc
    | Some "", _ -> loop ([] :: acc)
    | Some x, [] -> loop ((x :: []) :: acc)
    | Some x, _ ->
      let crnt = List.hd acc in
      loop ((x :: crnt) :: List.tl acc)
  in
  loop []
;;

parseToList test

let uniqueSymbols (lst : string list) =
  List.map (fun x -> String.to_seq x |> List.of_seq) lst
  |> List.flatten
  |> List.sort_uniq Char.compare
;;

let part1 fname =
  parseToList fname
  |> List.map uniqueSymbols
  |> List.map List.length
  |> List.fold_left ( + ) 0
;;

let _ = assert (11 = part1 test);;

Printf.printf "%d\n" (part1 input)

let zipList l1 l2 =
  let rec ziplst l1 l2 acc =
    match l1, l2 with
    | hl1 :: tl1, hl2 :: tl2 -> ziplst tl1 tl2 ((hl1, hl2) :: acc)
    | _ -> List.rev acc
  in
  ziplst l1 l2 []
;;

(* gets a list of characters and unique strings
   iterates over all the strings and creates a bitmask
   weather each of the strings contains each character of the first tuple element
   then we 'and' the masks and sum them
*)
let maskOfTup tup =
  let unq = fst tup in
  let toCheck = snd tup in
  let rec loopCreateMasks src acc =
    match src with
    | x :: xs ->
      let msk = List.map (String.contains x) unq in
      (* partial function
         application *)
      loopCreateMasks xs (msk :: acc)
    | [] -> acc
  in
  loopCreateMasks toCheck []
;;

(* ands two lists as if they are bits of numbers *)
let and2List l1 l2 =
  let rec loop l1 l2 acc =
    match l1, l2 with
    | x1 :: xs1, x2 :: xs2 -> loop xs1 xs2 ((x1 && x2) :: acc)
    | x1 :: xs1, [] ->
      loop xs1 [] (false :: acc)
      (* just like with numbers
         mismach in lenght, that wopuld just cut off all the bits.*)
    | [], x2 :: xs2 -> loop [] xs2 (false :: acc)
    | [], [] -> acc
  in
  loop (List.rev l1) (List.rev l2) []
;;

(* if lists mismatch in length ensure
   least significatn bits are first, jsut like numbers *)

(*
   ands the whole list with a initial mask full of 'true' the length of the each
   mask in the list*)
let foldListsWith fn lst =
  let initial = List.init (List.length (List.hd lst)) (fun _ -> true) in
  (* create initial msk the length of the first elemtn of the list *)
  List.fold_left fn initial lst
;;

let part2 fname =
  let inputList = parseToList fname in
  let zipped =
    zipList
      (List.map uniqueSymbols inputList)
      (* 1. we extract the unique symbols of the input list*)
      (List.map (fun x -> List.sort_uniq String.compare x) inputList)
    (*2. we extract all the unique strings of the list of answers, ["a","a"] becomes just ["a"]*)
    |> List.map maskOfTup
    (* 3. turns all the strigns in a mask the length of the unique symbols, checking if the string has those.*)
  in
  List.map (foldListsWith and2List) zipped (* partial function application *)
  |> List.flatten
     (* the list is flattened, as the index of each mask is
        meaningless now and we can just sum them*)
  |> List.map (fun x -> if x then 1 else 0)
    (* cant sum bools like in apl have to map them first :( *)
  |> List.fold_left ( + ) 0
;;

let _ = assert (6 = part2 test);;

Printf.printf "%d\n" (part2 input)
