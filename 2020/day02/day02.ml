let test = "./input_test"
let input = "./input"

let readLines fname =
  let ic = open_in fname in
  let try_read () =
    try Some (input_line ic) with
    | End_of_file -> None
  in
  let rec loop acc =
    match try_read () with
    | Some s -> loop (s :: acc)
    | None ->
      close_in ic;
      List.rev acc
  in
  loop []
;;

let printList lst =
  Printf.printf "(";
  let rec loop l =
    match l with
    | [] -> Printf.printf ")\n"
    | x :: xs ->
      print_endline x;
      loop xs
  in
  loop lst
;;

let bySpc s = String.split_on_char ' ' s
let byCol s = String.split_on_char ':' s
let byMns s = String.split_on_char '-' s

let parseString s =
  let isNotEmpty x = 0 != String.compare x "" in
  bySpc s
  |> List.map byCol
  |> List.flatten
  |> List.map byMns
  |> List.flatten
  |> List.filter isNotEmpty
;;

let explode s = List.init (String.length s) (String.get s)

let count_in s chr =
  let mask = explode s |> List.map (fun x -> if x = chr then 1 else 0) in
  List.fold_right ( + ) mask 0
;;

let validatePasswordCount pass =
  let parsed = parseString pass in
  let lower = List.nth parsed 0 |> int_of_string in
  let upper = List.nth parsed 1 |> int_of_string in
  let chr = (List.nth parsed 2).[0] in
  let pass = List.nth parsed 3 in
  let count = count_in pass chr in
  if lower <= count && count <= upper then true else false
;;

let xor a b = (a || b) && not (a && b)

let validatePasswordPlace pass =
  let parsed = parseString pass in
  let lower = List.nth parsed 0 |> int_of_string in
  let upper = List.nth parsed 1 |> int_of_string in
  let chr = (List.nth parsed 2).[0] in
  let pass = List.nth parsed 3 in
  (* Printf.printf "%s %d %d %d\n" pass (String.length pass) lower upper; *)
  if xor (pass.[lower - 1] = chr) (pass.[upper - 1] = chr) then true else false
;;

let validInFile fname fn =
  let mask = readLines fname |> List.map (fun pass -> if fn pass then 1 else 0) in
  List.fold_right ( + ) mask 0
;;

let _ = assert (validInFile test validatePasswordCount = 2)
let _ = assert (validInFile test validatePasswordPlace = 1);;

validInFile input validatePasswordCount |> Printf.printf "Valid in input %d\n";;
validInFile input validatePasswordPlace |> Printf.printf "Valid in input %d\n"
