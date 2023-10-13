let input = "./input"
let test = "./test"

let emptyHead (lst : string list) =
  match lst with
  | [] -> ""
  | x -> List.hd lst
;;

let emptyTail (lst : string list) =
  match lst with
  | [] -> []
  | x :: [] -> []
  | x :: xs -> xs
;;

let parseToList fname =
  let ic = open_in fname in
  let try_read () =
    try Some (input_line ic) with
    | End_of_file -> None
  in
  let rec loop lst =
    match try_read () with
    | Some "" -> loop ("" :: lst)
    | Some x -> loop ((emptyHead lst ^ " " ^ x) :: emptyTail lst)
    | None -> List.rev lst
  in
  loop []
;;

let print lst =
  let rec loop l =
    match l with
    | [] -> Printf.printf ")\n"
    | x :: xs ->
      Printf.printf "%s\n" x;
      loop xs
  in
  loop lst
;;

let listTakeTwo lst = List.init (List.length lst / 2) (fun x -> List.nth lst (x * 2))

let listSplit (passStr : string) =
  String.split_on_char ' ' passStr
  |> List.map (fun s -> String.split_on_char ':' s)
  |> List.flatten
  |> List.filter (fun s -> String.length s > 0)
;;

let listBundleInTup lst =
  List.init
    (List.length lst / 2)
    (fun x -> List.nth lst (x * 2), List.nth lst ((x * 2) + 1))
;;

let lablesPart1 = [ "byr"; "iyr"; "eyr"; "hgt"; "hcl"; "ecl"; "pid"; "cid" ]

let checkLables lbls =
  let is_in lbl =
    match List.find_index (fun x -> x = lbl || lbl = "cid") lbls with
    | Some x -> 1
    | None -> 0
  in
  List.map is_in lablesPart1 |> List.fold_left ( * ) 1
;;

let listEligable tup_lst =
  let lbls = List.init (List.length tup_lst) (fun x -> List.nth tup_lst x |> fst) in
  checkLables lbls
;;

let part1 fname =
  parseToList fname
  |> List.map (fun x -> listSplit x |> listBundleInTup |> listEligable)
  |> List.fold_left ( + ) 0
;;

let _ = assert (part1 test = 2);;

part1 input

(* all needed for part 2 *)

let suff s = String.sub s (String.length s - 2) 2
let pref s = String.sub s 0 (String.length s - 2)

let validateHeight s =
  match suff s with
  | "in" ->
    let inch =
      try int_of_string (pref s) with
      | Failure _ -> 0
    in
    59 <= inch && inch <= 76
  | "cm" ->
    let cm =
      try int_of_string (pref s) with
      | Failure _ -> 0
    in
    150 <= cm && cm <= 193
  | _ -> false
;;

(* map the string values to tselves
   if they are correct and in the correct place,
   otherwise otherwise they become ' ' which is an impossible character
   (* ' ' is impossible to be there as string is separated by ' ' in the steps  before *)
   this forms a new string.
   compare the original string with the result form the mapping
*)

let validateHairColor s =
  let check idx ch =
    match idx, ch with
    | 0, x -> if x = '#' then x else ' '
    | _, x -> if ('0' <= x && x <= '9') || ('a' <= x && x <= 'f') then x else ' '
    (* | _ -> ' ' *)
  in
  s = String.mapi check s
;;

let validateEyeColor s =
  let acceptable = [ "amb"; "blu"; "brn"; "gry"; "grn"; "hzl"; "oth" ] in
  match List.find_index (fun x -> x = s) acceptable with
  | Some _ -> true
  | None -> false
;;

let checkListTuples lst =
  let check tup =
    match tup with
    | "byr", x ->
      let x = int_of_string x in
      1920 <= x && x <= 2002
    | "iyr", x ->
      let x = int_of_string x in
      2010 <= x && x <= 2020
    | "eyr", x ->
      let x = int_of_string x in
      2020 <= x && x <= 2030
    | "hgt", x -> validateHeight x
    | "hcl", x -> validateHairColor x
    | "ecl", x -> validateEyeColor x
    | "pid", x -> String.length x = 9
    | "cid", x -> true
    | _ -> false
  in
  List.map (fun x -> if check x then 1 else 0) lst |> List.fold_left ( * ) 1
;;

let checkPassport tup_lst =
  if listEligable tup_lst == 1 then checkListTuples tup_lst else 0
;;

let part2 fname =
  parseToList fname
  |> List.map (fun x -> listSplit x |> listBundleInTup |> checkPassport)
  |> List.fold_left ( + ) 0
;;

part2 input
