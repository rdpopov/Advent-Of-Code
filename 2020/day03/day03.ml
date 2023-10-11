let test = "test"
let input = "input"

let parseToList fname =
    let ic = open_in fname in 
    let try_read () =
        try Some(input_line ic) with End_of_file -> None in
    let rec loop acc =
        match try_read() with
        | Some (s) -> loop (s :: acc)
        | None -> close_in ic; List.rev acc;
    in loop []
;;

let printList arg =
    Printf.printf "(\n";
    let rec loop lst =
        match lst with 
        | [] -> Printf.printf ")\n"
        | x::xs -> Printf.printf "%s\n" x; loop xs
    in loop arg
;;

let collumns x =
    List.init x (fun x -> x)
;;

let takeEach lst y =
    List.init ((List.length lst) / y) (fun i -> List.nth lst (i*y))
;;

let positionsToCheck dx dy field =
    let x = List.hd field |> String.length in
    let y = List.length field  in
    List.init y (fun idx -> (dx +( idx * 3)) mod x)
;;

let gettrees dx dy field =
    let lines = takeEach field dy in
    let indexes =(positionsToCheck dx dy field) in
    let mask = List.map2 (fun line idx -> if line.[idx] = '#' then 1 else 0) lines indexes in
    List.fold_right ( + ) mask 0
;;

(* let treesproduct field = *)
(*     let lcols = List.hd field |> String.length |> collumns in *)
(*     let res = List.map (fun col -> gettrees col field) lcols in *)
(*     res *)

    (* List.fold_right ( * ) res 1 ;; *)

parseToList test |> gettrees 1 3
let _ = assert (parseToList test |> gettrees 3 1 = 7);;
(* let _ = assert (parseToList test |> treesproduct = 336);; *)

(* parseToList test |> treesproduct *)

(* let part1 fname = *)
(*     parseToList input |> gettrees 0 |> Printf.printf "Part 1: Trees to hit %d\n";; *)

(* let part2 fname = *)
(*     parseToList input |> treesproduct |> Printf.printf "Part 2: Trees product %d\n";; *)


