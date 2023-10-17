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
  |> List.length
;;

let _ =
  assert (11 = (parseToList test |> List.map uniqueSymbols |> List.fold_left ( + ) 0))
;;

let part1 () = parseToList input |> List.map uniqueSymbols |> List.fold_left ( + ) 0;;

part1 ()
