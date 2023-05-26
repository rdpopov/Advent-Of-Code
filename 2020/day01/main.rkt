#lang racket

(define inp-file "./input")
(define (task-input fname) (map string->number (file->lines fname)))
(define (find-n-sum-impl lst sm crnt n)
  (cond
    [(empty? lst) '() ]
    [(and (= sm 0) (= n 0)) crnt]
    [(> n 0) (append
                (find-n-sum-impl (rest lst) (- sm (car lst)) (append crnt (list (car lst)) ) (- n 1))
                (find-n-sum-impl (rest lst) sm crnt n)
           )
        ]
    [true '()]
    )
  )

(define (find-n-sum lst n) (find-n-sum-impl lst 2020 (list) n))

(printf "Part 1 ~a\nPart 2 ~a"
    (apply * (find-n-sum (task-input inp-file) 2))
    (apply * (find-n-sum (task-input inp-file) 3))
    )
