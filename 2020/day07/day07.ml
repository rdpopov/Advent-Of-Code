let input = "./input"
let test = "./test"
let test2 = "./test2"

module StringSet = Set.Make (String)

type bag =
  { name : string
  ; content : (string * int) list
  }

let concatList = String.concat " "

let toBagContent lst =
  match List.length lst with
  | 3 -> concatList (List.tl lst), int_of_string (List.hd lst)
  | _ -> "", 0
;;

let splitFlatten ch str =
  List.map (String.split_on_char ch) str
  |> List.flatten
  |> List.filter (fun x -> String.length x != 0)
;;

let parseInputLine str =
  let inpLine = splitFlatten '.' [ str ] |> splitFlatten ',' |> splitFlatten ' ' in
  let rec loop lst localacc acc =
    match lst with
    | "bags" :: xs | "bag" :: xs -> loop xs [] (List.rev localacc :: acc)
    | "contain" :: xs | "no" :: xs | "other" :: xs -> loop xs localacc acc
    | x :: xs -> loop xs (x :: localacc) acc
    | [] -> List.rev acc |> List.filter (fun x -> 0 < List.length x)
  in
  let parsed = loop inpLine [] [] in
  { name = concatList (List.hd parsed); content = List.map toBagContent (List.tl parsed) }
;;

let parseInputFile fname =
  let ic = open_in fname in
  let try_read () =
    try Some (input_line ic) with
    | End_of_file -> None
  in
  let rec loop acc =
    match try_read () with
    | Some x -> loop (parseInputLine x :: acc)
    | None -> acc
  in
  loop []
;;

let bagContains crit bag =
  let sub_bags_check () =
    List.map (fun x -> StringSet.mem (fst x) crit) bag.content
    |> List.fold_left ( || ) false
  in
  match StringSet.find_opt bag.name crit with
  | Some x -> false
  | None -> sub_bags_check ()
;;

let print_set x = StringSet.to_seq x |> List.of_seq |> List.length

let findAllContaining (initial : string) container =
  let crit = StringSet.singleton initial in
  let rec loop new_crit =
    let canContian =
      List.filter (bagContains new_crit) container |> List.map (fun x -> x.name)
    in
    match canContian with
    | [] -> new_crit
    | _ -> loop (StringSet.add_seq (List.to_seq canContian) new_crit)
  in
  loop crit |> StringSet.remove initial |> print_set
;;

let part1 fname = parseInputFile fname |> findAllContaining "shiny gold"
let _ = assert (part1 test = 4);;

part1 input

let toHashMap map bag_struct = Hashtbl.add map bag_struct.name bag_struct

let foldByHashmap tbl start =
  let start_bag = Hashtbl.find tbl start in
  let rec traverse crnt_bag tb =
    match crnt_bag.content with
    | [] -> 0
    | _ ->
      List.map
        (fun x ->
          let bg = Hashtbl.find tbl (fst x) in
          let res = snd x + (snd x * traverse bg (String.cat tb "\t")) in
          Printf.printf "%s%s -> %d\n" tb (fst x) res;
          res)
        crnt_bag.content
      |> List.fold_left ( + ) 0
  in
  traverse start_bag ""
;;

let part2 fname =
  let tbl : (string, bag) Hashtbl.t = Hashtbl.create 0 in
  let _ = parseInputFile fname |> List.map (toHashMap tbl) in
  foldByHashmap tbl "shiny gold"
;;

part2 test;;
part2 test2;;
part2 input
