let test = "./test"
let input = "./input"

type execStatus =
  | Fin of int
  | Loop of int

let lineToInstruction line =
  let spl = String.split_on_char ' ' line in
  match spl with
  | x :: xs -> x, int_of_string (List.hd xs)
  | _ -> "", 0
;;

let parse_to_list fname =
  let ic = open_in fname in
  let read_opt () =
    try Some (input_line ic) with
    | End_of_file -> None
  in
  let rec loop acc =
    match read_opt () with
    | Some x -> loop (lineToInstruction x :: acc)
    | None -> acc
  in
  List.rev (loop [])
;;

module IntSet = Set.Make (Int)

let checkIfInf instr =
  let passed = IntSet.empty in
  let delta_ip ip acc =
    let res =
      match List.nth instr ip with
      | "nop", _ -> ip + 1, acc
      | "jmp", x -> ip + x, acc
      | "acc", x -> ip + 1, acc + x
      | _ -> ip, acc
    in
    (* Printf.printf "%s %d\n" (fst (List.nth instr ip)) (snd (List.nth instr ip)); *)
    res
  in
  let rec loop (ip : int) (acc : int) st =
    match IntSet.mem ip st, ip with
    | true, _ -> Loop acc
    | false, x ->
      if x >= List.length instr
      then Fin acc
      else (
        let ns = delta_ip ip acc in
        loop (fst ns) (snd ns) (IntSet.add ip st))
  in
  loop 0 0 passed
;;

let part1 fname = parse_to_list fname |> checkIfInf

let _ =
  assert (Loop 5 = part1 test);
  part1 input
;;

(* this could probably be easier achieved with a hash map. Read something that
   insertion of new keys shadow previous ones*)
let copyListflipIdx lst idx =
  let flippedElem i =
    if i = idx
    then (
      match List.nth lst i with
      | "nop", x -> "jmp", x
      | "jmp", x -> "nop", x
      | instr, x -> instr, x)
    else List.nth lst i
  in
  List.init (List.length lst) flippedElem
;;

let fixOneSwap instr =
  let isFin x =
    match x with
    | Fin k -> true
    | _ -> false
  in
  List.mapi (fun i x -> i, x) instr
  |> List.filter (fun x -> String.compare (fst (snd x)) "acc" > 0)
  |> List.map fst
  |> List.map (copyListflipIdx instr)
  |> List.map checkIfInf
  |> List.find isFin
;;

parse_to_list test |> fixOneSwap;;
parse_to_list input |> fixOneSwap
