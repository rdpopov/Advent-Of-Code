let input = "./input"
let test = "./test"

type bag = {
    name : string;
    content : (string * int) list
}


let concatList = (List.fold_left String.cat "") ;;

let toBagContent lst =
    match (List.length lst) with 
    | 3 -> (concatList (List.tl lst)), (int_of_string (List.hd lst))
    | _ -> ("", 0)
;;


let splitFlatten ch str =
    List.map (String.split_on_char ch) str 
    |> List.flatten 
    |> List.filter (fun x -> String.length x != 0 )

let parseInputLine str = 
    let inpLine = splitFlatten '.' [str]
    |> splitFlatten ','
    |> splitFlatten ' ' in
    let rec loop lst localacc acc =
        match lst with
        | "bags"::xs | "bag"::xs -> loop xs [] ((List.rev localacc)::acc)
        | "contain"::xs | "no"::xs | "other"::xs -> loop xs localacc acc
        | x::xs -> loop xs (x::localacc) acc
        | [] -> List.rev(acc) |> List.filter (fun x -> 0 < List.length x)
        | _ -> []
    in let parsed = loop inpLine [] [] in
        {
            name = (concatList (List.hd parsed)) ;
            content = ( List.map toBagContent (List.tl parsed)
            )
        }
    (* parsed *)
;;


let parseInputFile fname =
    let ic = open_in fname in
    let try_read () = try Some(input_line ic) with End_of_file -> None in
    let rec loop acc = 
        match try_read() with
        | Some(x) -> loop ((parseInputLine x)::acc)
        | None -> acc
    in loop [] 
;;

parseInputFile test;;

let findAllContaining criterias container= 
    

;;
(* parseInputFile input;; *)


 