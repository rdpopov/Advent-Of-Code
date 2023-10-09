let inp_test = "input_test"
let inp_goal = 2020

let readFileList fname : string list =
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

let asNumbers lst = List.map int_of_string lst

let rec print_rec lst =
  match lst with
  | [] -> Printf.printf "\n"
  | x :: xs ->
    Printf.printf "%d " x;
    print_rec xs
;;

let tail lst : 'a list =
  match lst with
  | [] -> []
  | _ :: xs -> xs
;;

let head lst =
  match lst with
  | [] -> 0
  | x :: _ -> x
;;

let rec fnd_n_len len acclist numbers left =
  if left = 0 && List.length acclist = len
  then acclist
  else if left <= 0 || List.length acclist = len || List.length numbers = 0
  then []
  else
    List.append
      (fnd_n_len len acclist (tail numbers) left)
      (fnd_n_len len (head numbers :: acclist) (tail numbers) (left - head numbers))
;;

let fnd_pairs numbers = fnd_n_len 2 [] numbers inp_goal
let fnd_trips numbers = fnd_n_len 3 [] numbers inp_goal
let input_nums = readFileList inp_test |> asNumbers;;

fnd_pairs input_nums |> print_rec;;
fnd_trips input_nums |> print_rec;;
Printf.printf "Part1 %d\n" (List.fold_right ( * ) (fnd_pairs input_nums) 1);;
Printf.printf "Part2 %d\n" (List.fold_right ( * ) (fnd_trips input_nums) 1)
