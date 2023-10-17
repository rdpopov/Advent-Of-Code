let input = "./input"
let test = "./test"

let parseToList fname =
  let ic = open_in fname in
  let try_read () =
    try Some (input_line ic) with
    | End_of_file -> None
  in
  let rec loop acc =
    match try_read () with
    | None -> List.rev acc
    | Some x -> loop (x :: acc)
  in
  loop []
;;

let toBinRepr (s : string) =
  let toBit acc ch =
    match ch with
    | 'R' | 'B' -> (acc * 2) + 1
    | 'F' | 'L' -> acc * 2
    | _ -> acc * 2
  in
  String.fold_left toBit 0 s
;;

parseToList test |> List.map toBinRepr |> List.fold_left max 0

let part1 () = parseToList input |> List.map toBinRepr |> List.fold_left max 0;;

part1 ()

let zipList l1 l2 =
  let rec ziplst l1 l2 acc =
    match l1, l2 with
    | hl1 :: tl1, hl2 :: tl2 -> ziplst tl1 tl2 ((hl1, hl2) :: acc)
    | _ -> List.rev acc
  in
  ziplst l1 l2 []
;;

let part2 () =
  let sorted = parseToList input |> List.map toBinRepr |> List.sort Int.compare in
  let prev =
    zipList sorted (List.tl sorted)
    |> List.filter (fun x -> fst x - snd x == -2)
    |> List.hd
    |> fst
  in
  prev + 1
;;

part2 ()
